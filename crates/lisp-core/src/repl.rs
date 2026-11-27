#![forbid(unsafe_code)]
//! Interactive REPL frontend: pure-Rust `rustyline` TTY editor + clean piped fallback.

#[cfg(feature = "repl-tty")]
use std::fs;
#[cfg(feature = "repl-tty")]
use std::io::IsTerminal;
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::brand;
use crate::engine::Engine;
use crate::error::Error;
use crate::rpc::has_complete_lisp_form;

/// Run the interactive Lisp REPL on `stdin` (TTY with line editing/history/completion, or pipe).
pub fn run_repl(engine: &mut Engine, history_path: Option<&Path>) -> Result<i32, Error> {
    // Interactive REPL handles SIGINT via quit/readline recovery instead of exiting immediately.
    let _ = engine.eval_silent("(setq kill-emacs-on-sigint nil)");
    #[cfg(feature = "repl-tty")]
    {
        if io::stdin().is_terminal() {
            return run_tty_repl(engine, history_path);
        }
    }
    let _ = history_path;
    run_pipe_repl(engine)
}

fn is_blank_or_comment_only(src: &str) -> bool {
    for line in src.lines() {
        let trimmed = line.trim_start();
        if !trimmed.is_empty() && !trimmed.starts_with(';') {
            return false;
        }
    }
    true
}

#[cfg(feature = "repl-tty")]
fn is_symbol_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || matches!(
            ch,
            '-' | '_' | '+' | '*' | '/' | ':' | '<' | '>' | '=' | '!' | '?' | '&' | '.' | '%' | '$' | '#' | '@' | '~' | '^'
        )
}

#[cfg(feature = "repl-tty")]
fn symbol_prefix_at(line: &str, pos: usize) -> (usize, &str) {
    let bounded = pos.min(line.len());
    let prefix_slice = &line[..bounded];
    let mut start = bounded;
    for (idx, ch) in prefix_slice.char_indices().rev() {
        if is_symbol_char(ch) {
            start = idx;
        } else {
            break;
        }
    }
    (start, &line[start..bounded])
}

fn run_pipe_repl(engine: &mut Engine) -> Result<i32, Error> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = match reader.read_line(&mut line) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{} repl read error: {e}", brand::CLI_NAME);
                return Ok(1);
            }
        };
        if bytes == 0 {
            if !buffer.trim().is_empty() && !is_blank_or_comment_only(&buffer) {
                match engine.eval_printed(&buffer) {
                    Ok(out) => {
                        println!("{out}");
                        let _ = io::stdout().flush();
                    }
                    Err(Error::Exit(code)) => return Ok(code),
                    Err(err) => {
                        eprintln!("error: {err}");
                        let _ = io::stderr().flush();
                    }
                }
            }
            return Ok(0);
        }

        buffer.push_str(&line);
        if buffer.trim().is_empty() || is_blank_or_comment_only(&buffer) {
            buffer.clear();
            continue;
        }
        if !has_complete_lisp_form(&buffer) {
            continue;
        }

        match engine.eval_printed(&buffer) {
            Ok(out) => {
                println!("{out}");
                let _ = io::stdout().flush();
            }
            Err(Error::Exit(code)) => return Ok(code),
            Err(err) => {
                eprintln!("error: {err}");
                let _ = io::stderr().flush();
            }
        }
        buffer.clear();
    }
}

#[cfg(feature = "repl-tty")]
mod tty {
    use rustyline::completion::{Completer, Pair};
    use rustyline::error::ReadlineError;
    use rustyline::highlight::Highlighter;
    use rustyline::hint::Hinter;
    use rustyline::validate::{ValidationContext, ValidationResult, Validator};
    use rustyline::{CompletionType, Config, Context, Editor, Helper};

    use super::*;

    pub(super) struct LispHelper {
        symbols: Vec<String>,
    }

    impl LispHelper {
        pub(super) fn new(symbols: Vec<String>) -> Self {
            Self { symbols }
        }

        pub(super) fn update_symbols(&mut self, symbols: Vec<String>) {
            self.symbols = symbols;
        }
    }

