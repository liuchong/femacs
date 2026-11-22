#![forbid(unsafe_code)]
//! Fe artwork and platform release packaging. The editor remains built by xtask.
use resvg::{tiny_skia, usvg};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

pub const LOGO: &[u8] = include_bytes!("../assets/fe-logo.png");
pub const APP_ICON_LIGHT: &[u8] = include_bytes!("../assets/Fe.icon/Assets/Light.svg");
pub const APP_ICON_DARK: &[u8] = include_bytes!("../assets/Fe.icon/Assets/Dark.svg");

fn icon_canvas(size: u32) -> Result<tiny_skia::Pixmap, String> {
    if !(16..=2048).contains(&size) {
        return Err("icon size must be 16..2048".into());
    }
    tiny_skia::Pixmap::new(size, size).ok_or_else(|| "icon allocation failed".to_string())
}

pub fn render(png: &[u8], size: u32) -> Result<Vec<u8>, String> {
    let mut pixels = icon_canvas(size)?;
    let source = tiny_skia::Pixmap::decode_png(png).map_err(|e| e.to_string())?;
    let paint = tiny_skia::PixmapPaint {
        quality: tiny_skia::FilterQuality::Bicubic,
        ..Default::default()
    };
    // The artwork may be rectangular; center it without stretching its glyphs.
    let scale = size as f32 / source.width().max(source.height()) as f32;
    let dx = (size as f32 - source.width() as f32 * scale) * 0.5;
    let dy = (size as f32 - source.height() as f32 * scale) * 0.5;
    pixels.draw_pixmap(
        0,
        0,
        source.as_ref(),
        &paint,
        tiny_skia::Transform::from_row(scale, 0.0, 0.0, scale, dx, dy),
        None,
    );
    pixels.encode_png().map_err(|e| e.to_string())
}

/// Render the square Dock master at its destination resolution.
pub fn render_app_icon(svg: &[u8], size: u32) -> Result<Vec<u8>, String> {
    let mut pixels = icon_canvas(size)?;
    let tree = usvg::Tree::from_data(svg, &usvg::Options::default()).map_err(|e| e.to_string())?;
    let master = tree.size();
    if master.width() != master.height() {
        return Err("Dock artwork must be square".into());
    }
    let scale = size as f32 / master.width();
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale),
        &mut pixels.as_mut(),
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
        let png = render_app_icon(APP_ICON_LIGHT, size)?;
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

fn compile_macos_icon(app: &Path, stage: &Path) -> Result<(), String> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/Fe.icon");
    let plist = app.join("Contents/Info.plist");
    run(
        Command::new("xcrun")
            .env("IBToolNeverDeque", "1")
            .env("IBCLIServerNeverDequeue", "1")
            .args(["--sdk", "macosx", "actool", "--compile"])
            .arg(stage)
            .args([
                "--platform",
                "macosx",
                "--minimum-deployment-target",
                "26.0",
                "--app-icon",
                "Fe",
                "--output-partial-info-plist",
            ])
            .arg(stage.join("icon-info.plist"))
            .arg(source),
        "compile light and dark Dock artwork (requires Xcode with Icon Composer support)",
    )?;
    let resources = app.join("Contents/Resources");
    for name in ["Assets.car", "Fe.icns"] {
        if !stage.join(name).is_file() {
            return Err(format!("icon compiler did not produce {name}"));
        }
    }
    for name in ["Assets.car", "Fe.icns"] {
        fs::rename(stage.join(name), resources.join(name)).map_err(|e| e.to_string())?;
    }
    for (key, value) in [
        ("CFBundleIconFile", "Fe"),
        ("CFBundleIconName", "Fe"),
        ("CFBundleName", "Fe"),
        ("CFBundleDisplayName", "Fe"),
    ] {
        run(
            Command::new("plutil")
                .args(["-replace", key, "-string", value])
                .arg(&plist),
            "set bundle artwork metadata",
        )?;
    }
    Ok(())
}

/// Called on an owned, unsigned bundle stage before the existing signing step.
pub fn install_macos_icon(app: &Path) -> Result<(), String> {
    if !cfg!(target_os = "macos") {
        return Err("macOS branding requires macOS".into());
    }
    validate_app(app)?;
    let stage = app.join("Contents/.fe-icon");
    fs::create_dir(&stage).map_err(|e| format!("create Dock icon stage: {e}"))?;
    let result = compile_macos_icon(app, &stage);
    let cleanup = fs::remove_dir_all(&stage).map_err(|e| format!("remove Dock icon stage: {e}"));
    result.and(cleanup)
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
    fn rectangular_artwork_keeps_proportions_and_transparent_padding() {
        let start = Instant::now();
        for (width, height, expected) in [
            (80, 40, (0, 8, 31, 23)),
            (40, 80, (8, 0, 23, 31)),
            (80, 80, (0, 0, 31, 31)),
        ] {
            let mut source = tiny_skia::Pixmap::new(width, height).unwrap();
            source.fill(tiny_skia::Color::WHITE);
            let input = source.encode_png().unwrap();
            let output = render(&input, 32).unwrap();
            let result = tiny_skia::Pixmap::decode_png(&output).unwrap();
            let mut bounds = (32, 32, 0, 0);
            for y in 0..32 {
                for x in 0..32 {
                    if result.pixels()[y * 32 + x].alpha() != 0 {
                        bounds.0 = bounds.0.min(x);
                        bounds.1 = bounds.1.min(y);
                        bounds.2 = bounds.2.max(x);
                        bounds.3 = bounds.3.max(y);
                    }
                }
            }
            assert_eq!(bounds, expected, "source {width}x{height}");
        }
        eprintln!("rectangular_artwork seconds={}", start.elapsed().as_secs_f64());
    }

    #[test]
    fn artwork_and_icns_representations() {
        let start = Instant::now();
        let light = render_app_icon(APP_ICON_LIGHT, 32).expect("render light Dock artwork");
        let dark = render_app_icon(APP_ICON_DARK, 32).expect("render dark Dock artwork");
        assert_ne!(light, dark);
        for png in [light, dark] {
            let image = tiny_skia::Pixmap::decode_png(&png).expect("decode PNG");
            assert_eq!((image.width(), image.height()), (32, 32));
            assert_eq!(image.pixel(0, 0).expect("corner").alpha(), 0);
            assert_eq!(image.pixel(16, 16).expect("center").alpha(), 255);
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
        assert!(render_app_icon(APP_ICON_LIGHT, 0).is_err());
        assert!(render_app_icon(APP_ICON_LIGHT, 4096).is_err());
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
