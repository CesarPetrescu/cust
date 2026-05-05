use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static TEMP_SOURCE_COUNTER: AtomicU64 = AtomicU64::new(0);

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

#[test]
fn tokens_flag_prints_lexer_tokens_without_interpreting_source() {
    let path = write_temp_source("int main() { return 1 / 0; }\n");

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg("--tokens")
        .arg(&path)
        .output()
        .expect("cust binary should run");

    fs::remove_file(&path).expect("temporary source should be removable");
    assert!(
        output.status.success(),
        "--tokens should lex without interpreting, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            "1:1 Int\n",
            "1:5 Ident(\"main\")\n",
            "1:9 LParen\n",
            "1:10 RParen\n",
            "1:12 LBrace\n",
            "1:14 Return\n",
            "1:21 Number(1)\n",
            "1:23 Slash\n",
            "1:25 Number(0)\n",
            "1:26 Semi\n",
            "1:28 RBrace\n",
            "2:1 Eof\n",
        )
    );
}

#[test]
fn tokens_flag_reports_lexer_errors_with_context() {
    let path = write_temp_source("int main() {\n@\n}\n");

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg("--tokens")
        .arg(&path)
        .output()
        .expect("cust binary should run");

    fs::remove_file(&path).expect("temporary source should be removable");
    assert!(
        !output.status.success(),
        "--tokens should reject lexer errors"
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "cust: unexpected character '@' at line 2, column 1\n@\n^\n"
    );
}

#[test]
fn ast_flag_prints_parsed_ast_without_interpreting_source() {
    let path = write_temp_source("int main() { return 1 / 0; }\n");

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg("--ast")
        .arg(&path)
        .output()
        .expect("cust binary should run");

    fs::remove_file(&path).expect("temporary source should be removable");
    assert!(
        output.status.success(),
        "--ast should parse without interpreting, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            "function main\n",
            "  params: []\n",
            "  body: [Return(Some(Binary(Number(1), Div, Number(0))))]\n",
        )
    );
}

#[test]
fn ast_flag_reports_parser_errors_without_interpreting_source() {
    let path = write_temp_source("int main() {\nreturn (1 + 2;\n}\n");

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg("--ast")
        .arg(&path)
        .output()
        .expect("cust binary should run");

    fs::remove_file(&path).expect("temporary source should be removable");
    assert!(
        !output.status.success(),
        "--ast should reject parser errors"
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "cust: expected ')' after grouped expression, found Semi at line 2, column 14\n"
    );
}

#[test]
fn max_steps_flag_limits_total_loop_iterations() {
    let path =
        write_temp_source("int main() {\nint i = 0;\nwhile (1) {\ni = i + 1;\n}\nreturn i;\n}\n");

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg("--max-steps")
        .arg("3")
        .arg(&path)
        .output()
        .expect("cust binary should run");

    fs::remove_file(&path).expect("temporary source should be removable");
    assert!(
        !output.status.success(),
        "runaway program should be bounded"
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "cust: execution step limit exceeded after 3 loop iterations\n"
    );
}

#[test]
fn max_steps_flag_allows_programs_within_the_loop_iteration_budget() {
    let path = write_temp_source(
        "int main() {\nint total = 0;\nfor (int i = 0; i < 4; i = i + 1) {\ntotal = total + i;\n}\nreturn total;\n}\n",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg("--max-steps")
        .arg("4")
        .arg(&path)
        .output()
        .expect("cust binary should run");

    fs::remove_file(&path).expect("temporary source should be removable");
    assert!(
        output.status.success(),
        "program within max-step budget should run, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "6\n");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}

#[test]
fn max_steps_flag_rejects_non_positive_limits() {
    let path = write_temp_source("int main() { return 0; }\n");

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg("--max-steps")
        .arg("0")
        .arg(&path)
        .output()
        .expect("cust binary should run");

    fs::remove_file(&path).expect("temporary source should be removable");
    assert!(!output.status.success(), "zero max-step limit should fail");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "cust: --max-steps requires a positive integer\n"
    );
}

fn write_temp_source(source: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    let unique_id = TEMP_SOURCE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "cust-cli-{}-{nanos}-{unique_id}.c",
        std::process::id()
    ));
    fs::write(&path, source).expect("temporary source should be writable");
    path.to_string_lossy().into_owned()
}
