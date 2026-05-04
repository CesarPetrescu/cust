use std::process::Command;

#[test]
fn version_flag_prints_package_version_without_requiring_a_source_file() {
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg("--version")
        .output()
        .expect("cust binary should run");

    assert!(
        output.status.success(),
        "--version should exit successfully, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("cust {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}
