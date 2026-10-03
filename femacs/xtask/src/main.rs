//! femacs build and gate driver.
//!
//! Replaces autoconf and make: it compiles one translation unit per rustc
//! invocation (`PORTING.md` 0.1), generates the configuration mirror and the
//! layout assertions from the frozen baselines, and runs the mechanical gates.
//!
//! `cargo run -p xtask -- <command>`, with the femacs directory as cwd.
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

const REPO: &str = "..";
const BUILD: &str = "../build/femacs";

/// The directories a ported unit can live in.  `docs/port-map.md` pairs every reference file
/// with a target path, and the C tree's non-`src/` directories keep their own names (PORTING.md
/// 0.1: `lib/`, `lib-src/`, `lwlib/`, `oldXMenu/`, `exec/`, `nt/`, `msdos/`), so the unit
/// lookup and the whole-tree walks have to cover all of them - before this list existed,
/// `build-tu lib-src/etags.c` could not even find its own file.  Order matters only for the
/// `.rs` lookup: the first directory holding the stem wins, and no stem exists twice.
const UNIT_DIRS: &[&str] = &["src", "lib", "lib-src", "lwlib", "oldXMenu", "exec", "nt", "msdos"];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    let code = match cmd {
        "gen-config" => gen_config(),
        "gen-globals" => gen_globals(),
        "layout-asserts" => match layout_asserts() {
            Ok(p) => {
                println!("wrote {}", p.display());
                0
            }
            Err(e) => fail(e),
        },
        "layout-check" => layout_check(),
        "build-tu" => build_tu(args.get(1)),
        "probe" => probe_build(&args[1..]),
        "gate-fast" => gate_fast(),
        "gate-probes" => gate_probes(),
        "dead-arms" => dead_arms(),
        "gate-full" => {
            eprintln!(
                "gate-full needs the reference Emacs configured, built and its ERT suite \
                 runnable; it is not available until the port reaches the loading ladder \
                 (docs/port-plan.md section 7). Nothing was checked."
            );
            2
        }
        _ => {
            println!(
                "usage: xtask <gen-config|layout-asserts|layout-check|build-tu <c-file>|\
                 gate-fast|gate-probes|dead-arms|gate-full>"
            );
            0
        }
    };
    exit(code);
}

fn fail(msg: impl std::fmt::Display) -> i32 {
    eprintln!("error: {msg}");
    1
}

// --- configuration ---------------------------------------------------------

/// Minimal reader for the frozen configuration files written by
/// tools/gen-config-baseline (a flat JSON object with a "macros" object).
fn read_config(path: &Path) -> BTreeMap<String, String> {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut out = BTreeMap::new();
    let mut in_macros = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("\"macros\"") {
            in_macros = true;
            continue;
        }
        if in_macros && t == "}" {
            break;
        }
        if in_macros {
            if let Some((k, v)) = t.split_once(':') {
                let k = k.trim().trim_matches('"').to_string();
                // A quoted value is a JSON *string*: its escapes have to come off, or the
                // reader hands the rest of the program the JSON-escaped text instead of the
                // macro body.  `SYSTEM_TYPE` is the entry where it showed: the file's body is
                // `"darwin"`, this reader used to deliver `\"darwin\`, and the generated
                // constant was a `&str` holding `"darwin"` *with* its C quotes - not the value
                // `intern_c_string` needs (docs/status.md 3.64).
                let raw = v.trim().trim_end_matches(',');
                let v = if raw.starts_with('"') && raw.ends_with('"') && raw.len() >= 2 {
                    json_unescape(&raw[1..raw.len() - 1])
                        .unwrap_or_else(|| raw.trim_matches('"').to_string())
                } else {
                    raw.trim().to_string()
                };
                out.insert(k, v);
            }
        }
    }
    out
}

fn gen_config() -> i32 {
    let dir = Path::new(REPO).join("docs/baseline/config");
    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)
        .map(|d| d.filter_map(Result::ok).map(|e| e.path()).collect())
        .unwrap_or_default();
    // The frozen configuration is the *`.json`* in that directory.  Without the filter this
    // took the first name alphabetically, and `docs/baseline/config/cond-eval-verified.txt`
    // sorts before `darwin-arm64.json` - so a regeneration wrote a three-line `config.rs`
    // and silently dropped every constant (caught while adding the `SYSTEM_TYPE` decoding
    // below, docs/status.md 3.64; the file is generated, so the mistake was one command away
    // from being committed).
    entries.retain(|p| p.extension().is_some_and(|e| e == "json"));
    entries.sort();
    let Some(target) = entries.into_iter().next() else {
        return fail(format!("no *.json frozen configuration in {}", dir.display()));
    };
    let macros = read_config(&target);
    // No inner attributes: this file is textually included by every translation
    // unit, and inner attributes are only legal at the top of a module.
    let mut rs = String::from(
        "// Generated by `xtask gen-config` from the frozen configuration; do not edit.\n\
         // Source: docs/baseline/config/*.json (PORTING.md P-08).\n\n",
    );
    for (k, v) in &macros {
        let line = match v.as_str() {
            "absent" => continue,                   // not in config.h at all
            "undefined" => format!("pub const {k}: bool = false; // #undef in config.h\n"),
            "1" | "true" => format!("pub const {k}: bool = true;\n"),
            "0" | "false" => format!("pub const {k}: bool = false;\n"),
            other => {
                if let Ok(n) = other.parse::<i64>() {
                    format!("pub const {k}: i64 = {n};\n")
                } else if let Some(value) = c_string_value(other) {
                    // A `#define NAME "text"` in config.h is a *C string*, and the C code that
                    // reads it passes it to a `char *`/`const char *` parameter
                    // (`intern_c_string (SYSTEM_TYPE)` in emacs.c 3514).  Emitting the macro
                    // *body* as a Rust `&str` (`"\"darwin\""`, whose value still carries the C
                    // quotes) was the mirror defect print's neighbour found in docs/status.md
                    // 3.64; the constant is a `&CStr` of the decoded value instead, the shape
                    // emacs.rs already uses for its other config.h strings (`PATH_EXEC`).
                    format!("pub const {k}: &core::ffi::CStr = c{}; // C string literal\n",
                            c_literal(&value))
                } else {
                    format!("pub const {k}: &str = {};\n", rust_str(other))
                }
            }
        };
        rs.push_str(&line);
    }
    let out = Path::new("src/include/config.rs");
    fs::create_dir_all(out.parent().unwrap()).ok();
    if let Err(e) = fs::write(out, rs) {
        return fail(e);
    }
    println!("wrote {} from {}", out.display(), target.display());
    0
}

/// A JSON string's escapes, decoded.  `None` for a form this reader does not spell (then the
/// caller keeps the text as written rather than guessing).
fn json_unescape(s: &str) -> Option<String> {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next()? {
            '"' => out.push('"'),
            '\\' => out.push('\\'),
            '/' => out.push('/'),
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            'b' => out.push('\u{8}'),
            'f' => out.push('\u{c}'),
            _ => return None,
        }
    }
    Some(out)
}

/// The value of a C string literal written as its `#define` body, or None when the text is not
/// one (or uses an escape this generator does not spell - then the body is kept verbatim rather
/// than guessed at).
fn c_string_value(body: &str) -> Option<String> {
    let inner = body.strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next()? {
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            '\\' => out.push('\\'),
            '"' => out.push('"'),
            '\'' => out.push('\''),
            '0' => out.push('\0'),
            _ => return None,
        }
    }
    Some(out)
}

