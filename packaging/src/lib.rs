//! Fe artwork and platform release packaging. The editor remains built by xtask.
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

pub const LOGO: &[u8] = include_bytes!("../assets/fe-logo.png");
pub const APP_ICON: &[u8] = LOGO;

pub fn render(png: &[u8], size: u32) -> Result<Vec<u8>, String> {
    if !(16..=2048).contains(&size) {
        return Err("icon size must be 16..2048".into());
    }
    let source = tiny_skia::Pixmap::decode_png(png).map_err(|e| e.to_string())?;
    let mut pixels = tiny_skia::Pixmap::new(size, size).ok_or("icon allocation failed")?;
    let paint = tiny_skia::PixmapPaint {
        quality: tiny_skia::FilterQuality::Bicubic,
        ..Default::default()
    };
    pixels.draw_pixmap(
        0,
        0,
        source.as_ref(),
        &paint,
        tiny_skia::Transform::from_scale(
            size as f32 / source.width() as f32,
            size as f32 / source.height() as f32,
        ),
        None,
    );
    pixels.encode_png().map_err(|e| e.to_string())
}

/// PNG-backed ICNS representations, including Retina sizes. No host fonts required.
pub fn icns() -> Result<Vec<u8>, String> {
    let mut data = b"icns\0\0\0\0".to_vec();
    for (kind, size) in [
        ("icp4", 16),
        ("icp5", 32),
        ("icp6", 64),
        ("ic07", 128),
        ("ic08", 256),
        ("ic09", 512),
        ("ic10", 1024),
        ("ic11", 32),
        ("ic12", 64),
        ("ic13", 256),
        ("ic14", 512),
    ] {
        let png = render(APP_ICON, size)?;
        data.extend_from_slice(kind.as_bytes());
        data.extend_from_slice(&((png.len() + 8) as u32).to_be_bytes());
        data.extend_from_slice(&png);
    }
    let length = (data.len() as u32).to_be_bytes();
    data[4..8].copy_from_slice(&length);
    Ok(data)
}

pub fn run(command: &mut Command, label: &str) -> Result<(), String> {
    let status = command.status().map_err(|e| format!("{label}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{label}: {status}"))
    }
}

/// Called on an owned, unsigned bundle stage before the existing signing step.
pub fn install_macos_icon(app: &Path) -> Result<(), String> {
    if !cfg!(target_os = "macos") {
        return Err("macOS branding requires macOS".into());
    }
    validate_app(app)?;
    fs::write(app.join("Contents/Resources/Fe.icns"), icns()?).map_err(|e| e.to_string())?;
    for (key, value) in [
        ("CFBundleIconFile", "Fe.icns"),
        ("CFBundleName", "Fe"),
        ("CFBundleDisplayName", "Fe"),
    ] {
        run(
            Command::new("plutil")
                .args(["-replace", key, "-string", value])
                .arg(app.join("Contents/Info.plist")),
            "set bundle artwork metadata",
        )?;
    }
    Ok(())
}

pub fn validate_app(app: &Path) -> Result<(), String> {
    for file in [
        "Contents/MacOS/Femacs",
        "Contents/Info.plist",
        "Contents/Resources/libexec/Femacs.pdmp",
        "Contents/Resources/etc/DOC",
        "Contents/Resources/lisp/loaddefs.el",
        "Contents/Resources/info/dir",
    ] {
        if !app.join(file).is_file() {
            return Err(format!("missing app member: {file}"));
        }
    }
    Ok(())
}

