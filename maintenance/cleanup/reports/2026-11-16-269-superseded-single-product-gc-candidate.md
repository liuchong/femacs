# CLN269 — Superseded single-product GC candidate and throwaway source-rule checks

After `build/acceptance/current-runtime-build-6` (`20` configurations, `61` profiles) completed naturally from the consolidated source manifest (`a72dd63a…`) and bound the installed `32.0.50` reference (`current-reference-selection-6`), the earlier single-product GC candidate `build/runtime/p466-gc-1` and two standalone source-rule expansion check directories (`build/acceptance/p466-srcrules-check-{1,2}`) completed their consumers.

Retired `3` owned directories on `2026-11-16`:
- `build/runtime/p466-gc-1` (`977M` `du -sh` at deletion; `runtime-product.json` SHA-256 `b77c10eaeaf6028262dfe36764d09070a8b35fc0bf2637c86cc793a6e119b24a`, program SHA-256 `a3b691437ab33b6623ec7f376361abe185cd141454f9117d8d40d0d95050bb9d`), superseded by the matching configuration leg in `current-runtime-build-6` and recoverable via `python3 tools/runtime-stage.py --action build`.
- `build/acceptance/p466-srcrules-check-1` (`41M` `du -sh` at deletion), recoverable via `python3 tools/stable-source-rules.py --out <fresh owned path>`.
- `build/acceptance/p466-srcrules-check-2` (`41M` `du -sh` at deletion), recoverable via `python3 tools/stable-source-rules.py --out <fresh owned path>`.

The installed reference application, optional reference checkout, Git `HEAD`/index, and active `current-runtime-build-6` / `p466-*` evidence directories were preserved untouched.
