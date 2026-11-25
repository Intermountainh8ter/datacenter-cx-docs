//! Runs the doccheck binary against small docs trees under tests/fixtures.

use std::process::Command;

fn run(fixture: &str) -> (bool, String) {
    let dir = format!("{}/tests/fixtures/{fixture}", env!("CARGO_MANIFEST_DIR"));
    let out = Command::new(env!("CARGO_BIN_EXE_doccheck"))
        .arg(&dir)
        .output()
        .expect("run doccheck");
    let text = String::from_utf8_lossy(&out.stdout).to_string()
        + &String::from_utf8_lossy(&out.stderr);
    (out.status.success(), text)
}

#[test]
fn alias_pointing_at_a_real_document_passes() {
    let (ok, text) = run("alias-ok");
    assert!(ok, "{text}");
}

#[test]
fn alias_pointing_at_a_missing_document_fails() {
    let (ok, text) = run("alias-broken");
    assert!(!ok, "{text}");
    assert!(text.contains("alias_of names a missing document"), "{text}");
}
