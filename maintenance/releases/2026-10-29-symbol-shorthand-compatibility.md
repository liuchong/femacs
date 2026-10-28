# Stable symbol shorthand compatibility — 2026-10-29

The pinned stable31.1 target expands `read-symbol-shorthands` in the string
arguments of `intern`, `intern-soft` and `unintern`. The later original port
baseline removed that behavior. Consequently, the unchanged original
`lread-unintern` case passed on the installed stable comparator and failed on
all four old terminal/Cocoa normal/checking products. The first failure was
the expected expanded symbol name, not mutation of a symbol or shared string.

P424 restores the three Lisp interfaces using the existing shorthand lookup
helper and obarray paths. Symbol arguments retain exact-identity lookup and
removal; internal C-string interning and the reader remain unchanged. Character
and byte lengths remain separate for multibyte names. Temporary longhand memory
is freed on normal new-symbol and existing-symbol paths. Allocation-related
nonlocal exits retain the stable source's unwind limitation; this component
does not claim exception-safe allocation cleanup or measured leak elimination.

The source comparison uses stable commit
`a360712c9d272d950d8d8255ef74570f7e90b7d9` and original port baseline
`fd7afced4036fc1a221567d70705d20a2bd64713`. Both were read from already-present,
read-only objects; the reference checkout is not a build dependency.

## Actual build and original integration

Four fresh products and matching dumps build successfully; both Cocoa strict
signature checks pass. Build cleanup completes in1067.523seconds, with
890109952 bytes peak accounted memory. The frozen source includes other retained
work; the results do not attribute every product difference solely to P424 or
certify a clean checkout of this isolated component as the whole current build.
The actual tested `src/lread.rs` SHA256 is
`26f64db76e7be2c4bbd32a4b66789ae706eb0a6bd1ae61228c37d1c6e5dae851`.

The unchanged complete original files `lread-tests.el`, `print-tests.el`,
`syntax-tests.el`, `alloc-tests.el`, `data-tests.el`, `eval-tests.el` and
`fns-tests.el` run on the installed comparator and all four new products:
1945 passes,10 original conditional skips,zero failures/timeouts/not-run cases.
Each product registers the same391 cases and matches the comparator's individual
statuses. All four previously failing `lread-unintern` instances now pass.
The two collation cases per product retain their original
`fns-tests--collate-enabled-p` guard returning nil; raw passed:false and native
exit1 are preserved. Source, inputs, harness,
executables, dumps and packaged resources remain unchanged during acceptance.
Integration takes43.138seconds; controller cleanup completes in47.012seconds.
These test/build durations are not performance measurements.

## Post-integration protection

`tools/fixtures/symbol-shorthand-protection.el` contains two bounded ERT guards:
existing longhand identity with repeated hits, malformed mapping entries and
first-match order; and exact symbol arguments with overlapping literal/expanded
names and dynamic-binding restoration. It asserts native primitive entry points.
The32-hit loop checks identity and absence of duplicate symbols, not allocator
accounting. It does not replace the original integration above.

Both cases pass on the comparator and each new product:10 current passes.
The old Cocoa product, selected only for the existing-longhand negative control,
actually fails as expected. Its failure and child exit1 remain in the receipt.
Fixture and product identities remain unchanged; the complete control run takes
17.274seconds, native exit0 and full cleanup21.524seconds.
Fixture SHA256:
`3763fabb31fe8791f39f9141071e947b37a435fb75859a95fbf8252ecc0ef5a4`.

With a matching program/dump and an existing output directory:

```sh
FE_SHORTHAND_RESULT=build/shorthand-protection.json "$PROGRAM" \
  --dump-file "$DUMP" --batch -Q \
  -l tools/fixtures/symbol-shorthand-protection.el -f fe-shorthand-protection-run
```

| Product | Executable SHA256 | Dump SHA256 |
| --- | --- | --- |
| cocoa-normal | `60bbc73b3dff57022652349db940486340e593f27afbb9d7d62ce446057e22ae` | `cf22208849f7ab91cca3a06121e8aeea23d1586fdbba649cd139f788d1dc04c0` |
| cocoa-checking | `03968a8e95a666ff324cd617a85ed35a44dbe804abce75d9174177180229e3e2` | `239e4ac241c0e8406f6f123ce724149b179c45769d281a43cb780af50e244445` |
| terminal-normal | `f47820db1251ebd8f43ac44b38b1e0a6e6065f92b15d20008c2fa16ecd6c63f7` | `9ccd254551a34a437156f9985ccf1850d99b5e573d5719241644805d201a8eb6` |
| terminal-checking | `0c55183d0d9acc58179cd70caf32d58ca9ffc6d0f994e86a5b9f4ee0b5fe29f5` | `48d81739bcf2a3ec561abe5a84a61ebbd7e86f8b166f6882e9058093b7eab9be` |

Raw records remain in the corresponding P424 build, stable-core-functional and
shorthand-protection directories under `build/acceptance`. This is a bounded
compatibility component. The same independent partner read the source, fixture
and actual build/integration/negative-control evidence and accepted this limited
component and its boundaries. The earlier remote09 intermittent Base64 failure,
remaining stable corpus, experimental native/GC configurations and full Goal
acceptance remain open; this correction does not waive them.
