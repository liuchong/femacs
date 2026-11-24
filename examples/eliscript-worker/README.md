# Opt-in Eliscript Worker Client for FEmacs

Strictly opt-in asynchronous Eliscript worker client (`femacs-eliscript-worker.el`) built over the frozen JS worker runtime, value codec/stream, and Lisp service layer. Loading or building FEmacs does not spawn any JavaScript runtime until explicitly started by the user.

## Responsibilities

- **Install & runtime validation (`femacs-eliscript-resolve-install`)**: locates the selected runner (`bun` or `node`), validates an existing frozen worker package directory and client closure (`eliscript-value-codec.el`, `eliscript-value-stream.el`, `eliscript-worker.el`, `eliscript-service.el`), and fails cleanly with typed `missing-js-runtime` or `missing-worker-resource` errors without falling back to source bootstrap.
- **Capability gate (`femacs-eliscript-require-capabilities`)**: checks advertised `ready` capabilities and optional live probes, signaling `unsupported-capability` when a runner does not fulfill a declared requirement.
- **Buffer analysis (`femacs-eliscript-analyze-buffer`)**: runs asynchronous document statistics or search over target buffer contents, guards edits with modification-tick validation (`stale-buffer`), applies buffer annotations inside `atomic-change-group`, and renders structured correlation metadata into `*Eliscript Analysis*`.
- **Single-run side-effect contract (`femacs-eliscript-call-side-effect`)**: forbids dual-run `:verify 'always`, never replays failed requests automatically, and reports `side-effect-completion-unknown` when timeout, disconnect, crash, or post-dispatch cancellation leaves external state uncertain.
