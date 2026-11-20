# Stage III (`GA5–GA8`) Closure Receipt and Final `GA1–GA8` Reverse Audit (`2026-11-20`)

## 1. Bound Source, Product, and Reference Identities

- **Source snapshot**: `eef7e612b7daf4b5ff07f2e0a7641f6d8ca716e9` (`git diff 26d81b95..eef7e612 -- src/` is empty; every VM, compiler, allocator, collector, and observer unit in `src/` is byte-identical across the delivered standard products and the `native-2` / `frozen-2` / `native-collector-1` runtime cohort).
- **Delivered macOS terminal trial product (`build/terminal/port`)**:
  - `src/emacs`: `2,798,256 B`, SHA-256 `a0dd02e116b9f37e8eb6fc86be630118bb7d0d82af5e9f3c2c8cdaf5d7960241`
  - `src/emacs.pdmp`: `13,101,368 B`, SHA-256 `cd076b1dc17a9b2c6cac605347442604214bf18d49ef41cdfaace31d62bfac18`
- **Delivered macOS Cocoa trial bundle (`dist/macos/Femacs.app`)**:
  - `Contents/MacOS/Femacs`: `4,367,952 B`, SHA-256 `02f5adafe3c1a5fc86276632e521ed5228ed37d5cfe4443377437d799098104e`
  - `Contents/Resources/libexec/Femacs.pdmp`: `18,600,752 B`, SHA-256 `e1515db2344d096fdf23aea1a376ae9628d93993e00fef4b308aea198b639e6b`
  - Signature boundary: `codesign --verify --deep --strict` `exit 0`; `spctl --assess --type execute -vv` `exit 3` (`rejected`, local-trial ad-hoc unnotarized boundary).
- **Authorized installed `32.0.50` system reference (`fd7afced4036fc1a221567d70705d20a2bd64713`)**:
  - Executable: `3,309,776 B`, SHA-256 `23f49087480c51347b31535a509aa391f008c4b49023aa31ca6271332d1dcbd6`
  - Portable dump: SHA-256 `e1d2a9f05fd3f29074db316c7302cb45d27d4103b4b267c6897f9fcab4fa7d92`
- **Measurement fixtures**:
  - `tools/fixtures/runtime-profile.el`: SHA-256 `8e58aa04e5be90309e2a2cb739c206bd7477b89eac4460cd4bd11aaeb922b6b0`
  - `tools/pgo/eval.el`: SHA-256 `bb7c0c1ef65cd835a1d95f4a50e87ef751f07918d7ac6c0576bf16d476071173`
- **Host load accounting**: Both comparison plans retain `max_load_per_core: 0.10` unchanged. Per explicit user instruction on `2026-11-20` authorizing execution at the prevailing desktop load (`load_per_core = 0.2666..0.4288` across all rounds), the full `2` warmup + `7` measured round matrices and 7-round `bench.py` suites were executed under `controlled-run --profile runtime` (`run_id` `e1aae58936a74a6f94c3efbac0003609`, `90.591s`, peak RSS `218,218,496 B`, `exit 0`, `cleanup_complete=true`), preserving `quiet: false` honestly in JSON without altering any threshold.

---

## 2. `GA5` — Real Lisp Workload and Collector Attribution (`s3-g5-comparison-formal`)

- **Receipt**: `s3-g5-comparison-formal/results.json` (`312,304 B`, SHA-256 `238308bdbd2dbd8d9fdbd3778948691624a9d0b0d683299fb4d5678d0a3ba439`, `2` warmups + `7` measured rounds per candidate, `51.442s`).

### Summary Table (`n = 7` measured rounds after `2` warmups)

| Candidate | Load/core (`r1..r7`) | Peak RSS | Startup Med / p95 (ms) | `load` Med / MAD / p95 (ms) | `edit` Med / MAD (ms) | `regex` Med / MAD (ms) | `bytecode` Med / MAD (ms) | `gc-churn` Med / MAD / p95 (ms) | `gc-retained` Med / MAD / p95 (ms) |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `native-product-interpreted` | `0.303..0.311` | `48.99 MB` | `27.25 / 37.32` | `7.853 / 0.361 / 12.936` | `0.654 / 0.030` | `0.625 / 0.019` | `7.413 / 0.145` | `217.318 / 12.172 / 255.470` | `32.760 / 1.526 / 41.617` |
| `native-product-bytecode` | `0.303..0.303` | `48.84 MB` | `37.12 / 39.80` | `3.314 / 0.110 / 4.730` | `0.530 / 0.022` | `0.534 / 0.023` | `0.017 / 0.001` | `142.406 / 3.564 / 175.826` | `23.126 / 0.740 / 27.287` |
| `native-product-native` | `0.271..0.303` | `52.95 MB` | `39.86 / 52.94` | `6.674 / 0.608 / 9.171` | `1.044 / 0.188` | `1.108 / 0.052` | `0.252 / 0.015` | `99.281 / 6.091 / 107.851` | `24.749 / 5.374 / 33.540` |
| `collector-young-interpreted` | `0.285..0.285` | `50.23 MB` | `27.17 / 47.14` | `13.904 / 1.206 / 18.050` | `0.950 / 0.012` | `0.938 / 0.022` | `7.661 / 0.129` | `345.809 / 6.316 / 374.412` | `52.622 / 2.550 / 73.529` |
| `collector-major-interpreted` | `0.271..0.271` | `46.50 MB` | `27.37 / 45.05` | `7.969 / 1.281 / 9.250` | `0.743 / 0.037` | `0.726 / 0.078` | `12.482 / 1.559` | `313.529 / 25.563 / 415.812` | `43.915 / 10.663 / 77.052` |
| `reference-interpreted` | `0.280..0.280` | `58.87 MB` | `142.75 / 152.42` | `7.373 / 0.433 / 8.798` | `5.686 / 0.265` | `0.744 / 0.008` | `4.681 / 0.058` | `225.935 / 1.275 / 259.066` | `38.094 / 0.562 / 39.277` |