/// A Rust `c"..."` literal for a value that has no NUL in it (the C string's terminator is the
/// literal's own), escaping what the literal grammar needs.
fn c_literal(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 || c as u32 == 0x7f => out.push_str(&format!("\\x{:02x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn rust_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Emit src/include/globals.rs from the frozen symbol table.  In the reference
/// this file comes from make-docfile (`#define iQfoo N`, `DEFINE_LISP_SYMBOL`);
/// the port declares each Q symbol as a static initialised at startup, because
/// Rust cannot fold an array address into a constant (PORTING.md P-17).
fn gen_globals() -> i32 {
    let tsv = Path::new(REPO).join("docs/baseline/globals/symbols.tsv");
    let text = match fs::read_to_string(&tsv) {
        Ok(t) => t,
        Err(e) => return fail(format!("{}: {e}", tsv.display())),
    };
    let mut rows = Vec::new();
    for line in text.lines().skip(1) {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() >= 3 {
            rows.push((f[0].to_string(), f[1].to_string()));
        }
    }
    let mut rs = String::from(
        "// Generated by `xtask gen-globals` from docs/baseline/globals/symbols.tsv;\n\
         // do not edit.  The reference generates globals.h with make-docfile\n\
         // (`#define iQfoo N` and `DEFINE_LISP_SYMBOL (Qfoo)`), which makes every Q\n\
         // symbol a compile-time constant.  Rust cannot fold the symbol array\n\
         // address into a constant, so the port defines an inline function per\n\
         // symbol: same name, same value, call shape `Qnil()` (PORTING.md P-17).\n\n",
    );
    for (index, name) in &rows {
        // The reference writes `#define iQfoo N` next to `DEFINE_LISP_SYMBOL (Qfoo)`;
        // the port keeps the constant so that derivations such as NIL_IS_ZERO stay
        // tied to the generated table (src/lisp.h 1999).
        rs.push_str(&format!("pub const i{name}: usize = {index};\n"));
        rs.push_str(&format!(
            "#[inline(always)]\npub fn {name} () -> Lisp_Object {{ builtin_lisp_symbol ({index}) }}\n"
        ));
    }
    rs.push_str(&format!(
        "\npub const lispsym_count: usize = {};\n\
         unsafe extern \"C\" {{ pub static mut lispsym: [Lisp_Symbol; lispsym_count]; }}\n",
        rows.len()
    ));
    // The other half of the generated globals: the V* variables that the DEFVAR_*
    // forms declare.  The reference keeps them inside struct emacs_globals
    // (`globals.f_Vfoo`) so a debugger can list them; the port declares one static
    // per variable, which keeps the reference's source shape (`Vfoo = x`) and
    // records the difference (PORTING.md P-21).
    let vars = Path::new(REPO).join("docs/baseline/globals/variables.tsv");
    let mut n_vars = 0usize;
    if let Ok(text) = fs::read_to_string(&vars) {
        // The reference keeps them inside struct emacs_globals (`globals.f_Vfoo`) so a
        // debugger can list them; the port declares each as an extern global in a
        // glob-imported module, and the unit that owns it defines it
        // `#[unsafe(no_mangle)] pub static mut` — a definition shadows the glob
        // import, exactly as a DEFUN shadows its generated prototype (PORTING.md
        // 0.1-10/0.1-3, P-21).  Defining them here instead would give every
        // translation unit its own NULL copy, which the first inlined reader of a
        // DEFVAR'd table would then dereference (review A, F4).
        rs.push_str("\n// --- variables declared by DEFVAR_* (generated) ---\n\
                     pub mod variables {\n    use super::*;\n    unsafe extern \"C\" {\n");
        for line in text.lines().skip(1) {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 3 {
                continue;
            }
            let (name, kind) = (f[0], f[1]);
            let ty = match kind {
                "LISP_OBJECT" => "Lisp_Object",
                "EMACS_INTEGER" => "EMACS_INT",
                "BOOLEAN" => "bool",
                "KBOARD" => "*mut c_void",
                _ => continue,
            };
            rs.push_str(&format!("        pub static mut {name}: {ty}; // {}\n", f[2]));
            n_vars += 1;
        }
        rs.push_str("    }\n}\nuse variables::*;\n");
    }

    // The other generated half of the reference's globals.h: make-docfile writes
    // `EXFUN (Ffoo, maxargs)` for every DEFUN (lib-src/make-docfile.c 756), so every
    // translation unit sees a prototype for every Lisp-callable function.  Rust
    // cannot declare and define a function under one name in one module, so the
    // prototypes live in a module that the unit glob-imports: a definition in the
    // unit shadows the glob import, which is C's prototype-plus-definition pair
    // (PORTING.md P-17).  Signatures come from the frozen DEFUN table, so they are
    // the arities the reference compiler saw.
    let subrs = Path::new(REPO).join("docs/baseline/subrs.tsv");
    let mut n_subrs = 0usize;
    if let Ok(text) = fs::read_to_string(&subrs) {
        // One prototype per C name, like globals.h: a name may be defined in more
        // than one configuration branch.  make-docfile refuses two definitions with
        // differing signatures, so the generator refuses them too; the attribute
        // flags are OR'ed (the reference keeps the flags of one record, the port
        // keeps every flag it has seen).
        let mut protos: std::collections::BTreeMap<String, (String, bool, String)> =
            std::collections::BTreeMap::new();
        let mut clashes = 0usize;
        for line in text.lines().skip(1) {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 7 {
                continue;
            }
            // make-docfile prefixes `_Noreturn` when the DEFUN lists the attribute;
            // Rust's spelling of "never returns" is the return type `!`.
            let noreturn = f[6].split_whitespace().any(|w| w == "noreturn");
            let args = match f[4] {
                "MANY" => "nargs: ptrdiff_t, args: *mut Lisp_Object".to_string(),
                "UNEVALLED" => "args: Lisp_Object".to_string(),
                n => match n.parse::<usize>() {
                    // C's DEFUN_ARGS_n has no parameter names; Rust extern blocks
                    // require them, so the port uses the union member names.
                    Ok(k) => (0..k).map(|i| format!("a{i}: Lisp_Object"))
                        .collect::<Vec<_>>().join(", "),
                    Err(_) => continue,
                },
            };
            match protos.get(f[1]) {
                Some((seen, _, seen_max)) => {
                    if *seen != args || *seen_max != f[4] {
                        eprintln!("subr {}: signature {} vs {}", f[1], seen, args);
                        clashes += 1;
                    }
                }
                None => {
                    protos.insert(f[1].to_string(), (args, noreturn, f[4].to_string()));
                }
            }
            if noreturn {
                if let Some(e) = protos.get_mut(f[1]) {
                    e.1 = true;
                }
            }
        }
        rs.push_str(
            "\n// --- EXFUN prototypes for every DEFUN (generated from subrs.tsv) ---\n\
             // C: globals.h writes `EXFUN (Ffoo, maxargs)`; lisp.h includes <globals.h>,\n\
             // so the prototype is visible in every translation unit.\n\
             pub mod subrs {\n    use super::*;\n    unsafe extern \"C\" {\n",
        );
        for (name, (args, noreturn, _)) in &protos {
            // C's globals.h writes `_Noreturn` before the EXFUN; Rust spells that
            // "never returns" as the return type `!` (PORTING.md 5.2).
            let ret = if *noreturn { "!" } else { "Lisp_Object" };
            rs.push_str(&format!("        pub fn {name} ({args}) -> {ret};\n"));
            n_subrs += 1;
        }
        rs.push_str("    }\n}\nuse subrs::*;\n");
        if clashes != 0 {
            eprintln!("{clashes} subr name(s) defined twice with different signatures");
            return 1;
        }
    }

    let out = Path::new("src/include/globals.rs");
    fs::create_dir_all(out.parent().unwrap()).ok();
    if let Err(e) = fs::write(out, rs) {
        return fail(e);
    }
    println!("wrote {} symbols, {} variables and {} subr prototypes to {}", rows.len(), n_vars, n_subrs, out.display());
    0
}

// --- layout assertions -----------------------------------------------------

/// C field/type names that Rust spells with a raw identifier (PORTING.md 0.2).  The
/// assertions are generated Rust, so `Lisp_Buffer_Local_Value.where` has to come out as
/// `r#where` — without this the layout probe does not compile (P-65's follow-up).
fn rust_ident(name: &str) -> String {
    // The 2015 strict keywords plus the 2018/2021/2024 reserved ones (review B's F6): the
    // list has to be drawn from the language, not from the names met so far.  `box`, `gen`,
    // `try` and the rest are reserved and cannot be used bare in a generated field access.
    const KEYWORDS: [&str; 51] = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false",
        "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
        "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
        "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
        "abstract", "become", "box", "do", "final", "gen", "macro", "override", "priv",
        "try", "typeof", "unsized", "virtual",
    ];
    // These four have no raw form (`r#self` is rejected), so a C member spelled that way
    // needs a hand-written assertion rather than a silent mis-generation.
    const NOT_RAW: [&str; 4] = ["self", "Self", "crate", "super"];
    if NOT_RAW.contains(&name) {
        panic!("{name} cannot be a raw identifier in a generated layout assertion");
    }
    if KEYWORDS.contains(&name) {
        format!("r#{name}")
    } else {
        name.to_string()
    }
}

fn rust_type_expr(kind: &str, key: &str) -> Option<String> {
    let path: Vec<&str> = key.split('.').collect();
    let ty = rust_ident(path[0]);
    match kind {
        "sizeof" => Some(format!("size_of::<{ty}>()")),
        "alignof" => Some(format!("align_of::<{ty}>()")),
        "offsetof" => {
            if path.len() < 2 {
                return None;
            }
            let field = path[1..].iter().map(|f| rust_ident(f))
                .collect::<Vec<_>>().join(".");
            Some(format!("offset_of!({ty}, {field})"))
        }
        _ => None,
    }
}

/// File-scope objects the mirrors declare, as name -> declared type text
/// (`pub static mut mpz: [mpz_t; 5];` -> `mpz` -> `[mpz_t; 5]`).  The layout keys of a
/// C *object* are resolved through this map instead of the type list.
fn declared_object_types() -> std::collections::BTreeMap<String, String> {
    let mut map = std::collections::BTreeMap::new();
    let mut stack = vec![PathBuf::from("src/include"), PathBuf::from("src"),
                         PathBuf::from("lib/include"), PathBuf::from("../lib/include")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for e in entries.filter_map(Result::ok) {
            let path = e.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().map(|x| x != "rs").unwrap_or(true) {
                continue;
            }
            let Ok(text) = fs::read_to_string(&path) else { continue };
            let code = strip_comments_and_strings(&text);
            for line in code.lines() {
                let line = line.trim();
                let rest = match line.strip_prefix("pub static ") {
                    Some(r) => r,
                    None => match line.strip_prefix("static ") {
                        Some(r) => r,
                        None => continue,
                    },
                };
                let rest = rest.strip_prefix("mut ").unwrap_or(rest);
                let Some((name, tail)) = rest.split_once(':') else { continue };
                // strip the terminating `;` first: an array type contains one too
                // (`[mpz_t; 5];`), and splitting on it truncated the type.
                let ty = tail.trim_end().trim_end_matches(';').split('=').next()
                    .unwrap_or("").trim();
                if !name.trim().is_empty() && !ty.is_empty() {
                    map.insert(name.trim().to_string(), ty.to_string());
                }
            }
        }
    }
    map
}

/// Type names the port has already declared, so assertions are only emitted for
/// what exists; the rest are reported as pending (a shrinking number).
///
/// The scan is whitespace- and attribute-tolerant and does not anchor at the line
/// start: an earlier line-prefix scan dropped a type -- and with it every one of its
/// assertions -- when the declaration merely had different whitespace or attributes,
/// which turned the layout gate off silently (review B, F1).
fn known_types() -> std::collections::BTreeSet<String> {
    let mut set = std::collections::BTreeSet::new();
    let mut stack = vec![PathBuf::from("src/include"), PathBuf::from("src"),
                         PathBuf::from("lib/include"), PathBuf::from("../lib/include")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for e in entries.filter_map(Result::ok) {
            let path = e.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().map(|x| x != "rs").unwrap_or(true) {
                continue;
            }
            let Ok(text) = fs::read_to_string(&path) else { continue };
            // Comments and string literals are dropped first: `lisp.rs` mentions
            // `struct itree_node` in a comment, and a mention must not count as a
            // declaration (it would keep a renamed type's assertions alive).
            let code = strip_comments_and_strings(&text);
            for kw in ["struct", "union", "enum", "type"] {
                for name in names_after_keyword(&code, kw) {
                    set.insert(name);
                }
            }
        }
    }
    set
}

/// TEXT without comments and without the contents of string/char literals, so a
/// mention inside either cannot look like a declaration.
fn strip_comments_and_strings(text: &str) -> String {
    let (b, mut out) = (text.as_bytes(), Vec::with_capacity(text.len()));
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'/' if i + 1 < b.len() && b[i + 1] == b'/' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if i + 1 < b.len() && b[i + 1] == b'*' => {
                i += 2;
                let mut depth = 1;
                while i < b.len() && depth > 0 {
                    if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'*' {
                        depth += 1;
                        i += 2;
                    } else if b[i] == b'*' && i + 1 < b.len() && b[i + 1] == b'/' {
                        depth -= 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                out.push(b' ');
            }
            b'\'' => {
                // A char literal (`'x'`, '\n'), not a lifetime: only a closing quote
                // two or three bytes away makes it one.
                let mut j = i + 1;
                if j < b.len() && b[j] == b'\\' {
                    j += 2;
                } else {
                    j += 1;
                }
                if j < b.len() && b[j] == b'\'' {
                    i = j + 1;
                    out.push(b' ');
                } else {
                    out.push(b'\'');
                    i += 1;
                }
            }
            b'"' => {
                i += 1;
                while i < b.len() {
                    if b[i] == b'\\' {
                        i += 2;
                    } else if b[i] == b'"' {
                        i += 1;
                        break;
                    } else {
                        i += 1;
                    }
                }
                out.push(b' ');
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Every identifier that follows KEYWORD as a whole word in TEXT (byte-wise, so the
/// scan is safe on non-ASCII comments).
fn names_after_keyword(text: &str, keyword: &str) -> Vec<String> {
    let (bytes, kw) = (text.as_bytes(), keyword.as_bytes());
    let mut out = Vec::new();
    let mut i = 0;
    while i + kw.len() <= bytes.len() {
        if &bytes[i..i + kw.len()] == kw {
            let before_ok = i == 0
                || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
            if before_ok && bytes.get(i + kw.len()).copied().map(|c| c.is_ascii_whitespace()).unwrap_or(false) {
                let mut j = i + kw.len();
                while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                    j += 1;
                }
                let start = j;
                while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                    j += 1;
                }
                if j > start {
                    out.push(text[start..j].to_string());
                }
            }
        }
        i += 1;
    }
    out
}

/// The asserted/pending counts the gate must not fall below: a type that disappears
/// from the scan would drop assertions, and a dropped assertion is invisible in the
/// compiled probe (review B, F1).  Adding mirrors only raises asserted / lowers
/// pending, so this file stays as a floor.
fn check_counts(asserted: usize, pending: usize) -> Result<(), String> {
    let path = Path::new(REPO).join("docs/baseline/layout/counts.tsv");
    // The ratchet must exist and parse: silently returning Ok would make every
    // shrinkage check vacuous (review B's B5).
    let text = fs::read_to_string(&path).map_err(|e| {
        format!("{}: {e} (the layout ratchet must exist; 0.1 records why)", path.display())
    })?;
    let mut want_asserted: Option<usize> = None;
    let mut want_pending: Option<usize> = None;
    for line in text.lines().skip(1) {
        match line.split_once('\t') {
            Some(("asserted", v)) => {
                want_asserted = Some(v.trim().parse().map_err(|e| {
                    format!("{}: asserted: {e}", path.display()) })?)
            }
            Some(("pending", v)) => {
                want_pending = Some(v.trim().parse().map_err(|e| {
                    format!("{}: pending: {e}", path.display()) })?)
            }
            _ => {}
        }
    }
    let (Some(want_asserted), Some(want_pending)) = (want_asserted, want_pending) else {
        return Err(format!("{}: needs both `asserted` and `pending` rows", path.display()));
    };
    if asserted < want_asserted || pending > want_pending {
        return Err(format!(
            "layout gate shrank: {asserted} asserted / {pending} pending, \
             floor is {want_asserted} / {want_pending} ({})",
            path.display()));
    }
    Ok(())
}

/// The *key set* ratchet.  `check_counts` only compares how many keys are asserted, so one
/// assertion could be traded for another: deleting `offsetof Lisp_Buffer_Local_Value.where`
/// and `…defcell` while adding `sizeof Emacs_GC` + `alignof Emacs_GC` kept the count at 247,
/// left the mirror's `where`/`defcell` swapped relative to C, and both gates stayed green
/// (review B's F4).  The recorded list is the set that was asserted when the baseline was
/// last accepted; it may only grow.  Removing a key is a reviewed edit to that file.
fn check_keys(asserted: &[String]) -> Result<(), String> {
    let path = Path::new(REPO).join("docs/baseline/layout/asserted-keys.txt");
    let text = fs::read_to_string(&path).map_err(|e| {
        format!("{}: {e} (the layout key ratchet must exist; PORTING.md P-68 records why)",
                path.display())
    })?;
    let have: std::collections::BTreeSet<&str> = asserted.iter().map(String::as_str).collect();
    let missing: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter(|l| !have.contains(l))
        .collect();
    if !missing.is_empty() {
        return Err(format!(
            "layout keys disappeared ({} of them): {}{} — a key that stops being asserted \
             stops being checked, and swapping one for another keeps the count the same \
             ({}); update {} only with a reason",
            missing.len(),
            missing.iter().take(6).cloned().collect::<Vec<_>>().join(", "),
            if missing.len() > 6 { ", …" } else { "" },
            path.display(),
            path.display()));
    }
    Ok(())
}

fn layout_asserts() -> Result<PathBuf, String> {
    let dir = Path::new(REPO).join("docs/baseline/layout");
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "tsv"))
        .filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy()
                                              .starts_with("c-layout-")))
        .collect();
    files.sort();
    // Exactly one `*.tsv`, and it is named: taking the first entry of the directory meant a
    // stray `README.md` or a second `A-extra.tsv` silently became the baseline, with the
    // real numbers ignored (review B's F5).  The name is logged so the gate's input is
    // visible in its own output.
    let [tsv] = files.as_slice() else {
        return Err(format!("{} needs exactly one c-layout-*.tsv baseline, found {} ({})",
                           dir.display(), files.len(),
                           files.iter().map(|p| p.display().to_string())
                               .collect::<Vec<_>>().join(", ")));
    };
    let tsv = tsv.clone();
    println!("layout baseline: {}", tsv.display());
    let text = fs::read_to_string(&tsv).map_err(|e| e.to_string())?;
    let mut out = String::from(
        "// Generated by `xtask layout-asserts` from the frozen C layout; do not edit.\n\
         // Every line is a number the C compiler printed for the reference headers.\n",
    );
    let known = known_types();
    let objects = declared_object_types();
    if std::env::var("XTASK_DEBUG_TYPES").is_ok() {
        let hits: Vec<&String> = known.iter().filter(|n| n.contains("itree")).collect();
        eprintln!("debug: itree-ish types: {hits:?}");
    }
    let mut n = 0;
    let mut pending: Vec<String> = Vec::new();
    let mut asserted_keys: Vec<String> = Vec::new();
    for line in text.lines().skip(1) {
        let Some((key, value)) = line.split_once('\t') else { continue };
        let mut parts = key.splitn(2, ' ');
        let (kind, rest) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
        let root = rest.split('.').next().unwrap_or("");
        // A key's root is usually a type, but C also has file-scope *objects*
        // (`extern mpz_t mpz[5]`): for those the mirror's own declaration supplies the
        // type (`static mut mpz: [mpz_t; 5]`), which is what makes the bound visible to
        // this gate (review B's B1: `[mpz_t; 6]` passed every gate).
        let rest = match objects.get(root) {
            Some(ty) if !known.contains(root) => ty.as_str(),
            _ => rest,
        };
        if !known.contains(root) && !objects.contains_key(root) {
            pending.push(format!("{key} = {value}"));
            continue;
        }
        let Some(expr) = rust_type_expr(kind, rest) else { continue };
        out.push_str(&format!(
            "const _: () = assert!({expr} == {value}, \"{kind} {rest}\");\n"
        ));
        asserted_keys.push(key.to_string());
        n += 1;
    }
    fs::create_dir_all(BUILD).ok();
    let path = Path::new(BUILD).join("layout_asserts.rs");
    fs::write(&path, out).map_err(|e| e.to_string())?;
    // The pending list is written before the ratchet check, so a failure still says which
    // keys regressed.
    if !pending.is_empty() {
        let list = Path::new(BUILD).join("layout_pending.txt");
        fs::write(&list, pending.join("\n") + "\n").ok();
    }
    asserted_keys.sort();
    let asserted_list = Path::new(BUILD).join("layout_asserted.txt");
    fs::write(&asserted_list, asserted_keys.join("\n") + "\n").ok();
    check_counts(n, pending.len())?;
    check_keys(&asserted_keys)?;
    println!("{n} layout assertions from {}", tsv.display());
    if !pending.is_empty() {
        println!("{} keys pending a mirror: {}", pending.len(),
                 pending.iter().take(8).cloned().collect::<Vec<_>>().join(", "));
    }
    Ok(path)
}

fn layout_check() -> i32 {
    let asserts = match layout_asserts() {
        Ok(p) => p,
        Err(e) => return fail(e),
    };
    fs::create_dir_all(BUILD).ok();
    let probe = Path::new(BUILD).join("layout_probe.rs");
    let mut src = String::new();
    src.push_str("#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]\n");
    src.push_str("use std::mem::{align_of, size_of, offset_of};\n");
    // Every header mirror, so the assertions run for all of them; globals.rs is
    // included by lisp.rs (mirroring lisp.h including <globals.h>).
    let mut mirrors: Vec<PathBuf> = fs::read_dir("src/include")
        .map(|d| d.filter_map(Result::ok).map(|e| e.path()).collect())
        .unwrap_or_default();
    mirrors.retain(|p| p.extension().map(|x| x == "rs").unwrap_or(false));
    // PORTING.md 0.1-4: expand the include! graph and dedupe by file, so a mirror that
    // (like lib/inttypes.in.rs will) includes another mirror is not listed twice.
    let mut included: std::collections::BTreeSet<PathBuf> = std::collections::BTreeSet::new();
    for p in &mirrors {
        collect_includes(p, &mut included);
    }
    mirrors.retain(|p| !included.contains(&canonical(p)));
    // lisp.rs first: it defines the macros (eassert, eassume, CHECK_*) the other
    // mirrors expand textually, exactly as C's headers get them from lisp.h.
    if let Some(i) = mirrors.iter().position(|p| p.file_name().map(|n| n == "lisp.rs").unwrap_or(false)) {
        let lisp = mirrors.remove(i);
        mirrors.insert(0, lisp);
    }
    // A lib mirror that carries macros another mirror expands must come first: the probe
    // compiles them in one crate, and `macro_rules!` are order-sensitive.  bignum.rs uses
    // `mpz_odd_p!` from mini-gmp.rs, exactly as bignum.h uses <gmp.h>.
    // `lib/include`, not `../lib/include`: the port's lib mirrors live inside femacs/
    // (the same mistake P-53 records for the type scan).
    let gmp = PathBuf::from("lib/include/mini-gmp.rs");
    if gmp.is_file() {
        mirrors.insert(1.min(mirrors.len()), gmp);
    }
    // <stdio.h> comes before lisp.h in C (lisp.h 38 includes it), and lisp.rs uses `FILE`
    // in the `emacs_fclose` macro's port, so the mirror has to be present too.
    let stdio = PathBuf::from("lib/include/stdio.rs");
    if stdio.is_file() {
        mirrors.insert(0, stdio);
    }
    mirrors.push(asserts);
    for path in mirrors {
        let full = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        src.push_str(&format!("include!({:?});\n", full.display().to_string()));
    }
    src.push_str("fn main() { println!(\"layout probe ok\"); }\n");
    if let Err(e) = fs::write(&probe, src) {
        return fail(e);
    }
    let dir = Path::new(BUILD).join("layout_probe_src");
    let root = match materialize(&probe, &dir) {
        Ok(p) => p,
        Err(e) => return fail(e),
    };
    let bin = Path::new(BUILD).join("layout_probe");
    let status = Command::new("rustc")
        .args(["--edition", "2024", "--crate-name", "layout_probe", "-C", "panic=abort", "-o"])
        .arg(&bin)
        .arg(&root)
        .status();
    match status {
        Ok(s) if s.success() => {}
        Ok(s) => return fail(format!("layout probe failed to compile ({s})")),
        Err(e) => return fail(e),
    }
    match Command::new(&bin).status() {
        Ok(s) if s.success() => {
            println!("layout-check passed (see build/femacs/layout_asserts.rs)");
            0
        }
        Ok(s) => fail(format!("layout probe exited with {s}")),
        Err(e) => fail(e),
    }
}

/// rustc's `--crate-name` accepts alphanumerics and underscores only, while the port map
/// keeps C's file names verbatim (`src/vm-limit.c` -> `femacs/src/vm-limit.rs`), so hyphens
/// fold when the stem is used as a crate name.  Paths keep C's spelling.
fn crate_name (stem: &str) -> String {
    stem.chars ()
        .map (|c| if c.is_ascii_alphanumeric () || c == '_' { c } else { '_' })
        .collect ()
}

// --- translation units -----------------------------------------------------

/// `xtask probe <rs-file> <out> [<extra rustc args>…]`: build one spike probe.
///
/// The shell probes under `spikes/` reach their mirrors through their own `include!` lists,
/// which have no include guard, so a file reached by two paths (`lisp.rs` ->
/// `thread.rs` -> `sys_select.rs` -> `time.in.rs` and `buffer.rs` -> `time.in.rs`) was
/// defined twice and `rustc` stopped with E0428.  They are materialized exactly as a unit is
/// (PORTING.md 0.1-4), and the extras after the output path are passed to `rustc` unchanged
/// (`macro-probe.sh` needs `--crate-type lib --emit metadata`).
fn probe_build(args: &[String]) -> i32 {
    let (Some(src), Some(out)) = (args.first(), args.get(1)) else {
        return fail("probe needs <rs-file> <out> [<extra rustc args>…]");
    };
    let src = Path::new(src);
    let stem = src.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let dir = Path::new("../build/probes").join(&stem);
    let root = match materialize(src, &dir) {
        Ok(root) => root,
        Err(e) => return fail(format!("probe {}: {e}", src.display())),
    };
    let status = Command::new("rustc")
        .args(["--edition", "2024", "-O"])
        .arg(&root)
        .arg("-o")
        .arg(out)
        .args(&args[2..])
        .status();
    match status {
        Ok(s) if s.success() => 0,
        Ok(s) => fail(format!("rustc failed for {} (exit status: {s})", src.display())),
        Err(e) => fail(format!("probe {}: {e}", src.display())),
    }
}

fn build_tu(rel: Option<&String>) -> i32 {
    let Some(rel) = rel else { return fail("build-tu needs a reference C file, e.g. src/doprnt.c") };
    let stem = Path::new(rel).file_stem().unwrap_or_default().to_string_lossy().to_string();
    let cname = crate_name (&stem);
    let (crate_root, out) = if rel.ends_with(".h") {
        // The header mirror lives next to its unit's directory too: lib/regex.h is
        // femacs/lib/include/regex.rs, not src/include/regex.rs.
        let unit = UNIT_DIRS
            .iter()
            .map(|dir| format!("{dir}/include/{stem}.rs"))
            .find(|cand| Path::new(cand).is_file())
            .unwrap_or_else(|| format!("src/include/{stem}.rs"));
        (unit, Path::new(BUILD).join(format!("include_{stem}.o")))
    } else {
        // A unit lives in one of UNIT_DIRS; `finished_units` looks the same list up, so
        // `build-tu lib/getline.c` and `build-tu lib-src/etags.c` both work.
        let unit = UNIT_DIRS
            .iter()
            .map(|dir| format!("{dir}/{stem}.rs"))
            .find(|cand| Path::new(cand).is_file())
            .unwrap_or_else(|| format!("src/{stem}.rs"));
        (unit, Path::new(BUILD).join(format!("{stem}.o")))
    };
    if !Path::new(&crate_root).is_file() {
        return fail(format!("{crate_root} does not exist yet (ports of {rel} not written)"));
    }
    fs::create_dir_all(BUILD).ok();
    let dir = Path::new(BUILD).join("units").join(&stem);
    let root = match materialize(Path::new(&crate_root), &dir) {
        Ok(p) => p,
        Err(e) => return fail(e),
    };
    let status = Command::new("rustc")
        .args(["--edition", "2024", "--crate-type", "lib", "--emit", "obj",
               "--crate-name", &cname,
               "-C", "panic=abort", "-C", "codegen-units=1", "-C", "opt-level=0"])
        .arg(&root)
        .arg("-o").arg(&out)
        .status();
    match status {
        Ok(s) if s.success() => {
            println!("built {crate_root} -> {}", out.display());
            0
        }
        Ok(s) => fail(format!("rustc failed for {crate_root} ({s})")),
        Err(e) => fail(e),
    }
}

// --- gates -----------------------------------------------------------------

fn gate_fast() -> i32 {
    // The inventory gate below reads `#if` conditions through the frozen macro table, so the
    // table itself is gated first: every file allowed to consult it must still agree with
    // clang on every branch it decides (PORTING.md P-66).  Without this step the table's
    // accuracy would be a claim in a help string instead of a measurement.
    println!("== condition evaluator ==");
    if !Path::new("../build/emacs-ref/src/config.h").is_file() {
        return fail("build/emacs-ref is missing: the `#if` differential needs the frozen \
configuration (see PORTING.md 2.1's setup steps)");
    }
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-cond-eval.py", "--verified",
               "docs/baseline/config/cond-eval-verified.txt"])
        .status()
    {
        Ok(s) if s.success() => println!("condition evaluator: verified list agrees with clang"),
        Ok(_) => return fail("the `#if` evaluator disagrees with clang on a verified header"),
        Err(e) => return fail(e),
    }
    println!("== inventory gate ==");
    let inv = Command::new("python3")
        .current_dir(REPO)
        .args(["tools/extract-abi.py", "--all", "--reference", "emacs", "--root", "femacs"])
        .status();
    match inv {
        Ok(s) if s.success() => println!("inventory: no differences"),
        Ok(_) => return fail("inventory differences (nothing is ported yet, so any entry is a bug)"),
        Err(e) => return fail(e),
    }
    // `DEFVAR_*`'s storage is not a C object definition, so the inventory gate above cannot
    // see it: a unit that owns a variable (C's `globals.f_<vname>`) may reference it through
    // `globals.rs`'s declaration and never define it, and only the eventual link would
    // notice.  The names are checked mechanically (status §3.29).
    println!("== globals gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-globals.py"])
        .status()
    {
        Ok(s) if s.success() => println!("globals: every DEFVAR_* variable has a definition"),
        Ok(_) => return fail("a DEFVAR_* variable has no definition in its own unit"),
        Err(e) => return fail(e),
    }
    println!("== workspace check ==");
    match Command::new("cargo")
        .env("CARGO_TARGET_DIR", "../build/cargo")
        .args(["check", "--quiet", "--manifest-path", "Cargo.toml"])
        .status()
    {
        Ok(s) if s.success() => println!("cargo check: ok"),
        Ok(s) => return fail(format!("cargo check failed ({s})")),
        Err(e) => return fail(e),
    }
    println!("== translation units ==");
    if let Err(e) = build_finished_tus() {
        return fail(e);
    }
    // A unit's C-visible symbol table is the one thing the textual gates cannot see: gnulib's
    // `#define foo rpl_foo` headers decide whether the *definition* of a replacement is named
    // `foo` or `rpl_foo` (and a file's own `# undef` decides which side its body's calls
    // reach), and a "header-bodied" unit exports the header's functions while the port's
    // mirror may inline them silently.  `docs/baseline/symbol-pending.txt` is the ratchet of
    // units still open (status §3.30).
    println!("== symbol gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-symbols.py", "--pending", "docs/baseline/symbol-pending.txt"])
        .status()
    {
        Ok(s) if s.success() => println!("symbols: every built unit matches its reference"),
        Ok(_) => return fail("a unit's symbol table differs from its reference object"),
        Err(e) => return fail(e),
    }
    // The gate above compares the symbols a unit *exports*; nothing compared the symbols a
    // unit *references*, and a wrong name there stays invisible until the link stage of the
    // loading ladder (`emacs_fclose`, status 3.57, is the worked example: textually faithful,
    // right exports, cannot link).  This one measures every `nm -u` entry against the frozen
    // reference objects and the SDK's libSystem export list, with
    // `docs/baseline/undef-pending.txt` as the ratchet.
    println!("== undefined-reference gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-undef.py"])
        .status()
    {
        Ok(s) if s.success() => println!("undefined references: nothing new is unresolved"),
        Ok(_) => return fail("a unit references a symbol no built object or library provides"),
        Err(e) => return fail(e),
    }
    // check-symbols.py above compares export tables for the units whose reference object
    // exists - all of them in `lib/`.  The same comparison for `src/` units runs here too,
    // and it is not vacuous any more: `build/emacs-ref/src/<stem>.o` can be built one unit at
    // a time (`make -C build/emacs-ref/src <stem>.o`), which is how the DEFVAR-carrier rule of
    // status 3.60 was found.
    println!("== symbol gate (src) ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-symbols.py", "--dir", "src", "--pending",
               "docs/baseline/symbol-pending.txt"])
        .status()
    {
        Ok(s) if s.success() => println!("symbols (src): every built unit matches its reference"),
        Ok(_) => return fail("a src unit's symbol table differs from its reference object"),
        Err(e) => return fail(e),
    }
    // The other direction of the same question: a C file-scope object that C does *not* mark
    // static must be a symbol in the port's object.  `re_match_object` was written without
    // `#[unsafe(no_mangle)]`, so the unit emitted a mangled name where C emits
    // `_re_match_object` - and no gate saw it, because the text rule compares names and only
    // units with a reference object had their table compared (status 3.61).
    println!("== linkage gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-linkage.py"])
        .status()
    {
        Ok(s) if s.success() => println!("linkage: every non-static C object is exported"),
        Ok(_) => return fail("a C file-scope object has no symbol in the port's object"),
        Err(e) => return fail(e),
    }
    // The four classes of declaration 0.1-19 keeps out of the tree - the copied
    // ownership block, the product name in a comment, the removed
    // `@ <rev> (<product> x.y.z)` parenthetical, and the comment that recites any of
    // the three instead of carrying one (status 3.100) - are prose, and prose comes
    // back: every unit ported after `fd9cf89` copies its C file's comments, and
    // `src/emacs.c`'s own literals are the worked example of the class that *stays*.
    // This gate reads the tree the way the rule does (comments and string literals
    // separated, `'a` lifetimes versus `'a'` chars) and compares with
    // `docs/baseline/declarations-allowlist.tsv`, so the literals that must stay are
    // counted with a reason rather than being invisible to every other gate (3.73).
    println!("== declaration gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-declarations.py"])
        .status()
    {
        Ok(s) if s.success() => {
            println!("declarations: no product-name declaration in the port")
        }
        Ok(_) => {
            return fail("a ported file carries a product-name declaration (PORTING.md 0.1-19)")
        }
        Err(e) => return fail(e),
    }
    // A unit that repeats a declaration a mirror *in its own include graph* already carries
    // compiles, passes every text gate, and becomes `clashing_extern_declarations` the moment
    // the two spellings disagree - the warning gate-fast's unit builds already print.  The
    // ratchet holds the rows that are deliberate (status 3.76/3.77).
    println!("== mirror-dupes gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-mirror-dupes.py"])
        .status()
    {
        Ok(s) if s.success() => println!("mirror duplicates: none beyond the ratchet"),
        Ok(_) => {
            return fail("a unit repeats a declaration a mirror in its include graph carries")
        }
        Err(e) => return fail(e),
    }
    println!("== probe programs ==");
    if let Err(e) = probe_programs() {
        return fail(e);
    }
    println!("== layout gate ==");
    for probe in ["spikes/fam-probe.sh", "spikes/shadow-probe.sh", "spikes/macro-probe.sh"] {
        println!("== {probe} ==");
        let status = Command::new("bash").arg(probe).current_dir(REPO).status();
        match status {
            Ok(s) if s.success() => {}
            Ok(s) => return fail(format!("{probe} failed ({s})")),
            Err(e) => return fail(format!("{probe}: {e}")),
        }
    }
    layout_check()
}

