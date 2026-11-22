//! Explicit optional reference-config measurement, never normal product input promotion.
#![forbid(unsafe_code)]
use super::*;
use serde_json::json;
use std::{fs, path::Path};
pub(crate) fn measure_reference_config(opts: &ConfigOptions) -> Result<ConfigReport, AndroidError> {
    let repo = config::repo();
    let reference = fs::canonicalize(&opts.reference).map_err(input)?;
    if !reference.join("configure").is_file() {
        return Err(input("reference configure is missing"));
    }
    let build = tooling::guard_scratch(&repo, &opts.build_dir).map_err(input)?;
    let output = tooling::guard_scratch(&repo, &opts.out).map_err(input)?;
    if build.exists() || output.exists() || build.starts_with(&output) || output.starts_with(&build)
    {
        return Err(input(
            "audit build/output must be fresh, separate owned directories",
        ));
    }
    let tools = discover_toolchain(&opts.tools)?;
    let run =
        tooling::owned_new_output(&repo, Some(&output), "android-config-audit").map_err(input)?;
    fs::create_dir_all(&build).map_err(input)?;
    let mut steps = vec![];
    let makeinfo = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .map(|p| p.join("makeinfo"))
        .find(|p| p.is_file())
        .ok_or_else(|| input("makeinfo required for reference config audit"))?;
    let mut env = vec![
        ("JAVA_HOME".into(), tools.jdk.home.clone().into_os_string()),
        (
            "SDK_BUILD_TOOLS".into(),
            tools.build_tools.clone().into_os_string(),
        ),
        ("ANDROID_SDK".into(), opts.tools.api.to_string().into()),
        ("ANDROID_CC".into(), tools.clang.clone().into_os_string()),
        (
            "JAVAC".into(),
            tools.jdk.home.join("bin/javac").into_os_string(),
        ),
        (
            "JARSIGNER".into(),
            tools.jdk.home.join("bin/jarsigner").into_os_string(),
        ),
        ("MAKEINFO".into(), makeinfo.into_os_string()),
    ];
    let mut path = vec![tools.jdk.home.join("bin")];
    path.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    env.push(("PATH".into(), std::env::join_paths(path).map_err(input)?));
    let plan = StepPlan {
        id: "configure".into(),
        program: reference.join("configure").into_os_string(),
        args: vec![
            "--host=aarch64-linux-android".into(),
            format!("--with-android={}", tools.android_jar.display()).into(),
            format!("CC={}", tools.clang.display()).into(),
            "--with-gnutls=ifavailable".into(),
            "emacs_cv_working_javac=yes".into(),
            "emacs_cv_android_x_or_later=yes".into(),
            "emacs_cv_tputs_lib=none required".into(),
        ],
        cwd: build.clone(),
        env,
    };
    let result = (|| -> Result<Value, AndroidError> {
        let receipt = tooling::execute_step(&plan, &run).map_err(AndroidError::Command)?;
        let result = tooling::require_success(&receipt).map_err(AndroidError::Command);
        steps.push(receipt);
        result?;
        let header = build.join("src/config.h.android");
        if !header.is_file() {
            return Err(AndroidError::Verification(
                "inner target config.h.android missing".into(),
            ));
        }
        execute(
            &repo,
            &run,
            &tools,
            &mut steps,
            "preprocessor",
            tools.clang.clone(),
            vec![
                "-std=gnu23".into(),
                "-E".into(),
                "-dM".into(),
                "-I".into(),
                build.join("src").into_os_string(),
                "-I".into(),
                reference.join("src").into_os_string(),
                "-I".into(),
                reference.join("lib").into_os_string(),
                "-x".into(),
                "c".into(),
                header.clone().into_os_string(),
            ],
        )?;
        // Measure header macros with this actual target compiler; do not infer from host values.
        let mut macros = vec![];
        for (id, header_name, name) in [
            ("signbit", "math.h", "signbit"),
            ("rename", "stdio.h", "RENAME_EXCL"),
            ("cspath", "unistd.h", "_CS_PATH"),
        ] {
            let source = run.path.join(format!("{id}.c"));
            fs::write(
                &source,
                format!("#include <{header_name}>\n#ifdef {name}\nFOUND_TARGET_MACRO\n#endif\n"),
            )
            .map_err(input)?;
            execute(
                &repo,
                &run,
                &tools,
                &mut steps,
                id,
                tools.clang.clone(),
                vec![
                    "-std=gnu23".into(),
                    "-E".into(),
                    "-P".into(),
                    source.into_os_string(),
                ],
            )?;
            let found = fs::read_to_string(run.path.join(format!("{id}.stdout.log")))
                .map_err(input)?
                .contains("FOUND_TARGET_MACRO");
            macros.push(format!(
                "{}={}",
                if name == "signbit" {
                    "HAVE_SIGNBIT"
                } else {
                    name
                },
                if found { "1" } else { "undefined" }
            ));
        }
        let api = run.path.join("api.c");
        fs::write(&api, "__ANDROID_API__\n").map_err(input)?;
        execute(
            &repo,
            &run,
            &tools,
            &mut steps,
            "api",
            tools.clang.clone(),
            vec!["-E".into(), "-P".into(), api.into_os_string()],
        )?;
        let api = fs::read_to_string(run.path.join("api.stdout.log"))
            .map_err(input)?
            .trim()
            .parse::<u32>()
            .map_err(input)?;
        macros.push(format!(
            "ANDROID_API_GE_9={}",
            if api >= 9 { "1" } else { "undefined" }
        ));
        let generated = run.path.join("config.json");
        let mut args = vec![
            repo.join("tools/gen-config-json.py").into_os_string(),
            "--keys-from-header".into(),
            "--preprocessed".into(),
            run.path.join("preprocessor.stdout.log").into_os_string(),
        ];
        for value in macros {
            args.extend(["--macro".into(), value.into()]);
        }
        args.extend([
            header.clone().into_os_string(),
            "--target".into(),
            "android-arm64 explicit reference audit".into(),
            "--out".into(),
            generated.clone().into_os_string(),
        ]);
        execute(&repo, &run, &tools, &mut steps, "json", "python3", args)?;
        Ok(
            json!({"schema":"femacs-android-config-audit-v1","status":"passed","steps":steps,"toolchain":tools,"inputs":{"configure_sha256":tooling::sha256_file(&reference.join("configure")).map_err(input)?},"products":{"configuration":generated,"sha256":tooling::sha256_file(&generated).map_err(input)?,"header_sha256":tooling::sha256_file(&header).map_err(input)?},"promoted":false,"errors":[]}),
        )
    })();
    match result {
        Ok(report) => {
            tooling::write_json(&run.path.join("report.json"), &report).map_err(input)?;
            Ok(report)
        }
        Err(e) => {
            tooling::write_json(&run.path.join("report.json"),&json!({"schema":"femacs-android-config-audit-v1","status":"failed","steps":steps,"errors":[e.to_string()],"promoted":false})).map_err(input)?;
            Err(e)
        }
    }
}
