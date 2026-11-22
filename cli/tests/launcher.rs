#![forbid(unsafe_code)]

#[cfg(target_os = "macos")]
mod macos {
    use std::{
        env, fs, io,
        os::unix::fs::PermissionsExt,
        path::{Path, PathBuf},
        process::{Command, Output},
        sync::atomic::{AtomicUsize, Ordering},
    };

    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    fn claim_sandbox_dir_in(base: &Path, counter: &AtomicUsize) -> PathBuf {
        let pid = std::process::id();
        for _ in 0..256 {
            let id = counter.fetch_add(1, Ordering::Relaxed);
            let candidate = base.join(format!("femacs-cli-test-{pid}-{id}"));
            match fs::create_dir(&candidate) {
                Ok(()) => return candidate,
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("could not create sandbox {}: {error}", candidate.display()),
            }
        }
        panic!("exhausted bounded sandbox directory candidates");
    }

    struct Sandbox {
        root: PathBuf,
        app: PathBuf,
        xdg: PathBuf,
        log: PathBuf,
    }

    impl Sandbox {
        fn new() -> Self {
            let root = claim_sandbox_dir_in(&env::temp_dir(), &NEXT_ID);
            let sandbox = Self {
                app: root.join("Fe.app"),
                xdg: root.join("xdg"),
                log: root.join("calls.log"),
                root,
            };
            let executable = sandbox.app.join("Contents/MacOS/Femacs");
            fs::create_dir_all(executable.parent().expect("bundle MacOS dir")).expect("mkdir app");
            fs::create_dir_all(&sandbox.xdg).expect("mkdir xdg");
            fs::write(
                &executable,
                concat!(
                    "#!/bin/sh\n",
                    "printf 'ARGV:%s|LC:%s\\n' \"$*\" \"${LC_ALL:-}\" >> \"$MOCK_LOG\"\n",
                    "printf 'NATIVE:%s\\n' \"$*\"\n",
                    "for arg in \"$@\"; do\n",
                    "  if [ \"$arg\" = \"fail.txt\" ]; then exit 42; fi\n",
                    "done\n",
                ),
            )
            .expect("write mock editor");
            fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).expect("chmod");
            sandbox
        }

        fn run(&self, bin: &str, args: &[&str], extra_env: &[(&str, &Path)]) -> Output {
            let mut command = Command::new(bin);
            command
                .args(args)
                .env("FEMACS_APP", &self.app)
                .env("XDG_CONFIG_HOME", &self.xdg)
                .env("MOCK_LOG", &self.log)
                .env("LC_ALL", "en_US.UTF-8")
                .env_remove("FEMACS_TTY_INIT")
                .env_remove("FEMACS_TTY_LOCALE");
            for (key, value) in extra_env {
                command.env(key, value);
            }
            command.output().expect("spawn launcher")
        }

