# CLN-20261025-22: archive legacy owned compiler/tool caches

Four stopped legacy compiler/tool cache roots are retained as verified private archives. The original project source, root Mac app/terminal and current Android5 APK remain expanded, with protected product hashes checked before and after. Every regular member content is verified; four representative members are actually recovered before expanded removal. Existing symlinks remain unchanged and are never traversed. No optional reference checkout, installed application, SDK or foreign tree is modified.

The [manifest](../manifests/2026-10-25-22-legacy-tool-caches.json) records each content identity, protected products and the private exact-path/recovery ledger. Restore regular archive members into the retained owned directory structure and links. This preserves file contents, not a newly verified restored build/app or extended attributes. No physical-volume release amount is inferred from logical byte sums.

Completed: 16648 expanded files, 719864741 logical bytes and 754991104 pre-removal allocated-byte accounting; verified archives occupy 160866063 bytes.

Public logical roles redact machine-root-encoded generated components using deterministic SHA256 names. Content hashes, byte counts and operation results are unchanged; exact recovery paths remain private.
