# Fe release packaging

This independent Cargo workspace module owns release artwork, icon generation
and platform packaging. It packages an existing editor build; it does not replace
`xtask`, compile another editor, or use the ignored reference checkout.

## Artwork

`assets/fe-logo.png` is the selected transparent master: an upright iron F
beside a smaller pair of GNU-inspired horns rotated vertically. The two inward
returns suggest e without an added middle branch or animal face. Worn gray iron
and restrained orange-red oxidation connect the name with its material.
The same master is used in both READMEs and the native icon; each square icon
fits the artwork uniformly with transparent padding, preserving its proportions.
Artwork provenance and attribution are in [assets/README.md](assets/README.md).

```sh
cargo run -p fe-packaging -- artwork --out build/fe-artwork
```

The fresh output contains 1024-pixel previews and an ICNS with normal/Retina
representations from 16 through 1024 pixels. Rectangular masters are uniformly fitted and centered with transparent padding,
so square icons preserve their source proportions. PNG resampling has no system-font or
network dependency. `Cargo.lock` pins the image renderer. Standard Cocoa builds install
`Fe.icns` and its bundle metadata before signing, so the icon persists on rebuild.
This is a conventional ICNS icon, not a claim of layered Tahoe appearance support.

## macOS

Build the editor first, then package it locally:

```sh
cargo run -p xtask -- build-macos
cargo run -p fe-packaging -- macos --app dist/macos/Femacs.app --out dist/releases/local
```

The output directory must be new. Outputs are `Fe.app`, `Fe-macos.dmg`,
`SHA256SUMS` and `release.json`. The input bundle is not modified. Packaging checks
its existing signature/resources, adds artwork to a copy, ad-hoc signs and checks
that copy, verifies unchanged non-signature Mach-O payload and dump hashes, then creates and verifies the
compressed disk image. A failed stage remains available for diagnosis; choose a
fresh directory after fixing the cause. Build/release processes run under the
caller's resource controller. No remote CI is involved.

The local artifact is **ad-hoc signed and not notarized**. Developer ID signing,
notarization and public release publication need separate credentials and release
policy. A valid package does not prove editing, input-method or compatibility
acceptance; use the matching product's native acceptance record.

## Platform adapters

| Platform | Status | Planned formats |
| --- | --- | --- |
| macOS | Local native bundle and verified disk image | `.app`, `.dmg` |
| Windows | Planned; no executable adapter yet | Installer and portable archive |
| Linux | Planned; no executable adapter yet | Debian `.deb`, RPM `.rpm`, AppImage, Flatpak |

Future adapters belong in this module and must validate the actual platform
product before packaging. Unsupported targets fail explicitly. Package formats
are a plan, not a platform-support or acceptance claim.