### GC Pause, Phase (`0, 3, 4, 10, 22, 23, 24`), Remembered-Set, Cadence, and Survival Attribution

- **`collector-young-interpreted` (`FEMACS_GC_COLLECTOR=young`, `FEMACS_GC_OBSERVATION=all`)**:
  - Verified `53` real Automatic minor collections (`reason 0 = 53`) per round (`edit`: `1` minor; `gc-churn`: `46` minors + `15` cadence-boundary `reason 4` Full + `1` `reason 5` Full; `gc-retained`: `6` minors + `2` `reason 4` Full + `1` explicit `reason 2` Full).
  - Remembered set (`valid = 1`): `115,050` (`load`), `116,016` (`gc-churn`), `127,573` (`gc-retained`) frozen non-cons registry entries.
  - `gc-churn` (`434` collections across `7` rounds, total `2,347.063 ms`): pause median `5.315 ms` / p95 `6.453 ms` / max `11.468 ms`; phase medians (ms): `phase 0` (whole cycle) `5.318`, `phase 3` (input-blocked) `5.316`, `phase 22` (`prepare_cycle` + `oldbits` revival) `3.208`, `phase 23` (frozen non-cons registry roots) `1.131`, `phase 24` (dirty/tainted old-cons children) `0.165`, `phase 4` (static roots) `0.039`, `phase 10` (`sweep_conses`) `0.693`; cons survival/death age buckets (`0, 1, 2–3, 4–7, >=8` cycles): `live = [2170, 9, 8, 4, 18439]`, `dead = [0, 319709, 310, 1127, 0]`.
  - `gc-retained` (`63` collections across `7` rounds, total `363.380 ms`): pause median `5.472 ms` / p95 `7.729 ms` / max `9.673 ms`; phase medians (ms): `phase 0` `5.474`, `phase 22` `3.311`, `phase 23` `1.177`, `phase 24` `0.214`, `phase 4` `0.042`, `phase 10` `0.683`; `live = [80, 1788, 6078, 10018, 20535]`, `dead = [0, 22489, 10, 20, 103]`.
- **`collector-major-interpreted` (`FEMACS_GC_COLLECTOR=major`, `FEMACS_GC_OBSERVATION=all`)**:
  - Verified `0` minor cycles (`reason 0 = 0`, `remembered_set = (0, 0)`).
  - `gc-churn` (`434` collections, total `2,036.453 ms`): pause median `4.560 ms` / p95 `7.206 ms` / max `10.802 ms`; phase medians (ms): `phase 0` `4.563`, `phase 4` `3.690`, `phase 10` `0.654`, `phase 22` `0.000`.
  - `gc-retained` (`63` collections, total `320.741 ms`): pause median `4.632 ms` / p95 `8.287 ms` / max `9.276 ms`; phase medians (ms): `phase 0` `4.636`, `phase 4` `3.695`, `phase 10` `0.610`.

### Complete 7-Round Measured Series (`s3-g5-comparison-formal`, ms)

