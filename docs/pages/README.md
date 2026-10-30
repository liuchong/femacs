# Fe project website

Entry: `docs/index.html`. Styles, approved artwork, three topic pages and the
maintained interaction source stay under `docs/pages/`. This is a static
website with an English body and switchable Chinese summaries, not a complete
translation of every engineering document.

The site uses the published `eliscript@0.1.1` browser host from unpkg. It fetches
`site.eli`, compiles it to an ESM Blob in the visitor's browser and runs it there.
The small HTML module only loads the compiler, resolves SDK imports and shows
an explicit failure state. Tabs, copying, filters, search and summaries are
implemented in Eliscript. No generated JavaScript or compiler bundle is checked
in. Internet access to the pinned SDK is needed for interactions; if it fails,
all command panels and static content remain readable and controls are disabled.
The copied logo is byte-identical to `packaging/assets/fe-logo.png`.

## Local preview and verification

From the repository root:

```sh
python3 -m http.server 8080 --bind 127.0.0.1 --directory docs
```

Open `http://127.0.0.1:8080/index.html` and stop the preview when finished.
This serves only documentation. Links to files outside the documentation
publication root point to their source-hosting location; they may require the
repository owner's access while the repository is private.

[Acceptance](acceptance.json) binds the seven browser-tested source/resource
hashes and current hashes. Later literal content corrections remove unsupported claims, correct command
ordering and modifier names, and require matching native-fixture builds. Exact
base/current hashes and the revision chain are retained. HTML attributes,
Eliscript and CSS are unchanged; the unaffected browser matrix is reused.
Actual Chrome checks cover the four pages at a docs root and a project subpath,
in 1440x900 and 390x844 viewports: 16 route/viewport checks. They passed live
browser compilation, loaded artwork, width checks, tabs by mouse and arrow/Home/
End keys, correct summary language metadata, outcome filtering and document
search/empty state. Clipboard success matches the exact source text; permission
denial displays selection guidance rather than false success. A blocked SDK
request separately verifies visible static commands and disabled controls.
There were no unexpected browser errors or HTTP failures in the normal runs.
The final bounded browser run took 32.887 seconds and fully reaped its owned
browser/server processes. Screenshots and original controller evidence stay
private; source files were unchanged during verification.

## Publication boundary

The website is implemented and locally verified. DNS/CNAME configuration does
not mean it is online. The current private repository's GitHub plan rejected
Pages creation; the repository remains private. Do not change visibility, buy a
plan, create a remote workflow or claim a working HTTPS website merely to close
this task. Publishing can continue when an authorized hosting option is available.
No remote CI was used. The README links to the local site source.

## Product claims

The sole modernization plan is `docs/modernization.md`: G1/G2/G4 accepted,
G3/G5/G6/G7 open. The site distinguishes actual macOS trial functionality,
configuration-specific native support and experimental collector candidates.
It does not certify all packages/platforms, a qualified speedup or a new default.
The intermittent remote decoder failure remains open without captured bad input.