        fn take_calls(&self) -> Vec<String> {
            let content = fs::read_to_string(&self.log).unwrap_or_default();
            let _ = fs::remove_file(&self.log);
            content.lines().map(String::from).collect()
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn standalone_help_preserves_token_and_skips_preset() {
        let sandbox = Sandbox::new();
        let missing_init = sandbox.root.join("missing.el");
        let fe = env!("CARGO_BIN_EXE_fe");

        for (args, expected_token) in [
            (&["--help"][..], "--help"),
            (&["-help"][..], "-help"),
            (&["-1", "a.txt", "-R", "-help"][..], "-help"),
        ] {
            let out = sandbox.run(fe, args, &[("FEMACS_TTY_INIT", &missing_init)]);
            assert!(out.status.success(), "{args:?}: {out:?}");
            let stdout = String::from_utf8_lossy(&out.stdout);
            assert!(stdout.starts_with("Usage: fe [OPTION-OR-FILENAME]...\n"));
            assert!(
                stdout.ends_with(&format!(
                    "Native editor help (macOS)\n\nNATIVE:{expected_token}\n"
                )),
                "unexpected stdout: {stdout}"
            );
            assert_eq!(
                sandbox.take_calls(),
                vec![format!("ARGV:{expected_token}|LC:en_US.UTF-8")]
            );
        }
    }

    #[test]
    fn combined_help_keeps_normal_preset_and_checks() {
        let sandbox = Sandbox::new();
        let fe = env!("CARGO_BIN_EXE_fe");

        for (args, expected) in [
            (&["--help", "--version"][..], "-Q -nw --help --version"),
            (&["--version", "--help"][..], "-Q -nw --version --help"),
            (&["notes.org", "--help"][..], "-Q -nw notes.org --help"),
            (&["--hel"][..], "-Q -nw --hel"),
            (&["--", "--help"][..], "-Q -nw -- --help"),
        ] {
            let out = sandbox.run(fe, args, &[]);
            assert!(out.status.success(), "{args:?}: {out:?}");
            let stdout = String::from_utf8_lossy(&out.stdout);
            assert!(!stdout.contains("Usage: fe"), "{args:?}: {stdout}");
            assert_eq!(stdout, format!("NATIVE:{expected}\n"));
            let _ = sandbox.take_calls();
        }

        let missing_init = sandbox.root.join("missing.el");
        let out = sandbox.run(
            fe,
            &["--help", "--version"],
            &[("FEMACS_TTY_INIT", &missing_init)],
        );
        assert_eq!(out.status.code(), Some(1));
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(stderr.starts_with("fe: FEMACS_TTY_INIT is not a file:"));
    }

    #[test]
    fn reset_dashdash_and_sequential_boundaries() {
        let sandbox = Sandbox::new();
        let fe = env!("CARGO_BIN_EXE_fe");

        let out = sandbox.run(
            fe,
            &[
                "-1",
                "ignored.txt",
                "-R",
                "-x",
                "--reset-args",
                "-nw",
                "notes.org",
            ],
            &[("FEMACS_TTY_LOCALE", Path::new("C"))],
        );
        assert!(out.status.success());
        assert_eq!(
            sandbox.take_calls(),
            vec!["ARGV:-nw notes.org|LC:en_US.UTF-8"]
        );

        let out = sandbox.run(fe, &["--", "-R", "-1"], &[]);
        assert!(out.status.success());
        assert_eq!(
            sandbox.take_calls(),
            vec!["ARGV:-Q -nw -- -R -1|LC:en_US.UTF-8"]
        );

        let out = sandbox.run(
            fe,
            &["-1", "--", "-R", "-1"],
            &[("FEMACS_TTY_LOCALE", Path::new("C"))],
        );
        assert!(out.status.success());
        assert_eq!(
            sandbox.take_calls(),
            vec!["ARGV:-Q -nw --file -R|LC:C", "ARGV:-Q -nw --file -1|LC:C"]
        );

        let out = sandbox.run(fe, &["-R", "-1", "fail.txt", "after.txt"], &[]);
        assert_eq!(out.status.code(), Some(42));
        assert_eq!(
            sandbox.take_calls(),
            vec!["ARGV:--file fail.txt|LC:en_US.UTF-8"]
        );
    }

    #[test]
    fn error_prefixes_and_native_passthrough() {
        let sandbox = Sandbox::new();
        let fe = env!("CARGO_BIN_EXE_fe");
        let femacs = env!("CARGO_BIN_EXE_femacs");

        let out = sandbox.run(fe, &["-1"], &[]);
        assert_eq!(out.status.code(), Some(1));
        assert_eq!(
            String::from_utf8_lossy(&out.stderr),
            "fe: -1 requires at least one file\n"
        );

        let out = Command::new(fe)
            .env("FEMACS_APP", "")
            .output()
            .expect("run fe");
        assert_eq!(out.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&out.stderr).starts_with("fe: FEMACS_APP is empty"));

        let out = Command::new(femacs)
            .env("FEMACS_APP", "")
            .output()
            .expect("run femacs");
        assert_eq!(out.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&out.stderr).starts_with("femacs: FEMACS_APP is empty"));

        let out = sandbox.run(femacs, &["-R", "-1", "--help"], &[]);
        assert!(out.status.success());
        assert_eq!(
            String::from_utf8_lossy(&out.stdout),
            "NATIVE:-R -1 --help\n"
        );
    }

    #[test]
    fn sandbox_claim_skips_existing_directory_without_deleting_it() {
        let sandbox = Sandbox::new();
        let counter = AtomicUsize::new(0);
        let occupied = sandbox
            .root
            .join(format!("femacs-cli-test-{}-0", std::process::id()));
        fs::create_dir(&occupied).expect("occupy candidate");
        let keep = occupied.join("keep.txt");
        fs::write(&keep, "foreign").expect("write foreign file");

        let claimed = claim_sandbox_dir_in(&sandbox.root, &counter);
        assert_eq!(
            claimed,
            sandbox
                .root
                .join(format!("femacs-cli-test-{}-1", std::process::id()))
        );
        assert!(keep.is_file());
    }
}

#[cfg(not(target_os = "macos"))]
#[test]
fn non_macos_reports_command_prefix() {
    use std::process::Command;

    for (bin, prefix) in [
        (env!("CARGO_BIN_EXE_fe"), "fe"),
        (env!("CARGO_BIN_EXE_femacs"), "femacs"),
    ] {
        let out = Command::new(bin).output().expect("spawn launcher");
        assert_eq!(out.status.code(), Some(1));
        assert_eq!(
            String::from_utf8_lossy(&out.stderr),
            format!("{prefix}: the Fe application launcher requires macOS\n")
        );
    }
}
