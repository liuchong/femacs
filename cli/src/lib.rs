//! Shared command-line entry point for Fe.

use std::process::ExitCode;

/// Select the startup behavior for an installed command.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DefaultMode {
    /// Start a clean terminal session with optional terminal settings.
    Terminal,
    /// Use the editor's native startup behavior.
    Native,
}

impl DefaultMode {
    const fn command_name(self) -> &'static str {
        match self {
            Self::Terminal => "fe",
            Self::Native => "femacs",
        }
    }
}

/// Start the installed Fe editor.
#[cfg(target_os = "macos")]
pub fn launch(default: DefaultMode) -> ExitCode {
    match macos::launch(default) {
        Ok(status) => status,
        Err(error) => {
            eprintln!("{}: {error}", default.command_name());
            ExitCode::FAILURE
        }
    }
}

/// Explain the platform requirement when invoked on another OS.
#[cfg(not(target_os = "macos"))]
pub fn launch(default: DefaultMode) -> ExitCode {
    eprintln!(
        "{}: the Fe application launcher requires macOS",
        default.command_name()
    );
    ExitCode::FAILURE
}

#[cfg(target_os = "macos")]
mod macos;
