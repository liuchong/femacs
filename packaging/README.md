# Fe release packaging

This independent Cargo workspace module owns release artwork, icon generation
and platform packaging. It packages an existing editor build; it does not replace
`xtask`, compile another editor, or use the ignored reference checkout.

## Artwork

`assets/fe-logo.png` is the original iron master used by both READMEs.
The macOS Dock artwork lives in [Fe.icon](assets/Fe.icon/icon.json), with
light and dark SVGs that use the same F and horn contours and restrained rust
accents.
Artwork provenance and attribution are in [assets/README.md](assets/README.md).

```sh
cargo run -p fe-packaging -- artwork --out build/fe-artwork
```

The fresh output contains the original logo fitted into a 1024-pixel square,
the light and dark Dock previews, and a light ICNS with representations from
16 through 1024 pixels. `Cargo.lock` pins the portable image renderer.
Cocoa bundling uses Xcode's `actool` with Icon Composer support to compile
`Assets.car` and a standard `Fe.icns` before signing. macOS 26 and later
select the artwork using the system's icon appearance setting. The ICNS
provides the default for systems that use conventional bundle icons.

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