- `native-product-interpreted`: `startup=[35.969, 37.321, 26.368, 34.094, 27.035, 25.829, 27.252]`, `load=[7.927, 8.049, 12.936, 7.492, 7.853, 7.163, 7.451]`, `edit=[0.663, 0.654, 0.847, 0.659, 0.621, 0.624, 0.622]`, `regex=[0.644, 0.602, 0.747, 0.625, 0.643, 0.599, 0.607]`, `bytecode=[7.412, 7.816, 8.310, 7.460, 7.268, 6.992, 7.413]`, `gc-churn=[213.893, 229.892, 255.470, 228.121, 217.318, 199.150, 205.146]`, `gc-retained=[33.947, 41.617, 31.120, 32.984, 32.760, 31.234, 29.389]`.
- `native-product-bytecode`: `startup=[37.068, 39.796, 37.121, 37.751, 27.549, 27.411, 39.547]`, `load=[3.225, 3.556, 4.730, 4.086, 3.245, 3.314, 3.204]`, `edit=[0.533, 0.530, 0.578, 0.552, 0.497, 0.508, 0.513]`, `regex=[0.511, 0.581, 0.542, 0.552, 0.500, 0.489, 0.534]`, `bytecode=[0.017, 0.017, 0.018, 0.018, 0.016, 0.016, 0.017]`, `gc-churn=[138.842, 148.518, 175.826, 155.963, 142.406, 140.956, 141.456]`, `gc-retained=[22.004, 23.855, 23.866, 27.287, 22.212, 23.126, 22.562]`.
- `native-product-native`: `startup=[39.860, 26.780, 39.946, 26.932, 38.989, 52.944, 40.923]`, `load=[9.171, 6.275, 6.674, 6.226, 8.178, 3.440, 7.282]`, `edit=[1.044, 0.773, 0.779, 0.516, 1.091, 1.232, 1.065]`, `regex=[1.108, 1.065, 1.108, 0.519, 1.160, 1.529, 0.912]`, `bytecode=[0.267, 0.133, 0.267, 0.252, 0.179, 0.253, 0.187]`, `gc-churn=[105.372, 86.870, 99.281, 104.084, 95.718, 107.851, 87.398]`, `gc-retained=[33.540, 22.483, 32.288, 19.375, 24.749, 24.594, 33.373]`.
- `collector-young-interpreted`: `startup=[27.761, 27.095, 27.449, 25.579, 27.027, 27.166, 47.142]`, `load=[15.239, 13.754, 13.904, 12.698, 12.821, 18.050, 15.310]`, `edit=[0.939, 0.950, 0.938, 0.946, 0.974, 1.489, 0.966]`, `regex=[0.976, 0.988, 0.918, 0.921, 0.916, 1.943, 0.938]`, `bytecode=[7.603, 7.790, 7.661, 7.069, 7.258, 10.583, 7.769]`, `gc-churn=[340.286, 339.493, 367.913, 331.352, 351.692, 345.809, 374.412]`, `gc-retained=[52.622, 55.947, 51.638, 46.727, 51.612, 55.172, 73.529]`.
- `collector-major-interpreted`: `startup=[27.190, 27.219, 38.439, 39.154, 45.051, 26.888, 27.374]`, `load=[6.442, 5.944, 9.250, 7.969, 8.748, 8.289, 6.437]`, `edit=[0.735, 0.663, 0.777, 0.780, 0.845, 0.743, 0.705]`, `regex=[0.726, 0.633, 0.690, 0.804, 0.839, 1.209, 0.691]`, `bytecode=[10.158, 8.982, 12.482, 12.960, 13.958, 14.041, 10.215]`, `gc-churn=[227.921, 220.054, 313.529, 329.442, 415.812, 324.128, 287.966]`, `gc-retained=[32.543, 33.252, 43.915, 66.513, 77.052, 41.072, 50.351]`.
- `reference-interpreted`: `startup=[135.866, 136.641, 146.914, 142.747, 137.943, 152.418, 148.153]`, `load=[7.165, 8.256, 6.940, 7.373, 8.109, 7.203, 8.798]`, `edit=[5.359, 5.662, 5.863, 5.686, 6.010, 5.421, 6.249]`, `regex=[0.728, 0.724, 0.752, 0.752, 0.763, 0.744, 0.741]`, `bytecode=[4.642, 4.650, 4.529, 4.781, 4.681, 4.860, 4.739]`, `gc-churn=[225.935, 223.957, 224.955, 232.590, 227.210, 225.653, 259.066]`, `gc-retained=[37.532, 38.094, 39.130, 37.824, 37.799, 39.269, 39.277]`.

---

## 3. `GA6` & `GA7` — Adoption and Rejection Decisions

### `GA6` (Interpreter, Bytecode VM, and `libgccjit` Native Backend)

- **Semantic & lifecycle evidence**: `stable-compat.py` `48/48` (`ce2a4c44…`), `runtime-hotpath-protection.el` `10/10` × 3 products, `runtime-call-frames-functional.el` `7/7` × 3 products, `bytecode-runtime.el` `10/10` × 3 products (`2dc5fc11…`), `runtime-native.el` + `runtime-native-lifecycle.el` `17/17` on `native-2`.
- **Measured trade-off (`execution-native-vs-interpreted` & `execution-native-vs-bytecode`)**:
  - `libgccjit` native beats `interpreted` on `bytecode` (`0.0340x`) and `gc-churn` (`0.4568x`), and beats `bytecode` on `gc-churn` (`0.6972x`, `99.281` vs `142.406 ms`), **but** incurs `420.3–440.8 ms` per-thunk JIT compile setup (vs `0.016–0.171 ms` for bytecode), runs `14.89x` slower than our peephole-fused bytecode VM on `bytecode` numeric loops (`0.252 ms` vs `0.017 ms`), `1.97x–2.08x` slower on `edit`/`regex`/`load`, raises max GC pause (`14.690 ms` vs `6.753 ms`) and peak RSS (`52.95 MB` vs `48.84 MB`), and depends on an external `libgccjit` + Homebrew `gcc` (`emutls_w`) driver.
