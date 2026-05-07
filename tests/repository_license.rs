use std::fs;

#[test]
fn repository_declares_strong_copyleft_license() {
    let cargo_toml = fs::read_to_string("Cargo.toml").expect("Cargo.toml should be readable");
    let license = fs::read_to_string("LICENSE").expect("LICENSE should be readable");

    assert!(cargo_toml.contains("license = \"AGPL-3.0-or-later\""));
    assert!(license.starts_with("GNU AFFERO GENERAL PUBLIC LICENSE"));
    assert!(license.contains("Version 3, 19 November 2007"));
    assert!(license.contains("Appropriate Legal Notices"));
}
