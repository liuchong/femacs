use fe_packaging::{
    APP_ICON_DARK, APP_ICON_LIGHT, LOGO, install_macos_icon, macho_payload_sha256, render,
    render_app_icon, run, sha256, validate_app,
};
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    match execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
fn execute() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("artwork") if args.len() == 3 && args[1] == "--out" => {
            let out = PathBuf::from(&args[2]);
            if let Some(parent) = out.parent().filter(|p| !p.as_os_str().is_empty()) { fs::create_dir_all(parent).map_err(|e|e.to_string())?; }
            fs::create_dir(&out).map_err(|e| format!("fresh artwork directory required: {e}"))?;
            fs::write(out.join("fe-logo.png"), render(LOGO, 1024)?).map_err(|e|e.to_string())?;
            for (name, svg) in [("fe-app", APP_ICON_LIGHT), ("fe-app-dark", APP_ICON_DARK)] {
                fs::write(out.join(format!("{name}.png")), render_app_icon(svg, 1024)?).map_err(|e|e.to_string())?;
            }
            fs::write(out.join("Fe.icns"), fe_packaging::icns()?).map_err(|e|e.to_string())?;
            Ok(())
        },
        Some("macos") if args.len() == 5 && args[1] == "--app" && args[3] == "--out" => {
            if !cfg!(target_os="macos") { return Err("macOS release packaging requires macOS".into()); }
            let input = fs::canonicalize(&args[2]).map_err(|e|e.to_string())?;
            let output = PathBuf::from(&args[4]);
            validate_app(&input)?;
            run(Command::new("codesign").args(["--verify","--strict","--deep"]).arg(&input),"verify input bundle")?;
            let program = "Contents/MacOS/Femacs";
            let dump = "Contents/Resources/libexec/Femacs.pdmp";
            let input_program = sha256(&input.join(program))?;
            let input_dump = sha256(&input.join(dump))?;
            let payload_sha = macho_payload_sha256(&input.join(program))?;
            if let Some(parent) = output.parent().filter(|p| !p.as_os_str().is_empty()) { fs::create_dir_all(parent).map_err(|e|e.to_string())?; }
            fs::create_dir(&output).map_err(|e|format!("fresh release directory required: {e}"))?;
            let output = fs::canonicalize(output).map_err(|e|e.to_string())?;
            // Keep the trial App beside the DMG; never rewrite the supplied editor product.
            let payload = output.join("payload");
            fs::create_dir(&payload).map_err(|e|e.to_string())?;
            let app = payload.join("Fe.app");
            run(Command::new("ditto").arg(&input).arg(&app),"copy release app")?;
            if sha256(&app.join(program))? != input_program || sha256(&app.join(dump))? != input_dump { return Err("copy changed program or dump".into()); }
            install_macos_icon(&app)?;
            run(Command::new("codesign").args(["--force","--sign","-"]).arg(&app),"ad-hoc sign release app")?;
            run(Command::new("codesign").args(["--verify","--strict","--deep"]).arg(&app),"verify release app")?;
            if macho_payload_sha256(&app.join(program))? != payload_sha || sha256(&app.join(dump))? != input_dump {
                return Err("packaging changed editor code or Lisp dump".into());
            }
            let image = output.join("Fe-macos.dmg");
            // The disk image root contains Fe.app, not its Contents directory.
            run(Command::new("hdiutil").args(["create","-volname","Fe","-fs","HFS+","-format","UDZO","-srcfolder"])
                .arg(&payload).arg(&image),"create release disk image")?;
            run(Command::new("hdiutil").arg("verify").arg(&image),"verify release disk image")?;
            let signed_program = sha256(&app.join(program))?;
            let icon_sha = sha256(&app.join("Contents/Resources/Fe.icns"))?;
            let icon_assets_sha = sha256(&app.join("Contents/Resources/Assets.car"))?;
            fs::rename(&app, output.join("Fe.app")).map_err(|e|e.to_string())?;
            fs::remove_dir(&payload).map_err(|e|e.to_string())?;
            let image_sha = sha256(&image)?;
            fs::write(output.join("SHA256SUMS"),format!("{image_sha}  Fe-macos.dmg\n")).map_err(|e|e.to_string())?;
            let manifest = serde_json::json!({"schema":"fe-release-v1","platform":"macos",
                "architecture":env::consts::ARCH,"signing":"ad-hoc; not notarized",
                "app":"Fe.app","image":"Fe-macos.dmg","image_sha256":image_sha,
                "input_program_sha256":input_program,"program_sha256":signed_program,
                "macho_payload_sha256":payload_sha,"dump_sha256":input_dump,
                "icon_sha256":icon_sha,"icon_assets_sha256":icon_assets_sha,
                "compatibility":"packaging verification only; see the matching native acceptance record"});
            fs::write(output.join("release.json"),format!("{}\n",serde_json::to_string_pretty(&manifest).map_err(|e|e.to_string())?)).map_err(|e|e.to_string())?;
            println!("{}", output.display());
            Ok(())
        },
        _ => Err("usage: cargo run -p fe-packaging -- artwork --out <fresh-directory>\n       cargo run -p fe-packaging -- macos --app <built-app> --out <fresh-directory>".into())
    }
}
