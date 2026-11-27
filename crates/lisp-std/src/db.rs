#![forbid(unsafe_code)]
//! Embedded ACID key-value store (`redb`) and pure-Rust transactional SQL engine (`std/kv`, `std/db`).

use std::cmp::Ordering as CmpOrdering;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use redb::backends::InMemoryBackend;
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};

use crate::StdHandler;
use crate::value::{
    StdValue, expect_bytes, expect_opt_list, expect_opt_str, expect_str, expect_u64,
};

const KV_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("kv");
const SQL_SCHEMAS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("__sql_schemas");
const SQL_ROWS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("__sql_rows");
const SQL_SEQ_TABLE: TableDefinition<&str, u64> = TableDefinition::new("__sql_seq");

fn open_redb_at(path: &str) -> Result<Database, String> {
    if path == ":memory:" {
        Database::builder()
            .create_with_backend(InMemoryBackend::new())
            .map_err(|e| format!("failed to create in-memory redb: {e}"))
    } else {
        let p = Path::new(path);
        if let Some(parent) = p.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|e| {
                    format!("failed to create DB parent dir {}: {e}", parent.display())
                })?;
            }
        }
        Database::create(p).map_err(|e| format!("failed to open redb '{}': {e}", p.display()))
    }
}

// --- Pure-Rust Embedded SQL Model ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum SqlVal {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}

impl SqlVal {
    fn from_std(v: &StdValue) -> Result<Self, String> {
        match v {
            StdValue::Nil => Ok(Self::Null),
            StdValue::Bool(b) => Ok(Self::Bool(*b)),
            StdValue::Int(i) => Ok(Self::Int(*i)),
            StdValue::Float(f) => Ok(Self::Float(*f)),
            StdValue::String(s) | StdValue::Symbol(s) => Ok(Self::Text(s.clone())),
            StdValue::Bytes(_) | StdValue::List(_) | StdValue::Vector(_) | StdValue::Map(_) => {
                Err("SQL parameters only support nil, bool, int, float, or string".to_string())
            }
        }
    }

    fn to_std(&self) -> StdValue {
        match self {
            Self::Null => StdValue::Nil,
            Self::Bool(b) => StdValue::Bool(*b),
            Self::Int(i) => StdValue::Int(*i),
            Self::Float(f) => StdValue::Float(*f),
            Self::Text(s) => StdValue::String(s.clone()),
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Bool(b) => *b,
            Self::Int(i) => *i != 0,
            Self::Float(f) => *f != 0.0,
            Self::Text(s) => !s.is_empty(),
        }
    }

