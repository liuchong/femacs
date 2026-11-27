#![forbid(unsafe_code)]
//! Pure-Rust time & timezone utilities (`std/time`) backed by `jiff`.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use jiff::Timestamp;
use jiff::tz::TimeZone;

use crate::StdHandler;
use crate::value::{StdValue, expect_int, expect_opt_str, expect_str, expect_u64};

pub(crate) fn register(epoch: Instant, mut reg: impl FnMut(&str, StdHandler)) {
    reg(
        "%std-time-now-utc",
        Arc::new(|_| Ok(StdValue::String(Timestamp::now().to_string()))),
    );

    reg(
        "%std-time-now-unix-ms",
        Arc::new(|_| Ok(StdValue::Int(Timestamp::now().as_millisecond()))),
    );

    reg(
        "%std-time-monotonic-ms",
        Arc::new(move |_| {
            let ms = i64::try_from(epoch.elapsed().as_millis()).unwrap_or(i64::MAX);
            Ok(StdValue::Int(ms))
        }),
    );

    reg(
        "%std-time-sleep-ms",
        Arc::new(|args| {
            let ms = expect_u64(args.first(), "ms")?;
            thread::sleep(Duration::from_millis(ms));
            Ok(StdValue::Nil)
        }),
    );

    reg(
        "%std-time-parse-rfc3339",
        Arc::new(|args| {
            let input = expect_str(args.first(), "timestamp")?;
            let ts: Timestamp = input
                .parse()
                .map_err(|e| format!("invalid RFC-3339 timestamp '{input}': {e}"))?;
            let dt = ts.to_zoned(TimeZone::UTC);
            let mut out = BTreeMap::new();
            out.insert("rfc3339".to_string(), StdValue::String(ts.to_string()));
            out.insert("unix-secs".to_string(), StdValue::Int(ts.as_second()));
            out.insert("unix-ms".to_string(), StdValue::Int(ts.as_millisecond()));
            out.insert("year".to_string(), StdValue::Int(i64::from(dt.year())));
            out.insert("month".to_string(), StdValue::Int(i64::from(dt.month())));
            out.insert("day".to_string(), StdValue::Int(i64::from(dt.day())));
            out.insert("hour".to_string(), StdValue::Int(i64::from(dt.hour())));
            out.insert("minute".to_string(), StdValue::Int(i64::from(dt.minute())));
            out.insert("second".to_string(), StdValue::Int(i64::from(dt.second())));
            out.insert("offset-secs".to_string(), StdValue::Int(0));
            Ok(StdValue::Map(out))
        }),
    );

    reg(
        "%std-time-format",
        Arc::new(|args| {
            let unix_secs = expect_int(args.first(), "unix-secs")?;
            let fmt = expect_str(args.get(1), "format")?;
            let tz_name = expect_opt_str(args.get(2), "timezone")?;
            let ts = Timestamp::from_second(unix_secs)
                .map_err(|e| format!("unix timestamp out of range: {e}"))?;
            let tz = match tz_name {
                None | Some("UTC") | Some("utc") => TimeZone::UTC,
                Some(name) => TimeZone::get(name)
                    .map_err(|e| format!("unknown timezone '{name}': {e}"))?,
            };
            let zoned = ts.to_zoned(tz);
            Ok(StdValue::String(zoned.strftime(fmt).to_string()))
        }),
    );

    reg(
        "%std-time-add-seconds",
        Arc::new(|args| {
            let input = expect_str(args.first(), "timestamp")?;
            let secs = expect_int(args.get(1), "seconds")?;
            let ts: Timestamp = input
                .parse()
                .map_err(|e| format!("invalid RFC-3339 timestamp '{input}': {e}"))?;
            let shifted = ts
                .checked_add(jiff::Span::new().seconds(secs))
                .map_err(|e| format!("timestamp arithmetic overflow: {e}"))?;
            Ok(StdValue::String(shifted.to_string()))
        }),
    );

    reg(
        "%std-time-diff-seconds",
        Arc::new(|args| {
            let a_raw = expect_str(args.first(), "a")?;
            let b_raw = expect_str(args.get(1), "b")?;
            let a: Timestamp = a_raw
                .parse()
                .map_err(|e| format!("invalid RFC-3339 timestamp '{a_raw}': {e}"))?;
            let b: Timestamp = b_raw
                .parse()
                .map_err(|e| format!("invalid RFC-3339 timestamp '{b_raw}': {e}"))?;
            let dur = b.duration_until(a);
            Ok(StdValue::Float(dur.as_secs_f64()))
        }),
    );
}
