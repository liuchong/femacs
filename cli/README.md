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
session. Run `fe --help` (or `fe -R --help`, `-help`) alone to print
`Usage: fe [OPTION-OR-FILENAME]...` and list the launcher options before
**Native editor help (macOS)**; combining `--help` with other arguments (such
as `fe --help --version` or `fe notes.org --help`) omits the `fe` header and
runs through `fe`'s normal startup (including its preset and `FEMACS_TTY_INIT`
check unless `-R` is used) before the editor handles the options. `femacs`
forwards every argument, including `--help`, directly to Fe. The native help
shows the path of the editor executable in the app bundle. On Linux and Windows
each launcher reports the macOS requirement with its own command prefix (`fe:`
or `femacs:`) when invoked.

`--version` reports the editor version. Cargo reports the launcher version
in `cargo install --list`.
To install only the long command name, use `cargo install femacs --locked --bin femacs`.

From a source checkout, use `cargo install --locked --path cli` instead.

### Shortcut options for `fe`

| Option | Behavior |
| --- | --- |
| `-R`, `--reset-args` | Discard the launcher preset and all earlier `fe` and editor arguments. Recognized anywhere before the first `--`; the last reset before `--` determines where processing resumes. |
| `-1`, `--sequential` | Open each file in its own editor session (`--file`). Recognized only as the first effective argument (at the start or immediately after the last `-R`), optionally followed by `--` and at least one file name. Exiting one session opens the next; a non-zero or signaled exit stops the sequence. |

Syntax boundaries:

- `--` ends launcher option scanning so following arguments are treated as literal operands rather than `-R` or `-1` (`fe -- -R -1` forwards `-- -R -1` to one session, while `fe -1 -- -R -1` passes each operand via `--file`).
- Because `-R` is scanned before `--` without parsing editor option arities, pass dash-prefixed option values before `--` in an unambiguous form such as `--file ./-R` or `--title=-R`.
- After `-1` (and an optional `--`), every remaining argument is treated as a file operand rather than an editor switch: `fe -1 a.txt b.txt` opens sequential sessions with the terminal preset, whereas `fe -R -1 a.txt b.txt` opens sequential sessions with native editor startup (without the preset).
- Ordinary editor options pass through in their original order when `-1` is not active.

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
