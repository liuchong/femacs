//! Explicit unit lists use the same normal/checking compiler as full builds.
use super::*;

pub(super) fn object_path(root: &Path, stem: &str, checking: bool, target: Option<&str>) -> PathBuf {
    let checking = if checking { ".checking" } else { "" };
    let target = target.filter(|t| !t.is_empty()).map(|t|format!(".{t}")).unwrap_or_default();
    root.join(format!("{stem}{checking}{target}.o"))
}

fn parse(text: &str) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    let mut seen = BTreeSet::new();
    for line in text.lines().map(str::trim).filter(|s| !s.is_empty() && !s.starts_with('#')) {
        let path = Path::new(line);
        if !path.components().all(|c| matches!(c, std::path::Component::Normal(_)))
            || !UNIT_DIRS.contains(&path.components().next().unwrap().as_os_str().to_str().unwrap_or("")) {
            return Err(format!("unit must be a repository-owned source: {line}"));
        }
        let unit = match path.extension().and_then(|s|s.to_str()) {
            Some("rs") => line.to_string(),
            Some("c") => path.with_extension("rs").to_string_lossy().to_string(),
            _ => return Err(format!("not a C/Rust translation unit: {line}")),
        };
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        if !repo.join(&unit).is_file() || !seen.insert(unit.clone()) {
            return Err(format!("missing or duplicate unit: {line}"));
        }
        result.push(unit);
    }
    if result.is_empty() { return Err("empty unit list".into()); }
    Ok(result)
}

pub(super) fn run(list: Option<&String>) -> i32 {
    let result = (|| -> Result<(),String> {
        let path = list.map(String::as_str).unwrap_or("docs/baseline/frozen-build-sources.txt");
        let units = parse(&fs::read_to_string(path).map_err(|e|format!("{path}: {e}"))?)?;
        let count = units.len();
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .map_err(|e|e.to_string())?.as_nanos();
        let materialization = build_root().join("check-units")
            .join(format!("{}-{nonce}",std::process::id()));
        fs::create_dir_all(&materialization).map_err(|e|e.to_string())?;
        // Preserve ordinary object locations for the existing link consumers;
        // isolate materialized source/metadata and never delete another run's tree.
        build_selected_units(units, &materialization)?;
        println!("checked {count} unit(s), normal and enable_checking, 0 failure(s)");
        Ok(())
    })();
    match result { Ok(()) => 0, Err(e) => fail(e) }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn timed(name: &str, f: impl FnOnce()) {
        let start = std::time::Instant::now(); f();
        println!("CASE {name} seconds={:.6}", start.elapsed().as_secs_f64());
    }
    #[test]
    fn maps_owned_c_and_rust_inputs() { timed("unit_list_mapping", || {
        assert_eq!(parse("# comment\nsrc/doprnt.c\n").unwrap(), vec!["src/doprnt.rs"]);
        assert_eq!(parse("src/doprnt.rs\n").unwrap(), vec!["src/doprnt.rs"]);
    }); }
    #[test]
    fn rejects_empty_unknown_and_duplicate_inputs() { timed("unit_list_invalid", || {
        for source in ["# empty", "src/no_such_unit.c", "src/doprnt.c\nsrc/doprnt.rs", "src/doprnt.h"] {
            assert!(parse(source).is_err(), "{source}");
        }
    }); }
    #[test]
    fn rejects_escape_from_owned_source_directories() { timed("unit_list_boundaries", || {
        for source in ["../src/doprnt.c", "src/../src/doprnt.c", "README.md", "tools/anything.rs"] {
            assert!(parse(source).is_err(), "{source}");
        }
    }); }
    #[test]
    fn target_objects_do_not_overwrite_host_objects() { timed("target_object_names", || {
        let root = Path::new("build/objects");
        assert_eq!(object_path(root,"doprnt",false,None),root.join("doprnt.o"));
        assert_eq!(object_path(root,"doprnt",true,Some("")),root.join("doprnt.checking.o"));
        assert_eq!(object_path(root,"doprnt",false,Some("aarch64-linux-android")),root.join("doprnt.aarch64-linux-android.o"));
        assert_eq!(object_path(root,"doprnt",true,Some("aarch64-linux-android")),root.join("doprnt.checking.aarch64-linux-android.o"));
    }); }
}