    impl Helper for LispHelper {}

    impl Hinter for LispHelper {
        type Hint = String;
    }

    impl Highlighter for LispHelper {}

    impl Validator for LispHelper {
        fn validate(&self, ctx: &mut ValidationContext<'_>) -> rustyline::Result<ValidationResult> {
            let input = ctx.input();
            if input.trim().is_empty()
                || is_blank_or_comment_only(input)
                || has_complete_lisp_form(input)
            {
                Ok(ValidationResult::Valid(None))
            } else {
                Ok(ValidationResult::Incomplete)
            }
        }
    }

    impl Completer for LispHelper {
        type Candidate = Pair;

        fn complete(
            &self,
            line: &str,
            pos: usize,
            _ctx: &Context<'_>,
        ) -> rustyline::Result<(usize, Vec<Pair>)> {
            let (start, prefix) = symbol_prefix_at(line, pos);
            if prefix.is_empty() {
                return Ok((pos, Vec::new()));
            }
            let matches: Vec<Pair> = self
                .symbols
                .iter()
                .filter(|sym| sym.starts_with(prefix))
                .take(64)
                .map(|sym| Pair {
                    display: sym.clone(),
                    replacement: sym.clone(),
                })
                .collect();
            Ok((start, matches))
        }
    }

    pub(super) fn run_tty_repl(
        engine: &mut Engine,
        history_path: Option<&Path>,
    ) -> Result<i32, Error> {
        let cfg = Config::builder()
            .auto_add_history(false)
            .completion_type(CompletionType::List)
            .build();
        let symbols = engine.complete_symbols().unwrap_or_default();
        let mut editor = match Editor::with_config(cfg) {
            Ok(ed) => ed,
            Err(err) => {
                eprintln!(
                    "{} tty repl unavailable ({err}); falling back to pipe mode",
                    brand::CLI_NAME
                );
                return run_pipe_repl(engine);
            }
        };
        editor.set_helper(Some(LispHelper::new(symbols)));

        if let Some(path) = history_path {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = editor.load_history(path);
        }

        let mut exit_code = 0;
        loop {
            match editor.readline(brand::REPL_PROMPT) {
                Ok(line) => {
                    if line.trim().is_empty() || is_blank_or_comment_only(&line) {
                        continue;
                    }
                    let _ = editor.add_history_entry(line.as_str());
                    match engine.eval_printed(&line) {
                        Ok(out) => {
                            println!("{out}");
                            let _ = io::stdout().flush();
                            if let Ok(syms) = engine.complete_symbols() {
                                if let Some(helper) = editor.helper_mut() {
                                    helper.update_symbols(syms);
                                }
                            }
                        }
                        Err(Error::Exit(code)) => {
                            exit_code = code;
                            break;
                        }
                        Err(err) => {
                            eprintln!("error: {err}");
                            let _ = io::stderr().flush();
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    let _ = engine.eval_silent("(setq quit-flag nil)");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    eprintln!("{} repl error: {err}", brand::CLI_NAME);
                    exit_code = 1;
                    break;
                }
            }
        }

        if let Some(path) = history_path {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = editor.save_history(path);
        }
        Ok(exit_code)
    }
}

#[cfg(feature = "repl-tty")]
use tty::run_tty_repl;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_prefix_extracts_lisp_identifiers() {
        let (start, pfx) = symbol_prefix_at("(defun foo-b", 12);
        assert_eq!(start, 7);
        assert_eq!(pfx, "foo-b");

        let (start2, pfx2) = symbol_prefix_at("  +alpha?", 9);
        assert_eq!(start2, 2);
        assert_eq!(pfx2, "+alpha?");

        let (start3, pfx3) = symbol_prefix_at("(list ", 6);
        assert_eq!(start3, 6);
        assert_eq!(pfx3, "");
    }

    #[test]
    fn blank_or_comment_detection_handles_semicolons() {
        assert!(is_blank_or_comment_only(""));
        assert!(is_blank_or_comment_only("  \n  ; comment\n\t"));
        assert!(!is_blank_or_comment_only("; header\n(+ 1 2)"));
    }
}
