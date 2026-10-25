# CLN-20261026-30: archive stopped D12 compiler intermediates

Only regular generated translation-unit copies, object/archive files and incremental compiler cache members from the stopped D12 attempt are selected. D12 is terminal; D13 has also been stopped after collecting its two independent failures. All original frozen D12 inputs, JSON/configuration, logs, executable/dump/resource trees and captured owner paths are excluded. D13, current source, trial apps, installed tools/reference and other conversations are excluded.

The [manifest](../manifests/2026-10-26-30-runtime-intermediates.json) binds every selected logical member to its hash and bytes; the retained private ledger maps exact owned paths. Every archive member is read and hashed before deletion, and representative Rust/object/archive members are actually restored and checked. Restore only to exact owned paths using that ledger. Original frozen inputs and current trial identities are protected. File-content recovery does not promise hardlink topology or exact physical-volume recovery.

Status: prepared before expanded removal. The original D12 nil native-ABI resource error and owner-loss incident remain recorded. This cleanup does not repair or accept a product.

Completed: 4717 expanded files, 166162858 logical bytes and 175923200 pre-removal allocated bytes; verified persistent archives occupy 40361952 bytes. All selected expanded files are absent, and all 7295 protected content hashes are unchanged. Current source and trial products are preserved; no product repair, compatibility or performance acceptance is inferred.