- **Verdict**: **Adopt** the modernized interpreter, bytecode VM, allocator, and primitive hot-paths (`src/eval.rs`, `src/bytecode.rs`, `src/alloc.rs`, `src/fns.rs`, `src/sort.rs`) in standard deliverables; **keep `libgccjit` native opt-in** via `xtask runtime-build --variant native --backend-prefix <prefix>`, with standard standalone builds preserving `HAVE_NATIVE_COMP=undefined` and clean `(native-comp-available-p) -> nil` fallback.

### `GA7` (Generational `Young` Collector Candidate)

- **Correctness, barrier, cadence, and stress evidence**: `docs/g7-young-root-store-binding.md`, `docs/gc-young-cadence.md`, `test-runtime-contracts.py` `28/28` B-vectors (`35a980ea…`), `runtime-gc.el` `10/10` (`young` & `major`), `runtime-gc-fallback.el` `rtg-fallback-01` `1/1` (`f60b8be1…`), `runtime-gc-root-phases.el` `rpc01` `1/1`, `gc-young-cadence.el` `2/2` (`young` & `major`), `g7-native-write-barrier.el` `3/3` (`b52385f8…`, native-compiled `setcar`/`setcdr` into surviving old cons across `reason 0` Automatic minors), and 564-file forced-Full `--gc-stress` (`gc-stress-current-1`).
- **Measured trade-off (`collector-young-vs-major`)**:
  - Across `53` real Automatic minors per round, non-moving `Young` is **`1.1030x` slower** on `gc-churn` (`345.809` vs `313.529 ms`, median pause `5.315` vs `4.560 ms`), **`1.1983x` slower** on `gc-retained` (`52.622` vs `43.915 ms`), **`1.7448x` slower** on `load` (`13.904` vs `7.969 ms`), and uses `1.0803x` peak RSS (`50.23` vs `46.50 MB`), because `phase 22` (`3.208–3.311 ms`) + `phase 23` (`1.131–1.177 ms` across `116k–128k` frozen non-cons registry owners) + `phase 24` (`0.165–0.214 ms`) + non-compacting block sweep (`phase 10` `0.693 ms`) exceed the `3.65 ms` saved on static roots (`phase 4`).
- **Verdict**: **Reject non-moving `Young` as the default collector** (do not claim a shipped default generational GC). Ship the optimized mark-sweep collector (`src/alloc.rs`, inline `pdumper` bitmap checks, immediate mark skips, all-dead cons-block release) in standard deliverables, and retain the verified `Young` implementation behind opt-in `xtask runtime-build --variant collector` (`femacs_gc_generational`).

---

## 4. `GA8` — Delivered Product Performance, Memory, and GC Latency vs Installed `32.0.50` Reference

### A. Matched-Mode Workload, Memory, Startup, and GC Pause Comparison (`s3-ga8-delivered-formal`)

- **Receipt**: `s3-ga8-delivered-formal/results.json` (`159,948 B`, SHA-256 `1f24b37b393000a1a2a4b372d0668d9cd427f9eabbe0226e933d4b6629d554bf`, `2` warmups + `7` measured rounds per candidate, `22.349s`).

| Comparison Axis (`n = 7`) | Peak RSS (`L / R`, Ratio) | Startup Med (`L / R`, Ratio) | `load` Ratio | `edit` Ratio | `regex` Ratio | `bytecode` Ratio | `gc-churn` Ratio & Median Pause (`L / R` ms) | `gc-retained` Ratio & Median Pause (`L / R` ms) |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- | --- |
| `delivered-cocoa-bytecode` vs `reference-bytecode` | `39.75 / 57.74 MB` (`0.6884x`) | `27.00 / 158.34 ms` (`0.1705x`) | `0.5625x` | `1.0355x` (`0.550 / 0.531 ms`) | `0.9616x` | `0.1140x` | `0.9674x` (`3.634 / 5.008`) | `0.9087x` (`3.850 / 4.989`) |
| `delivered-terminal-bytecode` vs `reference-bytecode` | `29.92 / 57.74 MB` (`0.5182x`) | `27.79 / 158.34 ms` (`0.1755x`) | `0.3047x` | `1.0436x` (`0.554 / 0.531 ms`) | `0.9200x` | `0.1079x` | `0.8483x` (`3.421 / 5.008`) | `0.7708x` (`3.396 / 4.989`) |
| `delivered-cocoa-interpreted` vs `reference-interpreted` | `39.71 / 58.88 MB` (`0.6745x`) | `25.93 / 148.11 ms` (`0.1751x`) | `0.8199x` | `0.1074x` | `0.8886x` | `1.5740x` | `1.0693x` (`3.318 / 4.434`) | `0.8759x` (`3.250 / 4.539`) |
| `delivered-terminal-interpreted` vs `reference-interpreted` | `29.28 / 58.88 MB` (`0.4972x`) | `26.99 / 148.11 ms` (`0.1822x`) | `0.7859x` | `0.1134x` | `0.8564x` | `1.4810x` | `0.7573x` (`2.737 / 4.434`) | `0.6660x` (`2.710 / 4.539`) |

