# Current-source runtime matrix: build and fixture preparation

The complete matrix was built from the clean, signed source `59d4215c9d0eb3706f366941d3c40e3a2b879dd5`. One frozen inventory binds 7,576 inputs. All 20 configurations completed both normal and checking legs, yielding 40 candidate product receipts and 61 registered profiles, including the installed reference. There are 41 distinct program/dump pairs, 32 native candidate profiles, 12 diagnostic profiles and 48 nondiagnostic candidate profiles.

This closes **build only**. Bootstrap, dump and resource generation used actual compilers/editors; the runtime semantic acceptance suite has not run. No historical failed observation was reused or promoted. G1/G2/G4 remain accepted, 3/7; GA3/GA5/GA6/GA7/GA8 remain open.

## Configuration results

Every row represents two successful build legs. All child process receipts report complete, exit zero, no timeout and no remaining process group.

| Configuration | Terminal seconds | Cocoa seconds |
| --- | ---: | ---: |
| native-collector observer diagnostic | 661.780 | 556.265 |
| baseline | 446.569 | 495.531 |
| baseline observer | 440.240 | 516.182 |
| native | 493.318 | 584.526 |
| native observer | 529.161 | 600.618 |
| collector | 510.888 | 593.558 |
| collector observer | 501.311 | 538.904 |
| native-collector | 488.924 | 550.899 |
| native-collector observer | 480.569 | 554.286 |
| baseline observer diagnostic | 435.641 | 502.018 |

The controller exited zero with complete cleanup after 10,692.071 seconds. Initial allowance was 8 GiB, hard limit 16 GiB, growth zero; peak tree accounting was 920,961,480 bytes. The recorded session peak was 7,191,397,096 bytes under its separate 24 GiB limit. These are protection/build readings, not G5 workload performance evidence.

## Exact evidence identities

The retained evidence root is `build/acceptance/runtime-review-p453-1`.

| File | Bytes | SHA256 |
| --- | ---: | --- |
| results.json | 1,129,102 | `e3a68738aa2c8bfea97ce0900e8822a10dd55923e390b88f5d9b820e8d1091da` |
| products.json | 181,588 | `cae6cecc38754885f981760870c80b68c3a541b926351e3e38634d1a150227aa` |
| source-inputs.json | 1,492,824 | `fd46d00644f960a1617ae966f55f13f98ddb434f1dddfaf9fe705c73d9e18de1` |
| artifact-preservation.json | 76,384,053 | `9fd112fa5d3af52fbb30d1b35eebb210c291800371134b396c38930fc10d06a5` |

The retained release-mode build driver is `def646d76cffca19c5fc798496cf8688544f0e7cd538d7edfd0ceced0311b610`; coordinator source is `38f2e1d8a5347dcfc81ef505fba72afa07b516613992b2ef7b1ba7d933ca0fcd`. Closed controller stderr is 677 bytes, SHA256 `c20fc817d85612ced75620e58753cdf9ee22e6f616d155c9525908b97e3d3249`.

The journal is released with no release failures. File retention verified all 198,268 regular files and 20 links; no duplicate content archive was created. Forty consumed compiler-copy retirements completed during the build; their full public identity projection and restoration boundary are recorded in [CLN220](../cleanup/reports/2026-11-10-220-consumed-runtime-compiler-copies.md).

## Native negative fixture preparation

Preparation subsequently passed for all 32 native candidate profiles using the same frozen source and products digest. The native-unit build child completed in 293.392 seconds, exit zero, no timeout or remaining process group. Overall preparation took 527.641 seconds; its controller exited zero with complete cleanup, peak accounting 3,328,065,536 bytes, initial 4 GiB/hard 16 GiB and no growth.

`build/acceptance/runtime-review-p453-prepare-1/results.json` is 4,023 bytes, SHA256 `e58e13d3ed54965f5713e4bde8ce5e0fd136274c4f254bf429e920e439b76457`. The bound inner preparation receipt is 2,919,429 bytes, SHA256 `916c8267b0700789f505b6fe836df8a9b04ef10abb1d1e2ad461b253d39a21ea`; all 32 product rows pass and full product inputs remain unchanged. Closed controller stderr is 680 bytes, SHA256 `a2ff812e98aa33c2ba0c11c9f057318dc01854dbca3dee557667509ea17b5669`.

Preparation verified retention of 204,742 files and 20 links, created no duplicate archive, and released its journal without failures. This prepares test artifacts; `semantic_tests_executed` and `protective_component_tests_executed` remain false.

## Affected standard-terminal TTY refresh

The separately delivered P453 terminal product completed all six existing TTY integration cases: missing/regular-file/directory targets, detached startup without a controlling TTY, undefined terminal type, and recoverable terminal/frame creation failures. No timeout; program/dump identities before and after agree. Case execution took 210.612 milliseconds; the controller exited zero with complete cleanup, 55,508,992 peak accounted bytes and no growth. This does not rerun or certify physical GUI/IME workflows.

`build/acceptance/review-p453/goal-stability-tty/results.json` is 12,213 bytes, SHA256 `15da118e0ad8b74ff8ba342765eba6f2c66631203042896ad8c2642908e1b1e7`; closed controller stderr is 671 bytes, SHA256 `4bc77db59b13b30aa48967cb6982106baf46d558e2c14217fd0ae2c5ff89e2c6`. This is a separate bounded functional refresh; the 61-profile runtime semantic suite remains unexecuted.

## Input and acceptance boundaries

- The live official release directory lookup observed 31.1 as the highest stable archive on 2026-11-10. The upstream tag object `1f9518d22ae0fe4490ea9761ea7d39fbb970a890` resolves to commit `a360712c9d272d950d8d8255ef74570f7e90b7d9`, exactly matching the stable corpus manifest. Sources: [official archive directory](https://ftp.gnu.org/pub/gnu/emacs/) and [tag object](https://api.github.com/repos/emacs-mirror/emacs/git/tags/1f9518d22ae0fe4490ea9761ea7d39fbb970a890). This establishes version provenance, not compatibility success.
- The explicit Tree-sitter 0.27.1 SDK route is documented in [bootstrap prerequisites](../../docs/repository-bootstrap.md#pinned-tree-sitter-sdk). The system 0.27.2 installation and the existing version/header gates are unchanged.
- Two initial supplementary source-review narratives used stale symbols and were withdrawn. Six subsequently bounded source facts were checked against eight current file hashes and exact call sites. No global static-correctness or runtime acceptance claim follows from those facts.
- Next: perform semantic/negative/dump/relocation, full stable-corpus and component acceptance. Review execution scale and quiet-host conditions before performance measurement. The 48 stable candidate profiles require 53,280 original file calls; none are claimed complete here.
- Existing P453 trial products/package remain separately identified in [the closure-hook delivery](2026-11-10-interpreted-closure-hook.md). Installed Fe still waits for the user to save and exit; this matrix does not claim the installation was replaced.

Current frozen inputs, selected products and the pinned SDK remain needed by downstream consumers. Retire them only after those consumers finish, with an explicit replacement/restoration record.