pub fn sha256(path: &Path) -> Result<String, String> {
    use std::io::Read;
    let mut f = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut hash = Sha256::new();
    let mut buffer = [0; 65536];
    loop {
        let n = f.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

/// Hash every non-signature byte of a thin 64-bit Mach-O, normalizing only the
/// signature length and __LINKEDIT sizes that codesign may legitimately adjust.
pub fn macho_payload_sha256(path: &Path) -> Result<String, String> {
    let length = fs::metadata(path).map_err(|e| e.to_string())?.len();
    if !(32..=64 * 1024 * 1024).contains(&length) {
        return Err("unsupported Mach-O size".into());
    }
    let mut data = fs::read(path).map_err(|e| e.to_string())?;
    let u32_at = |bytes: &[u8], offset: usize| -> Result<u32, String> {
        let word: [u8; 4] = bytes
            .get(offset..offset + 4)
            .ok_or("truncated Mach-O")?
            .try_into()
            .map_err(|_| "truncated word")?;
        Ok(u32::from_le_bytes(word))
    };
    if u32_at(&data, 0)? != 0xfeedfacf {
        return Err("only thin 64-bit Mach-O is supported".into());
    }
    let count = u32_at(&data, 16)? as usize;
    let end = 32usize
        .checked_add(u32_at(&data, 20)? as usize)
        .ok_or("load-command overflow")?;
    if end > data.len() || count > 4096 {
        return Err("invalid load commands".into());
    }
    let mut offset = 32;
    let mut signature = None;
    for _ in 0..count {
        let command = u32_at(&data, offset)?;
        let size = u32_at(&data, offset + 4)? as usize;
        if size < 8 || offset.checked_add(size).is_none_or(|x| x > end) {
            return Err("invalid command size".into());
        }
        if command == 0x1d {
            if size != 16 || signature.is_some() {
                return Err("invalid signature command".into());
            }
            let start = u32_at(&data, offset + 8)? as usize;
            let bytes = u32_at(&data, offset + 12)? as usize;
            if start < end || start.checked_add(bytes) != Some(data.len()) {
                return Err("signature must occupy the file tail".into());
            }
            signature = Some(start);
            data[offset + 12..offset + 16].fill(0);
        }
        if command == 0x19
            && size >= 72
            && &data[offset + 8..offset + 24] == b"__LINKEDIT\0\0\0\0\0\0"
        {
            data[offset + 32..offset + 40].fill(0); // vmsize
            data[offset + 48..offset + 56].fill(0); // filesize
        }
        offset += size;
    }
    if offset != end {
        return Err("load-command extent mismatch".into());
    }
    let start = signature.ok_or("signed input Mach-O required")?;
    Ok(format!("{:x}", Sha256::digest(&data[..start])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn artwork_and_icns_representations() {
        let start = Instant::now();
        for svg in [LOGO, APP_ICON] {
            let png = render(svg, 32).expect("render artwork");
            let image = tiny_skia::Pixmap::decode_png(&png).expect("decode PNG");
            assert_eq!((image.width(), image.height()), (32, 32));
            assert!(image.pixels().iter().any(|p| p.alpha() > 0));
        }
        let file = icns().expect("ICNS generation");
        assert_eq!(&file[..4], b"icns");
        assert_eq!(
            u32::from_be_bytes(file[4..8].try_into().unwrap()) as usize,
            file.len()
        );
        let mut offset = 8;
        let mut count = 0;
        while offset < file.len() {
            let size =
                u32::from_be_bytes(file[offset + 4..offset + 8].try_into().unwrap()) as usize;
            assert!(size > 8);
            let image = tiny_skia::Pixmap::decode_png(&file[offset + 8..offset + size])
                .expect("representation PNG");
            assert_eq!(image.width(), image.height());
            offset += size;
            count += 1;
        }
        assert_eq!(offset, file.len());
        assert_eq!(count, 11);
        assert!(render(APP_ICON, 0).is_err());
        assert!(render(APP_ICON, 4096).is_err());
        eprintln!(
            "artwork_and_icns_representations seconds={}",
            start.elapsed().as_secs_f64()
        );
    }

    fn fixture() -> Vec<u8> {
        let mut data = vec![0x35; 256];
        data[..32].fill(0);
        for (offset, word) in [
            (0, 0xfeedfacfu32),
            (16, 1),
            (20, 16),
            (32, 0x1d),
            (36, 16),
            (40, 192),
            (44, 64),
        ] {
            data[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
        }
        data
    }
    #[test]
    fn signature_changes_but_code_changes_do_not_match() {
        let start = Instant::now();
        let out = std::env::var_os("FE_PACKAGING_TEST_DIR").expect("fresh owned test directory");
        let out = Path::new(&out);
        assert!(out.is_absolute(), "absolute owned test output required");
        fs::create_dir_all(out).unwrap();
        let p = out.join("signature.fixture");
        let mut data = fixture();
        fs::write(&p, &data).unwrap();
        let original = macho_payload_sha256(&p).unwrap();
        data[220] ^= 1;
        fs::write(&p, &data).unwrap();
        assert_eq!(original, macho_payload_sha256(&p).unwrap());
        data[100] ^= 1;
        fs::write(&p, &data).unwrap();
        assert_ne!(original, macho_payload_sha256(&p).unwrap());
        data[44..48].copy_from_slice(&63u32.to_le_bytes());
        fs::write(&p, &data).unwrap();
        assert!(macho_payload_sha256(&p).is_err());
        eprintln!(
            "signature_changes_but_code_changes_do_not_match seconds={}",
            start.elapsed().as_secs_f64()
        );
    }

    #[test]
    fn malformed_macho_is_rejected() {
        let start = Instant::now();
        let out = std::env::var_os("FE_PACKAGING_TEST_DIR").expect("fresh owned test directory");
        let out = Path::new(&out);
        assert!(out.is_absolute(), "absolute owned test output required");
        fs::create_dir_all(out).unwrap();
        let p = out.join("invalid.fixture");
        for data in [
            vec![0; 8],
            {
                let mut x = fixture();
                x[0] = 0;
                x
            },
            {
                let mut x = fixture();
                x[36..40].copy_from_slice(&0u32.to_le_bytes());
                x
            },
            {
                let mut x = fixture();
                x[16..20].copy_from_slice(&4097u32.to_le_bytes());
                x
            },
        ] {
            fs::write(&p, data).unwrap();
            assert!(macho_payload_sha256(&p).is_err());
        }
        eprintln!(
            "malformed_macho_is_rejected seconds={}",
            start.elapsed().as_secs_f64()
        );
    }
}