Complete 7-round measured series (`s3-ga8-delivered-formal`, ms):
- `delivered-cocoa-interpreted`: `startup=[24.386, 26.833, 26.279, 25.929, 25.494, 24.616, 37.483]`, `load=[6.645, 6.039, 6.004, 5.984, 6.035, 7.084, 7.131]`, `edit=[0.610, 0.625, 0.612, 0.618, 0.605, 0.633, 0.637]`, `regex=[0.625, 0.641, 0.615, 0.639, 0.632, 0.655, 0.650]`, `bytecode=[7.483, 7.324, 6.739, 7.226, 7.161, 8.352, 7.728]`, `gc-churn=[223.485, 238.768, 228.358, 241.481, 249.387, 252.262, 258.555]`, `gc-retained=[33.849, 30.551, 29.727, 33.387, 33.972, 33.245, 51.697]`.
- `delivered-cocoa-bytecode`: `startup=[27.002, 26.384, 35.483, 24.887, 25.993, 34.526, 36.497]`, `load=[6.316, 7.446, 6.353, 10.627, 6.276, 6.474, 9.014]`, `edit=[0.529, 0.579, 0.555, 0.554, 0.531, 0.550, 0.540]`, `regex=[0.562, 0.565, 0.573, 0.578, 0.524, 0.583, 0.581]`, `bytecode=[0.018, 0.024, 0.018, 0.018, 0.017, 0.017, 0.017]`, `gc-churn=[172.334, 193.398, 206.234, 174.896, 165.653, 165.505, 218.951]`, `gc-retained=[27.993, 35.882, 29.776, 23.123, 24.220, 24.081, 31.124]`.
- `delivered-terminal-interpreted`: `startup=[25.289, 25.230, 27.162, 27.223, 25.474, 26.987, 27.537]`, `load=[5.707, 5.788, 5.855, 6.837, 5.278, 6.025, 5.788]`, `edit=[0.654, 0.650, 0.652, 0.697, 0.650, 0.611, 0.671]`, `regex=[0.616, 0.613, 0.603, 0.660, 0.605, 0.649, 0.621]`, `bytecode=[6.891, 6.489, 6.426, 7.190, 6.298, 6.980, 7.211]`, `gc-churn=[165.793, 193.095, 165.649, 171.023, 165.625, 218.591, 217.501]`, `gc-retained=[23.781, 26.429, 25.387, 24.675, 30.534, 24.191, 42.621]`.
- `delivered-terminal-bytecode`: `startup=[29.671, 25.180, 25.868, 27.323, 28.451, 27.787, 28.196]`, `load=[2.641, 3.540, 4.264, 3.507, 4.656, 2.712, 2.810]`, `edit=[0.542, 0.554, 0.564, 0.534, 0.531, 0.587, 0.640]`, `regex=[0.531, 0.559, 0.536, 0.548, 0.559, 0.549, 0.546]`, `bytecode=[0.017, 0.017, 0.018, 0.018, 0.017, 0.018, 0.017]`, `gc-churn=[153.358, 171.880, 206.852, 153.104, 156.465, 137.175, 140.029]`, `gc-retained=[27.721, 26.552, 36.618, 21.401, 22.444, 20.839, 23.744]`.
- `reference-interpreted`: `startup=[144.263, 136.600, 146.345, 150.364, 148.105, 153.011, 148.420]`, `load=[7.060, 7.950, 7.199, 7.240, 7.365, 9.135, 7.470]`, `edit=[5.970, 5.801, 5.652, 5.562, 5.752, 5.850, 5.669]`, `regex=[0.753, 0.719, 0.715, 0.771, 0.719, 0.714, 0.726]`, `bytecode=[4.653, 4.737, 4.595, 4.588, 4.645, 4.830, 4.682]`, `gc-churn=[224.965, 238.651, 223.107, 225.821, 221.177, 240.535, 235.054]`, `gc-retained=[38.696, 42.231, 37.045, 36.845, 37.113, 50.240, 38.117]`.
- `reference-bytecode`: `startup=[145.020, 158.339, 192.328, 170.062, 149.729, 159.980, 146.562]`, `load=[10.512, 10.870, 14.004, 13.862, 13.422, 11.510, 11.264]`, `edit=[0.504, 0.531, 0.535, 0.529, 0.643, 0.533, 0.493]`, `regex=[0.627, 0.593, 0.608, 0.596, 0.726, 0.574, 0.570]`, `bytecode=[0.152, 0.157, 0.156, 0.157, 0.839, 0.155, 0.162]`, `gc-churn=[173.228, 194.474, 206.276, 249.383, 180.791, 177.242, 171.917]`, `gc-retained=[29.696, 32.264, 34.107, 35.529, 27.647, 30.805, 27.291]`.