/// Compile every probe *program* the way its script does — from `femacs/`, so
/// `rust-toolchain.toml` decides the toolchain — without running the C side.
///
/// `gate-fast` was blind to the probes, and `ec20c48` broke three of them: the scripts
/// ran a bare `rustc` from the repository root, which has no `rust-toolchain.toml`, so
/// the *stable* toolchain resolved and the mirrors' `VaList` parameter became E0658.
/// The layout gate stayed green (review D's F1).  A probe that cannot even compile is
/// not evidence, so this is part of the fast gate now; the C-differential half is
/// `gate-probes`.
fn probe_programs() -> Result<(), String> {
    const PROBES: [&str; 7] = ["../spikes/inline_probe.rs", "../spikes/accessor_probe.rs",
                               "../spikes/write_probe.rs", "../spikes/va_probe.rs",
                               "../spikes/layout.rs", "../spikes/jump.rs", "../spikes/scan.rs"];
    fs::create_dir_all("../build/probes").ok();
    for probe in PROBES {
        if !Path::new(probe).is_file() {
            continue;
        }
        let stem = Path::new(probe).file_stem().unwrap().to_string_lossy().to_string();
        let out = format!("../build/probes/{stem}.rmeta");
        // The probe reaches its mirrors through its own `include!` list, so it needs the same
        // de-duplication the units get: the graph now has a file reached by two paths
        // (`lisp.rs` -> `thread.rs` -> `sys_select.rs` -> `time.in.rs` and `buffer.rs` ->
        // `time.in.rs`), and `include!` has no include guard (PORTING.md 0.1-4).  Materialize
        // it exactly as a unit is materialized, then compile that root.
        let root = match materialize(Path::new(probe),
                                     Path::new("../build/probes").join(&stem).as_path()) {
            Ok(root) => root,
            Err(e) => return Err(format!("probe {probe}: {e}")),
        };
        let status = Command::new("rustc")
            .args(["--edition", "2024", "--crate-type", "bin", "--emit", "metadata"])
            .arg(&root).arg("-o").arg(&out)
            .status();
        match status {
            Ok(s) if s.success() => println!("probe {stem}: the Rust side compiles"),
            Ok(s) => return Err(format!("probe {probe} does not compile ({s})")),
            Err(e) => return Err(format!("probe {probe}: {e}")),
        }
    }
    Ok(())
}

