# CLN274 — Cocoa default-font update

The Cocoa frame found requested fonts but ignored `set-face-attribute` changes to `:font`. The graphical branch now resolves the requested font and updates the frame parameter, including changes to `:height`. New frames import global defaults through the graphical parameter path without marking the font as an explicit frame override.

The [Cocoa font fixture](../../../tools/fixtures/macos-default-font.el) runs as `init.el` in a fresh `--init-directory`. It checks initial and new frames at 15 pt, changing one frame to 18 pt with `set-frame-font`, and changing that frame to 21 pt through `:height`. The earlier app failed at the first check with Menlo 12. The reference, the fixed bundle with Hack Nerd Font Mono and Monaco, and the installed app with Hack Nerd Font Mono each passed. The test does not load the rest of a user's configuration.

The signed bundle and installed copy match: executable SHA-256 `d9eb84c8f8610df1e56c30bd10dd04e46d2e300430272de96bd7472622613ce8`, dump SHA-256 `03b511d57b4f68f1df733bb6a98d443e1640b6805a2ffe859237024fa894f7f7`. Both passed `codesign --verify --deep --strict` after cleanup.

| Removed scope | Files | Directories | Symlinks | Logical bytes | Tree SHA-256 |
| --- | ---: | ---: | ---: | ---: | --- |
| `build/font-fix-20261120` | 581 | 289 | 8 | 160,450,683 | `f58e3fd931d562a9780a49fde82777277353fadc044ae3d4c5c50b6c4fdca975` |
| `.femacs-cache/macos/4cf5de8fd7d5ab60` | 5,366 | 172 | 0 | 182,212,078 | `7e2b61ec34c5ffa8620e68771ef4a09243593f00168b2c62c8d2cb9940de37e2` |

Both scopes contained only files generated for this build and its probes. Recreate the SDK with `tools/toolchain-pins.py install --prefix` and run `cargo run --release -p xtask -- build-macos` with that pinned SDK selected. Re-run the fixture to collect fresh verification evidence.
