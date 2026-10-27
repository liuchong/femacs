# Fe artwork and local macOS release module

The project and App share a flowing epsilon sculpture cast in iron: silver
worn cut faces, thick dark sidewalls and irregular orange-red oxide corrosion.
The transparent master is `packaging/assets/fe-logo.png`; both READMEs use it.
The original generated PNG and its standard content provenance are retained.
It is a raster master, not an editable 3D model or a promise that text generation
will reproduce identical pixels. Package generation uses committed pixels only.

`packaging/` is an independent Cargo workspace package. It owns artwork resizing,
normal/Retina ICNS assembly, a signed App copy, verified local DMG, SHA256SUMS
and a relative-path release manifest. The normal Cocoa package entry calls the
same icon installer before signing. Windows and Linux formats remain plans.

## Actual verification

- Full source-bound Cocoa build with the package integration:361.239s,
  867926016-byte accounted peak, controller0 and complete cleanup. Its613 core
  source inputs match the user-passed P394 candidate.
- That normal-build product starts as Fe, visibly accepts text and naturally
  exits0 via C-x C-c;101.394s,157581312-byte peak, controller0/reaped. Native
  menu Quit did not yield observed exit; this is retained as an observation,
  not an independently diagnosed new menu defect. A keyboard other-frame
  attempt stayed in composition and is not counted as a new switching pass.
- After the user changed the artwork direction, only bitmap/renderer/build-host
  inputs changed. Both hosts rebuild successfully. The shared icon installer is
  exercised by actual signed release packaging with the new master; no third
  whole-core build is claimed. The non-signature code payload and dump remain
  unchanged; re-signing legitimately changes signature bytes.
- New bitmap/ICNS, signature-vs-code and malformed-Mach-O protection:3/3 pass,
  actual longest case4.997s, full controller7.902s,236470272-byte peak/reaped.
- Final new-artwork release creation and DMG verification:20.412s,71270400-byte
  peak, controller0/reaped. macOS decodes the installed ICNS. Final read-only image mounting
  verifies Fe.app at the image root, signature and all recorded member hashes.
  Current trial App matches the release App.

The final epsilon-icon executable itself is now checked natively: `FE` is
visible, C-a/C-e reach the correct commands, then C-g and C-x C-c give natural
editor/controller exit0/reaped in37.369s,153616384-byte peak. Its exact program
and dump hashes match the delivered release manifest. This closes the prior
e405-versus-final-signature startup identity gap. The earlier manual two-window
Chinese acceptance remains separate; no second manual Chinese pass is invented.

## Delivered identities

- Trial App: `dist/macos/Femacs.app`
- Release App/image: `dist/releases/fe-iron/Fe.app`,
  `dist/releases/fe-iron/Fe-macos.dmg`
- Final executable SHA-256: `2cdeba9bd4b273380f7da729ff5074a77aea85228f61e74b8ebd557b009f5eda`
- Non-signature Mach-O payload: `587e9ae6dfce8c948ca98ac59a6e68c0080a4df4798a1224a0bc787f80168585`
- Lisp dump: `1a632e0654756c0ec8635df31c4b562294cea134e53c3be8a68e3f8c8691085d`
- Installed icon: `0c970d5fd8712f6be799afadf9f9bfe882c0b712cb2ac23561acb897c42dc995`
- DMG: `cfb023ea32fd7b6638b398f88a10de0311827fa8e8195673a2c0cce5e1847ddd`

Receipts are in `build/acceptance/fe-epsilon-artwork-1` and
`build/acceptance/fe-delivery-3` and `build/acceptance/fe-final-ui-2`; core build/startup receipts remain in the
corresponding `g1-fe-brand-cocoa-build-1` and `fe-final-ui-1` directories.
The local package is ad-hoc signed, not notarized; no public release or remote CI
ran. This is artwork/packaging delivery and the separately documented P394
repair, not full G1/compatibility/performance or whole-Goal completion.