/// Compile every finished translation unit with the **non-frozen** arms selected.
///
/// `config.rs` is generated from the frozen configuration, so the default build only
/// ever compiles the frozen branches; C's dead `#else`/`#ifdef` text would otherwise be
/// unchecked text (review B's B3).  This command copies `femacs/` into
/// `build/dead-arms/`, flips the constants the features stand for, and compiles each
/// finished unit with those features on.  It is part of `gate-probes` (batch boundaries),
/// not of `gate-fast`, because it is a second compile of every unit.
fn dead_arms() -> i32 {
    // feature -> (config key, value with the feature on)
    const FLIPS: [(&str, &str, &str); 6] = [
        ("no_rint", "HAVE_RINT", "false"),
        ("no_trunc", "HAVE_TRUNC", "false"),
        ("no_signbit", "HAVE_SIGNBIT", "false"),
        ("haiku", "HAIKU", "true"),
        // `native_comp` was the one structural arm *nothing* compiled: `cfg!(feature =
        // "native_comp")` is false in every real build (no `[features]`, and this list did
        // not name it), so `lisp.rs`'s `#[cfg(feature = "native_comp")]` pair — the
        // `NATIVE_COMP_FUNCTIONP`/`DYNP` bodies behind `HAVE_NATIVE_COMP` — had never been
        // through rustc, and the const assertion that claims the feature and the
        // configuration agree could not fail (review A's F6).  Flipping the configuration
        // constant *and* the feature together is what makes that claim executable.
        ("native_comp", "HAVE_NATIVE_COMP", "true"),
        // HAVE_LOG2 is not gated on a feature (the condition keeps the constant), but
        // the other arm is still C text that has to compile.
        ("", "HAVE_LOG2", "false"),
    ];
    let root = Path::new("../build/dead-arms");
    let _ = fs::remove_dir_all(root);
    for dir in UNIT_DIRS {
        if !Path::new(dir).is_dir() {
            continue; // a directory the port has not opened yet
        }
        if let Err(e) = copy_tree(Path::new(dir), &root.join(dir)) {
            return fail(format!("copying {dir}: {e}"));
        }
    }
    let config = root.join("src/include/config.rs");
    let Ok(text) = fs::read_to_string(&config) else {
        return fail(format!("{} is missing (run `xtask gen-config` first)", config.display()));
    };
    let mut flipped = text;
    for (_, key, value) in FLIPS {
        let mut touched = false;
        let mut out = String::new();
        for line in flipped.lines() {
            let is_key = line.contains(&format!("pub const {key}:"));
            if is_key {
                let ty = if value == "false" || value == "true" { "bool" } else { "i64" };
                out.push_str(&format!("pub const {key}: {ty} = {value};\n"));
                touched = true;
            } else {
                out.push_str(line);
                out.push('\n');
            }
        }
        if !touched {
            return fail(format!("config.rs has no {key} to flip"));
        }
        flipped = out;
    }
    if let Err(e) = fs::write(&config, flipped) {
        return fail(e);
    }
    let units = match finished_units() {
        Ok(u) => u,
        Err(e) => return fail(e),
    };
    let mut n = 0;
    for rel in units {
        // The unit goes through the same include-graph expansion as a normal build
        // (PORTING.md 0.1-4), inside the copy, so the flipped configuration and the
        // de-duplicated mirrors are what rustc sees.
        let stem = Path::new(&rel).file_stem().unwrap_or_default().to_string_lossy().to_string();
        let cname = crate_name (&stem);
        let src = root.join(&rel);
        let dir = root.join("units").join(&stem);
        let materialized = match materialize(&src, &dir) {
            Ok(p) => p,
            Err(e) => return fail(e),
        };
        let out = format!("{}.o", rel.replace('/', "_").replace(".rs", ""));
        let mut cmd = Command::new("rustc");
        cmd.args(["--edition", "2024", "--crate-type", "lib", "--emit", "obj",
                  "--crate-name", &cname,
                  "-C", "panic=abort", "-C", "codegen-units=1", "-C", "opt-level=0"]);
        for (feature, _, _) in FLIPS {
            if !feature.is_empty() {
                cmd.arg("--cfg").arg(format!("feature=\"{feature}\""));
            }
        }
        let status = cmd.current_dir(root).arg(&materialized).arg("-o").arg(&out).status();
        match status {
            Ok(s) if s.success() => {
                println!("dead arms compile: {rel}");
                n += 1;
            }
            Ok(s) => return fail(format!("{rel} does not compile with the dead arms ({s})")),
            Err(e) => return fail(format!("{rel}: {e}")),
        }
    }
    println!("{n} translation unit(s) compiled with the non-frozen arms");
    0
}

