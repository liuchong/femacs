# P452: Preserve discovered executable path during PATH_EXEC dump lookup

## Outcome and scope

`load_pdump` in `src/emacs.rs` previously evaluated `if ! (emacs_executable.is_null () && *emacs_executable != 0)` instead of `if ! (! emacs_executable.is_null () && *emacs_executable != 0)` at the `PATH_EXEC` portable-dumper fallback. Because a pointer cannot simultaneously be null and dereference to a non-zero byte, the condition was unconditionally true whenever dump lookup fell through from the executable directory to `PATH_EXEC` (such as packaged `dist/macos/Femacs.app/Contents/MacOS/Femacs` loading `Contents/Resources/libexec/Femacs.pdmp`). That overwrote the discovered executable path in `emacs_executable` (and therefore `initial_emacs_executable`) with a synthetic `../../../../bin/` relative path, causing `(kill-emacs 0 t)` restarts to fail because `initial_emacs_executable` pointed to a nonexistent sibling binary.

Restoring the missing inner negation in `src/emacs.rs` (`P452`) preserves the executable path already discovered by `find_emacs_executable` and only synthesises the `../../../../bin/` sibling fallback when executable discovery returned null or an empty string.

## Build, functional, and unit-test evidence

| Check | Result | Controlled-run receipt |
| --- | --- | --- |
| Incremental `src/emacs.c` rebuild, relink, dump, and app packaging | `build/terminal/port/src/emacs` and `dist/macos/Femacs.app` rebuilt, dumped, signed, and verified; exit 0, 217.618s, peak 838,500,352 B | `64c10f98292d4adca5a31d8db6972810` |
| Packaged `Femacs.app` and `terminal` `(kill-emacs 0 t)` restart functional check | Both `dist/macos/Femacs.app/Contents/MacOS/Femacs` (loading `Contents/Resources/libexec/Femacs.pdmp` via `PATH_EXEC`) and `build/terminal/port/src/emacs` re-executed cleanly across `(kill-emacs 0 t)`; exit 0, 0.815s, peak 2,342,912 B | `4af65b735de841cd9f8cb84e8cdfc672` |
| Local acceptance (`tools/local-acceptance.py`) | 9/9 checks passed (`root-inputs`, `ert-components`, `acceptance-components`, `bundle-signature`, `terminal-batch`, `gui-batch`, `terminal-user-init`, `terminal-keys`, `terminal-files`); exit 0, 28.651s, peak 70,352,896 B | `e610c5d233e947d99a0642547d0cd7c2` |
| Protective unit tests (`tools/test-macos-stability.py`) | 9/9 unit tests passed, including `test_load_pdump_preserves_discovered_executable_on_path_exec_lookup`; exit 0, 0.272s, peak 5,062,656 B | `3461803241f54a7998589a93f702080f` |

## Product identity

| Current artifact | Bytes | SHA-256 |
| --- | --- | --- |
| `build/terminal/port/src/emacs` | 2,698,016 | `48285a79f25ec40b278ef3359035c08450ed0301c1b94b297d63f826a25a5ca4` |
| `build/terminal/port/src/emacs.pdmp` | 13,103,344 | `291b755a748e5b902b64a418082d2133af348844de145ec811057fff71d67a01` |
| `dist/macos/Femacs.app/Contents/MacOS/Femacs` | 4,229,680 | `3325b36b5d98f409378773c4f3361319911926902837bf016caf76ea683bb702` |
| `dist/macos/Femacs.app/Contents/Resources/libexec/Femacs.pdmp` | 18,602,648 | `1b5764dce4a2d00cec594eb0dae024064c1c9dea905373c247de8ed643f96a79` |

Temporary build and acceptance directories (`build/cargo`, `build/stability-unit`, `build/acceptance/p452-local-acceptance`, and numbered dump intermediates) were reclaimed immediately after verification.
