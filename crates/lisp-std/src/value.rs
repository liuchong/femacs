#![forbid(unsafe_code)]
//! Shared value type and conversion helpers for `lisp-std`.

use std::collections::BTreeMap;

/// Safe owned value exchanged between `lisp-std` callbacks and an embedding VM host.
#[derive(Clone, Debug, PartialEq)]
pub enum StdValue {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Symbol(String),
    List(Vec<StdValue>),
    Vector(Vec<StdValue>),
    Map(BTreeMap<String, StdValue>),
}

impl StdValue {
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil | Self::Bool(false))
    }

    pub fn is_truthy(&self) -> bool {
        !self.is_nil()
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(x) => Some(*x),
            Self::Int(n) => Some(*n as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) | Self::Symbol(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(b) => Some(b.as_slice()),
            Self::String(s) => Some(s.as_bytes()),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[StdValue]> {
        match self {
            Self::Nil => Some(&[]),
            Self::List(v) | Self::Vector(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&BTreeMap<String, StdValue>> {
        match self {
            Self::Map(m) => Some(m),
            _ => None,
        }
    }
}

pub(crate) fn expect_str<'a>(arg: Option<&'a StdValue>, label: &str) -> Result<&'a str, String> {
    arg.and_then(StdValue::as_str)
        .ok_or_else(|| format!("expected string or symbol for {label}"))
}

pub(crate) fn expect_opt_str<'a>(arg: Option<&'a StdValue>, label: &str) -> Result<Option<&'a str>, String> {
    match arg {
        None | Some(StdValue::Nil) => Ok(None),
        Some(other) => other
            .as_str()
            .map(Some)
            .ok_or_else(|| format!("expected string, symbol, or nil for {label}")),
    }
}

pub(crate) fn expect_bytes<'a>(arg: Option<&'a StdValue>, label: &str) -> Result<&'a [u8], String> {
    arg.and_then(StdValue::as_bytes)
        .ok_or_else(|| format!("expected bytes or string for {label}"))
}

pub(crate) fn expect_int(arg: Option<&StdValue>, label: &str) -> Result<i64, String> {
    arg.and_then(StdValue::as_int)
        .ok_or_else(|| format!("expected integer for {label}"))
}

pub(crate) fn expect_u64(arg: Option<&StdValue>, label: &str) -> Result<u64, String> {
    let n = expect_int(arg, label)?;
    u64::try_from(n).map_err(|_| format!("{label} must be non-negative (got {n})"))
}

pub(crate) fn expect_opt_u64(arg: Option<&StdValue>, label: &str) -> Result<Option<u64>, String> {
    match arg {
        None | Some(StdValue::Nil) => Ok(None),
        Some(other) => expect_u64(Some(other), label).map(Some),
    }
}

pub(crate) fn expect_bool(arg: Option<&StdValue>) -> bool {
    arg.is_some_and(StdValue::is_truthy)
}

pub(crate) fn expect_list<'a>(arg: Option<&'a StdValue>, label: &str) -> Result<&'a [StdValue], String> {
    match arg {
        None | Some(StdValue::Nil) => Ok(&[]),
        Some(StdValue::List(items) | StdValue::Vector(items)) => Ok(items.as_slice()),
        _ => Err(format!("expected list, vector, or nil for {label}")),
    }
}

pub(crate) fn expect_opt_list<'a>(
    arg: Option<&'a StdValue>,
    label: &str,
) -> Result<Option<&'a [StdValue]>, String> {
    match arg {
        None | Some(StdValue::Nil) => Ok(None),
        Some(StdValue::List(items) | StdValue::Vector(items)) => Ok(Some(items.as_slice())),
        _ => Err(format!("expected list, vector, or nil for {label}")),
    }
}

pub(crate) fn expect_opt_map<'a>(
    arg: Option<&'a StdValue>,
    label: &str,
) -> Result<Option<BTreeMap<String, StdValue>>, String> {
    match arg {
        None | Some(StdValue::Nil) => Ok(None),
        Some(StdValue::Map(m)) => Ok(Some(m.clone())),
        Some(StdValue::List(items)) => {
            // Also accept association lists `((k . v) ...)` or `(k v ...)`? In `StdValue`,
            // non-dotted pairs come through as two-element lists if nested; accept `((k v) ...)`.
            let mut out = BTreeMap::new();
            for item in items {
                let Some(pair) = item.as_list() else {
                    return Err(format!("expected map or alist for {label}"));
                };
                if pair.len() != 2 {
                    return Err(format!("expected key/value entry in {label}"));
                }
                let k = pair[0]
                    .as_str()
                    .ok_or_else(|| format!("expected string/symbol key in {label}"))?;
                out.insert(k.to_string(), pair[1].clone());
            }
            Ok(Some(out))
        }
        _ => Err(format!("expected map or nil for {label}")),
    }
}

pub fn json_to_std_value(val: serde_json::Value) -> StdValue {
    match val {
        serde_json::Value::Null => StdValue::Nil,
        serde_json::Value::Bool(b) => StdValue::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                StdValue::Int(i)
            } else if let Some(u) = n.as_u64() {
                i64::try_from(u)
                    .map(StdValue::Int)
                    .unwrap_or_else(|_| StdValue::Float(u as f64))
            } else {
                StdValue::Float(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_json::Value::String(s) => StdValue::String(s),
        serde_json::Value::Array(items) => {
            StdValue::List(items.into_iter().map(json_to_std_value).collect())
        }
        serde_json::Value::Object(obj) => {
            let mut map = BTreeMap::new();
            for (k, v) in obj {
                map.insert(k, json_to_std_value(v));
            }
            StdValue::Map(map)
        }
    }
}

pub fn std_value_to_json(val: &StdValue) -> Result<serde_json::Value, String> {
    match val {
        StdValue::Nil => Ok(serde_json::Value::Null),
        StdValue::Bool(b) => Ok(serde_json::Value::Bool(*b)),
        StdValue::Int(i) => Ok(serde_json::Value::Number((*i).into())),
        StdValue::Float(f) => serde_json::Number::from_f64(*f)
            .map(serde_json::Value::Number)
            .ok_or_else(|| format!("cannot encode non-finite float {f} to JSON")),
        StdValue::String(s) | StdValue::Symbol(s) => Ok(serde_json::Value::String(s.clone())),
        StdValue::Bytes(bytes) => Ok(serde_json::Value::Array(
            bytes
                .iter()
                .map(|b| serde_json::Value::Number((*b).into()))
                .collect(),
        )),
        StdValue::List(items) | StdValue::Vector(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(std_value_to_json(item)?);
            }
            Ok(serde_json::Value::Array(out))
        }
        StdValue::Map(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                out.insert(k.clone(), std_value_to_json(v)?);
            }
            Ok(serde_json::Value::Object(out))
        }
    }
}
