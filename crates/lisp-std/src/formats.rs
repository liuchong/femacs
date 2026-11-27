#![forbid(unsafe_code)]
//! Pure-Rust serialization & encoding codecs (`std/json`, `std/toml`, `std/csv`, `std/yaml`,
//! `std/url`, `std/base64`).

use std::collections::BTreeMap;
use std::sync::Arc;

use base64::Engine as _;
use base64::engine::general_purpose::{STANDARD as B64_STD, URL_SAFE_NO_PAD as B64_URL};
use url::Url;
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader, yaml};

use crate::StdHandler;
use crate::value::{
    StdValue, expect_bool, expect_bytes, expect_list, expect_opt_list, expect_opt_map, expect_str,
    json_to_std_value, std_value_to_json,
};

fn toml_to_std_value(val: toml::Value) -> StdValue {
    match val {
        toml::Value::String(s) => StdValue::String(s),
        toml::Value::Integer(i) => StdValue::Int(i),
        toml::Value::Float(f) => StdValue::Float(f),
        toml::Value::Boolean(b) => StdValue::Bool(b),
        toml::Value::Datetime(dt) => StdValue::String(dt.to_string()),
        toml::Value::Array(items) => {
            StdValue::List(items.into_iter().map(toml_to_std_value).collect())
        }
        toml::Value::Table(tbl) => {
            let mut out = BTreeMap::new();
            for (k, v) in tbl {
                out.insert(k, toml_to_std_value(v));
            }
            StdValue::Map(out)
        }
    }
}

fn std_value_to_toml(val: &StdValue) -> Result<toml::Value, String> {
    match val {
        StdValue::Nil => Err("TOML does not support nil/null values inside arrays or roots".to_string()),
        StdValue::Bool(b) => Ok(toml::Value::Boolean(*b)),
        StdValue::Int(i) => Ok(toml::Value::Integer(*i)),
        StdValue::Float(f) => Ok(toml::Value::Float(*f)),
        StdValue::String(s) | StdValue::Symbol(s) => Ok(toml::Value::String(s.clone())),
        StdValue::Bytes(b) => Ok(toml::Value::Array(
            b.iter().map(|x| toml::Value::Integer(i64::from(*x))).collect(),
        )),
        StdValue::List(items) | StdValue::Vector(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(std_value_to_toml(item)?);
            }
            Ok(toml::Value::Array(out))
        }
        StdValue::Map(map) => {
            let mut tbl = toml::map::Map::new();
            for (k, v) in map {
                if !matches!(v, StdValue::Nil) {
                    tbl.insert(k.clone(), std_value_to_toml(v)?);
                }
            }
            Ok(toml::Value::Table(tbl))
        }
    }
}

fn yaml_to_std_value(val: &Yaml) -> StdValue {
    match val {
        Yaml::Real(s) => s
            .parse::<f64>()
            .map(StdValue::Float)
            .unwrap_or_else(|_| StdValue::String(s.clone())),
        Yaml::Integer(i) => StdValue::Int(*i),
        Yaml::String(s) => StdValue::String(s.clone()),
        Yaml::Boolean(b) => StdValue::Bool(*b),
        Yaml::Array(items) => StdValue::List(items.iter().map(yaml_to_std_value).collect()),
        Yaml::Hash(map) => {
            let mut out = BTreeMap::new();
            for (k, v) in map {
                let key = match k {
                    Yaml::String(s) => s.clone(),
                    Yaml::Integer(i) => i.to_string(),
                    Yaml::Boolean(b) => b.to_string(),
                    Yaml::Real(r) => r.clone(),
                    _ => format!("{k:?}"),
                };
                out.insert(key, yaml_to_std_value(v));
            }
            StdValue::Map(out)
        }
        Yaml::Alias(_) | Yaml::Null | Yaml::BadValue => StdValue::Nil,
    }
}

