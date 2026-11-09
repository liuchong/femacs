# P453: interpreted closure hook correctness

Final review of the eight commits from `a404c000` through `d22daf1e` found an observable closure-creation regression. A symbol-name shortcut bypassed advice and replacement definitions of the interpreted converter. The same call invoked before advice once on the installed 31.1 reference and zero times on both candidates. P453 restores the actual hook call; other hot paths remain. Independent source review accepted this boundary.

## Rebuilt products

Both standard targets were rebuilt from the reviewed working source, including the retained JNI declaration WIP. Terminal build: 257.375 seconds / 423,378,944 peak accounted bytes. Cocoa build: 276.474 seconds / 835,092,480 bytes. Both controllers exited zero, completed cleanup, and used a 16 GiB hard limit without growth. Cocoa strict signature verification passed.

The installed Tree-sitter dependency had advanced to 0.27.2. The unchanged SDK gate rejected it before compilation. The rebuild uses an independently captured official 0.27.1 bottle, with the required header digest `f8480415ae20cb5e0fd519ba44c84268c5acb4c918106238fc1d328d91431783`; the system installation and version gate were not changed. The first existing-staging refusal is also retained.

| Product | Program SHA256 | Dump SHA256 |
| --- | --- | --- |
| reference | `e4fef3c33a93e3d19924e81dc126d2d976bac6de10df7d67497838f69bcfdb4a` | `a7e3e3086ef817c7fbec000050cc1855a668e961a58dabd81f8f19bcf1304b20` |
| terminal | `2c5d40a28d21dc3970068ea340a63a4dfb94a728c2698904ee4f006404386e75` | `b02b79d0691af46c2b318a14378d872ba99582c8df344374c51c04c5ffc2a84c` |
| cocoa | `fe9d871c3e46756462f2adcbe733a1a9c334afcfdfb3ca3d5d6d7b9827ea317c` | `2bc0cc343ee60dc71ee114fbac2bb441fbedda33d8b4e279614f9d6929010d02` |

## Actual acceptance

- Six closure paths on three real products agree: advice count 1/result 3; replacement count 1/result 6; captured value 42; existing lexical binding retained at 12 after the local declaration; hook error propagates; hook-disabled result 9. Eighteen observations, no compiler/editor substitution. The first private fixture incorrectly expected 22 on the local-declaration case and failed on all three products; that attempt remains a failure.
- Eight complete stable-31.1 original files on three products: 24 completed file runs / 1,248 observations, comprising 1,224 ordinary passes, 15 original expected failures and nine original skips. Zero unexpected outcomes; per-file status maps agree. Files: eval, data, alloc, fns, cconv, bytecomp, cl-lib and cl-macs. Original bodies and assertions are unchanged. Raw expected failures and skips are not counted as ordinary passes.
- The first broader sweep accidentally used the development cl-macs file. The 31.1 reference reached the 4 GiB controller limit after 40/79 completed events; its aggregate remains interrupted/137. The other seven files are byte-identical to the stable manifest. A separate run of the manifest-bound stable cl-macs file completes 77 cases per product, with zero unexpected outcomes. This is a file-bound union of completed observations, not a passing rewrite of the interrupted sweep.
- After functional acceptance, three hook protections were added. The complete hot-path protection file passes 8/8 and the existing bytecode fixture 10/10 on each product: 54 ordinary passes.
- Current local acceptance passes, including terminal user-init, real PTY keys/file workflow and Cocoa batch/signature layers. Actual GUI input and IME were not requested by this rerun. The bounded stable language/release suite passes 16 cases per product (48 total), not the full stable corpus.
- Stable fixture contracts, runtime-stage contracts, measurement-report contracts, retention contracts, nine actual preservation/OS cases, seven client-receipt layout protections and own-voice pass. Two initial tool invocations omitted required output arguments; only those invocations were corrected and rerun, retaining their exit-2 logs.

## Evidence boundary

Full logs, per-case durations, input/product digests and controller receipts remain in the private review evidence. Public artifact identities above identify the actual tested candidates. This repair does not establish full 555-file/profile compatibility, quiet-host speedup, native or generational collector adoption, current physical GUI/IME acceptance, or completion of the seven-part Goal. GA3/GA5/GA6/GA7/GA8 remain open. Local packaging/installation and cleanup are recorded separately.
