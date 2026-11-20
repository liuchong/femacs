# CLN276 — macOS Dock artwork

The normal Cocoa bundler compiles the two source SVGs in `packaging/assets/Fe.icon` using Xcode's `actool` before signing. On macOS 27.2, Icon Composer rendered both variants at 256 pixels. Inspection of the installed asset catalog selected the Light vector for Aqua and the Dark vector for Dark Aqua. The bundle also contains a conventional ICNS. The user's appearance preferences and running editor session were preserved.

Both replacement bundles passed strict deep signature verification. The installed executable ran in batch mode and found its Lisp files inside the bundle. Packaging tests passed 4/4. The staging command finalized the signed app and `hdiutil verify` reported the image valid, but the enclosing controller returned 125 (`descendants_left`, cleanup complete). Acceptance here covers the independently verified app, not an installer image.

| Installed member | SHA-256 |
| --- | --- |
| Signed executable | `79de87e32b6c1d5273d377ee6ec90f12288d10640833654f1496db3fd3790d2c` |
| Mach-O payload, identical before and after signing | `93b56aefbaaca9481a3222ee92ed8142c4535e19b7b498a5d979c2c993ef7ee0` |
| Dump, identical before and after packaging | `181f681af4e9560524d395e5a5ffa447f432c7967224d90d9846b05c85ce865c` |
| Asset catalog | `8643732c6426630f6154e45f255f32dc4024acbdade8eacfff85b72beafe629c` |
| Conventional ICNS | `3fbc7805defe14bed0e5816abcfeaaae02fd52785d9b4e81e828d3eb1a3b09a5` |

| Retired scope | Files | Directories | Symlinks | Logical bytes | Tree SHA-256 |
| --- | ---: | ---: | ---: | ---: | --- |
| `build/fe-dock-refresh` | 5,595 | 382 | 1 | 804,437,555 | `56dd3f0cfdce40c940361b56b182108c81ab2bb1a1f29b10fd3c2d52fbac7b13` |
| Prior workspace bundle | 4,499 | 117 | 1 | 231,619,014 | `e2d5686c6d23cd4f737a3a784c579f1c15e161bc52e7d4927d93d82f8bc5cc34` |
| Prior installed bundle | 4,499 | 117 | 1 | 231,619,014 | `e2d5686c6d23cd4f737a3a784c579f1c15e161bc52e7d4927d93d82f8bc5cc34` |

The work tree held compiler outputs, the signed staging copy, previews, and a temporary image. The old bundles were replaced atomically. Build the app with `cargo run -p xtask -- build-macos` and the pinned SDK. Build the previous artwork from `ee587b53` if needed.
