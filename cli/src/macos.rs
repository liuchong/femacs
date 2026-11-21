use crate::DefaultMode;
use std::{
    env,
    ffi::OsString,
    io::{self, Error, Write},
    os::unix::process::{CommandExt, ExitStatusExt},
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

const HELP: &str = concat!(
    "Usage: fe [OPTION-OR-FILENAME]...\n\n",
    "Fe launcher options\n",
    "  -R, --reset-args  Discard the preset and all earlier arguments.\n",
    "                    The last reset before -- wins.\n",
    "  -1, --sequential  Open one file per session. Place before the file list.\n",
    "\n",
    "  fe starts a clean terminal session (-Q -nw) with optional terminal settings.\n",
    "  Run 'fe --help' (or 'fe -R --help') alone to include this launcher section.\n",
    "  femacs passes all arguments directly to the editor.\n",
    "  Use -- before file names that look like options.\n",
    "\n",
    "  fe notes.org            Open in the terminal\n",
    "  fe -R -nw notes.org     Use native terminal startup and ordinary init\n",
    "  fe -1 a.txt b.txt       Open one file at a time\n",
    "\n",
    "Native editor help (macOS)\n\n",
);

pub(super) fn launch(default: DefaultMode) -> Result<ExitCode, String> {
    // Deprecated in Rust 1.85 (our declared MSRV) and un-deprecated in 1.87.
    #[allow(deprecated)]
    let home = env::home_dir();
    let program = editor_program(env::var_os("FEMACS_APP"), home.as_deref())?;
    if default == DefaultMode::Native {
        let error = Command::new(&program).args(env::args_os().skip(1)).exec();
        return Err(start_error(&program, error));
    }
    launch_quick(&program, home.as_deref())
}

fn launch_quick(program: &Path, home: Option<&Path>) -> Result<ExitCode, String> {
    let original: Vec<_> = env::args_os().skip(1).collect();
    let reset = original
        .iter()
        .take_while(|argument| *argument != "--")
        .enumerate()
        .filter_map(|(index, argument)| {
            matches!(argument.to_str(), Some("-R" | "--reset-args")).then_some(index)
        })
        .last();
    let mut arguments = original
        .into_iter()
        .skip(reset.map_or(0, |index| index + 1))
        .peekable();
    let sequential = arguments
        .next_if(|argument| matches!(argument.to_str(), Some("-1" | "--sequential")))
        .is_some();
    if sequential {
        arguments.next_if(|argument| argument == "--");
        if arguments.peek().is_none() {
            return Err("-1 requires at least one file".into());
        }
    }

    let show_help = !sequential
        && arguments.len() == 1
        && matches!(
            arguments.peek().and_then(|argument| argument.to_str()),
            Some("--help" | "-help")
        );
    if show_help {
        let mut stdout = io::stdout().lock();
        stdout
            .write_all(HELP.as_bytes())
            .and_then(|()| stdout.flush())
            .map_err(|error| format!("could not print launcher help: {error}"))?;
    }

    let mut terminal_arguments = Vec::new();
    let mut terminal_locale = None;
    if !show_help && reset.is_none() {
        terminal_arguments.extend([OsString::from("-Q"), OsString::from("-nw")]);
        if let Some(init) = terminal_init(home)? {
            terminal_arguments.extend([OsString::from("-l"), init.into_os_string()]);
        }
        terminal_locale = env::var_os("FEMACS_TTY_LOCALE");
    }

    let editor_command = || {
        let mut command = Command::new(program);
        command.args(&terminal_arguments);
        if let Some(locale) = &terminal_locale {
            command.env("LC_ALL", locale);
        }
        command
    };
    if !sequential {
        let error = editor_command().args(arguments).exec();
        return Err(start_error(program, error));
    }

    for file in arguments {
        let status = editor_command()
            .arg("--file")
            .arg(file)
            .status()
            .map_err(|error| start_error(program, error))?;
        if !status.success() {
            let code = status
                .code()
                .or_else(|| status.signal().map(|signal| 128 + signal))
                .unwrap_or(1);
            return Ok(ExitCode::from(u8::try_from(code).unwrap_or(1)));
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn start_error(program: &Path, error: Error) -> String {
    format!("could not start {}: {error}", program.display())
}

fn terminal_init(home: Option<&Path>) -> Result<Option<PathBuf>, String> {
    if let Some(init) = env::var_os("FEMACS_TTY_INIT") {
        let init = PathBuf::from(init);
        if !init.is_file() {
            return Err(format!("FEMACS_TTY_INIT is not a file: {}", init.display()));
        }
        return Ok(Some(init));
    }
    let config = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| home.map(|home| home.join(".config")));
    Ok(config
        .map(|root| root.join("femacs/terminal.el"))
        .filter(|path| path.is_file()))
}

fn editor_program(
    selected_bundle: Option<OsString>,
    home: Option<&Path>,
) -> Result<PathBuf, String> {
    let executable = Path::new("Contents/MacOS/Femacs");
    if let Some(bundle) = selected_bundle {
        if bundle.is_empty() {
            return Err("FEMACS_APP is empty; set it to the Fe application bundle path".into());
        }
        let program = PathBuf::from(bundle).join(executable);
        if program.is_file() {
            return Ok(program);
        }
        return Err(format!(
            "FEMACS_APP did not locate an editor at {}",
            program.display()
        ));
    }

    let locations = std::iter::once(PathBuf::from("/Applications"))
        .chain(home.map(|home| home.join("Applications")));
    for directory in locations {
        for bundle in ["Fe.app", "Femacs.app"] {
            let program = directory.join(bundle).join(executable);
            if program.is_file() {
                return Ok(program);
            }
        }
    }
    Err(
        "Fe.app was not found; install the Fe application or set FEMACS_APP to its bundle path"
            .into(),
    )
}
