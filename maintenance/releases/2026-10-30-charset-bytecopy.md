# Charset generation from protected inputs

P438 fixes ten raw-map copy recipes in `build-support/generator-recipes.json`.
During a source-bound dual-leg build, the original copy utility propagated the
input's immutable flag to a generated output. Updating its timestamps failed;
the following generation pass also failed to overwrite that output. The
generated file was not a protected source input. No collector product was
successfully built by that attempt.

The recipes now stream input bytes into the same destinations, without copying
inode flags. The ten input/output paths and contents stay the same; no protected
source, controller limit or original assertion is changed.

## Verification

The isolated integration used the real generator with all copied source inputs
under the existing artifact-preservation guard. Both full charset passes ran
139 ordered recipes, producing 134 distinct recipe files and two additional
Unicode files. All 136 outputs were byte-identical between passes. Each of the
ten changed copies matched its raw input and the retained prior product's map,
had no inherited flags, and left input bytes/modes/flags unchanged. All input
flags were restored by the original guard. Integration: 23.538617542 seconds;
controller: exit 0, reaped, 23.705 seconds, 53,264,384 peak accounted bytes.

After integration passed, `tools/test-charset-bytecopy.py` exercised real recipe
commands with small fixed-byte fixtures. The two macOS protections passed:
immutable inputs and repeated regeneration, 0.787249958 seconds; missing input
must return an error, 0.376776417 seconds. Zero skips. Controller: exit 0,
reaped, 1.283 seconds, 18,874,368 peak accounted bytes. These test behavior,
not an exact spelling of the copy command. Non-macOS immutable coverage is not
claimed. Original generator ordering and intentional repeated output paths
are preserved.

The first integration wrapper wrongly counted 139 ordered recipes as 139
distinct output paths. Its first actual generation succeeded; the wrapper's
count assertion failed. The original failed wrapper/result remains false.
The corrected wrapper accounts for the actual 134 distinct recipe outputs;
no production change or original assertion was made to hide that mistake.

The independent partner reviewed the source, original failure, successful
integration and subsequent protections. Exact source/evidence hashes and
bounded controller results are in the [portable receipt](2026-10-30-charset-bytecopy-results.json).

## Collector and cleanup boundaries

New permanent-fallback inputs also ran on four retained prior collector profiles:
normal/checking, major/young; four discoveries and four real ERT runs passed.
The young-mode controls still rebuilt roughly 94,000 registry entries during
each forced Full collection, while preserving the tested live object graph and
module error. These are controls for P437, not a speedup or candidate acceptance.
Their valid results will be reused rather than rerun unchanged.

The failed collector source expansion and consumed generator/cache/case state
were retired with exact input recovery and original failures retained:
[CLN154](../cleanup/reports/2026-10-30-154-consumed-fallback-build-state.md).
The trial macOS App and working prior products were verified unchanged.
P437 still requires a new normal/checking build and candidate integration;
limited available disk space currently defers that build. G1/G2/G4 remain
accepted, 3/7. G3 and G5-G7, qualified performance improvement and whole-Goal
completion remain open.