fn std_value_to_yaml(val: &StdValue) -> Yaml {
    match val {
        StdValue::Nil => Yaml::Null,
        StdValue::Bool(b) => Yaml::Boolean(*b),
        StdValue::Int(i) => Yaml::Integer(*i),
        StdValue::Float(f) => Yaml::Real(f.to_string()),
        StdValue::String(s) | StdValue::Symbol(s) => Yaml::String(s.clone()),
        StdValue::Bytes(bytes) => Yaml::Array(
            bytes
                .iter()
                .map(|b| Yaml::Integer(i64::from(*b)))
                .collect(),
        ),
        StdValue::List(items) | StdValue::Vector(items) => {
            Yaml::Array(items.iter().map(std_value_to_yaml).collect())
        }
        StdValue::Map(map) => {
            let mut h = yaml::Hash::new();
            for (k, v) in map {
                h.insert(Yaml::String(k.clone()), std_value_to_yaml(v));
            }
            Yaml::Hash(h)
        }
    }
}

fn std_value_to_cell_string(val: &StdValue) -> String {
    match val {
        StdValue::Nil => String::new(),
        StdValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
        StdValue::Int(i) => i.to_string(),
        StdValue::Float(f) => f.to_string(),
        StdValue::String(s) | StdValue::Symbol(s) => s.clone(),
        other => std_value_to_json(other)
            .map(|j| j.to_string())
            .unwrap_or_default(),
    }
}