    fn sql_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        match (self, other) {
            (Self::Null, Self::Null) => None,
            (Self::Null, _) | (_, Self::Null) => None,
            (Self::Int(a), Self::Int(b)) => Some(a.cmp(b)),
            (Self::Float(a), Self::Float(b)) => a.partial_cmp(b),
            (Self::Int(a), Self::Float(b)) => (*a as f64).partial_cmp(b),
            (Self::Float(a), Self::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Self::Bool(a), Self::Bool(b)) => Some(a.cmp(b)),
            (Self::Text(a), Self::Text(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ColumnDef {
    name: String,
    primary_key: bool,
    not_null: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TableSchema {
    name: String,
    columns: Vec<ColumnDef>,
}

#[derive(Clone, Debug)]
struct TxOverlay {
    schemas: BTreeMap<String, TableSchema>,
    rows: BTreeMap<String, BTreeMap<String, SqlVal>>,
    seq: BTreeMap<String, u64>,
}

struct SqlDbHandle {
    db: Database,
    active_tx: Mutex<Option<TxOverlay>>,
}

impl SqlDbHandle {
    fn open(path: &str) -> Result<Self, String> {
        let db = open_redb_at(path)?;
        let wtx = db
            .begin_write()
            .map_err(|e| format!("redb init write tx failed: {e}"))?;
        {
            let _ = wtx.open_table(SQL_SCHEMAS_TABLE).map_err(|e| format!("{e}"))?;
            let _ = wtx.open_table(SQL_ROWS_TABLE).map_err(|e| format!("{e}"))?;
            let _ = wtx.open_table(SQL_SEQ_TABLE).map_err(|e| format!("{e}"))?;
        }
        wtx.commit().map_err(|e| format!("redb commit init failed: {e}"))?;
        Ok(Self {
            db,
            active_tx: Mutex::new(None),
        })
    }

    fn load_snapshot(&self) -> Result<TxOverlay, String> {
        let rtx = self
            .db
            .begin_read()
            .map_err(|e| format!("redb read tx failed: {e}"))?;
        let schemas_tbl = rtx
            .open_table(SQL_SCHEMAS_TABLE)
            .map_err(|e| format!("{e}"))?;
        let mut schemas = BTreeMap::new();
        for item in schemas_tbl.iter().map_err(|e| format!("{e}"))? {
            let (k, v) = item.map_err(|e| format!("{e}"))?;
            let schema: TableSchema = serde_json::from_str(v.value())
                .map_err(|e| format!("corrupt table schema JSON: {e}"))?;
            schemas.insert(k.value().to_string(), schema);
        }

        let rows_tbl = rtx.open_table(SQL_ROWS_TABLE).map_err(|e| format!("{e}"))?;
        let mut rows = BTreeMap::new();
        for item in rows_tbl.iter().map_err(|e| format!("{e}"))? {
            let (k, v) = item.map_err(|e| format!("{e}"))?;
            let row: BTreeMap<String, SqlVal> =
                serde_json::from_str(v.value()).map_err(|e| format!("corrupt row JSON: {e}"))?;
            rows.insert(k.value().to_string(), row);
        }

        let seq_tbl = rtx.open_table(SQL_SEQ_TABLE).map_err(|e| format!("{e}"))?;
        let mut seq = BTreeMap::new();
        for item in seq_tbl.iter().map_err(|e| format!("{e}"))? {
            let (k, v) = item.map_err(|e| format!("{e}"))?;
            seq.insert(k.value().to_string(), v.value());
        }

        Ok(TxOverlay { schemas, rows, seq })
    }

    fn flush_snapshot(&self, snap: &TxOverlay) -> Result<(), String> {
        let wtx = self
            .db
            .begin_write()
            .map_err(|e| format!("redb write tx failed: {e}"))?;
        {
            let mut schemas_tbl = wtx
                .open_table(SQL_SCHEMAS_TABLE)
                .map_err(|e| format!("{e}"))?;
            let existing_keys: Vec<String> = schemas_tbl
                .iter()
                .map_err(|e| format!("{e}"))?
                .filter_map(|res| res.ok().map(|(k, _)| k.value().to_string()))
                .collect();
            for k in existing_keys {
                schemas_tbl.remove(k.as_str()).map_err(|e| format!("{e}"))?;
            }
            for (k, v) in &snap.schemas {
                let s = serde_json::to_string(v).map_err(|e| format!("{e}"))?;
                schemas_tbl
                    .insert(k.as_str(), s.as_str())
                    .map_err(|e| format!("{e}"))?;
            }

            let mut rows_tbl = wtx.open_table(SQL_ROWS_TABLE).map_err(|e| format!("{e}"))?;
            let existing_rows: Vec<String> = rows_tbl
                .iter()
                .map_err(|e| format!("{e}"))?
                .filter_map(|res| res.ok().map(|(k, _)| k.value().to_string()))
                .collect();
            for k in existing_rows {
                rows_tbl.remove(k.as_str()).map_err(|e| format!("{e}"))?;
            }
            for (k, v) in &snap.rows {
                let s = serde_json::to_string(v).map_err(|e| format!("{e}"))?;
                rows_tbl
                    .insert(k.as_str(), s.as_str())
                    .map_err(|e| format!("{e}"))?;
            }

            let mut seq_tbl = wtx.open_table(SQL_SEQ_TABLE).map_err(|e| format!("{e}"))?;
            let existing_seq: Vec<String> = seq_tbl
                .iter()
                .map_err(|e| format!("{e}"))?
                .filter_map(|res| res.ok().map(|(k, _)| k.value().to_string()))
                .collect();
            for k in existing_seq {
                seq_tbl.remove(k.as_str()).map_err(|e| format!("{e}"))?;
            }
            for (k, &v) in &snap.seq {
                seq_tbl.insert(k.as_str(), v).map_err(|e| format!("{e}"))?;
            }
        }
        wtx.commit().map_err(|e| format!("redb commit failed: {e}"))
    }

    fn begin(&self) -> Result<(), String> {
        let mut guard = self.active_tx.lock().unwrap_or_else(|e| e.into_inner());
        if guard.is_some() {
            return Err("transaction already active".to_string());
        }
        *guard = Some(self.load_snapshot()?);
        Ok(())
    }

    fn commit(&self) -> Result<(), String> {
        let mut guard = self.active_tx.lock().unwrap_or_else(|e| e.into_inner());
        let snap = guard
            .take()
            .ok_or_else(|| "no active transaction to commit".to_string())?;
        self.flush_snapshot(&snap)
    }

    fn rollback(&self) -> Result<(), String> {
        let mut guard = self.active_tx.lock().unwrap_or_else(|e| e.into_inner());
        if guard.take().is_none() {
            return Err("no active transaction to rollback".to_string());
        }
        Ok(())
    }

    fn with_mut_state<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut TxOverlay) -> Result<(R, bool), String>,
    {
        let mut guard = self.active_tx.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(ref mut overlay) = *guard {
            let (res, _) = f(overlay)?;
            Ok(res)
        } else {
            let mut snap = self.load_snapshot()?;
            let (res, dirty) = f(&mut snap)?;
            if dirty {
                self.flush_snapshot(&snap)?;
            }
            Ok(res)
        }
    }

    fn with_read_state<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&TxOverlay) -> Result<R, String>,
    {
        let guard = self.active_tx.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(ref overlay) = *guard {
            f(overlay)
        } else {
            let snap = self.load_snapshot()?;
            f(&snap)
        }
    }
}

// --- SQL Lexer & Recursive-Descent Parser ---

#[derive(Clone, Debug, PartialEq)]
enum Tok {
    Word(String),
    Str(String),
    Int(i64),
    Float(f64),
    Param,
    LParen,
    RParen,
    Comma,
    Semi,
    Star,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
}

fn tokenize_sql(sql: &str) -> Result<Vec<Tok>, String> {
    let bytes = sql.as_bytes();
    let mut i = 0usize;
    let mut toks = Vec::new();
    while i < bytes.len() {
        let ch = bytes[i] as char;
        if ch.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if ch == '-' && i + 1 < bytes.len() && bytes[i + 1] == b'-' {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        match ch {
            '(' => {
                toks.push(Tok::LParen);
                i += 1;
            }
            ')' => {
                toks.push(Tok::RParen);
                i += 1;
            }
            ',' => {
                toks.push(Tok::Comma);
                i += 1;
            }
            ';' => {
                toks.push(Tok::Semi);
                i += 1;
            }
            '*' => {
                toks.push(Tok::Star);
                i += 1;
            }
            '?' => {
                toks.push(Tok::Param);
                i += 1;
            }
            '=' => {
                toks.push(Tok::Eq);
                i += 1;
            }
            '!' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                    toks.push(Tok::Neq);
                    i += 2;
                } else {
                    return Err("unexpected '!' in SQL".to_string());
                }
            }
            '<' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                    toks.push(Tok::Le);
                    i += 2;
                } else if i + 1 < bytes.len() && bytes[i + 1] == b'>' {
                    toks.push(Tok::Neq);
                    i += 2;
                } else {
                    toks.push(Tok::Lt);
                    i += 1;
                }
            }
            '>' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                    toks.push(Tok::Ge);
                    i += 2;
                } else {
                    toks.push(Tok::Gt);
                    i += 1;
                }
            }
            '\'' => {
                i += 1;
                let mut s = String::new();
                let mut closed = false;
                while i < bytes.len() {
                    let c = sql[i..].chars().next().unwrap();
                    if c == '\'' {
                        if i + 1 < bytes.len() && bytes[i + 1] == b'\'' {
                            s.push('\'');
                            i += 2;
                            continue;
                        }
                        i += 1;
                        closed = true;
                        break;
                    }
                    s.push(c);
                    i += c.len_utf8();
                }
                if !closed {
                    return Err("unterminated string literal in SQL".to_string());
                }
                toks.push(Tok::Str(s));
            }
            c if c.is_ascii_digit()
                || ((c == '-' || c == '+')
                    && i + 1 < bytes.len()
                    && (bytes[i + 1] as char).is_ascii_digit()) =>
            {
                let start = i;
                if c == '-' || c == '+' {
                    i += 1;
                }
                let mut is_float = false;
                while i < bytes.len() {
                    let dc = bytes[i] as char;
                    if dc.is_ascii_digit() {
                        i += 1;
                    } else if dc == '.' && !is_float {
                        is_float = true;
                        i += 1;
                    } else {
                        break;
                    }
                }
                let raw = &sql[start..i];
                if is_float {
                    let f: f64 = raw
                        .parse()
                        .map_err(|e| format!("invalid SQL float '{raw}': {e}"))?;
                    toks.push(Tok::Float(f));
                } else {
                    let n: i64 = raw
                        .parse()
                        .map_err(|e| format!("invalid SQL integer '{raw}': {e}"))?;
                    toks.push(Tok::Int(n));
                }
            }
            '"' | '`' => {
                let quote = ch;
                i += 1;
                let start = i;
                while i < bytes.len() && (bytes[i] as char) != quote {
                    i += 1;
                }
                if i >= bytes.len() {
                    return Err("unterminated identifier quote in SQL".to_string());
                }
                toks.push(Tok::Word(sql[start..i].to_string()));
                i += 1;
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let start = i;
                while i < bytes.len() {
                    let wc = bytes[i] as char;
                    if wc.is_ascii_alphanumeric() || wc == '_' {
                        i += 1;
                    } else {
                        break;
                    }
                }
                toks.push(Tok::Word(sql[start..i].to_string()));
            }
            other => return Err(format!("unexpected SQL token character '{other}'")),
        }
    }
    Ok(toks)
}