fn copy_tree(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

/// The C-differential tier: every probe that compares the C reference with the mirrors.
/// It needs the frozen configure output, which is not in version control, so it is a
/// command of its own — `gate-fast` must stay runnable in a clean checkout — but it is
/// the tier that produced P-30's evidence and is run at every batch boundary.
fn gate_probes() -> i32 {
    let cfg = std::env::var("CFG").unwrap_or_else(|_| "build/emacs-ref".to_string());
    let header = Path::new("../").join(&cfg).join("src/config.h");
    if !header.is_file() {
        return fail(format!(
            "{} is missing: configure the reference tree out of tree first \
             (docs/spikes/2026-10-02-three-spikes.md, Reproduction)", header.display()));
    }
    for probe in ["spikes/inline-probe.sh", "spikes/accessor-probe.sh",
                  "spikes/write-probe.sh", "spikes/va-probe.sh",
                  "spikes/syshdrs-probe.sh", "spikes/gmp-abi-probe.sh",
                  "spikes/lisp-abi-probe.sh"] {
        println!("== {probe} ==");
        let status = Command::new("bash").arg(probe).current_dir(REPO).status();
        match status {
            Ok(s) if s.success() => {}
            Ok(s) => return fail(format!("{probe} failed ({s})")),
            Err(e) => return fail(format!("{probe}: {e}")),
        }
    }
    println!("== dead arms ==");
    let code = dead_arms();
    if code != 0 {
        return code;
    }
    0
}

fn canonical(p: &Path) -> PathBuf {
    p.canonicalize().unwrap_or_else(|_| p.to_path_buf())
}


/// The `include!("…")` target of a line, if the line is such a call.
fn include_target(line: &str) -> Option<&str> {
    let pos = line.find("include!(")?;
    let rest = &line[pos + "include!(".len()..];
    let end = rest.find(')')?;
    rest[..end].trim().strip_prefix('"').and_then(|s| s.strip_suffix('"'))
}

/// Materialize ROOT's include graph under OUT and return the generated crate root.
///
/// PORTING.md 0.1-4: the tool expands the graph and de-duplicates it, because `include!` has no
/// include guard.  A file reached by two paths (`buffer.rs`'s `<time.h>` and `systime.rs`'s
/// `<timespec.h>` both reach `time.in.rs`) would otherwise define its items twice and rustc
/// would reject the unit.  Every reachable file is therefore written out once, at the position
/// of the `include!` that first reaches it in C's textual order - so a mirror included by
/// `lisp.h`'s mirror is expanded *inside* it and sees the macros `lisp.h` defines above its
/// `#include` line (`eassert!`/`XUNTAG!`), which is what C does and what the old
/// "dependencies first" flattening could not express (see docs/ports/include-thread.md's
/// "Changes outside the unit").  A later reach of an already-expanded file becomes a
/// dedupe comment, i.e. the include guard.  Line numbers inside a copy still point into the
/// real mirror, so diagnostics name a file and line a reader can look at.
fn materialize(root: &Path, out: &Path) -> Result<PathBuf, String> {
    let _ = fs::remove_dir_all(out);
    fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    // Absolute from here on: a copy names another copy with `include!`, which resolves
    // relative to the file holding the call.
    let out = canonical(out);
    let root = canonical(root);
    let mut expand = Expand { out: out.clone(), root: root.clone(), seen: vec![root.clone()] };
    let text = expand.render(&root)?;
    let root_file = out.join("_root.rs");
    fs::write(&root_file, text).map_err(|e| format!("{}: {e}", root_file.display()))?;
    // Absolute: the callers that run rustc with a different working directory (dead-arms)
    // hand it straight to `rustc`.
    Ok(canonical(&root_file))
}

/// The in-place expansion state: the file the walk started from (whose text is the crate
/// root) and the files already expanded, in the order the preprocessor would see them.
struct Expand {
    out: PathBuf,
    root: PathBuf,
    seen: Vec<PathBuf>,
}

impl Expand {
    /// The copy that carries PATH's own text.  Derived from the absolute path alone, so a
    /// file may name a copy that is written later in the walk.
    fn copy_path(&self, path: &Path) -> PathBuf {
        let name = path.to_string_lossy().replace(['/', '\\', ':', '.'], "_");
        self.out.join(format!("{name}.rs"))
    }

    /// PATH's text with every `include!` line resolved the way C's `#include` would be: the
    /// first file the walk reaches at that point is expanded recursively, later reaches are
    /// include-guard no-ops.  ROOT's text is returned (it becomes `_root.rs`); every other
    /// file is written to its copy first.
    fn render(&mut self, path: &Path) -> Result<String, String> {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let mut out = String::with_capacity(text.len() + 64);
        for line in text.split_inclusive('\n') {
            let (body, newline) = match line.strip_suffix('\n') {
                Some(b) => (b, "\n"),
                None => (line, ""),
            };
            match include_target(line).map(|name| canonical(&dir.join(name))) {
                Some(target) if target.is_file() => {
                    if self.seen.contains(&target) {
                        out.push_str(&format!("// [xtask dedupe] {} already included{newline}",
                                              target.display()));
                    } else {
                        self.seen.push(target.clone());
                        let copy = self.copy_path(&target);
                        let body_text = self.render(&target)?;
                        fs::write(&copy, body_text)
                            .map_err(|e| format!("{}: {e}", copy.display()))?;
                        out.push_str(&format!("include!({:?});{newline}",
                                              copy.display().to_string()));
                    }
                }
                // An `include!` whose target does not exist relative to the file that writes it:
                // C's mirrors reach `../src/include/config.rs` from a directory where that path
                // does not resolve, and the file it names is included by the *unit* instead.  In
                // a copy the line must be commented out, because the copy lives elsewhere and
                // the relative path would point at a file in the build tree (the old
                // `neutralize` step did this for every copy); the *root*'s own lines are kept
                // verbatim, so a typo in a unit still fails the compile with rustc's message.
                _ => {
                    if path != self.root && include_target(line).is_some() {
                        out.push_str(&format!("// [xtask dedupe] {body}{newline}"));
                    } else {
                        out.push_str(&format!("{body}{newline}"));
                    }
                }
            }
        }
        Ok(out)
    }
}

/// Every file the `include!("…")` chain of ROOT reaches, resolved the way rustc does
/// (relative to the file that contains the macro call) and added to SEEN.
fn collect_includes(root: &Path, seen: &mut std::collections::BTreeSet<PathBuf>) {
    let Ok(text) = fs::read_to_string(root) else { return };
    let dir = root.parent().unwrap_or(Path::new("."));
    let mut rest = text.as_str();
    while let Some(pos) = rest.find("include!(") {
        rest = &rest[pos + "include!(".len()..];
        let Some(end) = rest.find(")") else { break };
        let quoted = rest[..end].trim();
        if let Some(name) = quoted.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            let inc = canonical(&dir.join(name));
            if seen.insert(inc.clone()) && inc.is_file() {
                collect_includes(&inc, seen);
            }
        }
        rest = &rest[end..];
    }
}

