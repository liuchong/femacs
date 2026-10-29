# Consumed-state directory ownership

The automatic case-state cleanup recorded regular-file identities but omitted
directories, then called bottom-up `rmdir` without checking the original inode.
An empty directory replaced after planning could therefore be removed under
the previous owner's name. Empty-only cleanup had no directory inventory.

P434 records every selected directory in the durable private plan and a typed,
path-hashed public ledger before deleting state. It rechecks device, inode,
mode, directory type and protected flags before each removal. Original directory
mtime is retained as evidence, but child removal necessarily changes it.
Unexpected new contents still make `rmdir` fail. The occupancy query has a
ten-second deadline; timeout retains state without retry. Existing regular-file
rows and private `files` remain compatible; schema 2 adds `directories`.

## Actual validation

`tools/test-runtime-retire-functional.py` runs a real bounded child writer,
waits for exit 0, then removes three files and five directories through the
production owner. Eight decoded ledger rows match the captured identities;
the result evidence and production source remain unchanged. Runtime is 0.184
seconds. This verifies process/filesystem cleanup, not editor behavior.

After integration, `tools/test-runtime-retire.py` passes seven ownership cases:
directory replacement, mode change, symlink replacement, an unplanned new file,
failed process, open handles and occupancy-query timeout. They take 0.631
seconds; the outer runtime owner exits 0 with complete cleanup and a 30-second
deadline. The same seven cases against unchanged prior production code fail
four: the first two replacements are incorrectly removed; timeout escapes;
symlink replacement lacks the new ownership rejection. Three existing retention
conditions still pass. Keep this exit-1 negative result unchanged.

Local reproduction, each with fresh owned output:

```sh
python3 tools/test-runtime-retire-functional.py --out build/acceptance/retirement-functional
python3 tools/test-runtime-retire.py --out build/acceptance/retirement-protection
```

The formal results are `build/acceptance/p434-functional-1`,
`build/acceptance/p434-protection-1` and `build/acceptance/p434-negative-1`.
The protection driver accepts an explicit `--module` for a retained negative
implementation. Keep actual runtime/test limits when replaying.

## Historical and delivery limits

No historical receipt is rewritten to invent missing directory metadata.
[CLN144](../cleanup/reports/2026-10-30-144-consumed-measurement-directories.md)
uses the new owner on 215 actual consumed empty directories and preserves all
1,419 prior measurement/readiness evidence files. That later cleanup does not
repair metadata absent from earlier automatic removals. This change neither
rebuilds an editor nor changes compiler/collector defaults or the Goal scope.