pub(crate) fn register(mut reg: impl FnMut(&str, StdHandler)) {
    // --- JSON ---
    reg(
        "%std-json-parse",
        Arc::new(|args| {
            let raw = expect_str(args.first(), "json")?;
            let parsed: serde_json::Value =
                serde_json::from_str(raw).map_err(|e| format!("JSON parse error: {e}"))?;
            Ok(json_to_std_value(parsed))
        }),
    );

    reg(
        "%std-json-stringify",
        Arc::new(|args| {
            let val = args.first().unwrap_or(&StdValue::Nil);
            let pretty = expect_bool(args.get(1));
            let json = std_value_to_json(val)?;
            let out = if pretty {
                serde_json::to_string_pretty(&json)
            } else {
                serde_json::to_string(&json)
            }
            .map_err(|e| format!("JSON serialize error: {e}"))?;
            Ok(StdValue::String(out))
        }),
    );

    // --- TOML ---
    reg(
        "%std-toml-parse",
        Arc::new(|args| {
            let raw = expect_str(args.first(), "toml")?;
            let parsed: toml::Value =
                toml::from_str(raw).map_err(|e| format!("TOML parse error: {e}"))?;
            Ok(toml_to_std_value(parsed))
        }),
    );

    reg(
        "%std-toml-stringify",
        Arc::new(|args| {
            let val = args.first().unwrap_or(&StdValue::Nil);
            let toml_val = std_value_to_toml(val)?;
            let out = toml::to_string_pretty(&toml_val)
                .map_err(|e| format!("TOML serialize error: {e}"))?;
            Ok(StdValue::String(out))
        }),
    );

    // --- YAML ---
    reg(
        "%std-yaml-parse",
        Arc::new(|args| {
            let raw = expect_str(args.first(), "yaml")?;
            let docs =
                YamlLoader::load_from_str(raw).map_err(|e| format!("YAML parse error: {e}"))?;
            Ok(docs.first().map(yaml_to_std_value).unwrap_or(StdValue::Nil))
        }),
    );

    reg(
        "%std-yaml-stringify",
        Arc::new(|args| {
            let val = args.first().unwrap_or(&StdValue::Nil);
            let doc = std_value_to_yaml(val);
            let mut out = String::new();
            let mut emitter = YamlEmitter::new(&mut out);
            emitter
                .dump(&doc)
                .map_err(|e| format!("YAML serialize error: {e}"))?;
            Ok(StdValue::String(out))
        }),
    );

    // --- CSV ---
    reg(
        "%std-csv-parse",
        Arc::new(|args| {
            let raw = expect_str(args.first(), "csv")?;
            let has_headers = expect_bool(args.get(1));
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(has_headers)
                .from_reader(raw.as_bytes());
            if has_headers {
                let headers: Vec<String> = reader
                    .headers()
                    .map_err(|e| format!("CSV header error: {e}"))?
                    .iter()
                    .map(str::to_string)
                    .collect();
                let mut rows = Vec::new();
                for record in reader.records() {
                    let rec = record.map_err(|e| format!("CSV row error: {e}"))?;
                    let mut map = BTreeMap::new();
                    for (idx, h) in headers.iter().enumerate() {
                        let cell = rec.get(idx).unwrap_or("");
                        map.insert(h.clone(), StdValue::String(cell.to_string()));
                    }
                    rows.push(StdValue::Map(map));
                }
                Ok(StdValue::List(rows))
            } else {
                let mut rows = Vec::new();
                for record in reader.records() {
                    let rec = record.map_err(|e| format!("CSV row error: {e}"))?;
                    let cells = rec.iter().map(|s| StdValue::String(s.to_string())).collect();
                    rows.push(StdValue::List(cells));
                }
                Ok(StdValue::List(rows))
            }
        }),
    );

    reg(
        "%std-csv-stringify",
        Arc::new(|args| {
            let rows = expect_list(args.first(), "rows")?;
            let explicit_headers = expect_opt_list(args.get(1), "headers")?;
            let mut wtr = csv::WriterBuilder::new().from_writer(Vec::new());

            let headers: Option<Vec<String>> = match explicit_headers {
                Some(items) => {
                    let hs: Vec<String> = items
                        .iter()
                        .map(|v| {
                            v.as_str()
                                .map(str::to_string)
                                .ok_or_else(|| "CSV headers must be strings".to_string())
                        })
                        .collect::<Result<_, _>>()?;
                    Some(hs)
                }
                None => rows.first().and_then(StdValue::as_map).map(|m| m.keys().cloned().collect()),
            };

            if let Some(ref hs) = headers {
                wtr.write_record(hs)
                    .map_err(|e| format!("CSV write header error: {e}"))?;
            }

            for row in rows {
                match row {
                    StdValue::Map(m) => {
                        let keys = headers.as_ref().ok_or_else(|| {
                            "CSV stringify requires headers when writing map rows".to_string()
                        })?;
                        let rec: Vec<String> = keys
                            .iter()
                            .map(|k| m.get(k).map(std_value_to_cell_string).unwrap_or_default())
                            .collect();
                        wtr.write_record(&rec)
                            .map_err(|e| format!("CSV write row error: {e}"))?;
                    }
                    StdValue::List(cells) | StdValue::Vector(cells) => {
                        let rec: Vec<String> = cells.iter().map(std_value_to_cell_string).collect();
                        wtr.write_record(&rec)
                            .map_err(|e| format!("CSV write row error: {e}"))?;
                    }
                    StdValue::Nil => {
                        wtr.write_record(Vec::<String>::new())
                            .map_err(|e| format!("CSV write row error: {e}"))?;
                    }
                    other => {
                        wtr.write_record([std_value_to_cell_string(other)])
                            .map_err(|e| format!("CSV write row error: {e}"))?;
                    }
                }
            }

            let bytes = wtr
                .into_inner()
                .map_err(|e| format!("CSV flush error: {e}"))?;
            Ok(StdValue::String(String::from_utf8_lossy(&bytes).into_owned()))
        }),
    );

    // --- URL ---
    reg(
        "%std-url-parse",
        Arc::new(|args| {
            let raw = expect_str(args.first(), "url")?;
            let parsed = Url::parse(raw).map_err(|e| format!("URL parse error '{raw}': {e}"))?;
            let mut out = BTreeMap::new();
            out.insert("scheme".to_string(), StdValue::String(parsed.scheme().to_string()));
            out.insert(
                "host".to_string(),
                parsed
                    .host_str()
                    .map(|h| StdValue::String(h.to_string()))
                    .unwrap_or(StdValue::Nil),
            );
            out.insert(
                "port".to_string(),
                parsed
                    .port_or_known_default()
                    .map(|p| StdValue::Int(i64::from(p)))
                    .unwrap_or(StdValue::Nil),
            );
            out.insert("path".to_string(), StdValue::String(parsed.path().to_string()));
            out.insert(
                "query-string".to_string(),
                parsed
                    .query()
                    .map(|q| StdValue::String(q.to_string()))
                    .unwrap_or(StdValue::Nil),
            );
            let mut qmap = BTreeMap::new();
            for (k, v) in parsed.query_pairs() {
                qmap.insert(k.into_owned(), StdValue::String(v.into_owned()));
            }
            out.insert("query".to_string(), StdValue::Map(qmap));
            out.insert(
                "fragment".to_string(),
                parsed
                    .fragment()
                    .map(|f| StdValue::String(f.to_string()))
                    .unwrap_or(StdValue::Nil),
            );
            out.insert(
                "username".to_string(),
                StdValue::String(parsed.username().to_string()),
            );
            out.insert("href".to_string(), StdValue::String(parsed.to_string()));
            Ok(StdValue::Map(out))
        }),
    );

    reg(
        "%std-url-join",
        Arc::new(|args| {
            let base = expect_str(args.first(), "base")?;
            let rel = expect_str(args.get(1), "rel")?;
            let parsed = Url::parse(base).map_err(|e| format!("URL parse error '{base}': {e}"))?;
            let joined = parsed
                .join(rel)
                .map_err(|e| format!("URL join error '{base}' + '{rel}': {e}"))?;
            Ok(StdValue::String(joined.to_string()))
        }),
    );

    reg(
        "%std-url-encode",
        Arc::new(|args| {
            let raw = expect_str(args.first(), "text")?;
            let encoded: String = url::form_urlencoded::byte_serialize(raw.as_bytes()).collect();
            Ok(StdValue::String(encoded))
        }),
    );

    reg(
        "%std-url-decode",
        Arc::new(|args| {
            let raw = expect_str(args.first(), "text")?;
            let decoded: String = url::form_urlencoded::parse(raw.as_bytes())
                .map(|(k, v)| if v.is_empty() { k.into_owned() } else { format!("{k}={v}") })
                .collect::<Vec<_>>()
                .join("&");
            Ok(StdValue::String(decoded))
        }),
    );

    reg(
        "%std-url-build-query",
        Arc::new(|args| {
            let map = expect_opt_map(args.first(), "params")?.unwrap_or_default();
            let mut ser = url::form_urlencoded::Serializer::new(String::new());
            for (k, v) in map {
                if !v.is_nil() {
                    ser.append_pair(&k, &std_value_to_cell_string(&v));
                }
            }
            Ok(StdValue::String(ser.finish()))
        }),
    );

    // --- Base64 ---
    reg(
        "%std-base64-encode",
        Arc::new(|args| {
            let bytes = expect_bytes(args.first(), "data")?;
            let url_safe = expect_bool(args.get(1));
            let encoded = if url_safe {
                B64_URL.encode(bytes)
            } else {
                B64_STD.encode(bytes)
            };
            Ok(StdValue::String(encoded))
        }),
    );

    reg(
        "%std-base64-decode",
        Arc::new(|args| {
            let raw = expect_str(args.first(), "encoded")?.trim();
            let url_safe = expect_bool(args.get(1));
            let bytes = if url_safe {
                B64_URL.decode(raw)
            } else {
                B64_STD.decode(raw)
            }
            .map_err(|e| format!("base64 decode error: {e}"))?;
            Ok(StdValue::Bytes(bytes))
        }),
    );

    reg(
        "%std-base64-decode-string",
        Arc::new(|args| {
            let raw = expect_str(args.first(), "encoded")?.trim();
            let url_safe = expect_bool(args.get(1));
            let bytes = if url_safe {
                B64_URL.decode(raw)
            } else {
                B64_STD.decode(raw)
            }
            .map_err(|e| format!("base64 decode error: {e}"))?;
            let text =
                String::from_utf8(bytes).map_err(|e| format!("base64 UTF-8 decode error: {e}"))?;
            Ok(StdValue::String(text))
        }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toml_yaml_json_conversions_preserve_nested_maps_and_lists() {
        let toml_doc: toml::Value = toml::from_str("name = \"demo\"\ncount = 3\ntags = [\"a\", \"b\"]\n[nested]\nok = true\n").unwrap();
        let sv = toml_to_std_value(toml_doc);
        let round = std_value_to_toml(&sv).unwrap();
        assert_eq!(round.get("count").and_then(toml::Value::as_integer), Some(3));

        let docs = YamlLoader::load_from_str("a: 1\nb:\n  - x\n  - true\n").unwrap();
        let ysv = yaml_to_std_value(&docs[0]);
        let ymap = ysv.as_map().unwrap();
        assert_eq!(ymap.get("a"), Some(&StdValue::Int(1)));
        assert_eq!(
            ymap.get("b"),
            Some(&StdValue::List(vec![
                StdValue::String("x".to_string()),
                StdValue::Bool(true)
            ]))
        );
    }
}
