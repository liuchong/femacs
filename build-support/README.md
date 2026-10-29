# Owned build descriptions

These text inputs describe the supported Apple Silicon macOS terminal and Cocoa configurations. They replace runtime discovery from an optional reference checkout: ordered core object names, selected library members, system libraries, configuration macros and native symbol/global metadata. No object code, installed editor or dump is included. `symbols.json` declares Cocoa Lisp names and forwarding storage types; `tools/gen-macos-runtime.py` checks forwarding names against the actual Swift source.

`generator-recipes.json` stores repository-relative raw-data conversion recipes transcribed from the fixed baseline during migration. All data and generator programs are committed under `admin/`, `leim/` and `lisp/`. Recipes are executed by `tools/bootstrap-resources.py`, which supplies the newly built owned program, never a reference program. GNU awk and gzip are external tools. `doc-sections.txt` supplies only section order; documentation text is scanned from owned code.

The historical baseline is fd7afced4036fc1a221567d70705d20a2bd64713. Changing target features or library membership is a source change requiring a clean build and functional verification; these descriptions do not dynamically detect unsupported CPUs or SDKs. The builder rejects non-arm64 macOS hosts. Diagnostic/differential tools may optionally use a separately obtained read-only reference, but normal build and generation do not.

`module-api/` contains the original public C ABI template and versioned fragments,
with provenance and per-input identities in `sources.json`. These are module
development headers, not editor implementation units. The base generation step
renders the header from tracked inputs and the owned version; it requires neither
the optional source checkout nor an installed header. Terminal builds expose it
at `port/include/emacs-module.h`; terminal packages and Cocoa Resources include
the same generated API under `include/`. Updating its version or source requires
an explicit ABI change and original module compatibility checks. Current macOS
configs connect modules and pthreads; unchanged historical products do not
certify that new capability connection.

Both macOS configurations also enable the existing gzip/zlib region
decompressor and link the system zlib. This is a target capability, not a
dependency on a reference checkout or on an external gzip process. A feature
change requires fresh products and dumps; old disabled builds cannot certify it.

Both macOS targets also enable the existing LCMS color calculations. Install
`lcms2` and `pkg-config` before building; `FEMACS_LCMS2_PREFIX` can select an
existing installation. The builder binds the header and library closure before
compilation, stages and signs owned library copies, and packages their licenses.
Terminal packages and Cocoa applications load these bundled copies through
relative runtime paths; they do not require the build machine's installation.

Both macOS targets now select the complete stable Tree-sitter feature. The
supported independently installed SDK is `tree-sitter` 0.27.1 (grammar ABI
13–15) plus `pkg-config`; `FEMACS_TREE_SITTER_PREFIX` can select its prefix.
The build binds the exact API header and library closure before compilation,
retains that header, and ships signed parser runtime libraries and their
licenses using relative product paths. A different SDK requires explicit
ABI rebinding rather than silently trusting only the grammar ABI number.
The ignored reference checkout and installed reference application are never
build inputs. P430 runtime/ABI acceptance remains pending during construction.