#[derive(Clone, Debug)]
enum Expr {
    Lit(SqlVal),
    Param(usize),
    Col(String),
    Not(Box<Expr>),
    IsNull(Box<Expr>, bool), // bool = true means IS NOT NULL
    Bin {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Clone, Copy, Debug)]
enum BinOp {
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    Like,
}

#[derive(Clone, Debug)]
enum SelectItem {
    Wildcard,
    CountAll(String),
    Col { col: String, alias: Option<String> },
}

#[derive(Clone, Debug)]
enum Stmt {
    Begin,
    Commit,
    Rollback,
    CreateTable {
        name: String,
        if_not_exists: bool,
        columns: Vec<ColumnDef>,
    },
    DropTable {
        name: String,
        if_exists: bool,
    },
    Insert {
        table: String,
        cols: Option<Vec<String>>,
        rows: Vec<Vec<Expr>>,
    },
    Select {
        items: Vec<SelectItem>,
        table: String,
        where_clause: Option<Expr>,
        order_by: Option<(String, bool)>, // bool = desc
        limit: Option<usize>,
        offset: usize,
    },
    Update {
        table: String,
        assignments: Vec<(String, Expr)>,
        where_clause: Option<Expr>,
    },
    Delete {
        table: String,
        where_clause: Option<Expr>,
    },
}

struct Parser<'a> {
    toks: &'a [Tok],
    pos: usize,
    next_param: usize,
}

impl<'a> Parser<'a> {
    fn new(toks: &'a [Tok]) -> Self {
        Self {
            toks,
            pos: 0,
            next_param: 0,
        }
    }

    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    fn next(&mut self) -> Option<&Tok> {
        let t = self.toks.get(self.pos)?;
        self.pos += 1;
        Some(t)
    }

    fn eat_kw(&mut self, kw: &str) -> bool {
        if let Some(Tok::Word(w)) = self.peek() {
            if w.eq_ignore_ascii_case(kw) {
                self.pos += 1;
                return true;
            }
        }
        false
    }

    fn expect_kw(&mut self, kw: &str) -> Result<(), String> {
        if self.eat_kw(kw) {
            Ok(())
        } else {
            Err(format!("expected SQL keyword {kw}"))
        }
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match self.next() {
            Some(Tok::Word(w)) => Ok(w.clone()),
            other => Err(format!("expected identifier, got {other:?}")),
        }
    }

