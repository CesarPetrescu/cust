use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const COMPAT_FIXTURES: &[&str] = &[
    "tests/fixtures/compat/valid/arithmetic_control_flow.c",
    "tests/fixtures/compat/valid/functions_arrays_and_strings.c",
    "tests/fixtures/compat/valid/pointer_parameters_and_arrays.c",
    "tests/fixtures/compat/valid/pointers_scalars.c",
];

#[test]
fn supported_programs_match_c_compiler_exit_codes() {
    let compiler =
        c_compiler().expect("expected gcc, clang, or cc to be available for C compatibility tests");

    for fixture in COMPAT_FIXTURES {
        let source = fs::read_to_string(fixture).expect("compatibility fixture should be readable");
        let cust_result =
            cust::interpret(&source).expect("compatibility fixture should run under Cust");
        let c_result = compile_and_run(&compiler, Path::new(fixture));

        assert_eq!(
            cust_result, c_result,
            "Cust result should match C compiler exit code for {fixture}"
        );
    }
}

fn c_compiler() -> Option<String> {
    env::var("CC").ok().or_else(|| {
        ["gcc", "clang", "cc"]
            .into_iter()
            .find(|candidate| command_exists(candidate))
            .map(str::to_owned)
    })
}

fn command_exists(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn compile_and_run(compiler: &str, fixture: &Path) -> i64 {
    let executable = temp_executable_path(fixture);
    let compile_status = Command::new(compiler)
        .arg("-std=c11")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Werror")
        .arg(fixture)
        .arg("-o")
        .arg(&executable)
        .status()
        .expect("failed to run C compiler");

    assert!(
        compile_status.success(),
        "C compiler failed for {}",
        fixture.display()
    );

    let run_status = Command::new(&executable)
        .status()
        .expect("failed to run compiled C fixture");

    fs::remove_file(&executable).expect("temporary C compatibility executable should be removable");

    i64::from(
        run_status
            .code()
            .expect("compiled C fixture should exit normally"),
    )
}

fn temp_executable_path(fixture: &Path) -> PathBuf {
    let mut path = env::temp_dir();
    let name = fixture
        .file_stem()
        .expect("fixture should have a file stem")
        .to_string_lossy();
    path.push(format!("cust-c-compat-{}-{name}", std::process::id()));
    path
}
