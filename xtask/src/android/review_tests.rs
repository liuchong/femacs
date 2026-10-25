//! Independent regressions for the Android import-report contract.
use super::gate::validate_import_report;
use serde_json::{Value, json};

fn retained_ratchet() -> Value {
    json!({
        "unresolved": ["pending_a", "pending_b"],
        "new": [],
        "stale": [],
        "imports": 2,
        "unknown_needed": []
    })
}

#[test]
fn retained_unresolved_ratchet_is_not_treated_as_zero_imports() {
    let _timer = crate::test_support::CaseTimer::new(
        "retained_unresolved_ratchet_is_not_treated_as_zero_imports",
    );
    assert!(validate_import_report(&retained_ratchet()).is_ok());
}

#[test]
fn unknown_needed_library_is_rejected() {
    let _timer = crate::test_support::CaseTimer::new("unknown_needed_library_is_rejected");
    let mut report = retained_ratchet();
    report["unknown_needed"] = json!(["libmissing.so"]);
    assert!(validate_import_report(&report).is_err());
}

#[test]
fn new_import_is_rejected() {
    let _timer = crate::test_support::CaseTimer::new("new_import_is_rejected");
    let mut report = retained_ratchet();
    report["new"] = json!(["pending_a"]);
    assert!(validate_import_report(&report).is_err());
}

#[test]
fn malformed_import_report_is_rejected() {
    let _timer = crate::test_support::CaseTimer::new("malformed_import_report_is_rejected");
    let mut missing = retained_ratchet();
    missing.as_object_mut().unwrap().remove("unresolved");
    assert!(validate_import_report(&missing).is_err());

    let mut negative = retained_ratchet();
    negative["imports"] = json!(-1);
    assert!(validate_import_report(&negative).is_err());

    let mut textual = retained_ratchet();
    textual["imports"] = json!("2");
    assert!(validate_import_report(&textual).is_err());

    let mut wrong_type = retained_ratchet();
    wrong_type["unknown_needed"] = json!("libmissing.so");
    assert!(validate_import_report(&wrong_type).is_err());

    let mut empty_name = retained_ratchet();
    empty_name["unresolved"] = json!([""]);
    assert!(validate_import_report(&empty_name).is_err());

    let mut duplicate = retained_ratchet();
    duplicate["unresolved"] = json!(["pending_a", "pending_a"]);
    assert!(validate_import_report(&duplicate).is_err());
}

#[test]
fn new_is_unresolved_but_stale_means_no_longer_unresolved() {
    let _timer = crate::test_support::CaseTimer::new(
        "new_is_unresolved_but_stale_means_no_longer_unresolved",
    );
    let mut new_outside = retained_ratchet();
    new_outside["new"] = json!(["other"]);
    assert!(validate_import_report(&new_outside).is_err());

    // The actual checker computes stale = pending - unresolved. It must be
    // disjoint from unresolved; it is not a subset of unresolved.
    let mut stale = retained_ratchet();
    stale["stale"] = json!(["other"]);
    assert!(validate_import_report(&stale).is_ok());
    stale["stale"] = json!(["pending_a"]);
    assert!(validate_import_report(&stale).is_err());
}