    fn expect_tok(&mut self, expected: Tok) -> Result<(), String> {
        let got = self.next();
        if got == Some(&expected) {
            Ok(())
        } else {
            Err(format!("expected {expected:?}, got {got:?}"))
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        while matches!(self.peek(), Some(Tok::Semi)) {
            self.pos += 1;
        }
        let stmt = if self.eat_kw("BEGIN") {
            let _ = self.eat_kw("TRANSACTION");
            Stmt::Begin
        } else if self.eat_kw("COMMIT") {
            let _ = self.eat_kw("TRANSACTION");
            Stmt::Commit
        } else if self.eat_kw("ROLLBACK") {
            let _ = self.eat_kw("TRANSACTION");
            Stmt::Rollback
        } else if self.eat_kw("CREATE") {
            self.expect_kw("TABLE")?;
            let if_not_exists = if self.eat_kw("IF") {
                self.expect_kw("NOT")?;
                self.expect_kw("EXISTS")?;
                true
            } else {
                false
            };
            let name = self.expect_ident()?;
            self.expect_tok(Tok::LParen)?;
            let mut columns = Vec::new();
            loop {
                let col_name = self.expect_ident()?;
                // Optional type keyword(s) and constraints until comma or ')'.
                let mut primary_key = false;
                let mut not_null = false;
                while let Some(tok) = self.peek() {
                    match tok {
                        Tok::Comma | Tok::RParen => break,
                        Tok::LParen => {
                            // Consume type precision like VARCHAR(255)
                            self.next();
                            while let Some(inner) = self.next() {
                                if matches!(inner, Tok::RParen) {
                                    break;
                                }
                            }
                        }
                        Tok::Word(w) => {
                            let upper = w.to_ascii_uppercase();
                            self.next();
                            if upper == "PRIMARY" {
                                self.expect_kw("KEY")?;
                                primary_key = true;
                            } else if upper == "NOT" {
                                self.expect_kw("NULL")?;
                                not_null = true;
                            }
                        }
                        _ => {
                            self.next();
                        }
                    }
                }
                columns.push(ColumnDef {
                    name: col_name,
                    primary_key,
                    not_null,
                });
                if self.peek() == Some(&Tok::Comma) {
                    self.next();
                } else {
                    break;
                }
            }
            self.expect_tok(Tok::RParen)?;
            Stmt::CreateTable {
                name,
                if_not_exists,
                columns,
            }
        } else if self.eat_kw("DROP") {
            self.expect_kw("TABLE")?;
            let if_exists = if self.eat_kw("IF") {
                self.expect_kw("EXISTS")?;
                true
            } else {
                false
            };
            let name = self.expect_ident()?;
            Stmt::DropTable { name, if_exists }
        } else if self.eat_kw("INSERT") {
            self.expect_kw("INTO")?;
            let table = self.expect_ident()?;
            let cols = if self.peek() == Some(&Tok::LParen) {
                self.next();
                let mut list = Vec::new();
                loop {
                    list.push(self.expect_ident()?);
                    if self.peek() == Some(&Tok::Comma) {
                        self.next();
                    } else {
                        break;
                    }
                }
                self.expect_tok(Tok::RParen)?;
                Some(list)
            } else {
                None
            };
            self.expect_kw("VALUES")?;
            let mut rows = Vec::new();
            loop {
                self.expect_tok(Tok::LParen)?;
                let mut vals = Vec::new();
                loop {
                    vals.push(self.parse_expr()?);
                    if self.peek() == Some(&Tok::Comma) {
                        self.next();
                    } else {
                        break;
                    }
                }
                self.expect_tok(Tok::RParen)?;
                rows.push(vals);
                if self.peek() == Some(&Tok::Comma) {
                    self.next();
                } else {
                    break;
                }
            }
            Stmt::Insert { table, cols, rows }
        } else if self.eat_kw("SELECT") {
            let mut items = Vec::new();
            loop {
                if self.peek() == Some(&Tok::Star) {
                    self.next();
                    items.push(SelectItem::Wildcard);
                } else if self.eat_kw("COUNT") {
                    self.expect_tok(Tok::LParen)?;
                    self.expect_tok(Tok::Star)?;
                    self.expect_tok(Tok::RParen)?;
                    let alias = if self.eat_kw("AS") {
                        self.expect_ident()?
                    } else {
                        "count".to_string()
                    };
                    items.push(SelectItem::CountAll(alias));
                } else {
                    let col = self.expect_ident()?;
                    let alias = if self.eat_kw("AS") {
                        Some(self.expect_ident()?)
                    } else {
                        None
                    };
                    items.push(SelectItem::Col { col, alias });
                }
                if self.peek() == Some(&Tok::Comma) {
                    self.next();
                } else {
                    break;
                }
            }
            self.expect_kw("FROM")?;
            let table = self.expect_ident()?;
            let where_clause = if self.eat_kw("WHERE") {
                Some(self.parse_expr()?)
            } else {
                None
            };
            let order_by = if self.eat_kw("ORDER") {
                self.expect_kw("BY")?;
                let col = self.expect_ident()?;
                let desc = if self.eat_kw("DESC") {
                    true
                } else {
                    let _ = self.eat_kw("ASC");
                    false
                };
                Some((col, desc))
            } else {
                None
            };
            let limit = if self.eat_kw("LIMIT") {
                match self.next() {
                    Some(Tok::Int(n)) if *n >= 0 => Some(*n as usize),
                    _ => return Err("LIMIT requires a non-negative integer".to_string()),
                }
            } else {
                None
            };
            let offset = if self.eat_kw("OFFSET") {
                match self.next() {
                    Some(Tok::Int(n)) if *n >= 0 => *n as usize,
                    _ => return Err("OFFSET requires a non-negative integer".to_string()),
                }
            } else {
                0
            };
            Stmt::Select {
                items,
                table,
                where_clause,
                order_by,
                limit,
                offset,
            }
        } else if self.eat_kw("UPDATE") {
            let table = self.expect_ident()?;
            self.expect_kw("SET")?;
            let mut assignments = Vec::new();
            loop {
                let col = self.expect_ident()?;
                self.expect_tok(Tok::Eq)?;
                let val = self.parse_expr()?;
                assignments.push((col, val));
                if self.peek() == Some(&Tok::Comma) {
                    self.next();
                } else {
                    break;
                }
            }
            let where_clause = if self.eat_kw("WHERE") {
                Some(self.parse_expr()?)
            } else {
                None
            };
            Stmt::Update {
                table,
                assignments,
                where_clause,
            }
        } else if self.eat_kw("DELETE") {
            self.expect_kw("FROM")?;
            let table = self.expect_ident()?;
            let where_clause = if self.eat_kw("WHERE") {
                Some(self.parse_expr()?)
            } else {
                None
            };
            Stmt::Delete {
                table,
                where_clause,
            }
        } else {
            return Err("unsupported SQL statement (expected CREATE/DROP/INSERT/SELECT/UPDATE/DELETE/BEGIN/COMMIT/ROLLBACK)".to_string());
        };

        while matches!(self.peek(), Some(Tok::Semi)) {
            self.pos += 1;
        }
        if let Some(extra) = self.peek() {
            return Err(format!("unexpected trailing SQL token {extra:?}"));
        }
        Ok(stmt)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while self.eat_kw("OR") {
            let right = self.parse_and()?;
            left = Expr::Bin {
                op: BinOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_not()?;
        while self.eat_kw("AND") {
            let right = self.parse_not()?;
            left = Expr::Bin {
                op: BinOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_not(&mut self) -> Result<Expr, String> {
        if self.eat_kw("NOT") {
            let inner = self.parse_not()?;
            Ok(Expr::Not(Box::new(inner)))
        } else {
            self.parse_cmp()
        }
    }

    fn parse_cmp(&mut self) -> Result<Expr, String> {
        let left = self.parse_primary()?;
        if self.eat_kw("IS") {
            let is_not = self.eat_kw("NOT");
            self.expect_kw("NULL")?;
            return Ok(Expr::IsNull(Box::new(left), is_not));
        }
        if self.eat_kw("LIKE") {
            let right = self.parse_primary()?;
            return Ok(Expr::Bin {
                op: BinOp::Like,
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        let op = match self.peek() {
            Some(Tok::Eq) => Some(BinOp::Eq),
            Some(Tok::Neq) => Some(BinOp::Neq),
            Some(Tok::Lt) => Some(BinOp::Lt),
            Some(Tok::Le) => Some(BinOp::Le),
            Some(Tok::Gt) => Some(BinOp::Gt),
            Some(Tok::Ge) => Some(BinOp::Ge),
            _ => None,
        };
        if let Some(bin_op) = op {
            self.next();
            let right = self.parse_primary()?;
            Ok(Expr::Bin {
                op: bin_op,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.next().cloned() {
            Some(Tok::LParen) => {
                let inner = self.parse_expr()?;
                self.expect_tok(Tok::RParen)?;
                Ok(inner)
            }
            Some(Tok::Param) => {
                let idx = self.next_param;
                self.next_param += 1;
                Ok(Expr::Param(idx))
            }
            Some(Tok::Int(n)) => Ok(Expr::Lit(SqlVal::Int(n))),
            Some(Tok::Float(f)) => Ok(Expr::Lit(SqlVal::Float(f))),
            Some(Tok::Str(s)) => Ok(Expr::Lit(SqlVal::Text(s))),
            Some(Tok::Word(w)) => {
                if w.eq_ignore_ascii_case("NULL") {
                    Ok(Expr::Lit(SqlVal::Null))
                } else if w.eq_ignore_ascii_case("TRUE") {
                    Ok(Expr::Lit(SqlVal::Bool(true)))
                } else if w.eq_ignore_ascii_case("FALSE") {
                    Ok(Expr::Lit(SqlVal::Bool(false)))
                } else {
                    Ok(Expr::Col(w))
                }
            }
            other => Err(format!("unexpected token in SQL expression: {other:?}")),
        }
    }
}

fn sql_like_matches(text: &str, pattern: &str) -> bool {
    let mut rx = String::from("^");
    for ch in pattern.chars() {
        match ch {
            '%' => rx.push_str(".*"),
            '_' => rx.push('.'),
            other => rx.push_str(&regex::escape(&other.to_string())),
        }
    }
    rx.push('$');
    regex::RegexBuilder::new(&rx)
        .dot_matches_new_line(true)
        .build()
        .is_ok_and(|re| re.is_match(text))
}

fn eval_expr(
    expr: &Expr,
    row: Option<&BTreeMap<String, SqlVal>>,
    params: &[SqlVal],
) -> Result<SqlVal, String> {
    match expr {
        Expr::Lit(v) => Ok(v.clone()),
        Expr::Param(idx) => params
            .get(*idx)
            .cloned()
            .ok_or_else(|| format!("missing SQL parameter at index {idx}")),
        Expr::Col(name) => {
            let r = row.ok_or_else(|| format!("column '{name}' is not valid in this context"))?;
            Ok(r.get(name).cloned().unwrap_or(SqlVal::Null))
        }
        Expr::Not(inner) => {
            let v = eval_expr(inner, row, params)?;
            Ok(SqlVal::Bool(!v.is_truthy()))
        }
        Expr::IsNull(inner, want_not_null) => {
            let v = eval_expr(inner, row, params)?;
            let is_null = matches!(v, SqlVal::Null);
            Ok(SqlVal::Bool(if *want_not_null { !is_null } else { is_null }))
        }
        Expr::Bin { op, left, right } => match op {
            BinOp::And => {
                let l = eval_expr(left, row, params)?;
                if !l.is_truthy() {
                    Ok(SqlVal::Bool(false))
                } else {
                    let r = eval_expr(right, row, params)?;
                    Ok(SqlVal::Bool(r.is_truthy()))
                }
            }
            BinOp::Or => {
                let l = eval_expr(left, row, params)?;
                if l.is_truthy() {
                    Ok(SqlVal::Bool(true))
                } else {
                    let r = eval_expr(right, row, params)?;
                    Ok(SqlVal::Bool(r.is_truthy()))
                }
            }
            BinOp::Eq | BinOp::Neq | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                let l = eval_expr(left, row, params)?;
                let r = eval_expr(right, row, params)?;
                let cmp = l.sql_cmp(&r);
                let matched = match (op, cmp) {
                    (BinOp::Eq, Some(CmpOrdering::Equal)) => true,
                    (BinOp::Neq, Some(ord)) => ord != CmpOrdering::Equal,
                    (BinOp::Lt, Some(CmpOrdering::Less)) => true,
                    (BinOp::Le, Some(CmpOrdering::Less | CmpOrdering::Equal)) => true,
                    (BinOp::Gt, Some(CmpOrdering::Greater)) => true,
                    (BinOp::Ge, Some(CmpOrdering::Greater | CmpOrdering::Equal)) => true,
                    _ => false,
                };
                Ok(SqlVal::Bool(matched))
            }
            BinOp::Like => {
                let l = eval_expr(left, row, params)?;
                let r = eval_expr(right, row, params)?;
                match (l, r) {
                    (SqlVal::Text(s), SqlVal::Text(pat)) => {
                        Ok(SqlVal::Bool(sql_like_matches(&s, &pat)))
                    }
                    _ => Ok(SqlVal::Bool(false)),
                }
            }
        },
    }
}

fn row_prefix(table: &str) -> String {
    format!("{table}\0")
}

fn make_row_key(table: &str, rowid: u64) -> String {
    format!("{table}\0{rowid:020}")
}

fn exec_stmt_on_handle(
    handle: &SqlDbHandle,
    sql: &str,
    params: &[SqlVal],
) -> Result<(i64, i64, Option<Vec<BTreeMap<String, StdValue>>>), String> {
    let toks = tokenize_sql(sql)?;
    let stmt = Parser::new(&toks).parse_stmt()?;
    match stmt {
        Stmt::Begin => {
            handle.begin()?;
            Ok((0, 0, None))
        }
        Stmt::Commit => {
            handle.commit()?;
            Ok((0, 0, None))
        }
        Stmt::Rollback => {
            handle.rollback()?;
            Ok((0, 0, None))
        }
        Stmt::CreateTable {
            name,
            if_not_exists,
            columns,
        } => {
            handle.with_mut_state(|snap| {
                if snap.schemas.contains_key(&name) {
                    if if_not_exists {
                        return Ok(((0, 0, None), false));
                    }
                    return Err(format!("table '{name}' already exists"));
                }
                snap.schemas
                    .insert(name.clone(), TableSchema { name, columns });
                Ok(((0, 0, None), true))
            })
        }
        Stmt::DropTable { name, if_exists } => handle.with_mut_state(|snap| {
            if snap.schemas.remove(&name).is_none() {
                if if_exists {
                    return Ok(((0, 0, None), false));
                }
                return Err(format!("table '{name}' does not exist"));
            }
            let prefix = row_prefix(&name);
            snap.rows.retain(|k, _| !k.starts_with(&prefix));
            snap.seq.remove(&name);
            Ok(((0, 0, None), true))
        }),
        Stmt::Insert { table, cols, rows } => handle.with_mut_state(|snap| {
            let schema = snap
                .schemas
                .get(&table)
                .cloned()
                .ok_or_else(|| format!("no such table '{table}'"))?;

            let target_cols: Vec<String> = match cols {
                Some(c) => {
                    for name in &c {
                        if !schema.columns.iter().any(|col| &col.name == name) {
                            return Err(format!("table '{table}' has no column '{name}'"));
                        }
                    }
                    c
                }
                None => schema.columns.iter().map(|c| c.name.clone()).collect(),
            };

            let prefix = row_prefix(&table);
            let mut last_rowid = *snap.seq.get(&table).unwrap_or(&0);
            let mut changes = 0i64;

            for expr_row in rows {
                if expr_row.len() != target_cols.len() {
                    return Err(format!(
                        "INSERT into '{table}' expected {} values, got {}",
                        target_cols.len(),
                        expr_row.len()
                    ));
                }
                let mut row_map = BTreeMap::new();
                for col in &schema.columns {
                    row_map.insert(col.name.clone(), SqlVal::Null);
                }
                for (col_name, expr) in target_cols.iter().zip(expr_row.iter()) {
                    let val = eval_expr(expr, None, params)?;
                    row_map.insert(col_name.clone(), val);
                }
                for col in &schema.columns {
                    let val = row_map.get(&col.name).unwrap_or(&SqlVal::Null);
                    if (col.not_null || col.primary_key) && matches!(val, SqlVal::Null) {
                        return Err(format!(
                            "NOT NULL constraint failed: {table}.{}",
                            col.name
                        ));
                    }
                    if col.primary_key {
                        for (rk, existing) in &snap.rows {
                            if rk.starts_with(&prefix)
                                && existing.get(&col.name) == Some(val)
                            {
                                return Err(format!(
                                    "UNIQUE/PRIMARY KEY constraint failed: {table}.{}",
                                    col.name
                                ));
                            }
                        }
                    }
                }
                last_rowid += 1;
                snap.rows.insert(make_row_key(&table, last_rowid), row_map);
                changes += 1;
            }
            snap.seq.insert(table, last_rowid);
            Ok(((changes, last_rowid as i64, None), changes > 0))
        }),
        Stmt::Update {
            table,
            assignments,
            where_clause,
        } => handle.with_mut_state(|snap| {
            let schema = snap
                .schemas
                .get(&table)
                .cloned()
                .ok_or_else(|| format!("no such table '{table}'"))?;
            for (col, _) in &assignments {
                if !schema.columns.iter().any(|c| &c.name == col) {
                    return Err(format!("table '{table}' has no column '{col}'"));
                }
            }
            let prefix = row_prefix(&table);
            let keys: Vec<String> = snap
                .rows
                .keys()
                .filter(|k| k.starts_with(&prefix))
                .cloned()
                .collect();
            let mut changes = 0i64;
            for key in keys {
                let row = snap.rows.get(&key).cloned().unwrap();
                let matched = match &where_clause {
                    Some(w) => eval_expr(w, Some(&row), params)?.is_truthy(),
                    None => true,
                };
                if matched {
                    let mut updated = row;
                    for (col, expr) in &assignments {
                        let new_val = eval_expr(expr, Some(&updated), params)?;
                        updated.insert(col.clone(), new_val);
                    }
                    snap.rows.insert(key, updated);
                    changes += 1;
                }
            }
            Ok(((changes, 0, None), changes > 0))
        }),
        Stmt::Delete {
            table,
            where_clause,
        } => handle.with_mut_state(|snap| {
            if !snap.schemas.contains_key(&table) {
                return Err(format!("no such table '{table}'"));
            }
            let prefix = row_prefix(&table);
            let keys: Vec<String> = snap
                .rows
                .keys()
                .filter(|k| k.starts_with(&prefix))
                .cloned()
                .collect();
            let mut changes = 0i64;
            for key in keys {
                let matched = match &where_clause {
                    Some(w) => {
                        let row = snap.rows.get(&key).unwrap();
                        eval_expr(w, Some(row), params)?.is_truthy()
                    }
                    None => true,
                };
                if matched {
                    snap.rows.remove(&key);
                    changes += 1;
                }
            }
            Ok(((changes, 0, None), changes > 0))
        }),
        Stmt::Select {
            items,
            table,
            where_clause,
            order_by,
            limit,
            offset,
        } => handle.with_read_state(|snap| {
            let schema = snap
                .schemas
                .get(&table)
                .ok_or_else(|| format!("no such table '{table}'"))?;
            let prefix = row_prefix(&table);
            let mut matched_rows: Vec<BTreeMap<String, SqlVal>> = Vec::new();
            for (k, row) in &snap.rows {
                if !k.starts_with(&prefix) {
                    continue;
                }
                let ok = match &where_clause {
                    Some(w) => eval_expr(w, Some(row), params)?.is_truthy(),
                    None => true,
                };
                if ok {
                    matched_rows.push(row.clone());
                }
            }

            if items.len() == 1 {
                if let SelectItem::CountAll(ref alias) = items[0] {
                    let mut m = BTreeMap::new();
                    m.insert(alias.clone(), StdValue::Int(matched_rows.len() as i64));
                    return Ok((0, 0, Some(vec![m])));
                }
            }

            if let Some((ref col, desc)) = order_by {
                matched_rows.sort_by(|a, b| {
                    let va = a.get(col).unwrap_or(&SqlVal::Null);
                    let vb = b.get(col).unwrap_or(&SqlVal::Null);
                    let ord = va.sql_cmp(vb).unwrap_or(CmpOrdering::Equal);
                    if desc { ord.reverse() } else { ord }
                });
            }

            let sliced: Vec<BTreeMap<String, SqlVal>> = matched_rows
                .into_iter()
                .skip(offset)
                .take(limit.unwrap_or(usize::MAX))
                .collect();

            let mut out = Vec::with_capacity(sliced.len());
            for row in sliced {
                let mut proj = BTreeMap::new();
                for item in &items {
                    match item {
                        SelectItem::Wildcard => {
                            for col in &schema.columns {
                                let v = row.get(&col.name).unwrap_or(&SqlVal::Null).to_std();
                                proj.insert(col.name.clone(), v);
                            }
                        }
                        SelectItem::CountAll(alias) => {
                            proj.insert(alias.clone(), StdValue::Int(1));
                        }
                        SelectItem::Col { col, alias } => {
                            let key = alias.as_ref().unwrap_or(col);
                            let v = row.get(col).unwrap_or(&SqlVal::Null).to_std();
                            proj.insert(key.clone(), v);
                        }
                    }
                }
                out.push(proj);
            }
            Ok((0, 0, Some(out)))
        }),
    }
}

pub struct DbRegistry {
    next_id: AtomicU64,
    kvs: Mutex<HashMap<u64, Arc<Database>>>,
    sqls: Mutex<HashMap<u64, Arc<SqlDbHandle>>>,
}

impl DbRegistry {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            kvs: Mutex::new(HashMap::new()),
            sqls: Mutex::new(HashMap::new()),
        }
    }

    fn get_kv(&self, id: u64) -> Result<Arc<Database>, String> {
        self.kvs
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("invalid KV handle {id}"))
    }

    fn get_sql(&self, id: u64) -> Result<Arc<SqlDbHandle>, String> {
        self.sqls
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("invalid DB handle {id}"))
    }
}

fn parse_sql_params(arg: Option<&StdValue>) -> Result<Vec<SqlVal>, String> {
    let raw = expect_opt_list(arg, "params")?.unwrap_or(&[]);
    raw.iter().map(SqlVal::from_std).collect()
}

pub(crate) fn register(registry: &Arc<DbRegistry>, mut reg: impl FnMut(&str, StdHandler)) {
    // --- KV Store (`std/kv`) ---
    let r = Arc::clone(registry);
    reg(
        "%std-kv-open",
        Arc::new(move |args| {
            let path = expect_str(args.first(), "path")?;
            let db = open_redb_at(path)?;
            let wtx = db
                .begin_write()
                .map_err(|e| format!("kv init tx failed: {e}"))?;
            {
                let _ = wtx.open_table(KV_TABLE).map_err(|e| format!("{e}"))?;
            }
            wtx.commit().map_err(|e| format!("kv init commit failed: {e}"))?;
            let id = r.next_id.fetch_add(1, Ordering::Relaxed);
            r.kvs
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(id, Arc::new(db));
            Ok(StdValue::Int(id as i64))
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-kv-get",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "kv-id")?;
            let key = expect_str(args.get(1), "key")?;
            let db = r.get_kv(id)?;
            let rtx = db.begin_read().map_err(|e| format!("kv read error: {e}"))?;
            let tbl = rtx.open_table(KV_TABLE).map_err(|e| format!("{e}"))?;
            match tbl.get(key).map_err(|e| format!("{e}"))? {
                Some(guard) => {
                    let text = std::str::from_utf8(guard.value())
                        .map_err(|e| format!("kv value is not UTF-8 (use std-kv-get-bytes): {e}"))?;
                    Ok(StdValue::String(text.to_string()))
                }
                None => Ok(StdValue::Nil),
            }
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-kv-get-bytes",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "kv-id")?;
            let key = expect_str(args.get(1), "key")?;
            let db = r.get_kv(id)?;
            let rtx = db.begin_read().map_err(|e| format!("kv read error: {e}"))?;
            let tbl = rtx.open_table(KV_TABLE).map_err(|e| format!("{e}"))?;
            match tbl.get(key).map_err(|e| format!("{e}"))? {
                Some(guard) => Ok(StdValue::Bytes(guard.value().to_vec())),
                None => Ok(StdValue::Nil),
            }
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-kv-set",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "kv-id")?;
            let key = expect_str(args.get(1), "key")?;
            let val = expect_bytes(args.get(2), "value")?;
            let db = r.get_kv(id)?;
            let wtx = db.begin_write().map_err(|e| format!("kv write error: {e}"))?;
            {
                let mut tbl = wtx.open_table(KV_TABLE).map_err(|e| format!("{e}"))?;
                tbl.insert(key, val).map_err(|e| format!("{e}"))?;
            }
            wtx.commit().map_err(|e| format!("kv commit error: {e}"))?;
            Ok(StdValue::Bool(true))
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-kv-del",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "kv-id")?;
            let key = expect_str(args.get(1), "key")?;
            let db = r.get_kv(id)?;
            let wtx = db.begin_write().map_err(|e| format!("kv write error: {e}"))?;
            let existed = {
                let mut tbl = wtx.open_table(KV_TABLE).map_err(|e| format!("{e}"))?;
                tbl.remove(key).map_err(|e| format!("{e}"))?.is_some()
            };
            wtx.commit().map_err(|e| format!("kv commit error: {e}"))?;
            Ok(StdValue::Bool(existed))
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-kv-list",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "kv-id")?;
            let prefix = expect_opt_str(args.get(1), "prefix")?.unwrap_or("");
            let db = r.get_kv(id)?;
            let rtx = db.begin_read().map_err(|e| format!("kv read error: {e}"))?;
            let tbl = rtx.open_table(KV_TABLE).map_err(|e| format!("{e}"))?;
            let mut keys = Vec::new();
            for item in tbl.iter().map_err(|e| format!("{e}"))? {
                let (k, _) = item.map_err(|e| format!("{e}"))?;
                let ks = k.value();
                if ks.starts_with(prefix) {
                    keys.push(StdValue::String(ks.to_string()));
                }
            }
            Ok(StdValue::List(keys))
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-kv-close",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "kv-id")?;
            let removed = r
                .kvs
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(&id)
                .is_some();
            Ok(StdValue::Bool(removed))
        }),
    );

    // --- Relational SQL Store (`std/db`) ---
    let r = Arc::clone(registry);
    reg(
        "%std-db-open",
        Arc::new(move |args| {
            let path = expect_str(args.first(), "path")?;
            let handle = Arc::new(SqlDbHandle::open(path)?);
            let id = r.next_id.fetch_add(1, Ordering::Relaxed);
            r.sqls
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(id, handle);
            Ok(StdValue::Int(id as i64))
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-db-exec",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "db-id")?;
            let sql = expect_str(args.get(1), "sql")?;
            let params = parse_sql_params(args.get(2))?;
            let handle = r.get_sql(id)?;
            let (changes, last_rowid, _) = exec_stmt_on_handle(&handle, sql, &params)?;
            let mut out = BTreeMap::new();
            out.insert("changes".to_string(), StdValue::Int(changes));
            out.insert("last-insert-rowid".to_string(), StdValue::Int(last_rowid));
            Ok(StdValue::Map(out))
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-db-query",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "db-id")?;
            let sql = expect_str(args.get(1), "sql")?;
            let params = parse_sql_params(args.get(2))?;
            let handle = r.get_sql(id)?;
            let (_, _, rows_opt) = exec_stmt_on_handle(&handle, sql, &params)?;
            let rows = rows_opt.unwrap_or_default();
            Ok(StdValue::List(rows.into_iter().map(StdValue::Map).collect()))
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-db-tables",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "db-id")?;
            let handle = r.get_sql(id)?;
            let names: Vec<StdValue> = handle.with_read_state(|snap| {
                Ok(snap.schemas.keys().cloned().map(StdValue::String).collect())
            })?;
            Ok(StdValue::List(names))
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-db-begin",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "db-id")?;
            r.get_sql(id)?.begin()?;
            Ok(StdValue::Bool(true))
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-db-commit",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "db-id")?;
            r.get_sql(id)?.commit()?;
            Ok(StdValue::Bool(true))
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-db-rollback",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "db-id")?;
            r.get_sql(id)?.rollback()?;
            Ok(StdValue::Bool(true))
        }),
    );

    let r = Arc::clone(registry);
    reg(
        "%std-db-close",
        Arc::new(move |args| {
            let id = expect_u64(args.first(), "db-id")?;
            let removed = r
                .sqls
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(&id)
                .is_some();
            Ok(StdValue::Bool(removed))
        }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sql_engine_crud_constraints_tx_and_rollback() {
        let h = SqlDbHandle::open(":memory:").unwrap();
        exec_stmt_on_handle(
            &h,
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, score INTEGER)",
            &[],
        )
        .unwrap();
        exec_stmt_on_handle(
            &h,
            "INSERT INTO users (id, name, score) VALUES (1, 'Alice', 90), (2, 'Bob', 75)",
            &[],
        )
        .unwrap();

        // Primary key uniqueness violation
        let dup = exec_stmt_on_handle(
            &h,
            "INSERT INTO users (id, name, score) VALUES (1, 'Dup', 10)",
            &[],
        );
        assert!(dup.is_err());

        // Parameterized SELECT + ORDER BY + LIKE
        let (_, _, rows) = exec_stmt_on_handle(
            &h,
            "SELECT name, score FROM users WHERE score >= ? AND name LIKE 'A%' ORDER BY score DESC LIMIT 5",
            &[SqlVal::Int(80)],
        )
        .unwrap();
        let rows = rows.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get("name"), Some(&StdValue::String("Alice".to_string())));

        // Transaction rollback leaves pre-tx state intact
        h.begin().unwrap();
        exec_stmt_on_handle(&h, "DELETE FROM users WHERE id = 2", &[]).unwrap();
        h.rollback().unwrap();

        let (_, _, count_rows) =
            exec_stmt_on_handle(&h, "SELECT COUNT(*) AS n FROM users", &[]).unwrap();
        assert_eq!(count_rows.unwrap()[0].get("n"), Some(&StdValue::Int(2)));
    }
}