### B. Paired 7-Round `bench.py` (`tools/pgo/eval.el` 10 Workloads + Startup)

- **Receipts**:
  - Cocoa (`s3-ga8-bench-cocoa/results.json`): `25,442 B`, SHA-256 `fda8ceef09c82336b3034065ca363636c2b2d1832f2caa28559872b080e0a43c` (`load_per_round = [0.4288, 0.4288, 0.4288, 0.4034, 0.4034, 0.4034, 0.4034]`, `quiet = false`).
  - Terminal (`s3-ga8-bench-terminal/results.json`): `25,406 B`, SHA-256 `10b408980c1def2740e93f961bedf1ac25caacc1b31e126ac0555a278be15e88` (`load_per_round = [0.4022, 0.4022, 0.4022, 0.4022, 0.4022, 0.3878, 0.3878]`, `quiet = false`).

| Case (`n = 7`) | Cocoa Candidate Med / MAD (ms) | Reference Med / MAD (ms) | Cocoa / Ref Ratio | Terminal Candidate Med / MAD (ms) | Reference Med / MAD (ms) | Terminal / Ref Ratio |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `bc-loop` | `0.0088 / 0.0003` | `1.8821 / 0.0160` | `0.0047x` | `0.0081 / 0.0002` | `1.8699 / 0.0119` | `0.0043x` |
| `bc-sum` | `0.3250 / 0.0040` | `4.3240 / 0.0190` | `0.0752x` | `0.3290 / 0.0031` | `4.2460 / 0.0260` | `0.0775x` |
| `bc-car` | `0.0079 / 0.0000` | `1.8709 / 0.0799` | `0.0042x` | `0.0079 / 0.0000` | `1.7650 / 0.0112` | `0.0045x` |
| `regexp-repl` | `2.5840 / 0.0200` | `2.8410 / 0.0620` | `0.9095x` | `2.6240 / 0.0710` | `2.7359 / 0.0458` | `0.9591x` |
| `buffer-scan` | `0.9582 / 0.0093` | `1.1671 / 0.0282` | `0.8210x` | `0.9809 / 0.0160` | `1.1649 / 0.0050` | `0.8420x` |
| `sort-100k` | `0.4210 / 0.0090` | `1.9350 / 0.0279` | `0.2176x` | `0.4230 / 0.0060` | `1.9512 / 0.0317` | `0.2168x` |
| `load` | `1.7412 / 0.0341` | `1.6811 / 0.0780` | `1.0358x` | `1.7011 / 0.0462` | `1.6670 / 0.0220` | `1.0205x` |
| `edit` | `7.7350 / 0.0598` | `9.5670 / 0.0830` | `0.8085x` | `7.5541 / 0.0849` | `9.5749 / 0.0992` | `0.7889x` |
| `regex` | `2.1350 / 0.0179` | `2.6722 / 0.0577` | `0.7990x` | `2.1000 / 0.0319` | `2.6212 / 0.0501` | `0.8012x` |
| `gc` | `28.8129 / 0.2737` | `50.1649 / 1.0719` | `0.5744x` | `28.3959 / 0.4439` | `49.7420 / 0.4549` | `0.5709x` |
| `startup` | `20.3630 / 0.4116` | `184.6928 / 1.8383` | `0.1103x` | `20.3376 / 0.4248` | `185.4839 / 1.1287` | `0.1096x` |

