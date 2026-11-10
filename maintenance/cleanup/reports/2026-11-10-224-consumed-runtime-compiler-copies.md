# CLN224: current runtime compiler copies and review metadata

The completed clean `9e257276` matrix automatically retired its consumed
compiler source copies in20 configurations, normal/checking legs. CLN224
publishes those40 existing canonical ledgers; publishing creates no new
original-input or product deletion.

| Already retired compiler inputs | Count or bytes |
| --- | ---: |
| Files |195,460|
| Directories |6,800|
| Public rows |202,260|
| Logical bytes |6,503,533,814|
| Prior per-path allocated bytes |6,888,300,544|

The [main public manifest](../manifests/2026-11-10-224-consumed-runtime-compiler-copies.jsonl.gz)
is12,903,259 bytes, SHA256
`0f821b64f4fed79d648c87e829979e5fb56f29ceef33b0d704f318b213aa372d`.
All202,260 eight-field records were independently reconstructed from40
content-addressed audit copies and compared to EOF. Each scope's normalized
stream, counts and original ledger SHA matches the completed build result
`0f2370146cbb291c91892b7a7ef31e1ae3a166d0cc743628c1084c68a992e54f`.
Machine paths and encoded compiler filenames are represented by hashes.

After independent review accepted the main projection, the temporary metadata
copies were retired by the unchanged canonical primitive, after actual verifier
subprocesses exited successfully:

| Temporary material | Files | Directories | Logical bytes | Prior allocated bytes |
| --- | ---: | ---: | ---: | ---: |
| Audited ledger copies |40|1|12,324,215|12,451,840|
| Redundant working publication copy |1|1|12,903,259|12,906,496|
| Total |41|2|25,227,474|25,358,336|

The [41-row mirror ledger](../manifests/2026-11-10-224-reviewed-ledger-mirrors.jsonl.gz)
has SHA256 `7b6d96146636ec583fe9d1fa02f2e7cb2cb101374bb26a61ad05f98fc2689431`.
The [2-row publication-copy ledger](../manifests/2026-11-10-224-reviewed-publication-copy.jsonl.gz)
has SHA256 `6524db2129ccb583924ec6ccdd0e036a1c222c8f2f76212a6ca03908b70a9c08`.
The latter copy was moved into an isolated owned child before canonical
retirement; its identical public original stayed unchanged. Both selected
temporary directories and the original working-copy path are absent.

Original compiler ledgers, source inventory, current products, observations
and the original verification receipt remain retained and unchanged. Retired
compiler copies can be regenerated from retained inputs, without promising old
random names or inode identities. Review mirrors can be copied again from the
original ledgers/public manifest. Per-path allocation is not a whole-volume
causal reclamation measurement. No semantic, performance or Goal acceptance
follows from this publication.
