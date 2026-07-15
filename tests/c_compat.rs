use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const COMPAT_FIXTURES: &[&str] = &[
    "tests/fixtures/compat/valid/aggregate_field_declaration_lists.c",
    "tests/fixtures/compat/valid/aggregate_field_typedef_declaration_lists.c",
    "tests/fixtures/compat/valid/aggregate_tag_shadowing.c",
    "tests/fixtures/compat/valid/anonymous_aggregate_typedefs.c",
    "tests/fixtures/compat/valid/anonymous_aggregate_objects.c",
    "tests/fixtures/compat/valid/anonymous_aggregate_const_and_pointers.c",
    "tests/fixtures/compat/valid/anonymous_aggregate_array_pointer_lists.c",
    "tests/fixtures/compat/valid/anonymous_aggregate_for_initializers.c",
    "tests/fixtures/compat/valid/array_type_integer_constant_expressions.c",
    "tests/fixtures/compat/valid/qualified_anonymous_aggregate_for_initializers.c",
    "tests/fixtures/compat/valid/inline_enum_declaration_contexts.c",
    "tests/fixtures/compat/valid/inline_enum_cast_type_definitions.c",
    "tests/fixtures/compat/valid/inline_enum_sizeof_type_definitions.c",
    "tests/fixtures/compat/valid/inline_type_definitions_in_array_lengths.c",
    "tests/fixtures/compat/valid/inline_enum_alignof_type_definitions.c",
    "tests/fixtures/compat/valid/inline_enum_call_argument_type_definitions.c",
    "tests/fixtures/compat/valid/inline_enum_assignment_lvalue_type_definitions.c",
    "tests/fixtures/compat/valid/inline_enum_conditional_type_definitions.c",
    "tests/fixtures/compat/valid/integer_constant_designator_indexes.c",
    "tests/fixtures/compat/valid/inline_enum_control_expr_definitions.c",
    "tests/fixtures/compat/valid/inline_enum_switch_case_labels.c",
    "tests/fixtures/compat/valid/inline_enum_object_declarations.c",
    "tests/fixtures/compat/valid/inline_enum_return_types.c",
    "tests/fixtures/compat/valid/mixed_declaration_contexts.c",
    "tests/fixtures/compat/valid/anonymous_enum_typedefs.c",
    "tests/fixtures/compat/valid/address_of_dereference.c",
    "tests/fixtures/compat/valid/addressable_compound_literals.c",
    "tests/fixtures/compat/valid/arithmetic_control_flow.c",
    "tests/fixtures/compat/valid/array_initializers.c",
    "tests/fixtures/compat/valid/auto_register_storage_class.c",
    "tests/fixtures/compat/valid/array_typedef_aliases.c",
    "tests/fixtures/compat/valid/array_compound_literals.c",
    "tests/fixtures/compat/valid/array_typedef_compound_literals.c",
    "tests/fixtures/compat/valid/alignof_type_names.c",
    "tests/fixtures/compat/valid/alignas_specifiers.c",
    "tests/fixtures/compat/valid/thread_local_storage_class.c",
    "tests/fixtures/compat/valid/extern_function_storage_class.c",
    "tests/fixtures/compat/valid/function_specifiers.c",
    "tests/fixtures/compat/valid/extern_global_declarations.c",
    "tests/fixtures/compat/valid/volatile_type_qualifiers.c",
    "tests/fixtures/compat/valid/atomic_type_qualifiers.c",
    "tests/fixtures/compat/valid/atomic_enum_types.c",
    "tests/fixtures/compat/valid/atomic_inline_enum_type_definitions.c",
    "tests/fixtures/compat/valid/atomic_inline_aggregate_type_definitions.c",
    "tests/fixtures/compat/valid/atomic_anonymous_aggregate_type_definitions.c",
    "tests/fixtures/compat/valid/atomic_anonymous_aggregate_pointer_type_specifiers.c",
    "tests/fixtures/compat/valid/atomic_anonymous_aggregate_pointer_typedef_aliases.c",
    "tests/fixtures/compat/valid/atomic_anonymous_aggregate_value_typedef_aliases.c",
    "tests/fixtures/compat/valid/atomic_anonymous_aggregate_value_alias_derived_declarators.c",
    "tests/fixtures/compat/valid/atomic_anonymous_aggregate_derived_typedef_aliases.c",
    "tests/fixtures/compat/valid/atomic_anonymous_aggregate_const_pointer_views.c",
    "tests/fixtures/compat/valid/atomic_anonymous_aggregate_qualified_value_aliases.c",
    "tests/fixtures/compat/valid/atomic_anonymous_aggregate_qualified_array_aliases.c",
    "tests/fixtures/compat/valid/qualified_named_type_array_alias_parameters.c",
    "tests/fixtures/compat/valid/chained_qualified_array_alias_parameters.c",
    "tests/fixtures/compat/valid/atomic_aggregate_alias_declarations.c",
    "tests/fixtures/compat/valid/restrict_pointer_qualifiers.c",
    "tests/fixtures/compat/valid/designated_initializers.c",
    "tests/fixtures/compat/valid/direct_enum_types.c",
    "tests/fixtures/compat/valid/path_designated_initializers.c",
    "tests/fixtures/compat/valid/aggregate_array_field_path_designators.c",
    "tests/fixtures/compat/valid/unions.c",
    "tests/fixtures/compat/valid/nested_union_fields.c",
    "tests/fixtures/compat/valid/union_pointers.c",
    "tests/fixtures/compat/valid/union_return_functions.c",
    "tests/fixtures/compat/valid/aggregate_initializer_expressions.c",
    "tests/fixtures/compat/valid/aggregate_compound_literals.c",
    "tests/fixtures/compat/valid/aggregate_compound_literal_array_fields.c",
    "tests/fixtures/compat/valid/aggregate_compound_literal_array_field_indexing.c",
    "tests/fixtures/compat/valid/aggregate_compound_literal_array_field_lvalues.c",
    "tests/fixtures/compat/valid/aggregate_compound_literal_aggregate_field_addresses.c",
    "tests/fixtures/compat/valid/aggregate_compound_literal_field_addresses.c",
    "tests/fixtures/compat/valid/aggregate_compound_literal_field_lvalues.c",
    "tests/fixtures/compat/valid/aggregate_compound_literal_pointer_fields.c",
    "tests/fixtures/compat/valid/aggregate_compound_literal_pointer_field_lvalues.c",
    "tests/fixtures/compat/valid/aggregate_conditional_expressions.c",
    "tests/fixtures/compat/valid/aggregate_expr_field_access.c",
    "tests/fixtures/compat/valid/union_expr_field_access.c",
    "tests/fixtures/compat/valid/sizeof_aggregate_expression_fields.c",
    "tests/fixtures/compat/valid/sizeof_aggregate_expression_array_fields.c",
    "tests/fixtures/compat/valid/sizeof_aggregate_conditional_expressions.c",
    "tests/fixtures/compat/valid/sizeof_aggregate_assignment_expressions.c",
    "tests/fixtures/compat/valid/sizeof_aggregate_element_assignment_expressions.c",
    "tests/fixtures/compat/valid/sizeof_embedded_aggregate_array_element_fields.c",
    "tests/fixtures/compat/valid/aggregate_pointer_arithmetic.c",
    "tests/fixtures/compat/valid/aggregate_pointer_indexing.c",
    "tests/fixtures/compat/valid/aggregate_pointer_index_addresses.c",
    "tests/fixtures/compat/valid/aggregate_pointer_expression_index_addresses.c",
    "tests/fixtures/compat/valid/direct_aggregate_pointer_field_index_address.c",
    "tests/fixtures/compat/valid/arrow_aggregate_pointer_field_index_address.c",
    "tests/fixtures/compat/valid/embedded_aggregate_array_pointer_model_routes.c",
    "tests/fixtures/compat/valid/aggregate_pointer_indexed_values.c",
    "tests/fixtures/compat/valid/aggregate_pointer_dereference.c",
    "tests/fixtures/compat/valid/aggregate_pointer_declaration_lists.c",
    "tests/fixtures/compat/valid/aggregate_array_element_assignment.c",
    "tests/fixtures/compat/valid/aggregate_assignment_expressions.c",
    "tests/fixtures/compat/valid/aggregate_array_compound_literals.c",
    "tests/fixtures/compat/valid/anonymous_aggregate_compound_literals.c",
    "tests/fixtures/compat/valid/anonymous_aggregate_pointer_casts.c",
    "tests/fixtures/compat/valid/anonymous_aggregate_fields.c",
    "tests/fixtures/compat/valid/nested_named_aggregate_fields.c",
    "tests/fixtures/compat/valid/inline_enum_aggregate_fields.c",
    "tests/fixtures/compat/valid/aggregate_array_designated_initializers.c",
    "tests/fixtures/compat/valid/aggregate_array_decay_to_pointers.c",
    "tests/fixtures/compat/valid/address_of_struct_fields.c",
    "tests/fixtures/compat/valid/assignment_expressions.c",
    "tests/fixtures/compat/valid/bitwise_compound_assignments.c",
    "tests/fixtures/compat/valid/bitwise_operators.c",
    "tests/fixtures/compat/valid/block_comments.c",
    "tests/fixtures/compat/valid/block_scoped_typedefs.c",
    "tests/fixtures/compat/valid/block_scoped_aggregate_typedef_definitions.c",
    "tests/fixtures/compat/valid/named_aggregate_definition_declarators.c",
    "tests/fixtures/compat/valid/inline_aggregate_return_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_expression_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_control_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_for_clause_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_call_argument_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_static_assert_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_conditional_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_declaration_assignment_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_expression_statement_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_return_expression_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_initializer_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_pointer_initializer_type_definitions.c",
    "tests/fixtures/compat/valid/inline_aggregate_array_compound_literal_type_definitions.c",
    "tests/fixtures/compat/valid/char_array_string_initializers.c",
    "tests/fixtures/compat/valid/struct_char_array_string_initializers.c",
    "tests/fixtures/compat/valid/char_return_functions.c",
    "tests/fixtures/compat/valid/comma_operator.c",
    "tests/fixtures/compat/valid/const_qualifiers.c",
    "tests/fixtures/compat/valid/const_array_typedef_compound_literals.c",
    "tests/fixtures/compat/valid/chained_const_array_typedef_compound_literals.c",
    "tests/fixtures/compat/valid/const_typedef_aliases.c",
    "tests/fixtures/compat/valid/comma_separated_typedef_aliases.c",
    "tests/fixtures/compat/valid/parenthesized_typedef_declarators.c",
    "tests/fixtures/compat/valid/const_pointer_typedef_aliases.c",
    "tests/fixtures/compat/valid/postfix_const_qualifiers.c",
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
    "tests/fixtures/compat/valid/fixed_array_parameters_decay.c",
    "tests/fixtures/compat/valid/functions_arrays_and_strings.c",
    "tests/fixtures/compat/valid/global_variables.c",
    "tests/fixtures/compat/valid/increment_decrement.c",
    "tests/fixtures/compat/valid/inferred_aggregate_array_declarations.c",
    "tests/fixtures/compat/valid/integer_literal_bases.c",
    "tests/fixtures/compat/valid/integer_literal_suffixes.c",
    "tests/fixtures/compat/valid/unnamed_prototype_parameters.c",
    "tests/fixtures/compat/valid/mixed_pointer_string_array_conformance.c",
    "tests/fixtures/compat/valid/nested_struct_fields.c",
    "tests/fixtures/compat/valid/nested_struct_initializers.c",
    "tests/fixtures/compat/valid/numeric_escape_sequences.c",
    "tests/fixtures/compat/valid/pointer_array_elements.c",
    "tests/fixtures/compat/valid/pointer_arithmetic.c",
    "tests/fixtures/compat/valid/pointer_difference_scalar_expressions.c",
    "tests/fixtures/compat/valid/pointer_difference_const_metadata.c",
    "tests/fixtures/compat/valid/pointer_parameters_and_arrays.c",
    "tests/fixtures/compat/valid/pointer_ordering.c",
    "tests/fixtures/compat/valid/pointer_return_functions.c",
    "tests/fixtures/compat/valid/pointer_truthiness_and_equality.c",
    "tests/fixtures/compat/valid/reverse_subscript.c",
    "tests/fixtures/compat/valid/subscript_comma_expressions.c",
    "tests/fixtures/compat/valid/string_literal_element_address.c",
    "tests/fixtures/compat/valid/pointer_typedef_aliases.c",
    "tests/fixtures/compat/valid/pointers_scalars.c",
    "tests/fixtures/compat/valid/single_statement_control_bodies.c",
    "tests/fixtures/compat/valid/scalar_cast_expressions.c",
    "tests/fixtures/compat/valid/pointer_cast_expressions.c",
    "tests/fixtures/compat/valid/scalar_compound_literals.c",
    "tests/fixtures/compat/valid/scalar_compound_literal_lvalues.c",
    "tests/fixtures/compat/valid/scalar_array_field_pointer_decay.c",
    "tests/fixtures/compat/valid/signed_unsigned_int_types.c",
    "tests/fixtures/compat/valid/signed_unsigned_char_types.c",
    "tests/fixtures/compat/valid/long_short_type_spellings.c",
    "tests/fixtures/compat/valid/long_long_type_spellings.c",
    "tests/fixtures/compat/valid/line_comments.c",
    "tests/fixtures/compat/valid/bool_type_spellings.c",
    "tests/fixtures/compat/valid/permuted_scalar_type_specifiers.c",
    "tests/fixtures/compat/valid/parenthesized_function_declarators.c",
    "tests/fixtures/compat/valid/parenthesized_variable_declarators.c",
    "tests/fixtures/compat/valid/sizeof_aggregate_types.c",
    "tests/fixtures/compat/valid/sizeof_array_types.c",
    "tests/fixtures/compat/valid/sizeof_const_types.c",
    "tests/fixtures/compat/valid/sizeof_operator.c",
    "tests/fixtures/compat/valid/sizeof_array_compound_literals.c",
    "tests/fixtures/compat/valid/sizeof_pointer_expressions.c",
    "tests/fixtures/compat/valid/sizeof_comma_expression_types.c",
    "tests/fixtures/compat/valid/static_storage_class.c",
    "tests/fixtures/compat/valid/standard_escape_sequences.c",
    "tests/fixtures/compat/valid/struct_field_pointer_ordering.c",
    "tests/fixtures/compat/valid/struct_field_pointer_equality.c",
    "tests/fixtures/compat/valid/string_literal_concatenation.c",
    "tests/fixtures/compat/valid/static_local_storage.c",
    "tests/fixtures/compat/valid/static_local_unions.c",
    "tests/fixtures/compat/valid/struct_aggregate_array_field_decay.c",
    "tests/fixtures/compat/valid/union_aggregate_array_field_decay.c",
    "tests/fixtures/compat/valid/struct_pointer_union_array_field_decay.c",
    "tests/fixtures/compat/valid/struct_pointer_aggregate_array_field_decay.c",
    "tests/fixtures/compat/valid/nested_aggregate_array_field_decay.c",
    "tests/fixtures/compat/valid/struct_array_field_decay.c",
    "tests/fixtures/compat/valid/struct_pointer_array_field_decay.c",
    "tests/fixtures/compat/valid/nested_struct_array_field_decay.c",
    "tests/fixtures/compat/valid/struct_array_element_aggregate_field_addresses.c",
    "tests/fixtures/compat/valid/struct_array_fields.c",
    "tests/fixtures/compat/valid/struct_arrays.c",
    "tests/fixtures/compat/valid/struct_pointer_field_addresses.c",
    "tests/fixtures/compat/valid/struct_field_element_field_addresses.c",
    "tests/fixtures/compat/valid/struct_field_array_element_field_addresses.c",
    "tests/fixtures/compat/valid/struct_field_array_element_aggregate_field_addresses.c",
    "tests/fixtures/compat/valid/struct_field_element_aggregate_field_addresses.c",
    "tests/fixtures/compat/valid/struct_pointer_fields.c",
    "tests/fixtures/compat/valid/struct_pointer_field_arithmetic.c",
    "tests/fixtures/compat/valid/struct_pointer_arrow_field_arithmetic.c",
    "tests/fixtures/compat/valid/struct_pointer_field_const_pointee.c",
    "tests/fixtures/compat/valid/struct_initializers.c",
    "tests/fixtures/compat/valid/structs.c",
    "tests/fixtures/compat/valid/struct_lvalues_and_copy.c",
    "tests/fixtures/compat/valid/struct_parameters.c",
    "tests/fixtures/compat/valid/struct_pointers.c",
    "tests/fixtures/compat/valid/struct_return_functions.c",
    "tests/fixtures/compat/valid/switch_statements.c",
    "tests/fixtures/compat/valid/switch_enum_case_labels.c",
    "tests/fixtures/compat/valid/typedef_aliases.c",
    "tests/fixtures/compat/valid/typedef_aggregate_definitions.c",
    "tests/fixtures/compat/valid/uninitialized_global_declarations.c",
    "tests/fixtures/compat/valid/unsized_array_parameters.c",
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
