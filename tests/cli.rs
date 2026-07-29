use std::fs;
use std::process::Command;
#[cfg(unix)]
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(unix)]
use std::time::{Duration, Instant};
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
    assert_eq!(env!("CARGO_PKG_VERSION"), "0.4.0");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "cust 0.4.0\n");
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
fn ast_flag_preserves_control_statement_debug_output() {
    let path = write_temp_source("int main(void) { while (1) { break; } continue; return 0; }\n");

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg("--ast")
        .arg(&path)
        .output()
        .expect("cust binary should run");

    fs::remove_file(&path).expect("temporary source should be removable");
    assert!(output.status.success(), "control statements should parse");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            "function main\n",
            "  params: []\n",
            "  body: [While { cond: Number(1), body: [Block([Break(LocatedToken { kind: Break, line: 1, column: 30 })])] }, Continue(LocatedToken { kind: Continue, line: 1, column: 39 }), Return(Some(Number(0)))]\n",
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

#[cfg(not(target_os = "linux"))]
#[test]
fn run_mode_fails_closed_for_quoted_headers_without_secure_resolution() {
    let directory = temp_source_directory("unsupported-quoted-header");
    let source = directory.join("main.c");
    fs::write(directory.join("config.h"), "#define VALUE 7\n")
        .expect("temporary header should be writable");
    fs::write(
        &source,
        "#include \"config.h\"\nint main(void) { return VALUE; }\n",
    )
    .expect("temporary source should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    fs::remove_dir_all(&directory).expect("temporary source directory should be removable");

    assert!(
        !output.status.success(),
        "quoted include should fail closed"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("secure quoted header inclusion is not supported on this platform"),
        "unexpected unsupported-platform diagnostic: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(target_os = "linux")]
#[test]
fn run_mode_supports_project_relative_quoted_headers() {
    let directory = temp_source_directory("quoted-header");
    let header = directory.join("config.h");
    let source = directory.join("main.c");
    fs::write(
        &header,
        "#ifndef CONFIG_H\n#define CONFIG_H\n#define BASE 40\nint from_header(int value) { return value + BASE; }\n#endif\n",
    )
    .expect("temporary header should be writable");
    fs::write(
        &source,
        "#include \"config.h\"\nint main(void) { return from_header(2); }\n",
    )
    .expect("temporary source should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    let tokens_output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg("--tokens")
        .arg(&source)
        .output()
        .expect("cust binary should run in token mode");

    fs::remove_dir_all(&directory).expect("temporary source directory should be removable");
    assert!(
        output.status.success(),
        "quoted project header should run, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "42\n");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert!(
        tokens_output.status.success(),
        "included tokens should format"
    );
    assert!(
        String::from_utf8_lossy(&tokens_output.stdout).contains("config.h:4:1 Int"),
        "included token origin should be visible: {}",
        String::from_utf8_lossy(&tokens_output.stdout)
    );
}

#[cfg(target_os = "linux")]
#[test]
fn token_mode_distinguishes_project_relative_header_origins() {
    let directory = temp_source_directory("header-origins");
    fs::create_dir(directory.join("left")).expect("left header directory should be creatable");
    fs::create_dir(directory.join("right")).expect("right header directory should be creatable");
    fs::write(directory.join("left/common.h"), "int left_value = 1;\n")
        .expect("left header should be writable");
    fs::write(directory.join("right/common.h"), "int right_value = 2;\n")
        .expect("right header should be writable");
    fs::write(directory.join("left/outer.h"), "#include \"common.h\"\n")
        .expect("left outer header should be writable");
    fs::write(directory.join("right/outer.h"), "#include \"common.h\"\n")
        .expect("right outer header should be writable");
    let source = directory.join("main.c");
    fs::write(
        &source,
        concat!(
            "#include \"left/outer.h\"\n",
            "#include \"right/outer.h\"\n",
            "int main(void) { return left_value + right_value; }\n",
        ),
    )
    .expect("temporary source should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg("--tokens")
        .arg(&source)
        .output()
        .expect("cust binary should run in token mode");
    fs::remove_dir_all(&directory).expect("temporary source directory should be removable");

    assert!(output.status.success(), "included tokens should format");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("left/common.h:1:1 Int"),
        "left header origin should be project-relative: {stdout}"
    );
    assert!(
        stdout.contains("right/common.h:1:1 Int"),
        "right header origin should be project-relative: {stdout}"
    );
}

#[cfg(target_os = "linux")]
#[test]
fn run_mode_resolves_headers_from_logical_symlink_paths() {
    let directory = temp_source_directory("logical-header-paths");
    let front = directory.join("front");
    let real = directory.join("real");
    fs::create_dir(&front).expect("front directory should be creatable");
    fs::create_dir(&real).expect("real directory should be creatable");

    fs::write(front.join("config.h"), "#define PRIMARY_VALUE 7\n")
        .expect("front config should be writable");
    fs::write(
        real.join("main.c"),
        "#include \"config.h\"\nint main(void) { return PRIMARY_VALUE; }\n",
    )
    .expect("real source should be writable");
    std::os::unix::fs::symlink(real.join("main.c"), front.join("main.c"))
        .expect("primary source symlink should be creatable");
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(front.join("main.c"))
        .output()
        .expect("cust binary should run");
    assert!(
        output.status.success(),
        "primary symlink should search beside its logical path: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "7\n");

    let nested_real = front.join("real");
    fs::create_dir(front.join("alias")).expect("alias directory should be creatable");
    fs::create_dir(&nested_real).expect("nested real directory should be creatable");
    fs::write(front.join("alias/sibling.h"), "#define NESTED_VALUE 7\n")
        .expect("logical sibling should be writable");
    fs::write(nested_real.join("sibling.h"), "#define NESTED_VALUE 9\n")
        .expect("target sibling should be writable");
    fs::write(nested_real.join("outer.h"), "#include \"sibling.h\"\n")
        .expect("outer header should be writable");
    std::os::unix::fs::symlink(nested_real.join("outer.h"), front.join("alias/outer.h"))
        .expect("header symlink should be creatable");
    fs::write(
        front.join("direct.c"),
        "#include \"alias/outer.h\"\nint main(void) { return NESTED_VALUE; }\n",
    )
    .expect("direct source should be writable");
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(front.join("direct.c"))
        .output()
        .expect("cust binary should run");

    fs::remove_dir_all(&directory).expect("temporary source directory should be removable");
    assert!(
        output.status.success(),
        "nested include should search beside its logical path: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "7\n");
}

#[cfg(target_os = "linux")]
#[test]
fn run_mode_rejects_unsupported_missing_and_unsafe_headers() {
    let cases = [
        (
            "#include <stdio.h>\nint main(void) { return 0; }\n",
            "system header includes are not supported at line 1, column 10\n#include <stdio.h>\n         ^",
        ),
        (
            "#include \"missing.h\"\nint main(void) { return 0; }\n",
            "quoted header 'missing.h' was not found below the project include root at line 1, column 1\n#include \"missing.h\"\n^",
        ),
        (
            "#include \"../outside.h\"\nint main(void) { return 0; }\n",
            "quoted header path '../outside.h' escapes the project include root at line 1, column 1\n#include \"../outside.h\"\n^",
        ),
        (
            "#define HEADER \"config.h\"\n#include HEADER\nint main(void) { return 0; }\n",
            "macro-expanded include operands are not supported at line 2, column 10\n#include HEADER\n         ^",
        ),
    ];

    for (source_text, expected) in cases {
        let source = write_temp_source(source_text);
        let output = Command::new(env!("CARGO_BIN_EXE_cust"))
            .arg(&source)
            .output()
            .expect("cust binary should run");
        fs::remove_file(&source).expect("temporary source should be removable");
        assert!(
            !output.status.success(),
            "source should be rejected: {source_text}"
        );
        assert_eq!(String::from_utf8_lossy(&output.stdout), "");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            format!("cust: {expected}\n"),
            "source: {source_text}"
        );
    }

    let directory = temp_source_directory("header-security");
    let source = directory.join("main.c");
    let outside = directory.with_extension("outside.h");
    fs::write(&outside, "#define OUTSIDE 1\n").expect("outside header should be writable");
    fs::write(
        &source,
        format!(
            "#include \"{}\"\nint main(void) {{ return 0; }}\n",
            outside.display()
        ),
    )
    .expect("temporary source should be writable");
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    assert!(!output.status.success(), "absolute header path should fail");
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("must stay below the project include root"),
        "unexpected absolute-path error: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::write(
        &source,
        format!(
            "#include \"../{}\"\nint main(void) {{ return 0; }}\n",
            outside
                .file_name()
                .expect("outside header should have a filename")
                .to_string_lossy()
        ),
    )
    .expect("temporary source should be writable");
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    assert!(
        !output.status.success(),
        "escaping relative header should fail"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("escapes the project include root"),
        "unexpected relative traversal error: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&outside, directory.join("link.h"))
            .expect("outside header symlink should be creatable");
        fs::write(
            &source,
            "#include \"link.h\"\nint main(void) { return 0; }\n",
        )
        .expect("temporary source should be writable");
        let output = Command::new(env!("CARGO_BIN_EXE_cust"))
            .arg(&source)
            .output()
            .expect("cust binary should run");
        assert!(
            !output.status.success(),
            "escaping header symlink should fail"
        );
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("escapes the project include root"),
            "unexpected symlink error: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        std::os::unix::fs::symlink("loop.h", directory.join("loop.h"))
            .expect("header symlink loop should be creatable");
        fs::write(
            &source,
            "#include \"loop.h\"\nint main(void) { return 0; }\n",
        )
        .expect("temporary source should be writable");
        let output = Command::new(env!("CARGO_BIN_EXE_cust"))
            .arg(&source)
            .output()
            .expect("cust binary should run");
        assert!(!output.status.success(), "header symlink loop should fail");
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains("failed to open quoted header 'loop.h'"),
            "symlink-loop I/O error should not be reported as missing: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    fs::remove_dir_all(&directory).expect("temporary source directory should be removable");
    fs::remove_file(&outside).expect("outside header should be removable");
}

#[cfg(target_os = "linux")]
#[test]
fn run_mode_reports_included_header_locations_and_cycles() {
    let directory = temp_source_directory("header-diagnostics");
    let source = directory.join("main.c");
    fs::create_dir(directory.join("left")).expect("nested header directory should be creatable");
    fs::write(directory.join("left/outer.h"), "#include \"bad.h\"\n")
        .expect("temporary outer header should be writable");
    fs::write(directory.join("left/bad.h"), "#define VALUE @\n")
        .expect("temporary nested malformed header should be writable");
    fs::write(
        &source,
        "#include \"left/outer.h\"\nint main(void) { return 0; }\n",
    )
    .expect("temporary source should be writable");
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    assert!(
        !output.status.success(),
        "nested malformed header should fail"
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        concat!(
            "cust: in included header 'left/outer.h': in included header 'left/bad.h': ",
            "unexpected character '@' at line 1, column 15\n",
            "#define VALUE @\n",
            "              ^\n",
        )
    );

    fs::write(directory.join("bad.h"), "#define VALUE @\n")
        .expect("temporary malformed header should be writable");
    fs::write(
        &source,
        "#include \"bad.h\"\nint main(void) { return 0; }\n",
    )
    .expect("temporary source should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    assert!(!output.status.success(), "malformed header should fail");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        concat!(
            "cust: in included header 'bad.h': unexpected character '@' at line 1, column 15\n",
            "#define VALUE @\n",
            "              ^\n",
        )
    );

    fs::write(directory.join("syntax.h"), "int ;\n")
        .expect("temporary syntax-error header should be writable");
    fs::write(
        &source,
        "#include \"syntax.h\"\nint main(void) { return 0; }\n",
    )
    .expect("temporary source should be writable");
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    assert!(!output.status.success(), "header parser error should fail");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        concat!(
            "cust: expected variable name after type, found Semi at line 1, column 5 ",
            "in included header 'syntax.h'\n",
        )
    );

    fs::write(
        directory.join("a.h"),
        "#ifndef A_H\n#define A_H\n#include \"a.h\"\n#define GUARDED_VALUE 6\n#endif\n",
    )
    .expect("temporary guarded recursive header should be writable");
    fs::write(
        &source,
        "#include \"a.h\"\nint main(void) { return GUARDED_VALUE; }\n",
    )
    .expect("temporary source should be writable");
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    assert!(
        output.status.success(),
        "guarded recursive include should terminate, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "6\n");

    fs::write(
        directory.join("control.h"),
        "int illegal_control(void) {\n    break;\n    return 0;\n}\n",
    )
    .expect("temporary illegal-control header should be writable");
    fs::write(
        &source,
        "#include \"control.h\"\nint main(void) { return illegal_control(); }\n",
    )
    .expect("temporary source should be writable");
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    assert!(
        !output.status.success(),
        "illegal header control should fail"
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "cust: break outside loop or switch at line 2, column 5 in included header 'control.h'\n"
    );

    fs::write(
        directory.join("control.h"),
        "int illegal_control(void) {\n    continue;\n    return 0;\n}\n",
    )
    .expect("temporary illegal-control header should be writable");
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    assert!(
        !output.status.success(),
        "illegal header control should fail"
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "cust: continue outside loop at line 2, column 5 in included header 'control.h'\n"
    );

    fs::write(directory.join("a.h"), "#include \"b.h\"\n")
        .expect("temporary cycle header should be writable");
    fs::write(directory.join("b.h"), "#include \"a.h\"\n")
        .expect("temporary cycle header should be writable");
    fs::write(&source, "#include \"a.h\"\nint main(void) { return 0; }\n")
        .expect("temporary source should be writable");
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    fs::remove_dir_all(&directory).expect("temporary source directory should be removable");
    assert!(!output.status.success(), "include cycle should fail");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        concat!(
            "cust: in included header 'a.h': in included header 'b.h': ",
            "in included header 'a.h': in included header 'b.h': ",
            "quoted header include cycle detected for 'a.h' at line 1, column 1\n",
            "#include \"a.h\"\n",
            "^\n",
        )
    );
}

#[cfg(target_os = "linux")]
#[test]
fn run_mode_bounds_include_depth_and_total_source_size() {
    let directory = temp_source_directory("header-bounds");
    let source = directory.join("main.c");
    fs::write(
        &source,
        "#include \"depth-0.h\"\nint main(void) { return 0; }\n",
    )
    .expect("temporary source should be writable");
    for depth in 0..33 {
        let contents = if depth == 32 {
            "#define END 1\n".to_owned()
        } else {
            format!("#include \"depth-{}.h\"\n", depth + 1)
        };
        fs::write(directory.join(format!("depth-{depth}.h")), contents)
            .expect("temporary depth header should be writable");
    }
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    assert!(
        !output.status.success(),
        "excessive include depth should fail"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("quoted header include depth limit of 32 exceeded"),
        "unexpected depth error: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::write(
        &source,
        "#include \"large.h\"\nint main(void) { return 0; }\n",
    )
    .expect("temporary source should be writable");
    let mut oversized_invalid_utf8 = vec![b' '; 1_048_577];
    oversized_invalid_utf8.push(0xff);
    fs::write(directory.join("large.h"), oversized_invalid_utf8)
        .expect("temporary large header should be writable");
    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&source)
        .output()
        .expect("cust binary should run");
    fs::remove_dir_all(&directory).expect("temporary source directory should be removable");
    assert!(
        !output.status.success(),
        "excessive included source should fail"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("included source size limit of 1048576 bytes exceeded"),
        "unexpected source-size error: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(target_os = "linux")]
#[test]
fn run_mode_rejects_outside_non_regular_headers_before_inspection() {
    let directory = temp_source_directory("outside-header-containment");
    let project = directory.join("project");
    fs::create_dir(&project).expect("project directory should be creatable");
    let source = project.join("main.c");
    let fifo = directory.join("outside.h");
    fs::write(
        &source,
        "#include \"../outside.h\"\nint main(void) { return 0; }\n",
    )
    .expect("temporary source should be writable");
    let status = Command::new("mkfifo")
        .arg(&fifo)
        .status()
        .expect("mkfifo should run");
    assert!(status.success(), "mkfifo should create the outside header");

    let output = run_cust_with_deadline(&source);
    fs::remove_dir_all(&directory).expect("temporary source directory should be removable");

    assert!(!output.status.success(), "outside header should fail");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("escapes the project include root"),
        "containment should fail before file-type inspection: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(target_os = "linux")]
#[test]
fn run_mode_rejects_non_regular_headers_without_blocking() {
    let directory = temp_source_directory("header-non-regular");
    let source = directory.join("main.c");
    let fifo = directory.join("blocked.h");
    fs::write(
        &source,
        "#include \"blocked.h\"\nint main(void) { return 0; }\n",
    )
    .expect("temporary source should be writable");
    let status = Command::new("mkfifo")
        .arg(&fifo)
        .status()
        .expect("mkfifo should run");
    assert!(status.success(), "mkfifo should create the test header");

    let output = run_cust_with_deadline(&source);
    fs::remove_dir_all(&directory).expect("temporary source directory should be removable");

    assert!(!output.status.success(), "FIFO header should be rejected");
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("quoted header 'blocked.h' is not a regular file"),
        "unexpected non-regular-header error: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(unix)]
#[test]
fn run_mode_rejects_non_regular_primary_sources_without_blocking() {
    let directory = temp_source_directory("primary-non-regular");
    let fifo = directory.join("blocked.c");
    let status = Command::new("mkfifo")
        .arg(&fifo)
        .status()
        .expect("mkfifo should run");
    assert!(status.success(), "mkfifo should create the primary source");

    let output = run_cust_with_deadline(&fifo);
    assert_eq!(output.status.code(), Some(66));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("source is not a regular file"),
        "unexpected primary FIFO error: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(&directory)
        .output()
        .expect("cust binary should run");
    fs::remove_dir_all(&directory).expect("temporary source directory should be removable");
    assert_eq!(output.status.code(), Some(66));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("source is not a regular file"),
        "directory source should return an I/O error, not panic: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(unix)]
fn run_cust_with_deadline(path: &std::path::Path) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_cust"))
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("cust binary should start");
    let deadline = Instant::now() + Duration::from_secs(2);
    while child
        .try_wait()
        .expect("cust status should be readable")
        .is_none()
    {
        if Instant::now() >= deadline {
            child
                .kill()
                .expect("blocked cust process should be killable");
            child
                .wait()
                .expect("killed cust process should be reapable");
            panic!("Cust blocked while opening a non-regular source");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    child
        .wait_with_output()
        .expect("completed cust output should be readable")
}

fn write_temp_source(source: &str) -> String {
    let path = temp_source_directory("source").with_extension("c");
    fs::write(&path, source).expect("temporary source should be writable");
    path.to_string_lossy().into_owned()
}

fn temp_source_directory(label: &str) -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    let unique_id = TEMP_SOURCE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "cust-cli-{}-{nanos}-{unique_id}-{label}",
        std::process::id()
    ));
    if label != "source" {
        fs::create_dir(&path).expect("temporary source directory should be creatable");
    }
    path
}
