"""Explicit checkpoint recovery of a stopped matrix; use build protection.

Never edits the original frozen product source or retries inside the failed run.
Completed configurations are verified and reused; failed/pending configurations
use fresh case directories. The restored host must match its original hash.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
import math

REPO = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('femacs_runtime_resume_stage', REPO / 'tools/runtime-stage.py')
stage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(stage)
execution = stage.execution


def cases():
    for product in ('terminal', 'cocoa'):
        selected = [('native-collector', True, True)]
        selected += [(variant, observer, False)
                     for variant in ('baseline', 'native', 'collector', 'native-collector')
                     for observer in (False, True)]
        selected.append(('baseline', True, True))
        for variant, observer, hooks in selected:
            name = product + '-' + variant + ('-observer' if observer else '') + ('-diagnostic' if hooks else '')
            yield dict(id=name, product=product, variant=variant, observer=observer, test_hooks=hooks)


def verify_source(original, source_export):
    return execution.verify_source_export(original, source_export)


def verify_product_files(receipt_path):
    """Verify the complete finished artifact independently of link-input retention."""
    receipt_path = execution.safe_path(receipt_path)
    root = receipt_path.parent
    value = execution.bounded_json(receipt_path, 64 * 1024 * 1024)
    _, _, resources, _ = execution.product_receipt_layout(value, root)
    if value.get('binding') != execution.bounded_json(root/'runtime-binding.json'):
        raise ValueError('completed product binding changed')
    config = Path(value['binding']['config_relative'])
    if config.is_absolute() or '..' in config.parts or execution.digest(execution.safe_path(root/config)) != value['binding']['config_digest']:
        raise ValueError('completed product configuration changed')
    if value.get('build_complete') is not True or value['link_inputs'].get('link_complete') is not True:
        raise ValueError('only complete linked products can be reused')
    if not value.get('files') or not value['link_inputs'].get('objects') or not value.get('resources'):
        raise ValueError('incomplete product identity evidence')
    for relative, expected in value['files'].items():
        path = Path(relative)
        if path.is_absolute() or '..' in path.parts or execution.digest(execution.safe_path(root / path)) != expected:
            raise ValueError('completed product changed')
    for name, expected in value['resources'].items():
        path = resources[name]
        if execution.tree_identity(path) != expected:
            raise ValueError('completed product resource changed: ' + name)
    return value


def verify_product(receipt_path):
    """New builds and ordinary reuse require every actual link input still present."""
    receipt_path = execution.safe_path(receipt_path)
    value = verify_product_files(receipt_path)
    root = receipt_path.parent
    linked = value['link_inputs']
    for relative, expected in linked['objects'].items():
        path = Path(relative)
        if path.is_absolute() or '..' in path.parts or execution.digest(execution.safe_path(root / path)) != expected['sha256']:
            raise ValueError('completed product link input changed')
    archive = Path(linked['archive']['path'])
    if archive.is_absolute() or '..' in archive.parts or execution.digest(execution.safe_path(root / archive)) != linked['archive']['sha256']:
        raise ValueError('completed product link archive changed')
    return value


class SealedTerminals:
    """Explicit historical seals authorize only their exact old terminal receipts.

    This never authorizes a new build or a Cocoa receipt. Missing intermediate
    files are inventoried, not restored or claimed currently verified. Final
    artifacts, resources, binding/config and any surviving link inputs must
    still match. A certificate produced through this path cannot become a
    historical strict seal for a later recovery.
    """

    def __init__(self, proof_path, prior_build, stopped, old_export, current_export,
                 terminals, allowed_hosts):
        proof_path = execution.safe_path(proof_path)
        proof = execution.bounded_json(proof_path)
        seals = proof.get('seals')
        if (proof.get('schema') != 'femacs-terminal-retention-input-v1'
                or not isinstance(seals, list) or len(seals) != 2):
            raise ValueError('exactly two pinned historical strict terminal seals required')
        root = execution.safe_path(old_export['source'])
        expected = {str(execution.safe_path(p['product_receipt'])) for p in terminals}
        if len(expected) != 20 or any(not Path(p).is_relative_to(root) for p in expected):
            raise ValueError('sealed terminal receipts must be the twenty original source products')
        self.receipts, self.seals, self.inventory = None, [], {}
        self.profile_hosts = None
        for pin in seals:
            sha = pin.get('sha256')
            path = execution.safe_path(pin['path'])
            if (not isinstance(sha, str) or not execution.HEX.fullmatch(sha)
                    or execution.digest(path) != sha or not path.is_relative_to(REPO/'build')
                    or str(path) in {p['path'] for p in self.seals}):
                raise ValueError('historical strict terminal seal identity changed or duplicated')
            seal = execution.bounded_json(path)
            if (seal.get('schema') != 'femacs-terminal-source-certificate-v1'
                    or seal.get('passed') is not True
                    or seal.get('link_input_validation', 'current-strict') != 'current-strict'
                    or seal.get('prior_build') != str(prior_build)
                    or seal.get('stopped_controller') != stopped
                    or seal.get('original_source') != old_export
                    or seal.get('current_source') == current_export
                    or seal.get('host_cohorts') != allowed_hosts
                    or seal.get('uniform_source_snapshot') is not False
                    or seal.get('uniform_host_cohort') != (len(allowed_hosts) == 1)
                    or seal.get('retained_profiles') != sorted(p['id'] for p in terminals)):
                raise ValueError('historical strict terminal seal provenance differs')
            receipts = seal.get('verified_receipts')
            hosts = seal.get('profile_host_provenance')
            if (not isinstance(receipts, dict) or set(receipts) != expected
                    or not isinstance(hosts, dict) or set(hosts) != {p['id'] for p in terminals}
                    or (self.receipts is not None and (receipts != self.receipts or hosts != self.profile_hosts))):
                raise ValueError('historical strict terminal seals disagree on receipts or profiles')
            for name, digest in receipts.items():
                if not isinstance(digest, str) or not execution.HEX.fullmatch(digest) or execution.digest(name) != digest:
                    raise ValueError('sealed terminal receipt changed')
            self.receipts, self.profile_hosts = receipts, hosts
            self.seals.append(dict(path=str(path), sha256=sha))
        self.proof = dict(path=str(proof_path), sha256=execution.digest(proof_path))

    def verify(self, receipt_path):
        receipt_path = execution.safe_path(receipt_path)
        name = str(receipt_path)
        if name not in self.receipts or execution.digest(receipt_path) != self.receipts[name]:
            raise ValueError('receipt is not authorized by both historical strict terminal seals')
        value = verify_product_files(receipt_path)
        if value['binding'].get('product') != 'terminal':
            raise ValueError('historical terminal seals cannot authorize Cocoa products')
        root = receipt_path.parent
        linked = value['link_inputs']
        present, missing = [], []
        for relative, identity in linked['objects'].items():
            path = Path(relative)
            if path.is_absolute() or '..' in path.parts or path.suffix != '.o':
                raise ValueError('unsafe sealed terminal object path')
            target = root/path
            # A dangling symlink is not an absent reconstructible file.
            if target.is_symlink():
                raise ValueError('sealed terminal link input is a symlink')
            execution.safe_path(target.parent)
            if target.exists():
                if execution.digest(execution.safe_path(target)) != identity['sha256']:
                    raise ValueError('present sealed terminal link input changed')
                present.append(relative)
            else:
                missing.append(relative)
        archive = Path(linked['archive']['path'])
        if archive.is_absolute() or '..' in archive.parts or archive.suffix != '.a':
            raise ValueError('unsafe sealed terminal archive path')
        target = root/archive
        if target.is_symlink():
            raise ValueError('sealed terminal link archive is a symlink')
        execution.safe_path(target.parent)
        if target.exists():
            if execution.digest(execution.safe_path(target)) != linked['archive']['sha256']:
                raise ValueError('present sealed terminal link archive changed')
            status = 'present-verified'
        else:
            status = 'historical-missing'
        if execution.digest(receipt_path) != self.receipts[name]:
            raise ValueError('sealed terminal receipt changed during verification')
        self.inventory[name] = dict(receipt_sha256=self.receipts[name],
            expected_objects=len(linked['objects']), present_verified_objects=sorted(present),
            historical_missing_objects=sorted(missing), archive=dict(linked['archive'], status=status))
        return value


def append_products(report, case, root, host_provenance=None):
    for leg in ('normal', 'checking'):
        leg_root = root if leg == 'normal' else root/'checking'
        receipt_path = leg_root/'runtime-product.json'
        if not receipt_path.exists():
            continue
        receipt = execution.bounded_json(receipt_path, 64 * 1024 * 1024)
        if receipt.get('build_complete') is not True:
            continue
        verified = verify_product(receipt_path)
        program_relative, dump_relative, _, resources = execution.product_receipt_layout(verified, leg_root)
        source_identity = verified['binding']['source_digest']
        if report.setdefault('pinned_source_digest', source_identity) != source_identity:
            raise ValueError('product source binding differs from original checkpoint')
        program, dump = leg_root/program_relative, leg_root/dump_relative
        modes = ('major', 'young') if 'collector' in case['variant'] else ('off',)
        for mode in modes:
            identity = case['id'] + '-' + leg + ('-' + mode if mode != 'off' else '')
            report['products'].append(dict(id=identity, kind=case['product'], compiler_leg=leg,
                native='native' in case['variant'], observer=case['observer'], test_hooks=case['test_hooks'],
                collector=mode, program=str(program), dump=str(dump), resource_roots=resources,
                product_receipt=str(receipt_path), host_provenance=host_provenance,
                source_cohort=report['source_export']['source_manifest_sha256']))


def carry_terminals(prior_build, failed_controller, current_export, out, reference, retention_proof=None):
    """P-363: exact terminal-inactive Cocoa repairs, never arbitrary changed sources."""
    prior_build = execution.safe_path(prior_build)
    stopped = assert_stopped(failed_controller, prior_build)
    prior = execution.bounded_json(prior_build/'partial.json', 128*1024*1024)
    old_export = prior['source_export']
    original = execution.safe_path(Path(old_export['source']).parent)
    current = execution.safe_path(current_export['source'])
    if not original.is_relative_to(REPO/'build') or not current.is_relative_to(REPO/'build'):
        raise ValueError('source cohorts must be owned build snapshots')
    old = verify_source(original, old_export)
    old_inputs = execution.bounded_json(original/'source-inputs.json')['files']
    new_manifest = Path(out)/'source-inputs.json'
    if execution.digest(new_manifest) != current_export['source_manifest_sha256']:
        raise ValueError('new source inventory changed')
    new_inputs = execution.bounded_json(new_manifest)['files']
    for name, expected in new_inputs.items():
        relative = Path(name)
        if relative.is_absolute() or '..' in relative.parts or not relative.parts:
            raise ValueError('unsafe new cohort input')
        execution.safe_path((current/relative).parent)
        if execution.file_identity(current/relative) != expected:
            raise ValueError('new cohort input changed: '+name)
    metadata = {'PORTING.md', '.gitignore', 'docs/current-state.md', 'docs/status.md',
                'docs/modernization.md', 'docs/runtime-modernization.md',
                'compatibility/stable-31.1/README.md',
                'maintenance/cleanup/LOG.md',
                'maintenance/cleanup/manifests/2026-10-26-31-runtime-intermediates.json',
                'maintenance/cleanup/reports/2026-10-26-31-runtime-intermediates.md',
                'maintenance/incidents/2026-10-25-runtime-build-owner-loss.md',
                'tools/runtime-stage.py', 'tools/runtime-execution.py',
                'tools/runtime-owner.py', 'tools/runtime-resume.py',
                'tools/reference-corpus.py', 'tools/stable-suite.py',
                'tools/test-runtime-retention.py'}
    changed = {name for name in set(old_inputs)|set(new_inputs)
               if old_inputs.get(name) != new_inputs.get(name)}
    repairs = {'src/comp.rs','xtask/src/macos.rs','platform/macos/nsfns.swift','src/pdumper.rs'}
    if changed - metadata - repairs:
        raise ValueError('unrelated input changes forbid terminal reuse: '+', '.join(sorted(changed-metadata-repairs)))
    anchor = 'include!("include/window.rs");                  // C 34: `#include "window.h"`\n'
    addition = ('#[cfg (all (feature = "have_native_comp", feature = "have_window_system"))]\n'
                'include!("include/dispextern.rs");              // window.h\'s display dependency for cursor fields\n')
    old_comp = (old/'src/comp.rs').read_text()
    if (old_comp.count(anchor) != 1 or (current/'src/comp.rs').read_text() != old_comp.replace(anchor, anchor+addition)
            or old_inputs['src/comp.rs']['mode'] != new_inputs['src/comp.rs']['mode']):
        raise ValueError('compiler source is not the exact P-362 conditional include')
    old_mac = (old/'xtask/src/macos.rs').read_text()
    old_object_dir = '''fn object_dir(repo: &Path, root: &Path) -> PathBuf {
    use std::hash::{Hash, Hasher};
    let mut key = std::collections::hash_map::DefaultHasher::new();
    root.hash(&mut key);
    repo.join(".femacs-cache/macos")
        .join(format!("{:016x}", key.finish()))
}'''
    new_object_dir = '''fn object_dir(repo: &Path, root: &Path) -> Result<PathBuf, String> {
    if let Some(binding) = crate::runtime::verify_selected_root(repo, root)? {
        if binding["product"] != "cocoa" {
            return Err("Cocoa object root has a different product binding".into());
        }
        let objects = root.join("objects");
        if objects.is_symlink() {
            return Err("Cocoa object root is a symlink".into());
        }
        return Ok(objects);
    }
    use std::hash::{Hash, Hasher};
    let mut key = std::collections::hash_map::DefaultHasher::new();
    root.hash(&mut key);
    Ok(repo.join(".femacs-cache/macos")
        .join(format!("{:016x}", key.finish())))
}'''
    if (old_mac.count(old_object_dir)!=1
            or old_mac.count('let objects = object_dir(&repo, &root);')!=1
            or old_mac.count('let objects = object_dir(repo, root);')!=1):
        raise ValueError('original Cocoa helper does not match P-365 baseline')
    expected_mac = old_mac.replace(old_object_dir,new_object_dir)
    expected_mac = expected_mac.replace('let objects = object_dir(&repo, &root);','let objects = object_dir(&repo, &root)?;')
    expected_mac = expected_mac.replace('let objects = object_dir(repo, root);','let objects = object_dir(repo, root)?;')
    # P-366 changes only the private Cocoa compiler options and Swift ABI tail.
    old_swift_loop = '    for name in [\n        "macfont", "nsfns", "nsimage", "nsmenu", "nsselect", "nsterm",\n    ] {'
    native_setup = '    // Swift builtin records must select the same native ABI as the Rust core.\n    let target_config: serde_json::Value = serde_json::from_slice(\n        &fs::read(root.join("config/darwin-cocoa.json")).map_err(|e| e.to_string())?,\n    ).map_err(|e| e.to_string())?;\n    let native_comp = match target_config["features"].get("have_native_comp") {\n        Some(value) => match (value.as_str(), value.as_i64()) {\n            (Some("1"), _) | (_, Some(1)) => true,\n            (Some("0" | "undefined"), _) | (_, Some(0)) => false,\n            _ => return Err("invalid Cocoa native feature value".into()),\n        },\n        None => false,\n    };\n'
    old_swift_args = '                .args(["swiftc", "-parse-as-library", "-O", "-g", "-module-name"])'
    native_swift_args = '                .args(["swiftc", "-parse-as-library", "-O", "-g"])\n                .args(if native_comp { &["-D", "HAVE_NATIVE_COMP"][..] } else { &[][..] })\n                .arg("-module-name")'
    if expected_mac.count(old_swift_loop)!=1 or expected_mac.count(old_swift_args)!=1:
        raise ValueError('original Cocoa compiler options do not match P-366 baseline')
    expected_mac = expected_mac.replace(old_swift_loop,native_setup+old_swift_loop).replace(old_swift_args,native_swift_args)
    if ((current/'xtask/src/macos.rs').read_text()!=expected_mac
            or old_inputs['xtask/src/macos.rs']['mode']!=new_inputs['xtask/src/macos.rs']['mode']):
        raise ValueError('Cocoa host source is not the exact P-365/P-366 private helper repair')
    swift_path = 'platform/macos/nsfns.swift'
    old_swift = (old/swift_path).read_text()
    swift_anchor = '  var command_modes: Lisp_Object\n  var doc: Int\n}'
    swift_tail = '  var command_modes: Lisp_Object\n  var doc: Int\n  #if HAVE_NATIVE_COMP\n  var native_comp_u: Lisp_Object = Qnil()\n  var native_c_name: UnsafeMutablePointer<CChar>? = nil\n  var lambda_list: Lisp_Object = Qnil()\n  var type: Lisp_Object = Qnil()\n  #endif\n}'
    if (old_swift.count(swift_anchor)!=1 or (current/swift_path).read_text()!=old_swift.replace(swift_anchor,swift_tail)
            or old_inputs[swift_path]['mode']!=new_inputs[swift_path]['mode']):
        raise ValueError('Cocoa Swift builtin source is not the exact P-366 native ABI tail')
    # P-368 is compiled out of every retained terminal configuration.
    pdumper_path = 'src/pdumper.rs'
    old_pdumper = (old/pdumper_path).read_text()
    old_sort_comment = "  /* Sort the objects into the order in which they'll appear in the\n     \x45macs: this way, on startup, we'll do both the IO from the dump\n     file and the copy into \x45macs in-order, where prefetch will be\n     most effective.  */"
    new_sort_comment = "  /* Sort by address for sequential IO/copy, except that Cocoa's mixed\n     Rust/Swift section placement requires primitive subrs first (P-368).\n     Address order is retained within both groups.  */"
    old_sort_comparator = '\t return if dump_object_emacs_ptr (a) < dump_object_emacs_ptr (b) { Qt () } else { Qnil () };'
    cocoa_sort_priority = '         // Cocoa\'s Swift builtin records live in bss, which may follow\n         // lispsym.  Dump those referents before symbols read their function\n         // fields; address order alone would violate ASSERT_ALREADY_SEEN.\n         // Copy relocations retain explicit source offsets and are sorted\n         // separately by destination.  P-368 preserves the strict invariant.\n         #[cfg (feature = "have_ns")]\n         {\n           let a_subr = SUBRP (a) && !NATIVE_COMP_FUNCTIONP (a);\n           let b_subr = SUBRP (b) && !NATIVE_COMP_FUNCTIONP (b);\n           if a_subr != b_subr\n             { return if a_subr { Qt () } else { Qnil () }; }\n         }\n'
    old_copy_comment = "    /* Dump the objects and generate a copy relocation for each.  Don't\n       bother trying to reduce the number of copy relocations we\n       generate: we'll merge adjacent copy relocations upon output.\n       The overall result is that to the greatest extent possible while\n       maintaining strictly increasing address order, we copy into \x45macs\n       in nice big chunks.  */"
    new_copy_comment = "    /* Dump the objects and generate a copy relocation for each.  Final\n       relocation output sorts by destination and merges only adjacent\n       source/destination ranges.  Cocoa's primitive-first exception keeps\n       address order within each group; it need not be globally monotonic.\n       Every copy retains its explicit source offset (P-368).  */"
    if old_pdumper.count(old_sort_comment)!=1 or old_pdumper.count(old_sort_comparator)!=1 or old_pdumper.count(old_copy_comment)!=1:
        raise ValueError('original dumper does not match P-368 baseline')
    expected_pdumper = old_pdumper.replace(old_sort_comment,new_sort_comment).replace(old_sort_comparator,cocoa_sort_priority+old_sort_comparator).replace(old_copy_comment,new_copy_comment)
    if ((current/pdumper_path).read_text()!=expected_pdumper
            or old_inputs[pdumper_path]['mode']!=new_inputs[pdumper_path]['mode']):
        raise ValueError('dumper source is not the exact Cocoa-only P-368 ordering repair')
    references = [p for p in prior['products'] if p['kind']=='reference']
    if len(references)!=1 or references[0]!=reference:
        raise ValueError('retained reference configuration differs from current selection')
    if execution.product_identity(reference, resources=True) != prior['reference_identity']:
        raise ValueError('installed reference changed since retained products')
    expected_cases = {case['id']:case for case in cases() if case['product']=='terminal'}
    rows = [row for row in prior['builds'] if row.get('product')=='terminal']
    if len(rows)!=10 or {row['id'] for row in rows} != set(expected_cases):
        raise ValueError('all ten exact terminal configurations are required for reuse')
    for row in rows:
        if (row.get('passed') is not True or row['process'].get('exit_code')!=0
                or row['process'].get('complete') is not True
                or row['process'].get('remaining_group_terminated')):
            raise ValueError('unfinished terminal configuration cannot be retained')
        if any(row.get(key)!=value for key,value in expected_cases[row['id']].items()):
            raise ValueError('retained terminal build selection changed')
    terminals = [dict(p) for p in prior['products'] if p['kind']=='terminal']
    if len(terminals)!=30:
        raise ValueError('all thirty terminal execution profiles are required')
    verified = {}
    expected_profiles = {}
    for case in expected_cases.values():
        for leg in ('normal','checking'):
            for mode in (('major','young') if 'collector' in case['variant'] else ('off',)):
                identity = case['id']+'-'+leg+('-'+mode if mode!='off' else '')
                expected_profiles[identity] = dict(kind='terminal', compiler_leg=leg,
                    native='native' in case['variant'], observer=case['observer'],
                    test_hooks=case['test_hooks'], collector=mode)
    if {p['id'] for p in terminals}!=set(expected_profiles):
        raise ValueError('retained terminal profile identities differ from complete selection')
    allowed_hosts = {prior['original_host_sha256']:'original', prior['xtask_sha256']:'recovered'}
    if any(not isinstance(sha,str) or not execution.HEX.fullmatch(sha) for sha in allowed_hosts):
        raise ValueError('retained host cohort lacks exact original/recovered identities')
    sealed = (SealedTerminals(retention_proof, prior_build, stopped, old_export,
                              current_export, terminals, allowed_hosts)
              if retention_proof is not None else None)
    profile_hosts = {}
    for profile in terminals:
        if any(profile.get(key)!=value for key,value in expected_profiles[profile['id']].items()):
            raise ValueError('retained profile label does not select its declared configuration')
        host = profile.get('host_provenance')
        if (not isinstance(host,dict) or host.get('sha256') not in allowed_hosts
                or host.get('role')!=allowed_hosts[host['sha256']]):
            raise ValueError('retained profile has missing or foreign host provenance')
        case_id = next(name for name in expected_cases if profile['id'].startswith(name+'-'+profile['compiler_leg']))
        row = next(row for row in rows if row['id']==case_id)
        if host != row.get('host_provenance'):
            raise ValueError('retained profile host differs from its actual build configuration')
        if host['role']=='recovered':
            provenance = execution.safe_path(host['provenance_receipt'])
            if execution.digest(provenance)!=host['provenance_sha256']:
                raise ValueError('recovered host provenance receipt changed')
        profile_hosts[profile['id']] = dict(host,build_configuration=case_id)
        receipt_path = execution.safe_path(profile['product_receipt'])
        if str(receipt_path) not in verified:
            receipt = sealed.verify(receipt_path) if sealed else verify_product(receipt_path)
            binding = receipt['binding']
            config = execution.bounded_json(receipt_path.parent/binding['config_relative'])
            if ('have_window_system' in config['features'] or 'HAVE_WINDOW_SYSTEM' in config['macros']
                    or binding['source_digest']!=prior['pinned_source_digest']):
                raise ValueError('retained product selects the changed display branch or wrong source')
            for section in ('macros','features'):
                for key, value in config.get(section,{}).items():
                    encoded = value if isinstance(value,str) else json.dumps(value,separators=(',',':'))
                    if key.lower()=='have_ns' and encoded in ('1','true'):
                        raise ValueError('retained terminal selects the changed Cocoa dumper branch')
            verified[str(receipt_path)] = execution.digest(receipt_path)
        profile['source_cohort'] = old_export['source_manifest_sha256']
    if sealed and (verified != sealed.receipts or profile_hosts != sealed.profile_hosts):
        raise ValueError('current retained terminal receipt/profile mapping differs from historical seals')
    # The production loader verifies every profile/receipt/feature/resource mapping.
    selected = Path(out)/'retained-terminal-products.json'
    execution.write_json(selected,dict(schema=1,products=[reference,*terminals]))
    execution.load_products(selected)
    certificate = dict(schema='femacs-terminal-source-certificate-v1',
                       prior_build=str(prior_build), stopped_controller=stopped,
                       original_source=old_export, current_source=current_export,
                       changed_inputs=sorted(changed),
                       conditional_repair=dict(path='src/comp.rs',requires=['have_native_comp','have_window_system'],
                                               old_sha256=old_inputs['src/comp.rs']['sha256'],
                                               new_sha256=new_inputs['src/comp.rs']['sha256']),
                       cocoa_host_repair=dict(path='xtask/src/macos.rs',scope='private object_dir/callers and selected native Swift compiler define',
                                              old_sha256=old_inputs['xtask/src/macos.rs']['sha256'],
                                              new_sha256=new_inputs['xtask/src/macos.rs']['sha256']),
                       cocoa_swift_abi_repair=dict(path=swift_path,scope='conditional four-field native tail only',
                                                   old_sha256=old_inputs[swift_path]['sha256'],
                                                   new_sha256=new_inputs[swift_path]['sha256']),
                       cocoa_copied_order_repair=dict(path=pdumper_path,requires=['have_ns'],
                                                     scope='primitive copied subrs first; address order within each group',
                                                     old_sha256=old_inputs[pdumper_path]['sha256'],
                                                     new_sha256=new_inputs[pdumper_path]['sha256']),
                       verified_receipts=verified, retained_profiles=sorted(expected_profiles),
                       host_cohorts=allowed_hosts, profile_host_provenance=profile_hosts,
                       uniform_source_snapshot=False, uniform_host_cohort=len(allowed_hosts)==1, passed=True)
    certificate['link_input_validation'] = 'historical-strict-seals-current-final-products' if sealed else 'current-strict'
    if sealed:
        certificate['link_input_retention'] = dict(proof=sealed.proof, seals=sealed.seals,
            receipts=sealed.inventory, intermediates_restored=False,
            independent_original_sources=False)
    execution.write_json(Path(out)/'terminal-source-certificate.json',certificate)
    return dict(builds=[dict(row,reused=True,source_cohort=old_export['source_manifest_sha256']) for row in rows],
                products=terminals, original_source=old_export, original_history=prior,
                reference_identity=prior['reference_identity'],
                certificate=str(Path(out)/'terminal-source-certificate.json'))


def carry_native_cocoa(prior_build, failed_controller, current_export, out, reference):
    """P-369: reuse only completed positive-native cases across one exact host repair."""
    prior_build = execution.safe_path(prior_build)
    stopped = assert_stopped(failed_controller, prior_build)
    prior = execution.bounded_json(prior_build/'partial.json', 128*1024*1024)
    old_export = prior['source_export']
    original = execution.safe_path(Path(old_export['source']).parent)
    old = verify_source(original, old_export)
    current = verify_source(Path(out), current_export)
    if not original.is_relative_to(REPO/'build') or not current.is_relative_to(REPO/'build'):
        raise ValueError('Cocoa reuse requires owned immutable source cohorts')
    old_inputs = execution.bounded_json(original/'source-inputs.json')['files']
    new_inputs = execution.bounded_json(Path(out)/'source-inputs.json')['files']
    changed = {name for name in set(old_inputs)|set(new_inputs)
               if old_inputs.get(name)!=new_inputs.get(name)}
    metadata = {'PORTING.md','docs/current-state.md','docs/status.md','docs/modernization.md','docs/runtime-modernization.md',
                'tools/runtime-stage.py','tools/runtime-resume.py','tools/test-runtime-retention.py'}
    host_path = 'xtask/src/macos.rs'
    if changed-metadata-{host_path}:
        raise ValueError('unrelated input changes forbid native Cocoa reuse: '+', '.join(sorted(changed-metadata-{host_path})))
    old_branch = '            (Some("0"), _) | (_, Some(0)) => false,'
    new_branch = '            (Some("0" | "undefined"), _) | (_, Some(0)) => false,'
    old_host = (old/host_path).read_text()
    if (old_host.count(old_branch)!=1 or (current/host_path).read_text()!=old_host.replace(old_branch,new_branch)
            or old_inputs[host_path]['mode']!=new_inputs[host_path]['mode']):
        raise ValueError('Cocoa host is not the exact disabled-token P-369 repair')
    references = [profile for profile in prior['products'] if profile['kind']=='reference']
    if (references!=[reference]
            or execution.product_identity(reference, resources=True)!=prior['reference_identity']):
        raise ValueError('retained Cocoa comparator identity changed')
    expected = {case['id']:case for case in cases()
                if case['product']=='cocoa' and 'native' in case['variant']}
    selected = [row for row in prior['builds'] if row['id'] in expected and row.get('passed') is True]
    if not selected or len({row['id'] for row in selected})!=len(selected):
        raise ValueError('complete unique positive-native Cocoa configurations required')
    manifest = old_export['source_manifest_sha256']
    host = dict(role='fresh',sha256=prior['xtask_sha256'],source_manifest_sha256=manifest)
    owner_receipt = execution.bounded_json(prior_build/'build-owner-receipt.json')
    archive = execution.bounded_json(prior_build/'build-owner-archive.json')
    if (owner_receipt['sha256']!=host['sha256'] or archive['executable_sha256']!=host['sha256']
            or execution.digest(execution.safe_path(prior_build/archive['path']))!=archive['sha256']):
        raise ValueError('retained Cocoa host provenance changed')
    profiles, verified = [], {}
    for row in selected:
        case = expected[row['id']]
        process = row['process']
        if (any(row.get(key)!=value for key,value in case.items())
                or process.get('exit_code')!=0 or process.get('complete') is not True
                or process.get('remaining_group_terminated') or row.get('source_cohort')!=manifest
                or row.get('host_provenance')!=host):
            raise ValueError('unfinished or mismatched native Cocoa configuration')
        wanted = {}
        for leg in ('normal','checking'):
            for mode in (('major','young') if 'collector' in case['variant'] else ('off',)):
                name = case['id']+'-'+leg+('-'+mode if mode!='off' else '')
                wanted[name] = dict(kind='cocoa',compiler_leg=leg,native=True,observer=case['observer'],
                                    test_hooks=case['test_hooks'],collector=mode)
        actual = [profile for profile in prior['products'] if profile['id'] in wanted]
        if len(actual)!=len(wanted) or {profile['id'] for profile in actual}!=set(wanted):
            raise ValueError('incomplete native Cocoa execution profiles')
        for profile in actual:
            if (any(profile.get(key)!=value for key,value in wanted[profile['id']].items())
                    or profile.get('source_cohort')!=manifest or profile.get('host_provenance')!=host):
                raise ValueError('native Cocoa profile source/host selection changed')
            receipt_path = execution.safe_path(profile['product_receipt'])
            if str(receipt_path) not in verified:
                receipt = verify_product(receipt_path)
                binding = receipt['binding']
                if binding['product']!='cocoa':
                    raise ValueError('retained native Cocoa receipt selects another product')
                config = execution.bounded_json(receipt_path.parent/binding['config_relative'])
                # Match the old Swift parser's accepted positive branch exactly.
                for section,key in (('features','have_native_comp'),('macros','HAVE_NATIVE_COMP')):
                    value = config.get(section,{}).get(key)
                    if not (value=='1' or type(value) is int and value==1):
                        raise ValueError('retained Cocoa does not take the unchanged positive-native branch')
                verified[str(receipt_path)] = execution.digest(receipt_path)
            profiles.append(dict(profile))
    selection = Path(out)/'retained-native-cocoa-products.json'
    execution.write_json(selection,dict(schema=1,products=[reference,*profiles]))
    execution.load_products(selection)
    certificate = dict(schema='femacs-native-cocoa-source-certificate-v1',passed=True,
                       prior_build=str(prior_build),stopped_controller=stopped,
                       original_source=old_export,current_source=current_export,changed_inputs=sorted(changed),
                       repair=dict(path=host_path,scope='disabled undefined token only; positive branch unchanged',
                                   old_sha256=old_inputs[host_path]['sha256'],new_sha256=new_inputs[host_path]['sha256']),
                       retained_cases=[row['id'] for row in selected],retained_profiles=[p['id'] for p in profiles],
                       verified_receipts=verified,host_provenance=host,
                       uniform_source_snapshot=False,semantic_or_performance_accepted=False)
    execution.write_json(Path(out)/'native-cocoa-source-certificate.json',certificate)
    return dict(builds=[dict(row,reused=True) for row in selected],products=profiles,
                original_source=old_export,original_history=prior,
                certificate=str(Path(out)/'native-cocoa-source-certificate.json'))


def host_build_inputs(source, original, env):
    selected = {name:env[name] for name in (
        'PATH', 'RUSTUP_TOOLCHAIN', 'RUSTC', 'RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER',
        'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CARGO_BUILD_TARGET',
        'CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER', 'SDKROOT', 'MACOSX_DEPLOYMENT_TARGET',
        'CC', 'CXX', 'CARGO_TARGET_DIR') if name in env}
    commands = {'rustc':['rustc','-Vv'], 'cargo':['cargo','-V'],
                'linker':['cc','--version'], 'sdk_path':['xcrun','--sdk','macosx','--show-sdk-path'],
                'sdk_version':['xcrun','--sdk','macosx','--show-sdk-version']}
    toolchain = {}
    for name, argv in commands.items():
        result = subprocess.run(argv, env=env, cwd=source, capture_output=True, text=True, timeout=30)
        if result.returncode:
            raise ValueError('host toolchain metadata unavailable: ' + name)
        toolchain[name] = dict(argv=argv, stdout=result.stdout, stderr=result.stderr)
    return dict(source_manifest_sha256=execution.digest(original/'source-inputs.json'),
                manifests={name:execution.digest(source/name) for name in ('Cargo.toml','Cargo.lock','xtask/Cargo.toml')},
                cwd=str(source), environment=selected, toolchain=toolchain)


def verify_stop_observation(result_path, result, original):
    """Verify external post-stop facts without inventing a native final report."""
    if (result.get('native_exit_code') != 125
            or result.get('native_error') != dict(error_code='write_report', error_errno=28)
            or result.get('original_final_report_available') is not False
            or result.get('source_scope') != original.name
            or 'cleanup_complete' in result):
        raise ValueError('unsupported or mislabelled external stop observation')
    snapshots = {}
    for key in ('last_state', 'launch_state'):
        pin = result[key]
        name = Path(pin['path'])
        if name.name != str(name) or name.name in ('', '.', '..'):
            raise ValueError('stop observation snapshots must be adjacent ordinary files')
        path = execution.safe_path(result_path.parent / name)
        if execution.digest(path) != pin['sha256']:
            raise ValueError('original stop snapshot identity changed')
        snapshots[key] = execution.bounded_json(path)
    state, launch = snapshots['last_state'], snapshots['launch_state']
    pairs = [state['guard'], *state['owned']]
    if (state.get('version') != 1 or state.get('pgid') != result.get('root_pid')
            or launch.get('pid') not in [pair[0] for pair in state['owned']]
            or state['pgid'] not in [pair[0] for pair in state['owned']]):
        raise ValueError('original launcher/controller identities disagree')
    argv = launch['argv']
    if '--out' not in argv or argv[argv.index('--out') + 1] != str(original):
        raise ValueError('original launcher output differs from stop scope')
    for pair in pairs:
        if (not isinstance(pair, list) or len(pair) != 2 or type(pair[0]) is not int
                or pair[0] <= 1 or type(pair[1]) not in (int, float)
                or not math.isfinite(pair[1]) or pair[1] <= 0):
            raise ValueError('invalid original process identity')
        observed = subprocess.run(['ps', '-p', str(pair[0]), '-o', 'lstart='],
                                  capture_output=True, text=True)
        if observed.returncode not in (0, 1) or observed.stderr.strip():
            raise ValueError('original process identity cannot be independently checked')
        if (observed.returncode == 0 and ' '.join(observed.stdout.split())
                == ' '.join(time.ctime(pair[1]).split())):
            raise ValueError('recorded original process identity is still alive')
    if result.get('schema') == 'femacs-runtime-finalization-stop-observation-v1':
        if result.get('native_final_metrics_available') is not False:
            raise ValueError('lost final report cannot supply native final metrics')
        for key, actual in (('partial_state', original/'partial.json'),
                            ('product_state', original/'products.json'),
                            ('native_observation', None)):
            pin = result[key]
            name = Path(pin['path'])
            if name.name != str(name) or name.name in ('', '.', '..'):
                raise ValueError('finalization stop snapshots must be adjacent ordinary files')
            snapshot = execution.safe_path(result_path.parent/name)
            if (execution.digest(snapshot) != pin['sha256']
                    or (actual is not None and execution.digest(execution.safe_path(actual)) != pin['sha256'])):
                raise ValueError('finalization stop snapshot identity changed')
            snapshots[key] = execution.bounded_json(snapshot, 128*1024*1024)
        native = snapshots['native_observation']
        if (native.get('schema') != 'femacs-external-finalization-stop-observation-v1'
                or native.get('tool_exit_code') != 125
                or native.get('original_native_final_report_available') is not False
                or native.get('native_final_metrics_available') is not False
                or native.get('actual_returned_error') != dict(error_type='GuardError',
                    error_code='write_report', error_stage='admission', error_errno=28,
                    error_message='No space left on device')
                or 'cleanup_complete' in native):
            raise ValueError('finalization observation must preserve the actual lost-report boundary')
        partial, product_state = snapshots['partial_state'], snapshots['product_state']
        expected_cases = {row['id']: row for row in cases()}
        builds = partial.get('builds', [])
        if (len(builds) != 20 or {row['id'] for row in builds} != set(expected_cases)
                or len(partial.get('products', [])) != 61
                or product_state.get('schema') != 1
                or product_state.get('products') != partial['products']
                or len({p['id'] for p in partial['products']}) != 61):
            raise ValueError('finalization recovery requires the complete original matrix')
        for row in builds:
            process = row['process']
            if (any(row.get(key) != value for key, value in expected_cases[row['id']].items())
                    or row.get('passed') is not True or process.get('complete') is not True
                    or process.get('exit_code') != 0 or process.get('timed_out') is not False
                    or process.get('remaining_group_terminated') is not False):
                raise ValueError('an unfinished compiler case cannot be finalized as complete')
        return dict(result, stop_evidence_kind='independently-verified-finalization-stop',
                    verified_process_identities=len(pairs), verified_at=time.time(),
                    native_controller_metrics_available=False)
    case = result['failed_configuration']
    if case not in {row['id'] for row in cases()}:
        raise ValueError('unknown interrupted configuration')
    failure = execution.safe_path(original / 'builds' / case / 'stderr.txt')
    if execution.digest(failure) != result['failure_log_sha256']:
        raise ValueError('original interrupted compiler log changed')
    with failure.open(errors='replace') as stream:
        if not any('No space left on device' in line for line in stream):
            raise ValueError('original ENOSPC evidence is missing')
    return dict(result, stop_evidence_kind='independently-verified-post-stop',
                verified_process_identities=len(pairs), verified_at=time.time(),
                native_controller_metrics_available=False)


def verify_controller_loss(result, original):
    """Admit a reaped supervisor loss without inventing its final metrics.

    Only the prefix observed while both original process identities were live
    can be reused. Later completed rows remain history and are rebuilt.
    """
    required = {'schema', 'original_build', 'root_identity', 'guard_identity',
                'checkpoint', 'reaping', 'launch_record', 'partial_sha256',
                'source_manifest_sha256', 'released_journal',
                'captured_native_final_report_available',
                'native_controller_metrics_available'}
    if (set(result) != required or result['original_build'] != str(original)
            or result['captured_native_final_report_available'] is not False
            or result['native_controller_metrics_available'] is not False):
        raise ValueError('exact external controller-loss evidence required')
    partial = execution.bounded_json(original/'partial.json', 128*1024*1024)
    if (execution.digest(original/'partial.json') != result['partial_sha256']
            or partial['source_export']['source_manifest_sha256'] != result['source_manifest_sha256']):
        raise ValueError('interrupted source or partial evidence changed')
    identities = [result['guard_identity'], result['root_identity']]
    for identity in identities:
        pid, birth = identity.get('pid'), identity.get('birth')
        if type(pid) is not int or pid <= 1 or not isinstance(birth, str) or not birth:
            raise ValueError('original supervisor/root birth identities required')
        current = subprocess.run(['ps', '-p', str(pid), '-o', 'lstart='],
                                 capture_output=True, text=True, timeout=5)
        if current.returncode not in (0, 1) or current.stderr.strip():
            raise ValueError('original process identity is unknown')
        if current.returncode == 0 and current.stdout.strip() == birth:
            raise ValueError('original process identity is still alive')
    if identities[0]['pid'] == identities[1]['pid']:
        raise ValueError('supervisor and root identities must differ')
    launch_pin = result['launch_record']
    launch_path = execution.safe_path(launch_pin['path'])
    if execution.digest(launch_path) != launch_pin['sha256']:
        raise ValueError('original launch evidence changed')
    launch = execution.bounded_json(launch_path, 64*1024)
    argv = launch['argv']
    if (not isinstance(argv, list) or not all(isinstance(v, str) for v in argv)
            or argv.count('--out') != 1):
        raise ValueError('exact original launch argv required')
    output = Path(argv[argv.index('--out')+1])
    if execution.safe_path(output if output.is_absolute() else REPO/output) != original:
        raise ValueError('original launch output differs')
    source = Path(partial['source_export']['source'])
    for name, expected in launch['source_pins_at_launch'].items():
        relative = Path(name)
        if relative.is_absolute() or '..' in relative.parts:
            raise ValueError('unsafe original source pin')
        if execution.digest(execution.safe_path(source/relative)) != expected:
            raise ValueError('original launched source differs from frozen input')
    checkpoint, reaping = result['checkpoint'], result['reaping']
    count = checkpoint.get('completed')
    rows = partial['builds']
    if (type(count) is not int or not 0 < count <= len(rows)
            or checkpoint.get('passed') != count
            or checkpoint.get('latest') != rows[count-1]['id']
            or checkpoint.get('owner_identities') != [dict(pid=i['pid'], live=True) for i in identities]
            or reaping.get('guard_absent') is not True
            or reaping.get('root_parent') != 1
            or reaping.get('owned_tree_reaped') is not True):
        raise ValueError('live guarded prefix and actual post-stop reaping required')
    times = [datetime.fromisoformat(value).timestamp() for value in
             (checkpoint['checked_at'], reaping['observed_at'], reaping['reaped_at'])]
    if not (times[0] <= times[1] <= times[2] <= time.time()+5):
        raise ValueError('controller-loss observation chronology differs')
    owned = reaping['owned_processes']
    if str(result['root_identity']['pid']) not in owned:
        raise ValueError('reaped process set omits the original root')
    for pid, birth in owned.items():
        if not str(pid).isdigit() or int(pid) <= 1 or type(birth) not in (int, float) or not math.isfinite(birth):
            raise ValueError('invalid reaped process birth identity')
        current = subprocess.run(['ps', '-p', str(pid), '-o', 'lstart='],
                                 capture_output=True, text=True, timeout=5)
        if current.returncode not in (0, 1) or current.stderr.strip():
            raise ValueError('reaped descendant identity is unknown')
        if current.returncode == 0 and current.stdout.strip() == time.ctime(birth):
            raise ValueError('recorded descendant is still alive')
    journal_pin = result['released_journal']
    journal_path = execution.safe_path(journal_pin['path'])
    if (journal_path != original/'artifact-preservation.json'
            or execution.digest(journal_path) != journal_pin['sha256']):
        raise ValueError('original artifact-release evidence changed')
    journal = stage.owner.artifact_read(journal_path, require_owner=False)
    if (journal['state'] != 'released' or journal.get('release_failures') != []
            or journal['owner_pid'] != result['root_identity']['pid']
            or journal['owner_birth'] != result['root_identity']['birth']):
        raise ValueError('original artifact owner is not completely released')
    del journal
    planned = {case['id']: case for case in cases()}
    admitted = []
    for row in rows[:count]:
        process = row['process']
        if (row.get('id') not in planned or row['id'] in admitted
                or any(row.get(k) != v for k, v in planned[row['id']].items())
                or row.get('passed') is not True or process.get('complete') is not True
                or process.get('exit_code') != 0 or process.get('timed_out') is not False
                or process.get('remaining_group_terminated') is not False):
            raise ValueError('guarded prefix contains an incomplete configuration')
        admitted.append(row['id'])
    return dict(result, root_pid=result['root_identity']['pid'],
                stop_evidence_kind='verified-controller-loss-stop',
                verified_completed_configurations=admitted,
                ignored_later_completed_configurations=[row['id'] for row in rows[count:]])


class OutputOccupancyError(ValueError):
    """Retain the bounded query facts without exposing raw lsof output."""

    def __init__(self, observation):
        self.output_occupancy = dict(observation)
        super().__init__('original output has active or unknown users: '
                         + json.dumps(self.output_occupancy, sort_keys=True, allow_nan=False))


def require_idle_output(original):
    retirement = execution.load_tool('runtime-retire.py')
    observation = retirement.check_occupancy(original)
    if observation['clear'] is not True:
        raise OutputOccupancyError(observation)
    return observation


def assert_stopped(result_path, original):
    result_path = execution.safe_path(result_path)
    result = execution.bounded_json(result_path)
    if result.get('schema') == 'femacs-runtime-controller-loss-observation-v1':
        result = verify_controller_loss(result, original)
    elif result.get('schema') in ('femacs-runtime-stop-observation-v1',
                               'femacs-runtime-finalization-stop-observation-v1'):
        result = verify_stop_observation(result_path, result, original)
    elif result.get('cleanup_complete') is not True or result.get('exit_code') == 0:
        raise ValueError('explicit terminal failed controller/reaping evidence required')
    pid = result.get('root_pid')
    if type(pid) is not int or pid <= 1:
        raise ValueError('original controller root identity required')
    process = subprocess.run(['ps','-p',str(pid),'-o','command='], capture_output=True, text=True)
    if process.returncode not in (0, 1):
        raise ValueError('original process status is unknown')
    if (process.returncode == 0 and original.name in process.stdout
            and any(name in process.stdout for name in ('runtime-stage.py','runtime-resume.py'))):
        raise ValueError('original matrix owner is still alive')
    require_idle_output(original)
    return result


def resume(options, out, artifacts=None):
    original = execution.safe_path(options.build)
    if not original.is_relative_to(REPO/'build') or original == REPO/'build':
        raise ValueError('original matrix must be an owned build child')
    if out.is_relative_to(original) or original.is_relative_to(out):
        raise ValueError('recovery report must be separate from original output')
    stopped = assert_stopped(options.controller_result, original)
    prior = execution.bounded_json(original/'partial.json', 128 * 1024 * 1024)
    if (not (original/'fatal.json').is_file()
            and stopped.get('stop_evidence_kind') not in ('independently-verified-post-stop',
                                                        'independently-verified-finalization-stop',
                                                        'verified-controller-loss-stop')):
        raise ValueError('original failed-run evidence must remain')
    lineage = stage.build_source_lineage(original, prior)
    source_owner = lineage[-1]
    source = verify_source(source_owner, prior['source_export'])
    if artifacts is not None:
        artifacts.select_source(source_owner, prior['source_export'])
    expected = prior['xtask_sha256']
    if not execution.HEX.fullmatch(expected):
        raise ValueError('original host identity is missing')
    selected_host = options.recovered_host_sha256 or expected
    if not execution.HEX.fullmatch(selected_host):
        raise ValueError('explicit recovered host SHA256 is invalid')
    original_receipt_path = original/'build-owner-receipt.json'
    original_receipt = execution.bounded_json(original_receipt_path)
    if original_receipt.get('sha256') != expected:
        raise ValueError('original host receipt identity changed')
    owner_root = Path(original_receipt['owner_root'])
    if (owner_root.is_absolute() or '..' in owner_root.parts or len(owner_root.parts) != 3
            or owner_root.parts[:2] != ('.femacs-cache', 'runtime-build-owners')
            or original_receipt.get('path') != 'runtime-build-driver'):
        raise ValueError('original host receipt has an unsafe owner locator')
    original_owner = execution.safe_path(REPO/owner_root, must_exist=False)/original_receipt['path']
    if original_owner.exists() and execution.digest(original_owner) != expected:
        raise ValueError('original host path now contains a different artifact')
    original_host = dict(role='original', sha256=expected, bytes=original_receipt['bytes'],
                         receipt_sha256=execution.digest(original_receipt_path), available=original_owner.exists())
    plan = list(cases())
    planned = {item['id']:item for item in plan}
    if len(planned) != 20:
        raise ValueError('complete original matrix plan required')
    old_rows = {item['id']:item for item in prior['builds']}
    if len(old_rows) != len(prior['builds']) or not set(old_rows).issubset(planned):
        raise ValueError('unknown or duplicate original configurations')
    report = dict(source_export=prior['source_export'], original_build=str(original),
                  original_partial_sha256=execution.digest(original/'partial.json'),
                  original_failed_controller=stopped, original_history=prior['builds'],
                  builds=[], products=[], reused_configurations=[], passed=False)
    report.update(source_cohorts=prior.get('source_cohorts', [prior['source_export']]),
                  uniform_source_snapshot=prior.get('uniform_source_snapshot', True),
                  recovery_coordinator_sha256=execution.digest(Path(__file__)))
    input_paths = [execution.safe_path(options.controller_result), original/'partial.json',
                   original/'build-owner-receipt.json']
    if stopped.get('stop_evidence_kind') == 'verified-controller-loss-stop':
        input_paths.extend(execution.safe_path(stopped[key]['path'])
                           for key in ('launch_record', 'released_journal'))
    input_identities = {str(path): execution.file_identity(path) for path in input_paths}
    if artifacts is not None:
        journal = stage.owner.artifact_read(artifacts.journal)
        for name in input_identities:
            if name not in journal['allowed_roots']:
                journal['allowed_roots'].append(name)
        stage.owner.artifact_write(artifacts.journal, journal)
        stage.owner.capture_artifacts(artifacts.journal, input_identities)
    report['recovery_input_identities'] = input_identities
    references = [p for p in prior['products'] if p['kind']=='reference']
    if len(references) != 1:
        raise ValueError('one original installed reference is required')
    report['products'].extend(references)
    report['reference_identity'] = execution.product_identity(references[0], resources=True)
    if report['reference_identity'] != prior['reference_identity']:
        raise ValueError('installed reference changed since the interrupted build')
    if stopped.get('stop_evidence_kind') == 'verified-controller-loss-stop':
        admitted = set(stopped['verified_completed_configurations'])
        report['ignored_later_completed_configurations'] = stopped['ignored_later_completed_configurations']
    else:
        admitted = set(old_rows)
    for case in plan:
        old = old_rows.get(case['id'])
        if case['id'] not in admitted or old is None or old.get('passed') is not True:
            continue
        if (old['process'].get('exit_code') != 0 or old['process'].get('complete') is not True
                or old['process'].get('remaining_group_terminated') is not False):
            raise ValueError('inconsistent completed configuration')
        argv = old['process']['argv']
        root = execution.safe_path(argv[argv.index('--out')+1])
        allowed_roots = {source/'build/runtime'/case['id']}
        allowed_roots.update(source/'build/runtime'/(case['id']+'-recovery-'+producer.name)
                             for producer in lineage if producer != source_owner)
        if root not in allowed_roots:
            raise ValueError('original completed configuration root mismatch')
        for compiler_leg, leg in (('normal',root), ('checking',root/'checking')):
            binding = verify_product(leg/'runtime-product.json')['binding']
            if (binding.get('compiler_leg') != compiler_leg or binding.get('product') != case['product'] or binding.get('variant') != case['variant']
                    or binding.get('observer') != case['observer'] or binding.get('test_hooks') != case['test_hooks']):
                raise ValueError('completed product does not match original case')
            if artifacts is not None:
                stage.owner.capture_product(artifacts.journal, leg/'runtime-product.json', source)
        append_products(report, case, root, original_host)
        report['builds'].append(dict(old, reused=True, host_provenance=original_host))
        report['reused_configurations'].append(case['id'])
        if case['id'] in prior.get('compiler_copy_retirements', {}):
            saved = prior['compiler_copy_retirements'][case['id']]
            for compiler_leg, leg in (('normal', root), ('checking', root/'checking')):
                item = saved[compiler_leg]
                if (item['status'] != 'complete' or item['all_selected_absent'] is not True
                        or os.path.lexists(leg/'objects/units')
                        or execution.digest(leg/item['ledger']) != item['ledger_sha256']):
                    raise ValueError('consumed compiler-copy retirement changed')
            report.setdefault('compiler_copy_retirements', {})[case['id']] = saved
    execution.write_json(out/'partial.json', report)
    env = stage.clean_environment()
    env['CARGO_TARGET_DIR'] = str(original/'cargo')
    if artifacts is not None:
        env[stage.owner.ARTIFACT_ENV] = str(artifacts.journal)
        env['FEMACS_RUNTIME_HELPER_PYTHON'] = sys.executable
    before = host_build_inputs(source,source_owner,env)
    host_started = datetime.now(timezone.utc).isoformat()
    retained_host = original_owner.exists() and selected_host == expected
    if retained_host:
        stage.verify_build_owner(original_owner, expected)
        host = original_owner
        compiled = dict(skipped=True, reason='intact original host reused by exact receipt hash')
    else:
        compiled = execution.run_process(['cargo','build','--manifest-path',source/'Cargo.toml',
                                          '-p','xtask','-j','2','--locked','--offline'],
                                         out/'host-recovery-build',env,None,cwd=source)
        if compiled['exit_code'] != 0 or not compiled['complete'] or compiled['remaining_group_terminated']:
            raise ValueError('frozen host recovery compilation failed')
        host = execution.safe_path(original/'cargo/debug/xtask')
    report['host_recovery_build'] = compiled
    execution.write_json(out/'partial.json',report)
    if execution.digest(host) != selected_host:
        raise ValueError('recovered host does not match selected executable hash')
    verify_source(source_owner,prior['source_export'])
    after = host_build_inputs(source,source_owner,env)
    if before != after:
        raise ValueError('host source/toolchain/environment changed during recovery compilation')
    current_host = dict(role='retained-identical' if retained_host else 'recovered', sha256=selected_host, bytes=host.stat().st_size,
                        provenance_receipt=str(out/'host-provenance.json'))
    provenance = dict(schema=1, original_host=original_host, recovered_host=current_host,
                      original_host_unavailable=not original_host['available'],
                      host_identity_relation='identical-bytes' if selected_host == expected else 'distinct',
                      explicit_recovered_host_sha256=options.recovered_host_sha256,
                      source_file_count=prior['source_export']['file_count'], inputs=before,
                      started_at=host_started, finished_at=datetime.now(timezone.utc).isoformat(),
                      build_process=compiled, log_identities={} if retained_host else {
                          name:execution.file_identity(out/'host-recovery-build'/name)
                          for name in ('stdout.txt','stderr.txt')})
    execution.write_json(out/'host-provenance.json',provenance)
    current_host['provenance_sha256'] = execution.digest(out/'host-provenance.json')
    inputs = out/'owner-input';inputs.mkdir()
    retained = inputs/'runtime-build-driver'
    shutil.copyfile(host, retained)
    retained.chmod(host.stat().st_mode & 0o777)
    driver, identity = stage.stage_build_owner(retained,out)
    if identity != selected_host:
        raise ValueError('recovery capture changed selected host identity')
    owner_receipt = execution.bounded_json(driver.parent/'receipt.json')
    owner_receipt.update(original_host_sha256=expected, host_provenance=current_host)
    execution.write_json(driver.parent/'receipt.json',owner_receipt)
    execution.write_json(out/'build-owner-receipt.json',owner_receipt)
    report.update(xtask_sha256=identity, original_host_sha256=expected,
                  xtask_profile=prior['xtask_profile'],
                  mixed_host_matrix=identity != expected and bool(report['reused_configurations']),
                  host_provenance=current_host, build_owner_receipt=str(driver.parent/'receipt.json'))
    execution.write_json(out/'partial.json',report)
    with stage.owner.protect(driver,identity,out,REPO):
        for case in plan:
            if case['id'] in report['reused_configurations']:
                continue
            stage.verify_build_owner(driver,identity)
            verify_source(source_owner,prior['source_export'])
            root = source/'build/runtime'/(case['id']+'-recovery-'+out.name)
            execution.safe_path(root,must_exist=False)
            if root.exists():
                raise ValueError('recovery case output is not fresh')
            argv = [driver,'runtime-build','--product',case['product'],'--variant',case['variant'],'--out',root]
            if case['observer']:argv.append('--observer')
            if case['test_hooks']:argv.append('--test-hooks')
            if 'native' in case['variant']:argv.extend(['--backend-prefix',options.backend_prefix])
            result = execution.run_process(argv,out/'builds'/case['id'],env,None,cwd=source)
            passed = result['exit_code']==0 and result['complete'] and not result['remaining_group_terminated']
            report['builds'].append(dict(case,process=result,passed=passed,reused=False,host_provenance=current_host))
            if not passed:
                report['error'] = 'recovery configuration failed; remaining matrix not started'
                execution.write_json(out/'partial.json', report)
                return report
            append_products(report,case,root,current_host)
            retirement_spec = importlib.util.spec_from_file_location('resume_retirement', source/'tools/runtime-retire.py')
            retirement = importlib.util.module_from_spec(retirement_spec)
            retirement_spec.loader.exec_module(retirement)
            report.setdefault('compiler_copy_retirements', {})[case['id']] = {
                compiler_leg: retirement.retire(leg/'objects', ['units'],
                    leg/'runtime-build-outcome.json', result)
                for compiler_leg, leg in (('normal', root), ('checking', root/'checking'))}
            execution.write_json(out/'partial.json',report)
            stage.verify_build_owner(driver,identity)
        verify_source(source_owner,prior['source_export'])
        if execution.product_identity(references[0],resources=True) != report['reference_identity']:
            raise ValueError('installed reference changed during recovery')
    execution.write_json(out/'products.json',dict(schema=1,products=report['products'],
                                               source_cohorts=report['source_cohorts']))
    execution.load_products(out/'products.json')
    inputs_unchanged = all(execution.file_identity(Path(name)) == pin
                           for name, pin in input_identities.items())
    if (not inputs_unchanged
            or execution.digest(Path(__file__)) != report['recovery_coordinator_sha256']):
        raise ValueError('recovery evidence or coordinator changed during execution')
    report.update(source_root=str(source),product_list=str(out/'products.json'),semantic_tests_executed=False,
                  recovery_inputs_unchanged=inputs_unchanged,
                  passed=len(report['builds'])==20 and all(row['passed'] for row in report['builds'])
                         and len(report['products'])==61)
    return report


def main(argv=None):
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build',type=Path,required=True)
    parser.add_argument('--out',type=Path,required=True)
    parser.add_argument('--controller-result',type=Path,required=True)
    parser.add_argument('--backend-prefix',type=Path,required=True)
    parser.add_argument('--retention-mode',choices=('files',),default='files',
                        help='verify and retain selected physical files without a duplicate product archive')
    parser.add_argument('--recovered-host-sha256', help='Explicit distinct rebuilt host identity; default requires the original hash')
    options=parser.parse_args(argv)
    out=execution.new_output(options.out)
    try:
        with stage.owner.ArtifactRun(REPO, out) as artifacts:
            result=resume(options,out,artifacts)
            if result['passed']:
                result['artifact_retention'] = artifacts.retain_current_files()
    except BaseException as error:
        failure = dict(passed=False, complete=False, error=str(error))
        if isinstance(error, OutputOccupancyError):
            failure['output_occupancy'] = error.output_occupancy
        execution.write_json(out/'fatal.json', failure)
        raise
    result.update(schema='femacs-runtime-stage-v1',action='build',goal_complete=False,
                  coordinator_sha256=stage.COORDINATOR_SHA256)
    execution.write_json(out/'results.json',result)
    print(json.dumps(dict(passed=result['passed'],results=str(out/'results.json'))))
    return 0 if result['passed'] else 1


if __name__=='__main__':
    raise SystemExit(main())
