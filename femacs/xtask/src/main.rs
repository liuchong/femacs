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
        "gate-load" => gate_load(args.get(1).map(String::as_str)),
        "gate-full" => {
            eprintln!(
                "gate-full needs the reference configured, built and its ERT suite \
                 runnable; it is not available until the port reaches the loading ladder \
                 (docs/port-plan.md section 7). Nothing was checked."
            );
            2
        }
        _ => {
            println!(
                "usage: xtask <gen-config|layout-asserts|layout-check|build-tu <c-file>|\
                 gate-fast|gate-probes|dead-arms|gate-load [--report]|gate-full>"
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
            // `::core::assert!`, not `assert!`: the probe includes every mirror in one crate, and
            // a mirror that defines its own `assert` macro shadows the prelude's - android-asset.h
            // does, and the generated message form then failed to expand ("no rules expected `,`",
            // status 3.153).  The path keeps the generated assertions immune to that.
            "const _: () = ::core::assert! ({expr} == {value}, \"{kind} {rest}\");\n"
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

    // Platform-exclusive mirrors are skipped here and compiled by their own platform probes: the
    // frozen target never compiles their headers, and their X-compatibility names collide with
    // each other and with dispextern.rs by construction (status 3.147 - 36 E0428s across
    // androidgui.rs/haikugui.rs/androidterm.rs).  The list is data
    // (docs/baseline/layout-excluded-mirrors.txt), one file name per line, and the skip count is
    // printed so nothing is excluded silently (P-66).  It has to happen *before* the include-graph
    // dedupe below: `character.rs` is included only by androidterm.rs and haikuterm.rs, so
    // dropping those two first is what keeps `character.rs` in the probe (its `MAX_CHAR` is what
    // `dispextern.rs` names).
    // A reason may wrap onto continuation lines that start with whitespace, and one of those
    // lines' first word happens to be `dispextern.rs` (it is *named* in `nsgui.rs`'s reason) -
    // taking every line's first token excluded a mirror that is not on the list and silently
    // stopped its assertions from running (measured: the skip list grew by `mirs`, `shadowed`,
    // `compile`, `live`, `` `error[E0428]: ` `` and `dispextern.rs`).  An entry is a line that
    // starts in column 1 and whose first token is a mirror name, so both filters are required.
    let excluded: Vec<String> = fs::read_to_string("../docs/baseline/layout-excluded-mirrors.txt")
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.starts_with(char::is_whitespace))
        .map(|l| l.split('#').next().unwrap_or("").trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().next().unwrap_or("").to_string())
        .filter(|l| l.ends_with(".rs"))
        .collect();
    let total = mirrors.len();
    let skipped: Vec<String> = mirrors.iter()
        .filter(|p| p.file_name().map(|n| excluded.iter().any(|e| e == &n.to_string_lossy()))
                    .unwrap_or(false))
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .collect();
    let skipped = { let mut v = skipped; v.sort(); v };
    mirrors.retain(|p| !p.file_name()
        .map(|n| excluded.iter().any(|e| e == &n.to_string_lossy())).unwrap_or(false));
    // The lib/include mirrors that compile as their own crate root are seeded too, so the shared
    // gate really compiles them (status 3.11, closed 3.217).  A mirror is a header: most of
    // lib/include legitimately needs the including unit's scalars, so the seed is the measured
    // class in docs/baseline/layout-standalone-mirrors.txt (one file name per line) and the
    // complement is printed, not hidden.
    let standalone: Vec<String> =
        fs::read_to_string("../docs/baseline/layout-standalone-mirrors.txt")
            .unwrap_or_default()
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .map(|l| l.split_whitespace().next().unwrap_or("").to_string())
            .filter(|l| l.ends_with(".rs"))
            .collect();
    let mut lib_mirrors: Vec<PathBuf> = fs::read_dir("lib/include")
        .map(|d| d.filter_map(Result::ok).map(|e| e.path()).collect())
        .unwrap_or_default();
    lib_mirrors.retain(|p| p.extension().map(|x| x == "rs").unwrap_or(false));
    let lib_total = lib_mirrors.len();
    lib_mirrors.retain(|p| p.file_name()
        .map(|n| standalone.iter().any(|e| e == &n.to_string_lossy())).unwrap_or(false));
    let lib_seeded: Vec<String> = lib_mirrors.iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string())).collect();
    if lib_total > 0 {
        println!("layout-check: + {} of {} lib/include mirror(s) seeded as their own crate root \
({} need an includer, PORTING.md 0.1-9): {}",
                 lib_seeded.len(), lib_total, lib_total - lib_seeded.len(), lib_seeded.join(", "));
    }
    mirrors.extend(lib_mirrors);

    println!("layout-check: {} of {} src mirror(s) compiled, {} platform-exclusive skipped ({})",
             total - skipped.len(), total, skipped.len(), skipped.join(", "));
    // PORTING.md 0.1-4: expand the include! graph and dedupe by file, so a mirror that
    // (like lib/inttypes.in.rs will) includes another mirror is not listed twice.
    let mut included: std::collections::BTreeSet<PathBuf> = std::collections::BTreeSet::new();
    for p in &mirrors {
        collect_includes(p, &mut included);
    }
    mirrors.retain(|p| !included.contains(&canonical(p)));
    // The foundation mirrors, in C's header order, ahead of everything else.  Two reasons:
    // `macro_rules!` are textually scoped, so the mirror that defines `eassert!`/`eassume!`
    // (lisp.rs) has to be expanded before any mirror that uses them; and the mirrors C reaches
    // through the translation unit rather than through their own `#include` (PORTING.md 0.1-9 -
    // lisp.rs is the documented case, and character.h/systime.h/stdio.h are the same in
    // practice) name definitions that live in these files.  `Expand.seen` dedupes a file included
    // twice, so forcing them to the front cannot duplicate items; being first is what counts even
    // when the list-dedupe above already dropped them, because only the *first* occurrence's
    // position matters (status 3.147: character.rs was reachable only through
    // androidterm.rs/haikuterm.rs - both platform-excluded now - and lisp.rs was dropped by the
    // dedupe, so `dispextern.rs`'s `MAX_CHAR` and `character.rs`'s own `eassert!` were unresolved).
    // `lib/include` rather than `../lib/include`: the port's lib mirrors live inside femacs/
    // (the same mistake P-53 records for the type scan).
    let mut foundation: Vec<PathBuf> = vec![PathBuf::from("lib/include/stdio.rs"),
                                            PathBuf::from("lib/include/mini-gmp.rs"),
                                            PathBuf::from("src/include/lisp.rs"),
                                            PathBuf::from("src/include/character.rs"),
                                            PathBuf::from("src/include/systime.rs")];
    foundation.retain(|p| p.is_file());
    foundation.reverse();
    for f in foundation {
        if let Some(i) = mirrors.iter().position(|p| canonical(p) == canonical(&f)) {
            mirrors.remove(i);
        }
        mirrors.insert(0, f);
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

/// The port of one reference file, by PORTING.md 0.1's layout: a unit keeps the reference's
/// own directory (`lib/malloc/scratch_buffer_grow.c` -> `lib/malloc/scratch_buffer_grow.rs`)
/// and a header gets that directory's `include/` (`lib/regex.h` -> `lib/include/regex.rs`,
/// `lib/malloc/scratch_buffer.h` -> `lib/malloc/include/scratch_buffer.rs`).
///
/// The stem search over `UNIT_DIRS` is the fallback, for the directories the port flattens
/// (`nt/inc/*.c` -> `src/`) and the older flat layout.  It used to be the only rule, and it
/// answered `lib/malloc/scratch_buffer_grow.c` with a path no port has
/// (`lib/scratch_buffer_grow.rs`): the first ported file in a *nested* reference directory
/// is what showed the reference path has to be tried first (tools/extract-abi.py's `pairs()`
/// already resolves a reference file exactly this way).
fn port_path(rel: &str, stem: &str) -> Option<String> {
    let own = if rel.ends_with(".h") {
        Path::new(rel).parent().unwrap_or(Path::new("")).join("include")
            .join(format!("{stem}.rs"))
    } else {
        Path::new(rel).with_extension("rs")
    }
    .to_string_lossy().to_string();
    if Path::new(&own).is_file() {
        return Some(own);
    }
    let header = rel.ends_with(".h");
    UNIT_DIRS
        .iter()
        .map(|dir| if header { format!("{dir}/include/{stem}.rs") }
                    else { format!("{dir}/{stem}.rs") })
        .find(|cand| Path::new(cand).is_file())
}

fn build_tu(rel: Option<&String>) -> i32 {
    let Some(rel) = rel else { return fail("build-tu needs a reference C file, e.g. src/doprnt.c") };
    let stem = Path::new(rel).file_stem().unwrap_or_default().to_string_lossy().to_string();
    let cname = crate_name (&stem);
    let (crate_root, out) = if rel.ends_with(".h") {
        let unit = port_path(rel, &stem).unwrap_or_else(|| format!("src/include/{stem}.rs"));
        (unit, Path::new(BUILD).join(format!("include_{stem}.o")))
    } else {
        // A unit keeps the reference's own path; `port_path` is what lets
        // `build-tu lib/getline.c`, `build-tu lib-src/etags.c` and
        // `build-tu lib/malloc/scratch_buffer_grow.c` all find their file.
        let unit = port_path(rel, &stem).unwrap_or_else(|| format!("src/{stem}.rs"));
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
    // A ledger's own claim is a fifth reading of the tree (status 3.349/3.350): a ledger that
    // ticks a function no ported file carries is a contradiction the four gates around this one
    // cannot see, because each of them measures something else (line counts, unticked boxes,
    // the Swift units, the ABI difference).
    println!("== ledger ticks ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-ledger-ticks.py"])
        .status()
    {
        Ok(s) if s.success() => println!("ledger ticks: every ticked function is carried"),
        Ok(_) => return fail("a ledger ticks a function no ported file carries"),
        Err(e) => return fail(e),
    }
    // A P-86 file's ledger records C line ranges instead of ticked boxes (its checklist is empty),
    // and the table is prose unless something reads it (status 3.372).
    println!("== dead-file coverage ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-dead-file-coverage.py"])
        .status()
    {
        Ok(s) if s.success() => println!("dead-file coverage: every range table accounts for its reference"),
        Ok(_) => return fail("a P-86 ledger's C range table has a gap or a malformed row"),
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
    // The level above every gate so far: the *program*.  `build-tu` stops at one object,
    // `layout-check` compiles probes, `check-undef.py` reads one unit's undefined list, and
    // `check-swift-imports.py` covers the Swift side of the C-ABI seam - so a symbol one unit
    // defines and another references by the wrong name is invisible until the objects are
    // merged.  A partial link (`ld -r`) of C's own object set answers exactly that question,
    // and `docs/baseline/link-pending.txt` is the ratchet, compared the way the symbol gates
    // above compare theirs: a name that is new fails and a row nothing references any more is
    // stale.  It is what found the four invented externals of status 3.251 (docs/ports/link.md).
    println!("== link gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-link.py"])
        .status()
    {
        Ok(s) if s.success() => println!("link: the object set closes over itself"),
        Ok(_) => {
            return fail(
                "a partial link of the whole program leaves a name no member defines \
                 (a new one, or a stale ratchet row)",
            )
        }
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
    // The declaration gate above reads the name the rule names; the bare word is what nobody
    // read.  A ported unit copies its C file's comments, and those call the program they belong
    // to by name - so a comment line carrying the bare name is either the reference's own bytes
    // (a copy, or a citation inside backticks or quotes) or this repository's own voice, which
    // calls the reference `C` (PORTING.md 0.1-19, its twenty-second half).
    println!("== own-voice gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-own-voice.py"])
        .status()
    {
        Ok(s) if s.success() => {
            println!("own voice: the port calls the reference C, not by name")
        }
        Ok(_) => {
            return fail("a ported comment in this repository's own voice names the reference \
                         (PORTING.md 0.1-19)")
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
    // `f_inc_probe.rs` is the newest and the only one without a C side: it compiles the batch-F
    // header mirrors of `femacs/nt/inc/`, which no configuration reaches - `layout-check` covers
    // the `src/include` and `lib/include` mirrors only, and two real defects lived in that gap
    // until the probe existed (docs/status.md 3.102).
    // `haiku_inc_probe.rs` and `android_inc_probe.rs` are the platform-exclusive mirrors'
    // compiler: `layout-check` excludes those mirrors by data
    // (docs/baseline/layout-excluded-mirrors.txt, status 3.147 - the X-compatibility names
    // collide by construction), so this is the only place their assertions and declarations are
    // compiled.  Same shape as `f_inc_probe.rs`: the probe names its mirrors and the graph's
    // de-duplication does the rest.
    // `win_inc_probe.rs` is the same shape again, for the two `src/include` mirrors whose unit is
    // an empty shape (`src/w32inevt.h`, `src/w32select.h`): no configuration compiles a `src/w32*`
    // unit on this target, so those mirrors had no consumer at all until this probe (status 3.159).
    // `ns_inc_probe.rs` is the same shape once more, for the two NS mirrors whose headers are
    // dead on this target (`src/nsgui.h`, `src/nsxwidget.h`): `layout-check` excludes nsgui.rs
    // by data (the `Emacs_Cursor` collision above), and the pair has no consumer at all
    // otherwise (status 3.161).
    // `sfnt_inc_probe.rs` is the same shape for the two platform-independent orphans whose
    // *unit* is the empty shape (`src/sfnt.h`, `src/fontset.h`): the frozen configuration
    // builds neither `src/sfnt.c`/`src/sfntfont.c` nor `src/fontset.c`, and the header-form
    // `build-tu src/sfnt.h` cannot compile any mirror that names `c_int` (the crate root is the
    // mirror, which carries no `use`), so the probe is what compiles the pair with the
    // `lisp.h`/`<setjmp.h>` names they take from their translation unit.
    const PROBES: [&str; 16] = ["../spikes/inline_probe.rs", "../spikes/accessor_probe.rs",
                                "../spikes/write_probe.rs", "../spikes/va_probe.rs",
                                "../spikes/layout.rs", "../spikes/jump.rs", "../spikes/scan.rs",
                                "../spikes/f_inc_probe.rs", "../spikes/win_inc_probe.rs",
                                "../spikes/haiku_inc_probe.rs", "../spikes/android_inc_probe.rs",
                                "../spikes/android_asset_probe.rs", "../spikes/ns_inc_probe.rs",
                                "../spikes/x_inc_probe.rs", "../spikes/sfnt_inc_probe.rs",
                                "../spikes/stdbit_family_probe.rs"];
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

/// `gate-load`: the loading ladder's first rung (docs/port-plan.md section 7.1).
///
/// The frozen reference build knows its own `temacs` link command; this asks it with
/// `make -n temacs`, substitutes the port's objects for the reference ones, and links.  Every
/// symbol the linker cannot resolve is a unit that is not ported yet or a name the port spells
/// wrong - the worklist L0 needs, which no other gate can produce (the text gates compare
/// files, the symbol gate compares exports, the undefined-reference gate works per object
/// against the SDK's own export list).  `--report` prints the list and exits 0; without it the
/// command is a gate.
///
/// **The link is driven by `rustc`, not by the C compiler**, and that is the whole point of the
/// driver below.  A unit is a `--crate-type lib` object (PORTING.md 0.1), so it expects the
/// Rust runtime's own symbols - `core`'s panic machinery, `compiler_builtins`' intrinsics, and
/// the `rust_begin_unwind` lang item - which a plain `clang` link leaves undefined (measured:
/// 210 of the first 230 undefined symbols were `core::panicking::*`).  The driver is an empty
/// `#![no_std] #![no_main]` bin crate with the port's own `#[panic_handler]` (PORTING.md 424:
/// `panic = "abort"`, and a panic is a defect, so the handler aborts like C's `emacs_abort`),
/// so rustc hands clang the runtime rlibs and the program's entry stays the port's own `main`
/// (emacs.rs's `#[unsafe(no_mangle)]` one).  `-nodefaultlibs` comes with the bin crate, hence
/// the explicit `-lSystem` (libc is what the ported C code calls).  The objects, the port's
/// `lib/` objects in place of the reference `libgnu.a` (the port has no archive, and mixing the
/// reference library in is the C/Rust hybrid goal constraint 2 forbids), the frozen `-L` paths
/// and `-lncurses` all go through a response file.
fn gate_load(arg: Option<&str>) -> i32 {
    let report = arg == Some("--report");
    // `-W emacs.o`: the frozen build's `temacs` is already linked, and make prints *nothing*
    // for an up-to-date target - which is why this gate could not find the recipe at all
    // (measured 2026-10-05: `make -n temacs` is six lines of admin/ sub-makes and no link).
    // `-W` pretends one object is newer, so make prints the recipe; with `-n` it runs nothing,
    // and it never writes a file, so the reference checkout is untouched.  `-B` is *not* the
    // answer: it also remakes `config.status`, and the reconfigure aborts on this machine
    // (no makeinfo) before make ever reaches the link.
    let out = match Command::new("make").args(
        ["-C", "../build/emacs-ref/src", "-n", "-W", "emacs.o", "temacs"]).output() {
        Ok(o) => o,
        Err(e) => return fail(format!("make -n temacs: {e}")),
    };
    if !out.status.success() {
        return fail("make -n temacs failed: configure the reference build first");
    }
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let lines: Vec<&str> = stdout.lines().collect();
    let start = match lines.iter().position(|l| l.contains("-o temacs.tmp")) {
        Some(i) => i,
        None => return fail("`make -n temacs` has no `-o temacs.tmp` line"),
    };
    let mut end = start;
    while lines[end].trim_end().ends_with('\\') && end + 1 < lines.len() {
        end += 1;
    }
    // Strip the Makefile's line continuations: the backslash-newline pairs are shell syntax.
    let joined = lines[start..=end].iter()
        .map(|l| l.trim_end().strip_suffix('\\').unwrap_or(l.trim_end()).trim_end())
        .collect::<Vec<&str>>().join(" ");

    let _ = fs::create_dir_all("../build/load");
    let mut objs: Vec<String> = Vec::new();
    let mut libs: Vec<String> = Vec::new();
    let mut search: Vec<String> = Vec::new();
    let mut missing: Vec<String> = Vec::new();
    let mut extra_libs: Vec<String> = Vec::new();
    for tok in joined.split_whitespace() {
        let is_obj = tok.len() > 2 && tok.ends_with(".o")
            && tok[..tok.len() - 2].chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
        if is_obj {
            let p = Path::new(REPO).join("build/femacs").join(tok);
            if p.is_file() {
                objs.push(fs::canonicalize(&p).unwrap_or(p).display().to_string());
            } else {
                missing.push(tok.to_string());
            }
        } else if tok.ends_with("../lib/libgnu.a") {
            // the port's own objects for the `lib/` units stand in for the reference archive
            if let Ok(rd) = fs::read_dir("lib") {
                let mut stems: Vec<String> = rd.filter_map(|e| e.ok())
                    .filter_map(|e| e.path().file_stem().map(|s| s.to_string_lossy().into_owned()))
                    .collect();
                stems.sort();
                for stem in stems {
                    let p = Path::new(REPO).join("build/femacs").join(format!("{stem}.o"));
                    if p.is_file() {
                        libs.push(fs::canonicalize(&p).unwrap_or(p).display().to_string());
                    }
                }
            }
        } else if tok.starts_with("-L") {
            search.push(tok.to_string());
        } else if tok == "-lncurses" || tok.starts_with("-l") {
            extra_libs.push(tok.to_string());
        }
    }
    if !missing.is_empty() {
        println!("port objects the frozen link needs but the port has not built: {}", missing.join(" "));
        if !report {
            return fail(format!("{} reference object(s) have no port object", missing.len()));
        }
    }
    // The driver: an empty no_std/no_main bin crate + the port's panic handler.
    const DRIVER: &str = "#![no_std]\n#![no_main]\n\
// femacs link driver (build scaffolding, not a ported file): rustc links the runtime.\n\
use core::panic::PanicInfo;\n\
#[panic_handler]\n\
fn panic (_info: &PanicInfo) -> ! { unsafe { abort () } }\n\
unsafe extern \"C\" { fn abort () -> !; }\n\
// libcompiler_builtins' prebuilt rlib carries EH landing pads (it is built for the\n\
// toolchain's own std configuration), so it references the C-unmangled\n\
// rust_eh_personality symbol; nothing raises in a panic=abort link, and the port's panics\n\
// abort through the handler above, so this no-op satisfies the linker.\n\
#[unsafe(no_mangle)]\n\
pub extern \"C\" fn rust_eh_personality () {}\n";
    if let Err(e) = fs::write("../build/load/driver.rs", DRIVER) {
        return fail(format!("writing the link driver: {e}"));
    }
    let out_path = fs::canonicalize("..").map(|p| p.join("build/load/temacs-port"))
        .unwrap_or_else(|_| PathBuf::from("build/load/temacs-port"));
    // The reference link pulls `../lib/libgnu.a`, and an archive's members are only linked when
    // something needs them: `lib/getloadavg.c`'s `nlist` call is in an object the C program never
    // pulls, while passing every lib object individually would demand that every one of them
    // resolve (measured: `_nlist`, `_rpl_mempcpy`, ... - symbols C's own link never asks for).
    // So the port spells the same thing with its own archive, built from its own objects
    // (nothing of the reference library is linked; goal constraint 2).
    // Every ported `lib/` unit goes into the archive.  Some of them are files the frozen
    // configuration cannot compile at all (`lib/getloadavg.c` dies on `#include <a.out.h>`, which
    // macOS does not have - measured with `make -C build/emacs-ref/lib getloadavg.o`), so the
    // archive is not the same set as the reference's own; the symbols only those files can
    // demand are the ratchet's business below, not a reason to drop their text from the link.
    let archive = "../build/load/libgnu-port.a";
    let _ = fs::remove_file(archive);
    let mut ar = Command::new("ar");
    ar.args(["crs", archive]);
    for l in &libs {
        ar.arg(l);
    }
    match ar.output() {
        Ok(o) if o.status.success() => println!("   archiving {} lib object(s) into libgnu-port.a", libs.len()),
        Ok(o) => return fail(format!("ar: {}", String::from_utf8_lossy(&o.stderr).trim())),
        Err(e) => return fail(format!("ar: {e}")),
    }
    let archive_abs = fs::canonicalize(archive).map(|p| p.display().to_string())
        .unwrap_or_else(|_| archive.to_string());
    let mut rsp: Vec<String> = vec![format!("-o\n{}", out_path.display())];
    rsp.extend(objs.iter().cloned());
    rsp.push(archive_abs);
    rsp.extend(search.iter().cloned());
    rsp.extend(extra_libs.iter().cloned());
    rsp.push("-lSystem".to_string());
    if let Err(e) = fs::write("../build/load/link-args.rsp", rsp.join("\n") + "\n") {
        return fail(format!("writing the link response file: {e}"));
    }
    println!("== gate-load: linking the frozen temacs object list against the port ==");
    println!("   {} port object(s) + {} lib object(s), linked by rustc (panic=abort, no_std driver)",
             objs.len(), libs.len());
    let out = match Command::new("rustc")
        .args(["--edition", "2024", "--crate-type", "bin", "-C", "panic=abort", "-C", "linker=clang"])
        .arg("../build/load/driver.rs")
        .arg("-C").arg("link-arg=@../build/load/link-args.rsp")
        .arg("-o").arg(&out_path)
        .output()
    {
        Ok(o) => o,
        Err(e) => return fail(format!("rustc link: {e}")),
    };
    let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    let mut undefined: Vec<(String, Vec<String>)> = Vec::new();
    let mut current: Option<usize> = None;
    for line in text.lines() {
        let tr = line.trim_start();
        if tr.starts_with('"') && line.contains(", referenced from:") {
            let name = tr.trim_start_matches('"').split('"').next().unwrap_or("").to_string();
            undefined.push((name, Vec::new()));
            current = Some(undefined.len() - 1);
        } else if tr.starts_with('_') && tr.contains(" in ") {
            if let Some(i) = current {
                if let Some((_, referrers)) = undefined.get_mut(i) {
                    if let Some(obj) = tr.split(" in ").nth(1) {
                        // the linker writes the object's basename; keep it, drop duplicates later
                        referrers.push(obj.trim().to_string());
                    }
                }
            }
        }
    }
    // The ratchet: `docs/baseline/link-pending.txt` names the symbols the port's link cannot
    // resolve for a reason that is not a missing port (a unit C cannot compile at all, a libc
    // API macOS does not export), one `symbol<TAB>reason` row each.  They are reported, not
    // hidden, and a row nothing matches any more is stale - the two other gates' convention.
    let mut pending: BTreeMap<String, String> = BTreeMap::new();
    if let Ok(text) = fs::read_to_string("../docs/baseline/link-pending.txt") {
        for line in text.lines() {
            if line.trim().is_empty() || line.starts_with('#') { continue; }
            let mut parts = line.splitn(2, '\t');
            if let (Some(sym), Some(reason)) = (parts.next(), parts.next()) {
                pending.insert(sym.trim().to_string(), reason.trim().to_string());
            }
        }
    }
    let (known, fresh): (Vec<&(String, Vec<String>)>, Vec<&(String, Vec<String>)>) = undefined
        .iter().partition(|(name, _)| pending.contains_key(name));
    if !known.is_empty() {
        println!("{} undefined symbol(s) are ratcheted in docs/baseline/link-pending.txt:", known.len());
        for (name, _) in &known {
            println!("  {name}");
        }
    }
    let stale: Vec<&String> = pending.keys().filter(|s| !undefined.iter().any(|(n, _)| n == *s)).collect();
    if !stale.is_empty() {
        println!("{} ratchet row(s) are stale (the symbol now resolves): {}", stale.len(),
                 stale.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(" "));
    }

    let mut why: BTreeMap<String, usize> = BTreeMap::new();
    for (_, refs) in &undefined {
        for r in refs { *why.entry(r.clone()).or_default() += 1; }
    }
    let mut why: Vec<(String, usize)> = why.into_iter().collect();
    why.sort_by(|a, b| b.1.cmp(&a.1));
    println!("undefined symbols: {} (referenced from {} object(s))", undefined.len(), why.len());
    for (obj, n) in why.iter().take(10) {
        println!("  {n:4}  {obj}");
    }
    let list = undefined.iter()
        .map(|(name, refs)| {
            let mut seen: Vec<&String> = Vec::new();
            for r in refs { if !seen.contains(&r) { seen.push(r); } }
            format!("{name}\t{}", seen.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(","))
        })
        .collect::<Vec<String>>().join("\n");
    if let Err(e) = fs::write("../build/load/undefined.txt", list + "\n") {
        eprintln!("warning: could not write build/load/undefined.txt: {e}");
    } else {
        println!("full list: build/load/undefined.txt");
    }
    println!("undefined: {} total, {} ratcheted, {} new", undefined.len(), known.len(), fresh.len());
    if undefined.is_empty() {
        if out.status.success() {
            println!("link: temacs linked from the port's objects alone");
            return 0;
        }
        for line in text.lines().rev().take(15).collect::<Vec<&str>>().into_iter().rev() {
            println!("  {line}");
        }
        return fail("the link failed without an undefined-symbol list (the lines above)");
    }
    if !fresh.is_empty() {
        println!("gate-load --report: {} undefined symbol(s); the list is the L0 worklist", undefined.len());
        return fail(format!("{} undefined symbol(s) are not in docs/baseline/link-pending.txt", fresh.len()));
    }
    println!("gate-load: every unresolved symbol is ratcheted in docs/baseline/link-pending.txt");
    0
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
    const FLIPS: [(&str, &str, &str); 7] = [
        ("no_rint", "HAVE_RINT", "false"),
        ("no_trunc", "HAVE_TRUNC", "false"),
        ("no_signbit", "HAVE_SIGNBIT", "false"),
        ("have_haiku", "HAIKU", "true"),
        // `have_native_comp` was the one structural arm *nothing* compiled: `cfg!(feature =
        // "have_native_comp")` is false in every real build (the `[features]` table in this
        // manifest leaves every feature off, and this list did not name it until F6), so `lisp.rs`'s `#[cfg(feature = "have_native_comp")]` pair — the
        // `NATIVE_COMP_FUNCTIONP`/`DYNP` bodies behind `HAVE_NATIVE_COMP` — had never been
        // through rustc, and the const assertion that claims the feature and the
        // configuration agree could not fail (review A's F6).  Flipping the configuration
        // constant *and* the feature together is what makes that claim executable.
        ("have_native_comp", "HAVE_NATIVE_COMP", "true"),
        // HAVE_LOG2 is not gated on a feature (the condition keeps the constant), but
        // the other arm is still C text that has to compile.
        ("", "HAVE_LOG2", "false"),
        // 3.263 measured the rest of the carried surface (`tools/feature-coverage.py`: 730 of the
        // 736 features the tree mentions are enabled by no configuration) and tried the four
        // largest arms that need no platform toolchain - the two semantic switches and the two
        // optional-library switches whose constant the generator emits.
        ("glyph_debug", "", "true"),
        // The NS/Cocoa arms were the second structural pair nothing compiled (the shim lane's
        // honest note, status 3.257 ③), and 3.262 tried the flip: `("have_ns", "HAVE_NS",
        // "true")` and `("ns_impl_cocoa", "", "true")` (an empty key means "a feature with no
        // configuration constant to move") compile the `#[cfg (feature = "have_ns")]` region -
        // and it does not compile, on purpose rather than by accident: nine errors name the
        // platform types the region depends on (`CGFloat`, `NSRect`, `NSString`, `NSArray`,
        // `EmacsMenu`, `Window`), which `include/nsterm.rs` 50-52 says the *platform target*
        // resolves because the port has no AppKit or GNUstep mirror.  The flip's first finding
        // was the one defect in that region that is not platform-shaped - eleven bare `c_float`
        // spellings, now qualified as `core::ffi::c_float` (status 3.91 ②, a mirror must not
        // depend on the includer's prelude) - and the flip itself waits for the AppKit mirror
        // that would make the rest resolvable.  Re-adding it is two entries plus the array
        // length.
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
        if key.is_empty() {
            continue; // a feature with no configuration constant to flip
        }
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
        Ok(u) => c_translation_units(u),
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
    // `jni-probe.sh` is `jni.rs`'s only gate: the mirror is of a system header of another
    // target's toolchain, so there is no reference file to pair it with - the probe compiles the
    // NDK's own <jni.h> and the mirror side by side and diffs the 233 function-table entries and
    // their offsets (status 3.154).
    for probe in ["spikes/inline-probe.sh", "spikes/accessor-probe.sh",
                  "spikes/write-probe.sh", "spikes/va-probe.sh",
                  "spikes/syshdrs-probe.sh", "spikes/gmp-abi-probe.sh",
                  "spikes/lisp-abi-probe.sh", "spikes/jni-probe.sh"] {
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
    let mut expand = Expand { out: out.clone(), root: root.clone(), seen: vec![root.clone()],
                              ungated: ungated_reachable(&root) };
    let text = expand.render(&root, false)?;
    let root_file = out.join("_root.rs");
    fs::write(&root_file, text).map_err(|e| format!("{}: {e}", root_file.display()))?;
    // Absolute: the callers that run rustc with a different working directory (dead-arms)
    // hand it straight to `rustc`.
    Ok(canonical(&root_file))
}

/// The in-place expansion state: the file the walk started from (whose text is the crate
/// root) and the files already expanded, in the order the preprocessor would see them.
/// Walk TRIM from bracket depth DEPTH, ignoring brackets inside string literals, and return
/// the depth when the line ends if the attribute closed, or `None` when it is still open (the
/// next line continues it).  The `attr_open`/`attr_depth` pair this serves is `render`'s gate
/// bookkeeping (see the note there).
fn scan_brackets(trim: &str, depth: i32) -> (i32, bool) {
    let mut depth = depth;
    let mut in_string = false;
    let mut escaped = false;
    // `depth == 0` before the first `[` means "the attribute has not opened yet", not "it
    // closed": returning closed on the `#` of `#[cfg …` is what made every attribute read as
    // an empty one.
    let mut opened = depth > 0;
    for ch in trim.chars() {
        if in_string {
            if escaped { escaped = false; }
            else if ch == '\\' { escaped = true; }
            else if ch == '"' { in_string = false; }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '[' | '(' | '{' => { depth += 1; opened = true; }
            ']' | ')' | '}' => depth -= 1,
            _ => {}
        }
        if opened && depth == 0 { return (0, true); }
    }
    (depth, opened && depth == 0)
}

struct Expand {
    out: PathBuf,
    root: PathBuf,
    seen: Vec<PathBuf>,
    /// The files an *ungated* path reaches from the root (`ungated_reachable`).  A `#[cfg]`-gated
    /// `include!` must not claim the one expansion slot of one of these: the ungated reach is live
    /// in every configuration, so it - not the gated one - is the reach that has to expand the
    /// file.  See the note on `render`.
    ungated: std::collections::BTreeSet<PathBuf>,
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
    fn render(&mut self, path: &Path, path_gated: bool) -> Result<String, String> {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let mut out = String::with_capacity(text.len() + 64);
        // Whether the `include!` on each line is under an attribute (`gate_states`), so this
        // walk and `ungated_reachable` cannot disagree about which edge is gated.
        let gates = gate_states(&text);
        for (line_no, line) in text.split_inclusive('\n').enumerate() {
            let (body, newline) = match line.strip_suffix('\n') {
                Some(b) => (b, "\n"),
                None => (line, ""),
            };
            // Whether *this* line's own edge is gated; `path_gated` carries the same fact for
            // every edge above it on the path from the root.
            let gated_here = gates[line_no];
            match include_target(line).map(|name| canonical(&dir.join(name))) {
                Some(target) if target.is_file() => {
                    if self.seen.contains(&target) {
                        out.push_str(&format!("// [xtask dedupe] {} already included{newline}",
                                              target.display()));
                        // A comment is not an item, so an attribute that was written for this
                        // `include!` would attach to whatever comes next - with the gate off,
                        // that is the file the ungated reach put after it (measured on the
                        // zz_mlgate fixture: the ungated edge's items disappeared).  The
                        // anonymous const consumes the attribute and is dropped with it.
                        out.push_str(&format!("const _: () = ();{newline}"));
                    } else if (gated_here || path_gated)
                        && target != self.root
                        && self.ungated.contains(&target)
                    {
                        // P-89's root fix (status 3.429).  A gated `include!` does *not* set C's
                        // include guard when the gate is off, so it must not consume the file's
                        // one expansion slot either: an *ungated* reach is live in every
                        // configuration, so a file that has one expands there and every
                        // gated-path reach of it (this one, or one nested inside a gated file's
                        // copy) stays a no-op.  The reverse order is what broke `sfnt.rs`'s
                        // `#ifdef TEST` `<time.h>`: it suppressed `sys_select.rs`'s own ungated
                        // one, and five `E0425`s (`time_t`/`timespec`) were the arm's reading.
                        out.push_str(&format!(
                            "// [xtask dedupe] {} expands at its ungated reach (this include is under a #[cfg]){newline}",
                            target.display()));
                        // Same reason as the branch above: the attribute written for this edge
                        // must be consumed by an item, not by a comment.
                        out.push_str(&format!("const _: () = ();{newline}"));
                    } else {
                        self.seen.push(target.clone());
                        let copy = self.copy_path(&target);
                        let body_text = self.render(&target, path_gated || gated_here)?;
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

/// `(gated, target)` for every `include!` line of a file's TEXT, where `gated` is true when the
/// line is preceded (blank lines skipped) by a `#[cfg…]`/`#[cfg_attr…]` attribute.  A mirror's
/// comment describing an include is not one: only a real `include!(` line counts.
fn include_edges(text: &str, dir: &Path) -> Vec<(bool, PathBuf)> {
    let gates = gate_states(text);
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if let Some(name) = include_target(line) {
            out.push((gates[i], canonical(&dir.join(name))));
        }
    }
    out
}

/// Whether an `include!` written on each line of TEXT sits under an attribute.  The rule is
/// "the attribute run that reaches this line", where a run is opened by `#[…]`/`#![…]`, may
/// span several lines (a multi-line `#[cfg (all (…, not (…)))]`), is not interrupted by a
/// blank line or a `//` comment, and is consumed by the first real item after it.
///
/// Reading only the previous line was a real defect with a real cost: a multi-line gate read as
/// no gate at all, so the gated edge claimed the file's single expansion slot away from the
/// reach that is live in every configuration, and a frozen build that must produce the empty
/// object produced code instead (measured on gmalloc's first draft: 1240 bytes holding
/// `flush_stack_barrier`/`cons_to_integer`, docs/status.md 3.553; the SetAEQ-class units write
/// their gates as two or three `#[cfg]` lines).  `render` and `ungated_reachable` both read this
/// one function now, so the two readings cannot disagree again.
fn gate_states(text: &str) -> Vec<bool> {
    let mut out = Vec::new();
    let mut attr_run = false;
    let mut attr_open = false;
    let mut depth = 0i32;
    for line in text.lines() {
        out.push(attr_run);
        let trimmed = line.trim_start();
        if attr_open {
            let (d, closed) = scan_brackets(trimmed, depth);
            depth = d;
            if closed {
                attr_open = false;
                attr_run = true;
            }
        } else if trimmed.starts_with("#[") || trimmed.starts_with("#![") {
            let (d, closed) = scan_brackets(trimmed, 0);
            depth = d;
            attr_open = !closed;
            attr_run = true;
        } else if trimmed.is_empty() || trimmed.starts_with("//") {
            // A blank line or a line comment does not end the run: the attribute still
            // belongs to the next item.
        } else {
            // A real item: whatever run reached this line applied to it and is consumed.
            attr_run = false;
        }
    }
    out
}

/// The files an *ungated* `include!` path reaches from ROOT: the ones every configuration
/// expands.  `Expand::render` needs this set to keep a `#[cfg]`-gated edge from taking a file's
/// single expansion slot away from its always-live reach (P-89's root fix, status 3.429).
fn ungated_reachable(root: &Path) -> std::collections::BTreeSet<PathBuf> {
    let mut seen = std::collections::BTreeSet::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(path) = stack.pop() {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let dir = path.parent().unwrap_or(Path::new("."));
        for (gated, target) in include_edges(&text, dir) {
            if gated || !target.is_file() {
                continue;
            }
            if seen.insert(target.clone()) {
                stack.push(target);
            }
        }
    }
    seen
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
        let checklist = fs::read_to_string(&ledger).map_err(|e| e.to_string())?;
        if checklist.contains("- [ ]") {
            continue;                                  // still being ported
        }
        // The ledger's own first line names the reference file (`# emacs/lib/malloc/x.c (56
        // lines)`), and PORTING.md 0.1's layout is what pairs it with the port; the stem
        // search over UNIT_DIRS is the same fallback `build_tu` keeps.
        let unit = checklist.lines().next()
            .and_then(|line| line.strip_prefix("# emacs/"))
            .and_then(|line| line.split_once(" (").map(|(path, _)| path.to_string()))
            // A `.h` ledger is a mirror, never a unit to compile: it is reached through the
            // units that include it.  (`src/indent.h` has no ledger of its own and reads
            // `indent.md`, whose first line names the `.c`, so the stem search below still
            // finds the unit; this filter only stops a header from being handed to rustc.)
            .filter(|rel| !rel.ends_with(".h"))
            .and_then(|rel| port_path(&rel, &stem))
            .or_else(|| UNIT_DIRS
                .iter()
                .map(|dir| format!("{dir}/{stem}.rs"))
                .find(|cand| Path::new(cand).is_file()));
        let Some(unit) = unit else {
            continue;                                  // header mirror, not a unit
        };
        units.push(unit);
    }
    Ok(units)
}

/// The subset of `finished_units()` a **C** build can compile.
///
/// `build-tu`'s path is the C one, and the finished set also holds nine references that are
/// not translation units: the two header ledgers (`src/lisp.h`, `lib/arg-nonnull.h`) - mirrors
/// with no object of their own, since `layout-check` asserts their types and every unit that
/// includes them compiles them - and the seven Objective-C ledgers (`src/*.m`), whose ports are
/// the Swift units under `femacs/platform/macos/` (`tools/check-swift-units.py` reads those).
/// Measured 2026-10-15, the first round in which every finished unit was built: the leg failed
/// on exactly those nine entries (six "does not exist yet" for the `.m` files, whose Rust name
/// `build_tu` derives from the C stem, and three unresolved names in the two headers).
fn c_translation_units(units: Vec<String>) -> Vec<String> {
    let mut skipped: Vec<String> = Vec::new();
    let kept: Vec<String> = units
        .into_iter()
        .filter(|u| {
            if u.ends_with(".c") {
                true
            } else {
                skipped.push(u.clone());
                false
            }
        })
        .collect();
    if !skipped.is_empty() {
        println!("skipped {} non-C reference(s): {}", skipped.len(), skipped.join(", "));
    }
    kept
}

fn build_finished_tus() -> Result<(), String> {
    let units = c_translation_units(finished_units()?);
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
