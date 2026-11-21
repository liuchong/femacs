# Fe command-line launcher

The `femacs` package provides the `femacs` and `fe` commands for an existing
Fe installation on macOS. It requires the Fe application bundle. Follow the
[Fe build guide](https://femacs.com/pages/start.html) to build the editor.

```sh
cargo install femacs --locked
fe notes.org
fe -1 first.txt second.txt
femacs notes.org
fe -R -nw notes.org
```

Both names run the same editor. `fe` supplies `-Q -nw` for a clean terminal
session and lists its launcher options before the editor help.
`femacs` forwards every argument, including `--help`, directly to Fe.
`--version` reports the editor version. Cargo reports the launcher version
in `cargo install --list`.
To install only the long command name, use `cargo install femacs --locked --bin femacs`.

From a source checkout, use `cargo install --locked --path cli` instead.

### Shortcut options for `fe`

| Option | Behavior |
| --- | --- |
| `-R`, `--reset-args` | Discard the launcher preset and all earlier arguments. The last reset before `--` determines where processing resumes. |
| `-1`, `--sequential` | Open each file in its own editor session. Use as the first effective argument before the file list. Exiting one opens the next. A failed or interrupted session stops the sequence. |

Use `--` before any file name that begins with a dash. In sequential mode the
remaining arguments are file names. For a dash-prefixed option value, pass an
unambiguous path such as `--file ./-R`. Ordinary editor options pass through
in their original order after the reset.

The `fe` terminal preset loads
`${XDG_CONFIG_HOME:-$HOME/.config}/femacs/terminal.el` if present. You can select
another file with `FEMACS_TTY_INIT` and set `FEMACS_TTY_LOCALE` to override
`LC_ALL` for the editor process. A reset also clears these preset settings and
uses the locale inherited from the caller.
For example, set these once in your shell configuration:

```sh
export FEMACS_TTY_INIT="$HOME/my-terminal-init.el"
export FEMACS_TTY_LOCALE=zh_CN.UTF-8
```

The launcher looks for `Fe.app` and `Femacs.app` in `/Applications`, then
in `~/Applications`. For a bundle in another location, including the result of
a local build, set `FEMACS_APP` to its path:

```sh
FEMACS_APP="$PWD/dist/macos/Femacs.app" femacs
```

Maintainers can check for launcher option conflicts before publishing:

```sh
python3 tools/check-cli-options.py --upstream
```