/// Compile every translation unit whose ledger says it is finished.  The textual
/// gates cannot see a unit that rustc rejects (that is how the noreturn record
/// defect in the first DEFUN machinery passed them), so the fast gate compiles
/// what it certifies.  A unit is finished when its checklist has no unchecked
/// boxes (`docs/ports/README.md`); headers have `include-<stem>.md` ledgers and
/// are compiled as part of the units that include them.
/// The units whose ledger says they are finished (`docs/ports/README.md`): a checklist
/// with no unchecked box, and a `src/NAME.rs` to compile.
fn finished_units() -> Result<Vec<String>, String> {
    let ports = Path::new(REPO).join("docs/ports");
    let mut entries: Vec<PathBuf> = fs::read_dir(&ports)
        .map_err(|e| format!("{}: {e}", ports.display()))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    entries.sort();
    let mut units = Vec::new();
    for ledger in entries {
        if ledger.extension().map(|x| x != "md").unwrap_or(true) {
            continue;
        }
        let stem = ledger.file_stem().unwrap_or_default().to_string_lossy().to_string();
        if stem.starts_with("include-") || stem == "README" {
            continue;
        }
        // A unit lives in one of UNIT_DIRS; the ledger's stem is the C file's, and the port
        // map is what pairs them.
        let unit = UNIT_DIRS
            .iter()
            .map(|dir| format!("{dir}/{stem}.rs"))
            .find(|cand| Path::new(cand).is_file());
        let Some(unit) = unit else {
            continue;                                  // header mirror, not a unit
        };
        let checklist = fs::read_to_string(&ledger).map_err(|e| e.to_string())?;
        if checklist.contains("- [ ]") {
            continue;                                  // still being ported
        }
        units.push(unit);
    }
    Ok(units)
}