Complete 7-round `bench.py` series (ms):
- Cocoa `candidate`: `bc-loop=[0.0081, 0.0091, 0.0079, 0.0091, 0.0079, 0.0088, 0.0091]`, `bc-sum=[0.3030, 0.3378, 0.3290, 0.3250, 0.3219, 0.3231, 0.3390]`, `bc-car=[0.0079, 0.0119, 0.0079, 0.0079, 0.0081, 0.0079, 0.0079]`, `regexp-repl=[2.5840, 2.6889, 2.5129, 2.5952, 2.5640, 2.5830, 2.6381]`, `buffer-scan=[0.9520, 0.9840, 0.9370, 0.9589, 0.9489, 0.9582, 0.9820]`, `sort-100k=[0.3979, 0.4342, 0.4261, 0.4129, 0.4210, 0.4120, 0.4330]`, `load=[1.7071, 1.8082, 1.7271, 1.7550, 1.7412, 1.7011, 1.9040]`, `edit=[7.6752, 8.2469, 7.4379, 7.7350, 7.7002, 7.8249, 7.7691]`, `regex=[2.1710, 2.1231, 2.1529, 2.1350, 2.0769, 2.1570, 2.1322]`, `gc=[28.5392, 29.1998, 28.2662, 28.7781, 28.8129, 28.9240, 37.4842]`, `startup=[20.7625, 20.4583, 19.9513, 19.4282, 20.3630, 131.4515, 19.4577]`.
- Cocoa `reference`: `bc-loop=[1.8661, 1.9109, 1.7571, 1.8749, 1.8821, 1.8902, 1.9240]`, `bc-sum=[4.1759, 4.3371, 4.3199, 4.3240, 4.3430, 4.2748, 4.4100]`, `bc-car=[1.8981, 1.7719, 1.7540, 1.8709, 1.8761, 1.7910, 2.0092]`, `regexp-repl=[2.9640, 2.9001, 2.8410, 2.7888, 2.7790, 2.7568, 3.2880]`, `buffer-scan=[1.1811, 1.1370, 1.1330, 1.1749, 1.1671, 1.1389, 1.2879]`, `sort-100k=[1.9720, 1.9519, 1.9150, 1.9071, 1.9350, 1.9028, 2.1508]`, `load=[1.8208, 1.6620, 1.6031, 1.6811, 1.9670, 1.6448, 1.9970]`, `edit=[9.5670, 9.5842, 9.6500, 9.4469, 9.4831, 9.5019, 10.0820]`, `regex=[2.7299, 2.6722, 2.6598, 2.6281, 2.7809, 2.6040, 2.8629]`, `gc=[50.5509, 51.6591, 48.6758, 48.9030, 49.0930, 50.6771, 50.1649]`, `startup=[181.2673, 187.3945, 184.6928, 186.5311, 186.3680, 183.3320, 178.8261]`.
- Terminal `candidate`: `bc-loop=[0.0079, 0.0081, 0.0079, 0.0119, 0.0079, 0.0088, 0.0081]`, `bc-sum=[0.3290, 0.3290, 0.3259, 0.4790, 0.3352, 0.3419, 0.3259]`, `bc-car=[0.0079, 0.0079, 0.0079, 0.0119, 0.0091, 0.0088, 0.0079]`, `regexp-repl=[2.5530, 2.6240, 2.5170, 3.2029, 2.6369, 2.6350, 2.5499]`, `buffer-scan=[0.9851, 0.9620, 0.9809, 1.2081, 0.9809, 0.9649, 0.9549]`, `sort-100k=[0.4170, 0.4139, 0.4251, 0.5140, 0.4230, 0.4098, 0.4239]`, `load=[1.6549, 1.9839, 1.7011, 2.4619, 1.8041, 1.6749, 1.6801]`, `edit=[7.5090, 8.0140, 7.5340, 7.9491, 7.8089, 7.4692, 7.5541]`, `regex=[2.0380, 2.1920, 2.1129, 2.1319, 2.1000, 2.0492, 2.0728]`, `gc=[28.3959, 27.7529, 28.0681, 30.0019, 28.6999, 29.1650, 27.9520]`, `startup=[19.2577, 19.7263, 19.5518, 20.3376, 20.7624, 20.6677, 20.6040]`.
- Terminal `reference`: `bc-loop=[1.7879, 1.8699, 1.8890, 1.8818, 1.8799, 1.7810, 1.8659]`, `bc-sum=[4.3190, 4.2260, 4.2009, 4.2460, 4.2961, 4.2200, 4.2651]`, `bc-car=[1.7660, 1.7538, 1.7650, 1.8880, 1.8940, 1.7529, 1.7569]`, `regexp-repl=[2.7521, 2.8009, 2.6560, 2.7359, 2.9740, 2.7020, 2.6901]`, `buffer-scan=[1.1661, 1.1649, 1.1599, 1.1129, 1.1690, 1.1880, 1.1311]`, `sort-100k=[1.9429, 1.9512, 2.0001, 2.0571, 1.8439, 1.9829, 1.9200]`, `load=[1.6649, 1.6301, 1.6890, 1.8890, 1.6670, 1.8721, 1.6608]`, `edit=[9.6741, 9.5122, 9.4330, 9.3699, 9.6951, 9.5749, 9.6071]`, `regex=[2.6140, 2.6870, 2.5711, 2.7242, 2.6798, 2.6050, 2.6212]`, `gc=[49.7179, 50.6670, 50.1969, 49.7420, 49.9742, 49.2520, 48.6901]`, `startup=[183.6831, 187.5337, 184.3552, 186.2000, 186.4581, 181.9710, 185.4839]`.

---

## 5. Final Item-by-Item Reverse Audit (`GA1–GA8`)

