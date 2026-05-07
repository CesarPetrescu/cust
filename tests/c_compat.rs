use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const COMPAT_FIXTURES: &[&str] = &[
    "tests/fixtures/compat/valid/arithmetic_control_flow.c",
    "tests/fixtures/compat/valid/array_initializers.c",
    "tests/fixtures/compat/valid/designated_initializers.c",
    "tests/fixtures/compat/valid/path_designated_initializers.c",
    "tests/fixtures/compat/valid/unions.c",
    "tests/fixtures/compat/valid/nested_union_fields.c",
    "tests/fixtures/compat/valid/union_pointers.c",
    "tests/fixtures/compat/valid/union_return_functions.c",
    "tests/fixtures/compat/valid/aggregate_initializer_expressions.c",
    "tests/fixtures/compat/valid/address_of_struct_fields.c",
    "tests/fixtures/compat/valid/assignment_expressions.c",
    "tests/fixtures/compat/valid/bitwise_compound_assignments.c",
    "tests/fixtures/compat/valid/bitwise_operators.c",
    "tests/fixtures/compat/valid/block_comments.c",
    "tests/fixtures/compat/valid/block_scoped_typedefs.c",
    "tests/fixtures/compat/valid/char_return_functions.c",
    "tests/fixtures/compat/valid/comma_operator.c",
    "tests/fixtures/compat/valid/const_qualifiers.c",
    "tests/fixtures/compat/valid/const_pointer_conversions.c",
    "tests/fixtures/compat/valid/const_pointer_qualifiers.c",
    "tests/fixtures/compat/valid/const_struct_qualifiers.c",
    "tests/fixtures/compat/valid/const_struct_fields.c",
    "tests/fixtures/compat/valid/compound_assignments.c",
    "tests/fixtures/compat/valid/conditional_operator.c",
    "tests/fixtures/compat/valid/do_while_loops.c",
    "tests/fixtures/compat/valid/enum_typedef_aliases.c",
    "tests/fixtures/compat/valid/enums.c",
    "tests/fixtures/compat/valid/function_prototypes.c",
    "tests/fixtures/compat/valid/functions_arrays_and_strings.c",
    "tests/fixtures/compat/valid/global_variables.c",
    "tests/fixtures/compat/valid/increment_decrement.c",
    "tests/fixtures/compat/valid/mixed_pointer_string_array_conformance.c",
    "tests/fixtures/compat/valid/nested_struct_fields.c",
    "tests/fixtures/compat/valid/nested_struct_initializers.c",
    "tests/fixtures/compat/valid/pointer_array_elements.c",
    "tests/fixtures/compat/valid/pointer_arithmetic.c",
    "tests/fixtures/compat/valid/pointer_parameters_and_arrays.c",
    "tests/fixtures/compat/valid/pointer_truthiness_and_equality.c",
    "tests/fixtures/compat/valid/pointer_typedef_aliases.c",
    "tests/fixtures/compat/valid/pointers_scalars.c",
    "tests/fixtures/compat/valid/single_statement_control_bodies.c",
    "tests/fixtures/compat/valid/sizeof_const_types.c",
    "tests/fixtures/compat/valid/sizeof_operator.c",
    "tests/fixtures/compat/valid/static_storage_class.c",
    "tests/fixtures/compat/valid/static_local_storage.c",
    "tests/fixtures/compat/valid/struct_array_fields.c",
    "tests/fixtures/compat/valid/struct_arrays.c",
    "tests/fixtures/compat/valid/struct_pointer_fields.c",
    "tests/fixtures/compat/valid/struct_pointer_field_const_pointee.c",
    "tests/fixtures/compat/valid/struct_initializers.c",
    "tests/fixtures/compat/valid/structs.c",
    "tests/fixtures/compat/valid/struct_lvalues_and_copy.c",
    "tests/fixtures/compat/valid/struct_parameters.c",
    "tests/fixtures/compat/valid/struct_pointers.c",
    "tests/fixtures/compat/valid/struct_return_functions.c",
    "tests/fixtures/compat/valid/switch_statements.c",
    "tests/fixtures/compat/valid/typedef_aliases.c",
    "tests/fixtures/compat/valid/uninitialized_global_declarations.c",
    "tests/fixtures/compat/valid/void_functions.c",
    "tests/fixtures/compat/valid/void_parameter_lists.c",
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
