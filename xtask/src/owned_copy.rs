//! Copy product data and permissions, never the source run's preservation flags.
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub(crate) fn copy_file(from: &Path, to: &Path) -> Result<(), String> {
    let source = File::open(from).map_err(|e| e.to_string())?;
    let before = source.metadata().map_err(|e| e.to_string())?;
    if !before.is_file() { return Err("owned copy requires a regular source file".into()); }
    match fs::symlink_metadata(to) {
        Ok(target) => {
            if !target.is_file() { return Err("owned copy refuses a nonregular destination".into()); }
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                if before.dev() == target.dev() && before.ino() == target.ino() {
                    return Err("owned copy refuses the source inode as destination".into());
                }
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.to_string()),
    }
    let mut original = source;
    let mut copied = File::create(to).map_err(|e| e.to_string())?;
    // Explicit streaming avoids platform copy primitives that also propagate
    // user-immutable/xattr policy from a selected read-only input into its output.
    let mut buffer = [0u8; 128 * 1024];
    loop {
        let size = original.read(&mut buffer).map_err(|e| e.to_string())?;
        if size == 0 { break; }
        copied.write_all(&buffer[..size]).map_err(|e| e.to_string())?;
    }
    copied.set_permissions(before.permissions()).map_err(|e| e.to_string())?;
    Ok(())
}