fn build_finished_tus() -> Result<(), String> {
    let units = finished_units()?;
    let mut built = 0usize;
    fs::create_dir_all(BUILD).ok();
    for unit in &units {
        let stem = Path::new(unit).file_stem().unwrap_or_default().to_string_lossy().to_string();
        let cname = crate_name (&stem);
        let out = Path::new(BUILD).join(format!("{stem}.o"));
        let dir = Path::new(BUILD).join("units").join(&stem);
        let root = match materialize(Path::new(unit), &dir) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        let status = Command::new("rustc")
            .args(["--edition", "2024", "--crate-type", "lib", "--emit", "obj",
                   "--crate-name", &cname,
                   "-C", "panic=abort", "-C", "codegen-units=1", "-C", "opt-level=0"])
            .arg(&root)
            .arg("-o")
            .arg(&out)
            .status();
        match status {
            Ok(s) if s.success() => { println!("built {unit}"); built += 1; }
            Ok(s) => return Err(format!("rustc failed for {unit} ({s})")),
            Err(e) => return Err(e.to_string()),
        }
        // The reference is built with --enable-checking by default, and the port's
        // `eassert!`/`eassume!` are `#[cfg (enable_checking)]`: without this pass their
        // bodies (die, stringify!, the conditions) are never compiled at all, so a
        // checking build is the only way to see them (review C's F4).
        let checked = Path::new(BUILD).join(format!("{stem}.checking.o"));
        let status = Command::new("rustc")
            .args(["--edition", "2024", "--crate-type", "lib", "--emit", "obj",
                   "--crate-name", &cname,
                   "--cfg", "enable_checking",
                   "-C", "panic=abort", "-C", "codegen-units=1", "-C", "opt-level=0"])
            .arg(&root)
            .arg("-o")
            .arg(&checked)
            .status();
        match status {
            Ok(s) if s.success() => { println!("built {unit} (--cfg enable_checking)"); built += 1; }
            Ok(s) => return Err(format!("rustc failed for {unit} with --cfg enable_checking ({s})")),
            Err(e) => return Err(e.to_string()),
        }
    }
    println!("{built} finished translation unit(s) compiled");
    Ok(())
}