| Item | Acceptance Requirement | Bound Implementation & Raw Evidence | Status |
| --- | --- | --- | --- |
| `GA1 / G1` | macOS terminal & Cocoa workflows: external Finder drag, multi-window IME (`A` & `B` composition/commit/cancel/focus transfer), save/quit, 6-case no-controlling-TTY | `platform/macos/nsterm.swift`, `src/term.rs`, `tools/test-macos-{ime,drag,delivery,stability}.py`; `s1-ime-multi` (`07a5fcbf…`, `407` callbacks), `s1-finder-drag` (`b1dab2e1…`, `25` motion callbacks), `s1-local-acceptance-3` (`b7813993…`, `14/14`), `s1-tty-stability` (`6d0f381a…`, `16/16`); [Stage I closure](2026-11-20-stage1-ga1-ga2-ga3-closure.md) | **Accepted (`6065cf1b`)** |
| `GA2 / G2` | Clean tracked-only Git export independent resource generation, build, bundle, relocation, startup, strict `codesign`, Gatekeeper boundary | `xtask/src/{standalone,macos,bundle}.rs`, `tools/bootstrap-resources.py`; `s1-ga2-receipt.json` (`1311e0ad…`, `7,629` files, tree `ee9725fd…`, `17` `@rpath` dylibs, `codesign` `exit 0`, `spctl` `exit 3` local-trial boundary); zero production build/VM edits since `6065cf1b` | **Accepted (`6065cf1b`)** |
| `GA3 / G3` | Declared macOS `32.0.50` stable Lisp parity: `555` entrypoints + `11` subset, pinned-tool Eglot strict probes, raw failure conservation, `0` effective blocking | `tools/functional-strict-probe.py`, `tools/stable-{source-rules,classification}.py`; `s1-eglot-probe-insert-replace` (`951577dc…`), `s1-eglot-probe-text-edit` (`a5a40a64…`), `g3-scoped-consume-555-20` (`717c821e…`, `565/574` raw conserved), `g3-scoped-consume-subset-5` (`a0f0a6c2…`, `37/37`), `g3-effective-blocking-2.json` (`7d504814…`, `3 -> 0`), `s3-stable-compat-32` (`ce2a4c44…`, `48/48`) | **Accepted (`6065cf1b`, `eef7e612`)** |
| `GA4 / G4` | Obsolete tool/infrastructure/device cleanup, reader inventory, support matrix, per-batch SHA-256 cleanup ledgers & recovery | `maintenance/tooling-inventory.json` (`407` entries, `test-tooling-modernization` `33/33`, `cargo test -p xtask` `115/115`), `docs/{support-matrix,retired-scaffolding}.md`, `CLN269`–`CLN273` (`maintenance/cleanup/LOG.md`) | **Accepted (`60ce6ecd`, final commit)** |
| `GA5 / G5` | Real Lisp workload & GC attribution (`2+7` rounds, 6 candidates, allocation/survival/phase/pause/memory/startup/load) | `tools/runtime-{profile,comparison}.py`; `s3-g5-comparison-formal` (`238308bd…`, all 7-round series, MAD/p95, phases `0..24`, survival buckets, remembered-set, cadence, RSS, startup, and per-round load archived above) | **Accepted** |
| `GA6 / G6` | Real-editor interpreter, bytecode, and `libgccjit` native design, implementation, semantic/lifecycle/performance comparison & decision | `src/{eval,bytecode,comp}.rs`; `10/10` hotpath × 3, `7/7` call-frames × 3, `10/10` bytecode × 3 (`2dc5fc11…`), `17/17` native lifecycle, `s3-g5-comparison-formal`; baseline VM/bytecode shipped as default, `libgccjit` retained opt-in | **Accepted** |
| `GA7 / G7` | Real-editor generational `Young` GC design, root/store/barrier/cadence/fallback proof, native write-barrier, `--gc-stress`, `Young` vs `major` comparison & decision | `src/{alloc,runtime_gc_collector}.rs`, `docs/{g7-young-root-store-binding,gc-young-cadence}.md`; `28/28` B-vectors (`35a980ea…`), `runtime-gc` `10/10` × 2, `rtg-fallback-01`, `rpc01`, `gc-young-cadence` `2/2` × 2, `g7-native-write-barrier` `3/3` (`b52385f8…`), `564`-file `--gc-stress`, `53` minors/round in `s3-g5-comparison-formal`; non-moving `Young` rejected as default, retained opt-in | **Accepted** |
| `GA8 / all` | Reproducible real-workload speedup, lower memory & shorter GC pauses vs installed `32.0.50` reference, usable macOS trial deliverables, clean worktree & signed push | `s3-ga8-delivered-formal` (`1f24b37b…`), `s3-ga8-bench-cocoa` (`fda8ceef…`), `s3-ga8-bench-terminal` (`10b40898…`); `CLN273` post-cleanup verified `build/terminal/port/src/emacs` (`a0dd02e1…`) & `dist/macos/Femacs.app` (`02f5adaf…`) | **Accepted** |
