#![forbid(unsafe_code)]
//! Typed general-purpose Lisp value representation for Rust embedding.

use std::collections::BTreeMap;
use std::fmt;

#[cfg(feature = "vm")]
use lisp_vm::host_runtime::VmValue;

/// Safe, owned value exchanged across the [`crate::Engine`] embedding boundary.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Symbol(String),
    List(Vec<Value>),
    Vector(Vec<Value>),
    Map(BTreeMap<String, Value>),
    Opaque(String),
}

impl Value {
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil | Self::Bool(false))
    }

    pub fn is_truthy(&self) -> bool {
        !self.is_nil()
    }

    pub fn as_bool(&self) -> bool {
        self.is_truthy()
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Int(n) => Some(*n as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) | Self::Symbol(s) | Self::Opaque(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(bytes) => Some(bytes.as_slice()),
            Self::String(s) => Some(s.as_bytes()),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Self::Nil => Some(&[]),
            Self::List(items) | Self::Vector(items) => Some(items.as_slice()),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Self::Map(map) => Some(map),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => f.write_str("nil"),
            Self::Bool(true) => f.write_str("t"),
            Self::Bool(false) => f.write_str("nil"),
            Self::Int(n) => write!(f, "{n}"),
            Self::Float(x) => write!(f, "{x}"),
            Self::String(s) => write!(f, "{s:?}"),
            Self::Bytes(bytes) => write!(f, "#<bytes len={}>", bytes.len()),
            Self::Symbol(s) | Self::Opaque(s) => f.write_str(s),
            Self::List(items) => {
                f.write_str("(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" ")?;
                    }
                    write!(f, "{item}")?;
                }
                f.write_str(")")
            }
            Self::Vector(items) => {
                f.write_str("[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" ")?;
                    }
                    write!(f, "{item}")?;
                }
                f.write_str("]")
            }
            Self::Map(map) => {
                f.write_str("{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{k:?}: {v}")?;
                }
                f.write_str("}")
            }
        }
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Int(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self::Int(i64::from(v))
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Self::Bytes(v)
    }
}

impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Self {
        Self::Bytes(v.to_vec())
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Self::List(v)
    }
}

impl From<BTreeMap<String, Value>> for Value {
    fn from(v: BTreeMap<String, Value>) -> Self {
        Self::Map(v)
    }
}

#[cfg(feature = "vm")]
impl From<VmValue> for Value {
    fn from(v: VmValue) -> Self {
        match v {
            VmValue::Nil => Self::Nil,
            VmValue::Bool(b) => Self::Bool(b),
            VmValue::Int(n) => Self::Int(n),
            VmValue::Float(x) => Self::Float(x),
            VmValue::String(s) => Self::String(s),
            VmValue::Bytes(bytes) => Self::Bytes(bytes),
            VmValue::Symbol(s) => Self::Symbol(s),
            VmValue::List(items) => Self::List(items.into_iter().map(Self::from).collect()),
            VmValue::Vector(items) => Self::Vector(items.into_iter().map(Self::from).collect()),
            VmValue::Map(entries) => {
                let mut map = BTreeMap::new();
                for (k, val) in entries {
                    map.insert(k, Self::from(val));
                }
                Self::Map(map)
            }
            VmValue::Opaque(s) => Self::Opaque(s),
        }
    }
}

#[cfg(feature = "vm")]
impl From<&Value> for VmValue {
    fn from(v: &Value) -> Self {
        match v {
            Value::Nil => Self::Nil,
            Value::Bool(b) => Self::Bool(*b),
            Value::Int(n) => Self::Int(*n),
            Value::Float(x) => Self::Float(*x),
            Value::String(s) => Self::String(s.clone()),
            Value::Bytes(bytes) => Self::Bytes(bytes.clone()),
            Value::Symbol(s) => Self::Symbol(s.clone()),
            Value::List(items) => Self::List(items.iter().map(Self::from).collect()),
            Value::Vector(items) => Self::Vector(items.iter().map(Self::from).collect()),
            Value::Map(map) => {
                Self::Map(map.iter().map(|(k, val)| (k.clone(), Self::from(val))).collect())
            }
            Value::Opaque(s) => Self::Opaque(s.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truthiness_and_accessors_cover_all_variants() {
        assert!(Value::Nil.is_nil());
        assert!(!Value::Nil.is_truthy());
        assert!(Value::Bool(false).is_nil());
        assert!(!Value::Bool(false).as_bool());
        assert!(Value::Bool(true).is_truthy());
        assert!(Value::Int(0).is_truthy());

        assert_eq!(Value::from(42i64).as_int(), Some(42));
        assert_eq!(Value::from(7i32).as_float(), Some(7.0));
        assert_eq!(Value::from(3.5f64).as_float(), Some(3.5));
        assert_eq!(Value::from("hi").as_str(), Some("hi"));
        assert_eq!(Value::Symbol("foo".to_string()).as_str(), Some("foo"));
        assert_eq!(Value::Opaque("#<x>".to_string()).as_str(), Some("#<x>"));
        assert_eq!(Value::Int(1).as_str(), None);
        assert_eq!(Value::Nil.as_list(), Some(&[][..]));

        let vec_val = Value::Vector(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(vec_val.as_list().map(<[Value]>::len), Some(2));

        let mut map = BTreeMap::new();
        map.insert("k".to_string(), Value::Int(9));
        let map_val = Value::from(map);
        assert_eq!(map_val.as_map().and_then(|m| m.get("k")), Some(&Value::Int(9)));
        assert_eq!(Value::Nil.as_map(), None);

        let bytes_owned = Value::from(vec![0u8, 255]);
        assert_eq!(bytes_owned.as_bytes(), Some(&[0u8, 255][..]));
        assert_eq!(bytes_owned.as_str(), None);
        assert_eq!(Value::from(&[10u8, 20][..]), Value::Bytes(vec![10, 20]));
    }

    #[test]
    fn display_formats_values_concisely() {
        assert_eq!(Value::Nil.to_string(), "nil");
        assert_eq!(Value::Bool(true).to_string(), "t");
        assert_eq!(Value::Bool(false).to_string(), "nil");
        assert_eq!(Value::Int(-5).to_string(), "-5");
        assert_eq!(Value::Float(2.5).to_string(), "2.5");
        assert_eq!(Value::String("a".to_string()).to_string(), "\"a\"");
        assert_eq!(Value::Bytes(vec![0, 255]).to_string(), "#<bytes len=2>");
        assert_eq!(Value::Symbol("sym".to_string()).to_string(), "sym");
        assert_eq!(
            Value::from(vec![Value::Int(1), Value::Symbol("x".to_string())]).to_string(),
            "(1 x)"
        );
        assert_eq!(
            Value::Vector(vec![Value::Int(1), Value::Int(2)]).to_string(),
            "[1 2]"
        );
        let mut map = BTreeMap::new();
        map.insert("k".to_string(), Value::Int(9));
        assert_eq!(Value::Map(map).to_string(), "{\"k\": 9}");
    }
}
