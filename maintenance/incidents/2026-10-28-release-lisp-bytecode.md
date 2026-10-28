# Complete release Lisp bytecode coverage

The original debugger file completes 49 cases on the installed stable reference.
The earlier owned terminal release stops after four passing cases: source
navigation exits with excessive Lisp nesting, leaving 45 cases unexecuted.
Those unexecuted cases are not counted as semantic failures.

The release held only 182 bytecode files for 1678 Lisp sources, compared with
1640 bytecode files for 1673 sources in the installed stable distribution.
Forcing unchanged ERT/Edebug source libraries on the reference reproduces the
same excessive-nesting exit. The native stack shows repeated closure creation,
evaluation and mapping; it does not establish a VM defect. No depth limit,
original test, original library or evaluation behavior is changed here.

## Distribution change

The default generation phase now compiles all owned Lisp sources, following the
stable compiler-first and main-first ordering rather than only the dump closure.
The actual compiler handles file-local exclusions: `no-byte-compile` is a
recorded skip, not a successful output. Every other source requires a successful
compiler return and a fresh bytecode file. Old and orphan bytecode is removed
before the release phase. Explicit developer scopes remain available.

The source-only bootstrap dump can predate generated autoloads and Unicode
property definitions. Compilation explicitly loads the object-system helper and
owned generated character properties where available. Generated parser tables
remain a separate final compiler group with mandatory output verification.
Each group records source paths and compiler outcomes; exit zero alone is
insufficient. No reference checkout or installed editor supplies build inputs.

## Preserved failures and verification boundary

Initial cold generation first exposed an unloaded object-system helper, then
missing character properties. Both process failures are retained. A later
relocated-bootstrap diagnostic reached 1346 bytecode files but correctly failed
`require-with-check` because its dump retained old library paths. Additional
copied-bootstrap attempts exposed missing packaged runtime libraries and bare
startup preparation errors. These are retained as failed diagnostic preparation,
not represented as successful release builds or VM fixes.

A fresh production frontend build now performs bootstrap, complete generation,
final dump and packaging in one source-bound build directory. Both normal and
checking products compile 1645 of 1678 sources, with 33 compiler-declared skips
and zero compilation failures. The unchanged debugger file passes 49/49 on the
reference and each candidate; the unchanged C parser file passes 2/2 on each.
All 153 cases pass, product identities remain unchanged, frozen inputs verify,
and the controller exits zero with its owned processes fully reclaimed.

Focused filesystem/compiler-boundary protection passes nine scenarios on both
the working generator and the independently committable generator. These use a
compiler substitute; actual editor coverage is the fresh production build and
153 original cases above. The candidate's initialization, process/environment
setup and bytecode methods are identical to those of the exercised generator;
unrelated ongoing module/native changes remain outside this repair's commit.
Independent review accepts this bounded terminal repair and the independent
commit's bytecode-path dependencies. The actual frontend build uses the working
generator; method binding establishes the independent candidate's same bytecode
path, rather than claiming a second full candidate build. The legacy
`--all-bytecode` flag remains an alias for the now-default full range, not an
additional coverage mode. Character-property loading remains conditional on its
generated input existing; the complete frontend recipe generates it first.
Full stable corpus, Cocoa/UI delivery and the overall Goal remain open.
