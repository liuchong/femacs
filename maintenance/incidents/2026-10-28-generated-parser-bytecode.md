# Generated parser tables require release bytecode

The original C preprocessor simulation found no tags on all four owned baseline
products, while passing on the installed stable reference. Input C files existed
and were byte-bound. Source-test context checks alone did not close this failure.

## Cause and isolation

C-mode source crosses did not change either engine's result. Parser activation,
parse-table setup and lexical token production succeeded. Reloading the parser
implementation as source also left the failure unchanged.

The generated C grammar source was identical, but its compiled form was absent
from the owned package. Its action closures became `interpreted-function`
objects. The original Bovine parser dispatch recognizes compiled functions or
legacy lists, so these closures were treated as unmatched grammar symbols.
Loading the same uncompiled table into the installed reference reproduced the
failure; compiling it with the owned program restored the original two-case
C test result. These crosses diagnose the cause; they are not release acceptance.

## Repair

Default release bytecode generation includes every declared Bovine and Wisent
parser output. Missing source inputs fail explicitly; stale compiled parser
files are removed before compilation; all new compiled outputs are required
afterward because successful batch process exit alone does not prove compilation
succeeded. Compile tables in a fresh process after the bootstrap closure: the
first clean build exposed an object-system dependency failure when tables shared
a compiler process with earlier bootstrap inputs. Its original failure is kept;
a populated checking leg does not prove a cold normal release worked.
A second fresh build showed process separation alone was insufficient. The
bootstrap dump predates regenerated autoloads, so table compilation explicitly
loads `cl-extra`, which defines the needed `cl-find-class`. Compilation of that
library alone does not install its definitions into a fresh process.
Explicit bytecode scope behavior is preserved.

## Actual verification and limits

No Lisp source, grammar, original test, lexer algorithm or VM dispatch behavior
is changed for this repair. Four fresh complete baseline products (terminal and
Cocoa, normal and checking) contain all eight compiled tables; both Cocoa
signatures pass. Three unchanged original files on the reference and those four
products complete 60 cases: 45 pass, 15 skip, none fail. The same three original
skip conditions occur on every product; they remain skips, so the original
aggregate remains false. C parsing is 10/10, Makefile 5/5, and getset 5/5.

Actual cold generation from source-only Lisp inputs produces all eight tables
without changing their source bytes. This also passes using the independently
committable generator, not only the working generator with other ongoing
changes. Focused component protection passes five cases on each version; the
compiler substitute checks file-generation boundaries, not editor compatibility.

The first two fresh-build failures and an initial relative-path test-fixture
failure remain recorded. Full stable corpus acceptance, clean delivery of other
ongoing changes, actual UI acceptance and the modernization Goal remain open.
