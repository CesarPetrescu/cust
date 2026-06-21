use cust::interpret;

#[test]
fn runs_integer_arithmetic_and_return() {
    let program = r#"
        int main() {
            return 2 + 3 * 4;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_variables_assignment_and_while_loop() {
    let program = r#"
        int main() {
            int i = 0;
            int sum = 0;
            while (i < 5) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 10);
}

#[test]
fn supports_if_else_and_comparisons() {
    let program = r#"
        int main() {
            int x = 7;
            if (x >= 7) {
                return 1;
            } else {
                return 0;
            }
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 1);
}

#[test]
fn reports_division_by_zero() {
    let program = "int main() { return 10 / 0; }";

    let err = interpret(program).unwrap_err();
    assert!(err.to_string().contains("division by zero"));
}

#[test]
fn reports_source_context_for_unexpected_character() {
    let program = "int main() {\nreturn 1;\n@\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "unexpected character '@' at line 3, column 1\n@\n^"
    );
}

#[test]
fn reports_source_context_for_out_of_range_integer_literal() {
    let program = "int main() {\nreturn 999999999999999999999999999999;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "integer literal out of range at line 2, column 8\nreturn 999999999999999999999999999999;\n       ^"
    );
}

#[test]
fn rejects_preprocessor_directives_with_context() {
    let program = include_str!("fixtures/invalid/preprocessor_directive.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "preprocessor directives are not supported at line 1, column 1\n#include <stdio.h>\n^"
    );
}

#[test]
fn rejects_aggregate_forward_declarations_with_context() {
    let program = include_str!("fixtures/invalid/struct_forward_declaration.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "forward struct declarations are not supported at line 1, column 13"
    );

    let program = include_str!("fixtures/invalid/union_forward_declaration.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "forward union declarations are not supported at line 1, column 13"
    );
}

#[test]
fn rejects_aggregate_bit_fields_with_context() {
    let program = include_str!("fixtures/invalid/aggregate_bit_fields.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "bit-field aggregate fields are not supported at line 2, column 20"
    );

    let program = include_str!("fixtures/invalid/union_bit_fields.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "bit-field aggregate fields are not supported at line 2, column 15"
    );
}

#[test]
fn rejects_generic_selections_with_context() {
    let program = include_str!("fixtures/invalid/generic_selection.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "generic selections are not supported at line 2, column 12"
    );
}

#[test]
fn supports_hexadecimal_and_octal_integer_literals() {
    let program = include_str!("fixtures/valid/integer_literal_bases.c");

    assert_eq!(interpret(program).unwrap(), 106);
}

#[test]
fn supports_c_integer_literal_suffixes() {
    let program = include_str!("fixtures/valid/integer_literal_suffixes.c");

    assert_eq!(interpret(program).unwrap(), 122);
}

#[test]
fn supports_signed_unsigned_int_type_spellings() {
    let program = include_str!("fixtures/valid/signed_unsigned_int_types.c");

    assert_eq!(interpret(program).unwrap(), 22);
}

#[test]
fn supports_signed_unsigned_char_type_spellings() {
    let program = include_str!("fixtures/valid/signed_unsigned_char_types.c");

    assert_eq!(interpret(program).unwrap(), 23);
}

#[test]
fn supports_long_short_type_spellings() {
    let program = include_str!("fixtures/valid/long_short_type_spellings.c");

    assert_eq!(interpret(program).unwrap(), 47);
}

#[test]
fn supports_long_long_type_spellings() {
    let program = include_str!("fixtures/valid/long_long_type_spellings.c");

    assert_eq!(interpret(program).unwrap(), 53);
}

#[test]
fn supports_bool_type_spellings() {
    let program = include_str!("fixtures/valid/bool_type_spellings.c");

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn supports_permuted_scalar_type_specifiers() {
    let program = include_str!("fixtures/valid/permuted_scalar_type_specifiers.c");

    assert_eq!(interpret(program).unwrap(), 33);
}

#[test]
fn supports_auto_and_register_local_storage_class_specifiers() {
    let program = include_str!("fixtures/valid/auto_register_storage_class.c");

    assert_eq!(interpret(program).unwrap(), 23);
}

#[test]
fn supports_array_typedef_aliases() {
    let program = include_str!("fixtures/valid/array_typedef_aliases.c");

    assert_eq!(interpret(program).unwrap(), 52);
}

#[test]
fn supports_extern_function_storage_class_specifiers() {
    let program = include_str!("fixtures/valid/extern_function_storage_class.c");

    assert_eq!(interpret(program).unwrap(), 10);
}

#[test]
fn supports_extern_global_declarations() {
    let program = include_str!("fixtures/valid/extern_global_declarations.c");

    assert_eq!(interpret(program).unwrap(), 24);
}

#[test]
fn supports_extern_local_declarations() {
    let program = include_str!("fixtures/valid/extern_local_declarations.c");

    assert_eq!(interpret(program).unwrap(), 28);
}

#[test]
fn supports_local_function_prototypes() {
    let program = include_str!("fixtures/valid/local_function_prototypes.c");

    assert_eq!(interpret(program).unwrap(), 68);
}

#[test]
fn supports_inferred_array_declarations() {
    let program = include_str!("fixtures/valid/inferred_array_declarations.c");

    assert_eq!(interpret(program).unwrap(), 139);
}

#[test]
fn supports_inferred_aggregate_array_declarations() {
    let program = include_str!("fixtures/valid/inferred_aggregate_array_declarations.c");

    assert_eq!(interpret(program).unwrap(), 66);
}

#[test]
fn supports_comma_separated_scalar_declarations() {
    let program = include_str!("fixtures/valid/comma_separated_scalar_declarations.c");

    assert_eq!(interpret(program).unwrap(), 52);
}

#[test]
fn supports_comma_separated_pointer_array_and_aggregate_declarations() {
    let program = include_str!("fixtures/valid/comma_separated_mixed_declarations.c");

    assert_eq!(interpret(program).unwrap(), 125);
}

#[test]
fn supports_aggregate_pointer_declaration_lists() {
    let program = include_str!("fixtures/valid/aggregate_pointer_declaration_lists.c");

    assert_eq!(interpret(program).unwrap(), 62);
}

#[test]
fn rejects_initialized_extern_local_declarations() {
    let program = include_str!("fixtures/invalid/extern_local_initializer.c");

    let err = interpret(program).unwrap_err();

    assert!(
        err.to_string()
            .contains("extern local declarations cannot have initializers")
    );
}

#[test]
fn supports_volatile_type_qualifiers() {
    let program = include_str!("fixtures/valid/volatile_type_qualifiers.c");

    assert_eq!(interpret(program).unwrap(), 74);
}

#[test]
fn supports_atomic_type_qualifiers() {
    let program = include_str!("fixtures/valid/atomic_type_qualifiers.c");

    assert_eq!(interpret(program).unwrap(), 52);
}

#[test]
fn supports_restrict_pointer_qualifiers() {
    let program = include_str!("fixtures/valid/restrict_pointer_qualifiers.c");

    assert_eq!(interpret(program).unwrap(), 28);
}

#[test]
fn supports_function_specifiers() {
    let program = include_str!("fixtures/valid/function_specifiers.c");

    assert_eq!(interpret(program).unwrap(), 11);
}

#[test]
fn supports_const_typedef_aliases() {
    let program = include_str!("fixtures/valid/const_typedef_aliases.c");

    assert_eq!(interpret(program).unwrap(), 35);
}

#[test]
fn supports_comma_separated_typedef_aliases() {
    let program = include_str!("fixtures/valid/comma_separated_typedef_aliases.c");

    assert_eq!(interpret(program).unwrap(), 81);
}

#[test]
fn supports_const_pointer_typedef_aliases() {
    let program = include_str!("fixtures/valid/const_pointer_typedef_aliases.c");

    assert_eq!(interpret(program).unwrap(), 56);
}

#[test]
fn supports_postfix_const_qualifiers() {
    let program = include_str!("fixtures/valid/postfix_const_qualifiers.c");

    assert_eq!(interpret(program).unwrap(), 75);
}

#[test]
fn supports_static_assertions() {
    let program = include_str!("fixtures/valid/static_assertions.c");

    assert_eq!(interpret(program).unwrap(), 17);
}

#[test]
fn supports_alignof_type_names() {
    let program = include_str!("fixtures/valid/alignof_type_names.c");

    assert_eq!(interpret(program).unwrap(), 51);
}

#[test]
fn supports_alignas_specifiers() {
    let program = include_str!("fixtures/valid/alignas_specifiers.c");

    assert_eq!(interpret(program).unwrap(), 16);
}

#[test]
fn supports_thread_local_storage_class_specifiers() {
    let program = include_str!("fixtures/valid/thread_local_storage_class.c");

    assert_eq!(interpret(program).unwrap(), 17);
}

#[test]
fn rejects_alignof_void() {
    let program = include_str!("fixtures/invalid/alignof_void.c");

    let err = interpret(program).unwrap_err();
    assert!(
        err.to_string().contains("_Alignof(void) is not supported"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_failing_static_assertions() {
    let program = include_str!("fixtures/invalid/static_assertion_failure.c");

    let err = interpret(program).unwrap_err();
    assert!(
        err.to_string()
            .contains("static assertion failed: zero is false"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_assignment_to_const_typedef_alias_variables() {
    let program = include_str!("fixtures/invalid/const_typedef_alias_assignment.c");

    let err = interpret(program).unwrap_err();
    assert!(
        err.to_string()
            .contains("cannot assign to const variable 'value'"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_const_pointer_typedef_alias_const_discard() {
    let program = include_str!("fixtures/invalid/const_pointer_typedef_alias_const_discard.c");

    let err = interpret(program).unwrap_err();
    assert!(
        err.to_string()
            .contains("cannot discard const qualifier from pointer target"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_const_pointer_typedef_alias_slot_assignment() {
    let program = include_str!("fixtures/invalid/const_pointer_typedef_alias_slot_assignment.c");

    let err = interpret(program).unwrap_err();
    assert!(
        err.to_string()
            .contains("cannot assign to const variable 'slot'"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_const_pointer_slots_from_comma_separated_typedef_aliases() {
    let program = include_str!("fixtures/invalid/comma_typedef_const_pointer_slot_assignment.c");

    let err = interpret(program).unwrap_err();
    assert!(
        err.to_string()
            .contains("cannot assign to const variable 'slot'"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_pointer_array_typedef_aliases() {
    let program = include_str!("fixtures/invalid/pointer_array_typedef_alias.c");

    let err = interpret(program).unwrap_err();
    assert!(
        err.to_string()
            .contains("pointer array typedef aliases are not supported"),
        "unexpected error: {err}"
    );
}

#[test]
fn supports_standard_simple_escape_sequences() {
    let program = include_str!("fixtures/valid/standard_escape_sequences.c");

    assert_eq!(interpret(program).unwrap(), 228);
}

#[test]
fn supports_adjacent_string_literal_concatenation() {
    let program = include_str!("fixtures/valid/string_literal_concatenation.c");

    assert_eq!(interpret(program).unwrap(), 23);
}

#[test]
fn supports_static_local_unions() {
    let program = include_str!("fixtures/valid/static_local_unions.c");

    assert_eq!(interpret(program).unwrap(), 21);
}

#[test]
fn supports_octal_and_hex_escape_sequences() {
    let program = include_str!("fixtures/valid/numeric_escape_sequences.c");

    assert_eq!(interpret(program).unwrap(), 124);
}

#[test]
fn supports_char_arrays_initialized_from_string_literals() {
    let program = include_str!("fixtures/valid/char_array_string_initializers.c");

    assert_eq!(interpret(program).unwrap(), 8);
}

#[test]
fn supports_address_of_dereference_as_pointer_identity() {
    let program = include_str!("fixtures/valid/address_of_dereference.c");

    assert_eq!(interpret(program).unwrap(), 31);
}

#[test]
fn supports_c_style_reverse_subscript_expressions() {
    let program = include_str!("fixtures/valid/reverse_subscript.c");

    assert_eq!(interpret(program).unwrap(), 29);
}

#[test]
fn supports_pointer_ordering_within_same_array_storage() {
    let program = include_str!("fixtures/valid/pointer_ordering.c");

    assert_eq!(interpret(program).unwrap(), 26);
}

#[test]
fn supports_pointer_ordering_within_embedded_aggregate_array_fields() {
    let program = include_str!("fixtures/valid/struct_field_pointer_ordering.c");

    assert_eq!(interpret(program).unwrap(), 21);
}

#[test]
fn supports_pointer_equality_within_embedded_aggregate_array_fields() {
    let program = include_str!("fixtures/valid/struct_field_pointer_equality.c");

    assert_eq!(interpret(program).unwrap(), 31);
}

#[test]
fn supports_pointer_equality_for_fields_reached_through_embedded_aggregate_array_pointers() {
    let program = include_str!("fixtures/valid/struct_field_element_field_pointer_equality.c");

    assert_eq!(interpret(program).unwrap(), 97);
}

#[test]
fn rejects_pointer_ordering_between_different_embedded_aggregate_array_fields() {
    let program = include_str!("fixtures/invalid/struct_field_pointer_ordering_different_fields.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot compare pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_ordering_between_different_arrays() {
    let program = include_str!("fixtures/invalid/pointer_ordering_different_arrays.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot compare pointers to different arrays"
    );
}

#[test]
fn supports_struct_char_array_fields_initialized_from_string_literals() {
    let program = include_str!("fixtures/valid/struct_char_array_string_initializers.c");

    assert_eq!(interpret(program).unwrap(), 11);
}

#[test]
fn supports_struct_aggregate_array_fields() {
    let program = include_str!("fixtures/valid/struct_aggregate_array_fields.c");

    assert_eq!(interpret(program).unwrap(), 25);
}

#[test]
fn supports_struct_aggregate_array_field_decay_and_address_of() {
    let program = include_str!("fixtures/valid/struct_aggregate_array_field_decay.c");

    assert_eq!(interpret(program).unwrap(), 71);
}

#[test]
fn supports_union_aggregate_array_field_decay_and_address_of() {
    let program = include_str!("fixtures/valid/union_aggregate_array_field_decay.c");

    assert_eq!(interpret(program).unwrap(), 66);
}

#[test]
fn supports_struct_pointer_union_array_field_decay_and_address_of() {
    let program = include_str!("fixtures/valid/struct_pointer_union_array_field_decay.c");

    assert_eq!(interpret(program).unwrap(), 42);
}

#[test]
fn reports_array_compound_literal_sizes_without_evaluating_initializers() {
    let program = include_str!("fixtures/valid/sizeof_array_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 54);
}

#[test]
fn rejects_struct_aggregate_array_field_decay_that_discards_const() {
    let program = include_str!("fixtures/invalid/struct_aggregate_array_field_const_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_union_array_field_decay_that_discards_const() {
    let program = include_str!("fixtures/invalid/union_array_field_const_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn supports_struct_pointer_aggregate_array_field_decay_and_address_of() {
    let program = include_str!("fixtures/valid/struct_pointer_aggregate_array_field_decay.c");

    assert_eq!(interpret(program).unwrap(), 85);
}

#[test]
fn rejects_struct_pointer_aggregate_array_field_decay_that_discards_const() {
    let program =
        include_str!("fixtures/invalid/struct_pointer_aggregate_array_field_const_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn supports_nested_aggregate_array_field_decay_and_address_of() {
    let program = include_str!("fixtures/valid/nested_aggregate_array_field_decay.c");

    assert_eq!(interpret(program).unwrap(), 81);
}

#[test]
fn rejects_nested_aggregate_array_field_decay_that_discards_const() {
    let program = include_str!("fixtures/invalid/nested_aggregate_array_field_const_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn reports_char_array_string_initializer_too_long() {
    let program = include_str!("fixtures/invalid/char_array_string_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "initializer string for char array 'short_text' is too long"
    );
}

#[test]
fn reports_struct_char_array_string_initializer_too_long() {
    let program = include_str!("fixtures/invalid/struct_char_array_string_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "initializer string for char array 'text' is too long"
    );
}

#[test]
fn reports_hex_escape_sequences_without_digits() {
    let program = include_str!("fixtures/invalid/hex_escape_without_digits.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "hex escape sequence requires at least one digit at line 2, column 12\n    return '\\x';\n           ^"
    );
}

#[test]
fn reports_invalid_octal_integer_digits() {
    let program = include_str!("fixtures/invalid/invalid_octal_integer_literal.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "invalid digit '8' in octal integer literal at line 2, column 12\n    return 08;\n           ^"
    );
}

#[test]
fn supports_c_style_block_comments_as_whitespace() {
    let program = include_str!("fixtures/valid/block_comments.c");

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_global_scalar_array_and_pointer_variables() {
    let program = include_str!("fixtures/valid/global_variables.c");

    assert_eq!(interpret(program).unwrap(), 22);
}

#[test]
fn supports_sizeof_operator_for_types_scalars_arrays_strings_and_pointers() {
    let program = include_str!("fixtures/valid/sizeof_operator.c");

    assert_eq!(interpret(program).unwrap(), 85);
}

#[test]
fn supports_sizeof_const_qualified_types() {
    let program = include_str!("fixtures/valid/sizeof_const_types.c");

    assert_eq!(interpret(program).unwrap(), 58);
}

#[test]
fn supports_sizeof_aggregate_type_names() {
    let program = include_str!("fixtures/valid/sizeof_aggregate_types.c");

    assert_eq!(interpret(program).unwrap(), 42);
}

#[test]
fn supports_sizeof_array_type_names() {
    let program = include_str!("fixtures/valid/sizeof_array_types.c");

    assert_eq!(interpret(program).unwrap(), 95);
}

#[test]
fn supports_sizeof_pointer_expressions_without_evaluating_operands() {
    let program = include_str!("fixtures/valid/sizeof_pointer_expressions.c");

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_uninitialized_scalar_and_pointer_declarations() {
    let program = include_str!("fixtures/valid/uninitialized_declarations.c");

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn supports_scalar_cast_expressions() {
    let program = include_str!("fixtures/valid/scalar_cast_expressions.c");

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_pointer_cast_expressions() {
    let program = include_str!("fixtures/valid/pointer_cast_expressions.c");

    assert_eq!(interpret(program).unwrap(), 29);
}

#[test]
fn rejects_pointer_casts_that_discard_const_pointees() {
    let program = include_str!("fixtures/invalid/pointer_cast_const_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn supports_void_cast_expressions() {
    let program = include_str!("fixtures/valid/void_cast_expressions.c");

    assert_eq!(interpret(program).unwrap(), 15);
}

#[test]
fn rejects_void_cast_expressions_used_as_scalar() {
    let program = include_str!("fixtures/invalid/void_cast_used_as_scalar.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "void expression used as scalar");
}

#[test]
fn supports_scalar_compound_literals_in_expression_contexts() {
    let program = include_str!("fixtures/valid/scalar_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 45);
}

#[test]
fn supports_scalar_compound_literals_as_modifiable_lvalues() {
    let program = include_str!("fixtures/valid/scalar_compound_literal_lvalues.c");

    assert_eq!(interpret(program).unwrap(), 36);
}

#[test]
fn supports_array_compound_literals_as_pointer_expressions() {
    let program = include_str!("fixtures/valid/array_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 40);
}

#[test]
fn supports_aggregate_array_compound_literals_as_pointer_expressions() {
    let program = include_str!("fixtures/valid/aggregate_array_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 48);
}

#[test]
fn supports_addressable_scalar_and_aggregate_compound_literals() {
    let program = include_str!("fixtures/valid/addressable_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 61);
}

#[test]
fn supports_addresses_of_aggregate_compound_literal_scalar_fields() {
    let program = include_str!("fixtures/valid/aggregate_compound_literal_field_addresses.c");

    assert_eq!(interpret(program).unwrap(), 33);
}

#[test]
fn supports_addresses_of_aggregate_compound_literal_aggregate_fields() {
    let program =
        include_str!("fixtures/valid/aggregate_compound_literal_aggregate_field_addresses.c");

    assert_eq!(interpret(program).unwrap(), 51);
}

#[test]
fn supports_addresses_of_struct_pointer_scalar_fields() {
    let program = include_str!("fixtures/valid/struct_pointer_field_addresses.c");

    assert_eq!(interpret(program).unwrap(), 48);
}

#[test]
fn supports_addresses_of_fields_through_embedded_aggregate_array_pointers() {
    let program = include_str!("fixtures/valid/struct_field_element_field_addresses.c");

    assert_eq!(interpret(program).unwrap(), 110);
}

#[test]
fn supports_direct_addresses_of_embedded_aggregate_array_element_fields() {
    let program = include_str!("fixtures/valid/struct_field_array_element_field_addresses.c");

    assert_eq!(interpret(program).unwrap(), 221);
}

#[test]
fn supports_addresses_of_struct_array_element_aggregate_fields() {
    let program = include_str!("fixtures/valid/struct_array_element_aggregate_field_addresses.c");

    assert_eq!(interpret(program).unwrap(), 64);
}

#[test]
fn supports_addresses_of_aggregate_fields_through_embedded_aggregate_array_pointers() {
    let program = include_str!("fixtures/valid/struct_field_element_aggregate_field_addresses.c");

    assert_eq!(interpret(program).unwrap(), 187);
}

#[test]
fn supports_direct_addresses_of_embedded_aggregate_array_element_aggregate_fields() {
    let program =
        include_str!("fixtures/valid/struct_field_array_element_aggregate_field_addresses.c");

    assert_eq!(interpret(program).unwrap(), 100);
}

#[test]
fn rejects_aggregate_array_compound_literals_longer_than_declared_length() {
    let program =
        include_str!("fixtures/invalid/aggregate_array_compound_literal_too_many_initializers.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "too many initializers for aggregate array compound literal"
    );
}

#[test]
fn rejects_scalar_compound_literals_with_too_many_initializers() {
    let program = include_str!("fixtures/invalid/scalar_compound_literal_too_many_initializers.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "too many initializers for scalar compound literal"
    );
}

#[test]
fn rejects_array_compound_literals_longer_than_declared_length() {
    let program = include_str!("fixtures/invalid/array_compound_literal_too_many_initializers.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "too many initializers for array compound literal"
    );
}

#[test]
fn rejects_array_compound_literal_string_initializers_that_are_too_long() {
    let program = include_str!("fixtures/invalid/array_compound_literal_string_too_long.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "initializer string for char array compound literal is too long"
    );
}

#[test]
fn supports_direct_enum_type_declarations_parameters_returns_and_sizeof() {
    let program = include_str!("fixtures/valid/direct_enum_types.c");

    assert_eq!(interpret(program).unwrap(), 24);
}

#[test]
fn rejects_aggregate_cast_expressions() {
    let program = include_str!("fixtures/invalid/aggregate_cast_unsupported.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "aggregate casts are not supported at line 5, column 13"
    );
}

#[test]
fn supports_scalar_array_initializers() {
    let program = include_str!("fixtures/valid/array_initializers.c");

    assert_eq!(interpret(program).unwrap(), 68);
}

#[test]
fn supports_braced_scalar_initializers_in_declarations_and_aggregates() {
    let program = include_str!("fixtures/valid/braced_scalar_initializers.c");

    assert_eq!(interpret(program).unwrap(), 70);
}

#[test]
fn supports_designated_array_and_struct_initializers() {
    let program = include_str!("fixtures/valid/designated_initializers.c");

    assert_eq!(interpret(program).unwrap(), 133);
}

#[test]
fn supports_anonymous_aggregate_object_declarations() {
    let program = include_str!("fixtures/valid/anonymous_aggregate_objects.c");

    assert_eq!(interpret(program).unwrap(), 42);
}

#[test]
fn supports_const_and_pointer_anonymous_aggregate_declaration_lists() {
    let program = include_str!("fixtures/valid/anonymous_aggregate_const_and_pointers.c");

    assert_eq!(interpret(program).unwrap(), 26);
}

#[test]
fn supports_anonymous_aggregate_array_pointer_declaration_lists() {
    let program = include_str!("fixtures/valid/anonymous_aggregate_array_pointer_lists.c");

    assert_eq!(interpret(program).unwrap(), 31);
}

#[test]
fn rejects_assignment_to_const_anonymous_aggregate_fields() {
    let program = include_str!("fixtures/invalid/const_anonymous_aggregate_field_assignment.c");

    let err = interpret(program).unwrap_err();
    assert!(
        err.to_string()
            .contains("cannot assign to const variable 'point'"),
        "unexpected error: {err}"
    );
}

#[test]
fn supports_path_designated_struct_initializers() {
    let program = include_str!("fixtures/valid/path_designated_initializers.c");

    assert_eq!(interpret(program).unwrap(), 156);
}

#[test]
fn supports_scalar_union_variables_fields_initializers_and_sizeof() {
    let program = include_str!("fixtures/valid/unions.c");

    assert_eq!(interpret(program).unwrap(), 103);
}

#[test]
fn supports_nested_union_fields_initializers_copy_and_parameters() {
    let program = include_str!("fixtures/valid/nested_union_fields.c");

    assert_eq!(interpret(program).unwrap(), 68);
}

#[test]
fn supports_union_pointers_and_pointer_fields() {
    let program = include_str!("fixtures/valid/union_pointers.c");

    assert_eq!(interpret(program).unwrap(), 61);
}

#[test]
fn supports_union_return_functions_and_prototypes() {
    let program = include_str!("fixtures/valid/union_return_functions.c");

    assert_eq!(interpret(program).unwrap(), 26);
}

#[test]
fn supports_aggregate_initializer_expressions_from_returned_structs_and_unions() {
    let program = include_str!("fixtures/valid/aggregate_initializer_expressions.c");

    assert_eq!(interpret(program).unwrap(), 47);
}

#[test]
fn supports_aggregate_compound_literals_in_expression_contexts() {
    let program = include_str!("fixtures/valid/aggregate_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 55);
}

#[test]
fn supports_aggregate_compound_literal_field_lvalues() {
    let program = include_str!("fixtures/valid/aggregate_compound_literal_field_lvalues.c");

    assert_eq!(interpret(program).unwrap(), 46);
}

#[test]
fn supports_pointer_fields_on_aggregate_compound_literals() {
    let program = include_str!("fixtures/valid/aggregate_compound_literal_pointer_fields.c");

    assert_eq!(interpret(program).unwrap(), 12);
}

#[test]
fn supports_pointer_field_lvalues_on_aggregate_compound_literals() {
    let program = include_str!("fixtures/valid/aggregate_compound_literal_pointer_field_lvalues.c");

    assert_eq!(interpret(program).unwrap(), 24);
}

#[test]
fn supports_array_fields_on_aggregate_compound_literals() {
    let program = include_str!("fixtures/valid/aggregate_compound_literal_array_fields.c");

    assert_eq!(interpret(program).unwrap(), 21);
}

#[test]
fn supports_direct_indexing_and_address_of_array_fields_on_aggregate_compound_literals() {
    let program = include_str!("fixtures/valid/aggregate_compound_literal_array_field_indexing.c");

    assert_eq!(interpret(program).unwrap(), 42);
}

#[test]
fn supports_lvalue_writes_to_array_fields_on_aggregate_compound_literals() {
    let program = include_str!("fixtures/valid/aggregate_compound_literal_array_field_lvalues.c");

    assert_eq!(interpret(program).unwrap(), 39);
}

#[test]
fn supports_lvalue_writes_to_aggregate_array_fields_on_aggregate_compound_literals() {
    let program =
        include_str!("fixtures/valid/aggregate_compound_literal_aggregate_array_field_lvalues.c");

    assert_eq!(interpret(program).unwrap(), 55);
}

#[test]
fn supports_field_access_on_aggregate_valued_expressions() {
    let program = include_str!("fixtures/valid/aggregate_expr_field_access.c");

    assert_eq!(interpret(program).unwrap(), 31);
}

#[test]
fn supports_field_access_on_union_valued_expressions() {
    let program = include_str!("fixtures/valid/union_expr_field_access.c");

    assert_eq!(interpret(program).unwrap(), 23);
}

#[test]
fn supports_sizeof_fields_on_aggregate_valued_expressions() {
    let program = include_str!("fixtures/valid/sizeof_aggregate_expression_fields.c");

    assert_eq!(interpret(program).unwrap(), 28);
}

#[test]
fn supports_sizeof_array_fields_on_aggregate_valued_expressions() {
    let program = include_str!("fixtures/valid/sizeof_aggregate_expression_array_fields.c");

    assert_eq!(interpret(program).unwrap(), 93);
}

#[test]
fn rejects_union_function_used_as_scalar_expression() {
    let program = include_str!("fixtures/invalid/union_function_used_as_scalar.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "union function 'make_number' used as scalar expression"
    );
}

#[test]
fn rejects_pointer_function_used_as_scalar_expression() {
    let program = include_str!("fixtures/invalid/pointer_function_used_as_scalar.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "pointer function 'choose' used as scalar expression"
    );
}

#[test]
fn rejects_union_values_used_as_scalar_expressions() {
    let program = include_str!("fixtures/invalid/union_value_used_as_scalar.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "union value used as scalar");
}

#[test]
fn rejects_assignment_to_const_fields_on_aggregate_compound_literals() {
    let program =
        include_str!("fixtures/invalid/aggregate_compound_literal_const_field_assignment.c",);

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'magic'"
    );
}

#[test]
fn rejects_const_discard_from_pointer_fields_on_aggregate_compound_literals() {
    let program =
        include_str!("fixtures/invalid/aggregate_compound_literal_pointer_field_const_discard.c",);

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_assignment_to_const_pointer_fields_on_aggregate_compound_literals() {
    let program = include_str!(
        "fixtures/invalid/aggregate_compound_literal_const_pointer_field_assignment.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign to const struct field 'p'");
}

#[test]
fn rejects_const_discard_from_array_fields_on_aggregate_compound_literals() {
    let program =
        include_str!("fixtures/invalid/aggregate_compound_literal_array_field_const_discard.c",);

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_const_discard_from_array_field_elements_on_aggregate_compound_literals() {
    let program = include_str!(
        "fixtures/invalid/aggregate_compound_literal_array_field_element_const_discard.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn supports_conditional_and_comma_expressions_for_aggregates() {
    let program = include_str!("fixtures/valid/aggregate_conditional_expressions.c");

    assert_eq!(interpret(program).unwrap(), 63);
}

#[test]
fn supports_pointer_arithmetic_for_struct_and_union_arrays() {
    let program = include_str!("fixtures/valid/aggregate_pointer_arithmetic.c");

    assert_eq!(interpret(program).unwrap(), 53);
}

#[test]
fn supports_pointer_indexing_for_struct_and_union_arrays() {
    let program = include_str!("fixtures/valid/aggregate_pointer_indexing.c");

    assert_eq!(interpret(program).unwrap(), 87);
}

#[test]
fn supports_pointer_indexed_aggregate_values_as_copies() {
    let program = include_str!("fixtures/valid/aggregate_pointer_indexed_values.c");

    assert_eq!(interpret(program).unwrap(), 79);
}

#[test]
fn supports_designated_initializers_for_aggregate_arrays() {
    let program = include_str!("fixtures/valid/aggregate_array_designated_initializers.c");

    assert_eq!(interpret(program).unwrap(), 40);
}

#[test]
fn supports_aggregate_array_decay_to_pointer_parameters() {
    let program = include_str!("fixtures/valid/aggregate_array_decay_to_pointers.c");

    assert_eq!(interpret(program).unwrap(), 55);
}

#[test]
fn supports_aggregate_array_element_copy_assignment() {
    let program = include_str!("fixtures/valid/aggregate_array_element_assignment.c");

    assert_eq!(interpret(program).unwrap(), 49);
}

#[test]
fn supports_embedded_aggregate_array_element_copy_assignment() {
    let program = include_str!("fixtures/valid/embedded_aggregate_array_element_assignment.c");

    assert_eq!(interpret(program).unwrap(), 72);
}

#[test]
fn rejects_embedded_aggregate_array_element_assignment_type_mismatch() {
    let program = include_str!(
        "fixtures/invalid/embedded_aggregate_array_element_assignment_type_mismatch.c"
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign struct 'Size' to struct 'Point'"
    );
}

#[test]
fn supports_aggregate_pointer_dereference_values_and_assignment() {
    let program = include_str!("fixtures/valid/aggregate_pointer_dereference.c");

    assert_eq!(interpret(program).unwrap(), 165);
}

#[test]
fn supports_aggregate_assignment_expressions_returning_copies() {
    let program = include_str!("fixtures/valid/aggregate_assignment_expressions.c");

    assert_eq!(interpret(program).unwrap(), 66);
}

#[test]
fn rejects_aggregate_assignment_expression_type_mismatches() {
    let program = include_str!("fixtures/invalid/aggregate_assignment_expression_type_mismatch.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign struct 'Size' to struct 'Point'"
    );
}

#[test]
fn rejects_aggregate_pointer_dereference_assignment_through_const_views() {
    let program = include_str!("fixtures/invalid/const_aggregate_pointer_deref_assignment.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign through pointer to const");
}

#[test]
fn supports_unsized_array_parameters_as_pointer_parameters() {
    let program = include_str!("fixtures/valid/unsized_array_parameters.c");

    assert_eq!(interpret(program).unwrap(), 109);
}

#[test]
fn supports_fixed_array_parameters_as_pointer_parameters() {
    let program = include_str!("fixtures/valid/fixed_array_parameters_decay.c");

    assert_eq!(interpret(program).unwrap(), 19);
}

#[test]
fn supports_array_parameter_qualifiers_as_pointer_metadata() {
    let program = include_str!("fixtures/valid/array_parameter_qualifiers.c");

    assert_eq!(interpret(program).unwrap(), 41);
}

#[test]
fn rejects_array_parameter_const_slot_reassignment() {
    let program = include_str!("fixtures/invalid/array_parameter_const_slot_reassignment.c");

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "cannot assign to const variable 'values'"
    );
}

#[test]
fn supports_pointer_return_functions_and_prototypes() {
    let program = include_str!("fixtures/valid/pointer_return_functions.c");

    assert_eq!(interpret(program).unwrap(), 147);
}

#[test]
fn supports_typedef_aggregate_definitions() {
    let program = include_str!("fixtures/valid/typedef_aggregate_definitions.c");

    assert_eq!(interpret(program).unwrap(), 34);
}

#[test]
fn supports_anonymous_aggregate_typedef_definitions() {
    let program = include_str!("fixtures/valid/anonymous_aggregate_typedefs.c");

    assert_eq!(interpret(program).unwrap(), 65);
}

#[test]
fn supports_block_scoped_typedef_aggregate_definitions() {
    let program = include_str!("fixtures/valid/block_scoped_aggregate_typedef_definitions.c");

    assert_eq!(interpret(program).unwrap(), 39);
}

#[test]
fn supports_aggregate_tag_shadowing_with_distinct_type_identities() {
    let program = include_str!("fixtures/valid/aggregate_tag_shadowing.c");

    assert_eq!(interpret(program).unwrap(), 49);
}

#[test]
fn rejects_block_scoped_aggregate_typedef_alias_after_scope_exit() {
    let program = include_str!("fixtures/invalid/block_aggregate_typedef_alias_out_of_scope.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "undefined struct type 'Hidden'");
}

#[test]
fn rejects_pointer_return_type_mismatches() {
    let program = include_str!("fixtures/invalid/pointer_return_type_mismatch.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot convert pointer to char to pointer to int"
    );
}

#[test]
fn rejects_pointer_return_const_discard() {
    let program = include_str!("fixtures/invalid/pointer_return_const_discard.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_const_pointer_returning_call_to_mutable_pointer() {
    let program = include_str!("fixtures/invalid/pointer_return_call_const_discard.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_const_aggregate_array_decay_to_mutable_pointer() {
    let program = include_str!("fixtures/invalid/const_aggregate_array_decay_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_const_scalar_array_decay_to_mutable_unsized_parameter() {
    let program = include_str!("fixtures/invalid/unsized_array_parameter_const_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_scalar_pointer_type_mismatches() {
    let program = include_str!("fixtures/invalid/scalar_pointer_type_mismatch.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot convert pointer to char to pointer to int"
    );
}

#[test]
fn rejects_aggregate_pointer_type_mismatches() {
    let program = include_str!("fixtures/invalid/aggregate_pointer_type_mismatch.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot convert pointer to union 'Number' to pointer to struct 'Point'"
    );
}

#[test]
fn rejects_pointer_assignment_type_mismatches() {
    let program = include_str!("fixtures/invalid/pointer_assignment_type_mismatch.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot convert pointer to struct 'Size' to pointer to struct 'Point'"
    );
}

#[test]
fn rejects_struct_array_designators_out_of_bounds() {
    let program = include_str!("fixtures/invalid/struct_array_designator_out_of_bounds.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "array designator index 2 out of bounds for struct array 'points'"
    );
}

#[test]
fn rejects_const_aggregate_pointer_index_writes() {
    let program = include_str!("fixtures/invalid/const_aggregate_pointer_index_write.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign through pointer to const");
}

#[test]
fn rejects_struct_pointer_arithmetic_out_of_bounds() {
    let program = include_str!("fixtures/invalid/struct_pointer_arithmetic_out_of_bounds.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "struct array pointer index 2 out of bounds for length 2"
    );
}

#[test]
fn rejects_aggregate_conditional_type_mismatches() {
    let program = include_str!("fixtures/invalid/aggregate_conditional_type_mismatch.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign struct 'Size' to struct 'Point'"
    );
}

#[test]
fn rejects_aggregate_initializer_expression_type_mismatches() {
    let program = include_str!("fixtures/invalid/aggregate_initializer_type_mismatch.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign struct 'Size' to struct 'Point'"
    );
}

#[test]
fn rejects_pointer_to_pointer_union_fields() {
    let program = include_str!("fixtures/invalid/union_pointer_to_pointer_field.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "pointer-to-pointer union fields are not supported at line 3, column 17"
    );
}

#[test]
fn rejects_union_initializers_longer_than_one_field() {
    let program = include_str!("fixtures/invalid/union_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "too many initializers for union 'Number'");
}

#[test]
fn supports_scalar_struct_initializers() {
    let program = include_str!("fixtures/valid/struct_initializers.c");

    assert_eq!(interpret(program).unwrap(), 75);
}

#[test]
fn supports_aggregate_field_declaration_lists() {
    let program = include_str!("fixtures/valid/aggregate_field_declaration_lists.c");

    assert_eq!(interpret(program).unwrap(), 133);
}

#[test]
fn supports_typedef_aliases_in_aggregate_field_declaration_lists() {
    let program = include_str!("fixtures/valid/aggregate_field_typedef_declaration_lists.c");

    assert_eq!(interpret(program).unwrap(), 188);
}

#[test]
fn rejects_assignment_to_const_pointer_slot_typedef_field_in_declaration_list() {
    let program = include_str!("fixtures/invalid/aggregate_field_typedef_const_slot_assignment.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'fixed'"
    );
}

#[test]
fn rejects_duplicate_aggregate_fields_in_declaration_lists() {
    let program = include_str!("fixtures/invalid/duplicate_aggregate_field_in_declaration_list.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "duplicate struct field 'x' at line 1, column 24"
    );
}

#[test]
fn supports_nested_struct_fields() {
    let program = include_str!("fixtures/valid/nested_struct_fields.c");

    assert_eq!(interpret(program).unwrap(), 54);
}

#[test]
fn supports_nested_struct_initializers() {
    let program = include_str!("fixtures/valid/nested_struct_initializers.c");

    assert_eq!(interpret(program).unwrap(), 85);
}

#[test]
fn supports_struct_array_fields() {
    let program = include_str!("fixtures/valid/struct_array_fields.c");

    assert_eq!(interpret(program).unwrap(), 197);
}

#[test]
fn supports_struct_array_field_decay_and_element_address_of() {
    let program = include_str!("fixtures/valid/struct_array_field_decay.c");

    assert_eq!(interpret(program).unwrap(), 55);
}

#[test]
fn supports_struct_pointer_array_field_decay_and_element_address_of() {
    let program = include_str!("fixtures/valid/struct_pointer_array_field_decay.c");

    assert_eq!(interpret(program).unwrap(), 223);
}

#[test]
fn supports_nested_struct_array_field_decay_and_element_address_of() {
    let program = include_str!("fixtures/valid/nested_struct_array_field_decay.c");

    assert_eq!(interpret(program).unwrap(), 41);
}

#[test]
fn rejects_struct_pointer_array_field_decay_that_discards_const() {
    let program = include_str!("fixtures/invalid/struct_pointer_array_field_const_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_nested_struct_array_field_decay_that_discards_const() {
    let program = include_str!("fixtures/invalid/nested_struct_array_field_const_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_nested_struct_array_element_field_decay_that_discards_const() {
    let program =
        include_str!("fixtures/invalid/nested_struct_array_element_field_const_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn supports_arrays_of_structs() {
    let program = include_str!("fixtures/valid/struct_arrays.c");

    assert_eq!(interpret(program).unwrap(), 125);
}

#[test]
fn supports_address_of_struct_array_elements_and_fields() {
    let program = include_str!("fixtures/valid/address_of_struct_fields.c");

    assert_eq!(interpret(program).unwrap(), 175);
}

#[test]
fn supports_struct_pointer_fields() {
    let program = include_str!("fixtures/valid/struct_pointer_fields.c");

    assert_eq!(interpret(program).unwrap(), 41);
}

#[test]
fn supports_struct_pointer_field_arithmetic() {
    let program = include_str!("fixtures/valid/struct_pointer_field_arithmetic.c");

    assert_eq!(interpret(program).unwrap(), 101);
}

#[test]
fn supports_struct_pointer_arrow_field_arithmetic() {
    let program = include_str!("fixtures/valid/struct_pointer_arrow_field_arithmetic.c");

    assert_eq!(interpret(program).unwrap(), 112);
}

#[test]
fn supports_struct_pointer_fields_with_const_pointee_views() {
    let program = include_str!("fixtures/valid/struct_pointer_field_const_pointee.c");

    assert_eq!(interpret(program).unwrap(), 18);
}

#[test]
fn rejects_pointer_to_pointer_struct_fields() {
    let program = include_str!("fixtures/invalid/struct_pointer_to_pointer_field.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "pointer-to-pointer struct fields are not supported at line 2, column 10"
    );
}

#[test]
fn rejects_struct_pointer_field_const_discard() {
    let program = include_str!("fixtures/invalid/struct_pointer_field_const_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_struct_pointer_field_type_mismatch() {
    let program = include_str!("fixtures/invalid/struct_pointer_field_type_mismatch.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot convert pointer to struct 'Size' to pointer to struct 'Point'"
    );
}

#[test]
fn rejects_const_struct_field_address_discard() {
    let program = include_str!("fixtures/invalid/const_struct_field_address_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_struct_array_initializers_longer_than_declared_length() {
    let program = include_str!("fixtures/invalid/struct_array_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "too many initializers for array 'values'");
}

#[test]
fn rejects_struct_array_variable_initializers_longer_than_declared_length() {
    let program = include_str!("fixtures/invalid/struct_array_variable_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "too many initializers for struct array 'points'"
    );
}

#[test]
fn rejects_unknown_nested_struct_fields() {
    let program = include_str!("fixtures/invalid/nested_struct_unknown_field.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "struct 'Point' has no field 'z'");
}

#[test]
fn rejects_array_initializers_longer_than_declared_length() {
    let program = include_str!("fixtures/invalid/array_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "too many initializers for array 'values'");
}

#[test]
fn rejects_array_designators_outside_declared_length() {
    let program = include_str!("fixtures/invalid/array_designator_out_of_bounds.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array designator index 3 out of bounds for array 'values'"
    );
}

#[test]
fn rejects_unknown_struct_field_designators() {
    let program = include_str!("fixtures/invalid/struct_designator_unknown_field.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "struct 'Point' has no field 'z'");
}

#[test]
fn rejects_unknown_path_designator_fields() {
    let program = include_str!("fixtures/invalid/struct_path_designator_unknown_field.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "struct 'Inner' has no field 'missing'");
}

#[test]
fn rejects_struct_array_path_designators_outside_declared_length() {
    let program = include_str!("fixtures/invalid/struct_array_path_designator_out_of_bounds.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array designator index 2 out of bounds for array field 'values'"
    );
}

#[test]
fn rejects_struct_initializers_longer_than_declared_fields() {
    let program = include_str!("fixtures/invalid/struct_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "too many initializers for struct 'Point'");
}

#[test]
fn rejects_nested_struct_initializers_longer_than_nested_fields() {
    let program = include_str!("fixtures/invalid/nested_struct_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "too many initializers for struct 'Point'");
}

#[test]
fn supports_const_qualified_scalars_arrays_and_parameters() {
    let program = include_str!("fixtures/valid/const_qualifiers.c");

    assert_eq!(interpret(program).unwrap(), 83);
}

#[test]
fn supports_const_qualified_pointer_pointees_and_pointer_slots() {
    let program = include_str!("fixtures/valid/const_pointer_qualifiers.c");

    assert_eq!(interpret(program).unwrap(), 56);
}

#[test]
fn supports_pointer_const_preserving_conversions() {
    let program = include_str!("fixtures/valid/const_pointer_conversions.c");

    assert_eq!(interpret(program).unwrap(), 36);
}

#[test]
fn supports_const_qualified_structs_and_struct_pointers() {
    let program = include_str!("fixtures/valid/const_struct_qualifiers.c");

    assert_eq!(interpret(program).unwrap(), 41);
}

#[test]
fn supports_const_qualified_struct_fields() {
    let program = include_str!("fixtures/valid/const_struct_fields.c");

    assert_eq!(interpret(program).unwrap(), 77);
}

#[test]
fn supports_static_storage_class_at_top_level() {
    let program = include_str!("fixtures/valid/static_storage_class.c");

    assert_eq!(interpret(program).unwrap(), 22);
}

#[test]
fn supports_persistent_static_local_storage() {
    let program = include_str!("fixtures/valid/static_local_storage.c");

    assert_eq!(interpret(program).unwrap(), 286);
}

#[test]
fn supports_void_parameter_lists_for_functions_and_prototypes() {
    let program = include_str!("fixtures/valid/void_parameter_lists.c");

    assert_eq!(interpret(program).unwrap(), 6);
}

#[test]
fn rejects_named_void_parameters() {
    let program = include_str!("fixtures/invalid/void_parameter_named.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "void parameter lists must be empty at line 1, column 12"
    );
}

#[test]
fn keeps_static_local_names_block_scoped() {
    let program = include_str!("fixtures/invalid/static_local_out_of_scope.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "undefined variable 'hidden'");
}

#[test]
fn rejects_assignment_to_const_struct_fields() {
    let program = include_str!("fixtures/invalid/const_struct_member_assignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'magic'"
    );
}

#[test]
fn rejects_pointer_writes_to_const_struct_fields() {
    let program = include_str!("fixtures/invalid/const_struct_member_pointer_write.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'magic'"
    );
}

#[test]
fn rejects_copy_assignment_to_structs_with_const_fields() {
    let program = include_str!("fixtures/invalid/const_struct_member_copy_assignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot assign to struct 'Config' with const fields"
    );
}

#[test]
fn rejects_direct_field_assignment_to_const_structs() {
    let program = include_str!("fixtures/invalid/const_struct_field_assignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "cannot assign to const variable 'point'");
}

#[test]
fn rejects_field_writes_through_const_struct_pointers() {
    let program = include_str!("fixtures/invalid/const_struct_pointer_write.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "cannot assign through pointer to const");
}

#[test]
fn rejects_struct_pointer_declarations_that_discard_const_pointee() {
    let program = include_str!("fixtures/invalid/const_struct_pointer_discard.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_pointer_declarations_that_discard_const_pointee() {
    let program = include_str!("fixtures/invalid/const_pointer_discard_decl.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_pointer_assignments_that_discard_const_pointee() {
    let program = include_str!("fixtures/invalid/const_pointer_discard_assignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_pointer_arguments_that_discard_const_pointee() {
    let program = include_str!("fixtures/invalid/const_pointer_discard_argument.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_writes_through_const_pointer_pointees() {
    let program = include_str!("fixtures/invalid/const_pointer_write.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "cannot assign through pointer to const");
}

#[test]
fn rejects_index_writes_through_const_pointer_pointees() {
    let program = include_str!("fixtures/invalid/const_pointer_index_write.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "cannot assign through pointer to const");
}

#[test]
fn rejects_reassignment_of_const_pointer_slots() {
    let program = include_str!("fixtures/invalid/const_pointer_reassignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "cannot assign to const variable 'p'");
}

#[test]
fn rejects_assignment_to_const_scalars() {
    let program = include_str!("fixtures/invalid/const_assignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "cannot assign to const variable 'value'");
}

#[test]
fn rejects_assignment_to_const_arrays() {
    let program = include_str!("fixtures/invalid/const_array_assignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "cannot modify read-only array 'values'");
}

#[test]
fn rejects_assignment_to_const_parameters() {
    let program = include_str!("fixtures/invalid/const_parameter_assignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "cannot assign to const variable 'value'");
}

#[test]
fn supports_enum_constants_with_implicit_values_and_block_scope() {
    let program = include_str!("fixtures/valid/enums.c");

    assert_eq!(interpret(program).unwrap(), 33);
}

#[test]
fn supports_function_prototypes_before_definitions() {
    let program = include_str!("fixtures/valid/function_prototypes.c");

    assert_eq!(interpret(program).unwrap(), 12);
}

#[test]
fn supports_unnamed_function_prototype_parameters() {
    let program = include_str!("fixtures/valid/unnamed_prototype_parameters.c");

    assert_eq!(interpret(program).unwrap(), 47);
}

#[test]
fn supports_char_return_functions_and_prototypes() {
    let program = include_str!("fixtures/valid/char_return_functions.c");

    assert_eq!(interpret(program).unwrap(), 3);
}

#[test]
fn supports_const_qualified_return_types() {
    let program = include_str!("fixtures/valid/const_return_types.c");

    assert_eq!(interpret(program).unwrap(), 142);
}

#[test]
fn supports_struct_declarations_and_member_access() {
    let program = include_str!("fixtures/valid/structs.c");

    assert_eq!(interpret(program).unwrap(), 20);
}

#[test]
fn supports_struct_copy_and_field_lvalue_expressions() {
    let program = include_str!("fixtures/valid/struct_lvalues_and_copy.c");

    assert_eq!(interpret(program).unwrap(), 21);
}

#[test]
fn supports_struct_by_value_function_parameters() {
    let program = include_str!("fixtures/valid/struct_parameters.c");

    assert_eq!(interpret(program).unwrap(), 0);
}

#[test]
fn supports_struct_return_functions_and_prototypes() {
    let program = include_str!("fixtures/valid/struct_return_functions.c");

    assert_eq!(interpret(program).unwrap(), 31);
}

#[test]
fn supports_struct_pointers_arrow_and_deref_field_access() {
    let program = include_str!("fixtures/valid/struct_pointers.c");

    assert_eq!(interpret(program).unwrap(), 0);
}

#[test]
fn supports_typedef_aliases_for_scalars_structs_and_pointers() {
    let program = include_str!("fixtures/valid/typedef_aliases.c");

    assert_eq!(interpret(program).unwrap(), 28);
}

#[test]
fn supports_enum_typedef_aliases_as_integer_types() {
    let program = include_str!("fixtures/valid/enum_typedef_aliases.c");

    assert_eq!(interpret(program).unwrap(), 20);
}

#[test]
fn supports_anonymous_enum_typedefs_as_integer_types() {
    let program = include_str!("fixtures/valid/anonymous_enum_typedefs.c");

    assert_eq!(interpret(program).unwrap(), 36);
}

#[test]
fn rejects_typedef_aliases_to_unknown_enum_tags() {
    let program = include_str!("fixtures/invalid/typedef_unknown_enum.c");
    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "undefined enum type 'Missing' at line 2, column 14"
    );
}

#[test]
fn rejects_typedef_aliases_to_block_scoped_enum_tags_after_scope_exit() {
    let program = include_str!("fixtures/invalid/block_enum_tag_out_of_scope.c");
    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "undefined enum type 'Local' at line 5, column 18"
    );
}

#[test]
fn supports_pointer_typedef_aliases_for_declarations_params_and_sizeof() {
    let program = include_str!("fixtures/valid/pointer_typedef_aliases.c");

    assert_eq!(interpret(program).unwrap(), 0);
}

#[test]
fn rejects_pointer_typedef_aliases_to_pointer_aliases() {
    let program = include_str!("fixtures/invalid/pointer_typedef_to_pointer.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "pointer-to-pointer typedef aliases are not supported at line 2, column 16"
    );
}

#[test]
fn rejects_direct_pointer_to_pointer_typedef_aliases() {
    let program = include_str!("fixtures/invalid/direct_pointer_to_pointer_typedef.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "pointer-to-pointer typedef aliases are not supported at line 1, column 14"
    );
}

#[test]
fn supports_block_scoped_typedef_aliases_and_shadowing() {
    let program = include_str!("fixtures/valid/block_scoped_typedefs.c");

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn rejects_block_typedef_aliases_after_scope_exit() {
    let program = include_str!("fixtures/invalid/block_typedef_alias_out_of_scope.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '=' after assignment, found Ident(\"leaked\") at line 9, column 11"
    );
}

#[test]
fn rejects_typedefs_without_alias_names() {
    let program = include_str!("fixtures/invalid/typedef_missing_alias_name.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected typedef alias name after type, found Semi at line 2, column 14"
    );
}

#[test]
fn rejects_null_struct_pointer_field_access() {
    let program = include_str!("fixtures/invalid/struct_pointer_null_dereference.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "null pointer dereference");
}

#[test]
fn rejects_out_of_scope_struct_pointer_field_access() {
    let program = include_str!("fixtures/invalid/struct_pointer_out_of_scope.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "pointer to out-of-scope variable 'point'");
}

#[test]
fn rejects_mismatched_struct_return_values() {
    let program = include_str!("fixtures/invalid/struct_return_type_mismatch.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "struct function 'bad' expected return struct 'Point', got struct 'Pair'"
    );
}

#[test]
fn rejects_empty_returns_from_struct_functions() {
    let program = include_str!("fixtures/invalid/struct_function_empty_return.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "struct function 'bad' returned without a value"
    );
}

#[test]
fn rejects_mismatched_struct_function_arguments() {
    let program = include_str!("fixtures/invalid/struct_parameter_type_mismatch.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "function 'read_x' struct parameter 'p' expected struct 'Point', got struct 'Pair'"
    );
}

#[test]
fn rejects_non_struct_arguments_for_struct_parameters() {
    let program = include_str!("fixtures/invalid/struct_parameter_non_struct_argument.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "function 'read_x' struct parameter 'p' requires a struct argument"
    );
}

#[test]
fn rejects_mismatched_struct_copy_assignment() {
    let program = include_str!("fixtures/invalid/struct_assignment_type_mismatch.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot assign struct 'Pair' to struct 'Point'"
    );
}

#[test]
fn rejects_unknown_struct_fields() {
    let program = include_str!("fixtures/invalid/unknown_struct_field.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "struct 'Point' has no field 'y'");
}

#[test]
fn rejects_empty_returns_from_char_functions() {
    let program = include_str!("fixtures/invalid/char_function_empty_return.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "char function 'bad' returned without a value"
    );
}

#[test]
fn rejects_conflicting_function_prototypes() {
    let program = include_str!("fixtures/invalid/conflicting_function_prototype.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "function prototype for 'helper' conflicts with previous declaration"
    );
}

#[test]
fn rejects_assignment_to_enum_constants() {
    let program = include_str!("fixtures/invalid/enum_constant_assignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "cannot assign to enum constant 'READY'");
}

#[test]
fn rejects_missing_enum_constant_values_with_context() {
    let program = include_str!("fixtures/invalid/enum_missing_value.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected integer constant after enum constant '=', found RBrace at line 2, column 36"
    );
}

#[test]
fn rejects_sizeof_void_with_context() {
    let program = include_str!("fixtures/invalid/sizeof_void.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "sizeof(void) is not supported at line 2, column 19"
    );
}

#[test]
fn rejects_sizeof_const_void_with_context() {
    let program = include_str!("fixtures/invalid/sizeof_const_void.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "sizeof(void) is not supported at line 2, column 25"
    );
}

#[test]
fn reports_duplicate_global_variables() {
    let program = include_str!("fixtures/invalid/duplicate_global_variable.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "variable 'total' already declared in this scope"
    );
}

#[test]
fn reports_source_context_for_unterminated_block_comments() {
    let program = include_str!("fixtures/invalid/unterminated_block_comment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "unterminated block comment at line 3, column 5\n    /* unterminated block comment\n    ^"
    );
}

#[test]
fn reports_line_and_column_for_parser_expression_errors() {
    let program = "int main() {\nreturn +;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected expression, found Semi at line 2, column 9"
    );
}

#[test]
fn reports_context_for_unterminated_function_blocks() {
    let program = "int main() {\nreturn 0;\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "unterminated block after function header at line 3, column 1"
    );
}

#[test]
fn reports_context_for_unterminated_nested_blocks() {
    let program = "int main() {\n{\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "unterminated block after function header at line 5, column 1"
    );
}

#[test]
fn reports_context_for_unterminated_control_flow_blocks() {
    let program = "int main() {\nif (1) {\nreturn 1;\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "unterminated block after if condition at line 4, column 1"
    );
}

#[test]
fn reports_missing_commas_in_function_parameter_lists() {
    let program = "int add(int a int b) { return a + b; }\nint main() { return add(1, 2); }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ',' or ')' after function parameter, found Int at line 1, column 15"
    );
}

#[test]
fn reports_missing_commas_in_function_call_argument_lists() {
    let program = "int add(int a, int b) { return a + b; }\nint main() { return add(1 2); }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ',' or ')' after function call argument, found Number(2) at line 2, column 27"
    );
}

#[test]
fn reports_trailing_commas_in_function_call_argument_lists() {
    let program = "int add(int a, int b) { return a + b; }\nint main() { return add(1,); }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected function call argument after ',', found RParen at line 2, column 27"
    );
}

#[test]
fn reports_trailing_commas_in_function_parameter_lists() {
    let program = "int add(int a,) { return a; }\nint main() { return add(1); }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected function parameter after ',', found RParen at line 1, column 15"
    );
}

#[test]
fn reports_missing_closing_brackets_after_array_lengths() {
    let program = "int main() {\nint values[2;\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ']' after array length, found Semi at line 2, column 13"
    );
}

#[test]
fn reports_inferred_array_declarations_without_initializers() {
    let program = "int main() {\nint values[];\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '=' after inferred array declaration, found Semi at line 2, column 13"
    );
}

#[test]
fn reports_inferred_aggregate_array_declarations_without_initializers() {
    let program = "struct Point { int x; };\nint main() {\nstruct Point points[];\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '=' after inferred aggregate array declaration, found Semi at line 3, column 22"
    );
}

#[test]
fn reports_negative_array_lengths() {
    let program = "int main() {\nint values[-1];\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array length must be positive at line 2, column 12"
    );
}

#[test]
fn reports_negative_array_parameter_lengths() {
    let program = "int first(int values[-1]) { return values[0]; }\nint main() { int xs[1]; return first(xs); }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array length must be positive at line 1, column 22"
    );
}

#[test]
fn reports_missing_closing_brackets_after_array_parameter_lengths() {
    let program = "int first(int values[2) { return values[0]; }\nint main() { int xs[2]; return first(xs); }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ']' after array parameter length, found RParen at line 1, column 23"
    );
}

#[test]
fn reports_missing_closing_brackets_after_array_assignment_indices() {
    let program = "int main() {\nint values[2];\nvalues[0 = 3;\nreturn values[0];\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ']' after array index, found Assign at line 3, column 10"
    );
}

#[test]
fn reports_missing_closing_brackets_after_array_expression_indices() {
    let program = "int main() {\nint values[2];\nreturn values[0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ']' after array index, found Semi at line 3, column 16"
    );
}

#[test]
fn reports_missing_closing_brackets_after_string_literal_indices() {
    let program = "int main() {\nreturn \"ab\"[1;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ']' after string literal index, found Semi at line 2, column 14"
    );
}

#[test]
fn reports_missing_opening_parens_after_function_names() {
    let program = "int main { return 0; }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '(' after function name, found LBrace at line 1, column 10"
    );
}

#[test]
fn reports_missing_opening_parens_after_if_keywords() {
    let program = "int main() {\nif 1) {\nreturn 1;\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '(' after if, found Number(1) at line 2, column 4"
    );
}

#[test]
fn reports_missing_opening_parens_after_while_keywords() {
    let program = "int main() {\nwhile 1) {\nreturn 1;\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '(' after while, found Number(1) at line 2, column 7"
    );
}

#[test]
fn reports_missing_opening_parens_after_for_keywords() {
    let program = "int main() {\nfor int i = 0; i < 3; i = i + 1) {\nreturn i;\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '(' after for, found Int at line 2, column 5"
    );
}

#[test]
fn reports_missing_closing_parens_after_grouped_expressions() {
    let program = "int main() {\nreturn (1 + 2;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ')' after grouped expression, found Semi at line 2, column 14"
    );
}

#[test]
fn reports_missing_closing_parens_after_function_definition_parameters() {
    let program = "int add(int a, int b { return a + b; }\nint main() { return add(1, 2); }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ')' after function parameters, found LBrace at line 1, column 22"
    );
}

#[test]
fn reports_missing_closing_parens_after_function_call_arguments() {
    let program = "int add(int a, int b) { return a + b; }\nint main() {\nreturn add(1, 2;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ')' after function call arguments, found Semi at line 3, column 16"
    );
}

#[test]
fn reports_missing_closing_parens_after_if_conditions() {
    let program = "int main() {\nif (1 {\nreturn 1;\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ')' after if condition, found LBrace at line 2, column 7"
    );
}

#[test]
fn reports_missing_closing_parens_after_while_conditions() {
    let program = "int main() {\nwhile (1 {\nreturn 1;\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ')' after while condition, found LBrace at line 2, column 10"
    );
}

#[test]
fn reports_missing_closing_parens_after_for_clauses() {
    let program = "int main() {\nfor (int i = 0; i < 3; i = i + 1 {\nreturn i;\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ')' after for clauses, found LBrace at line 2, column 34"
    );
}

#[test]
fn reports_missing_opening_braces_after_function_headers() {
    let program = "int main()\nreturn 0;\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '{' after function header, found Return at line 2, column 1"
    );
}

#[test]
fn supports_single_statement_if_bodies() {
    let program = "int main() {\nif (1) return 1;\nreturn 0;\n}\n";

    assert_eq!(interpret(program).unwrap(), 1);
}

#[test]
fn supports_single_statement_else_bodies() {
    let program = "int main() {\nif (0) { return 0; } else return 1;\n}\n";

    assert_eq!(interpret(program).unwrap(), 1);
}

#[test]
fn supports_single_statement_while_bodies() {
    let program = "int main() {\nint x = 0;\nwhile (x < 3) x++;\nreturn x;\n}\n";

    assert_eq!(interpret(program).unwrap(), 3);
}

#[test]
fn supports_single_statement_for_bodies() {
    let program =
        "int main() {\nint total = 0;\nfor (int i = 0; i < 3; i++) total += i;\nreturn total;\n}\n";

    assert_eq!(interpret(program).unwrap(), 3);
}

#[test]
fn reports_unmatched_closing_parens_in_statements() {
    let program = "int main() {\n)\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "unmatched ')' in statement at line 2, column 1"
    );
}

#[test]
fn reports_unmatched_closing_brackets_in_statements() {
    let program = "int main() {\n]\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "unmatched ']' in statement at line 2, column 1"
    );
}

#[test]
fn reports_unmatched_closing_braces_at_top_level() {
    let program = "int main() {\nreturn 0;\n}\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "unmatched '}' at top level at line 4, column 1"
    );
}

#[test]
fn reports_missing_function_names_after_return_types() {
    let program = "int () { return 0; }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected function name after return type, found LParen at line 1, column 5"
    );
}

#[test]
fn reports_missing_variable_names_after_declaration_types() {
    let program = "int main() {\nint = 1;\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected variable name after type, found Assign at line 2, column 5"
    );
}

#[test]
fn reports_missing_pointer_names_after_pointer_declaration_stars() {
    let program = "int main() {\nint x = 1;\nint * = &x;\nreturn x;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected pointer name after '*', found Assign at line 3, column 7"
    );
}

#[test]
fn reports_missing_parameter_names_after_parameter_types() {
    let program = "int identity(int) { return 0; }\nint main() { return identity(1); }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected parameter name after type, found RParen at line 1, column 17"
    );
}

#[test]
fn reports_missing_parameter_types_before_parameter_names() {
    let program = "int identity(value) { return value; }\nint main() { return identity(1); }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected parameter type, found Ident(\"value\") at line 1, column 14"
    );
}

#[test]
fn rejects_pointer_return_types_with_context() {
    let program = "int **identity(int *x) { return &x; }\nint main() { return 0; }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "pointer-to-pointer return types are not supported at line 1, column 6"
    );
}

#[test]
fn rejects_pointer_array_parameters_with_context() {
    let program = "int first(int *values[2]) { return values[0]; }\nint main() { return 0; }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "pointer array parameters are not supported at line 1, column 22"
    );
}

#[test]
fn rejects_pointer_array_declarations_with_context() {
    let program = "int main() {\nint *values[2];\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "pointer array declarations are not supported at line 2, column 12"
    );
}

#[test]
fn rejects_parenthesized_pointer_parameters_with_context() {
    let program = include_str!("fixtures/invalid/parenthesized_pointer_parameter.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "parenthesized pointer parameters are not supported at line 1, column 13"
    );
}

#[test]
fn rejects_multidimensional_array_declarations_with_context() {
    let program = include_str!("fixtures/invalid/multidimensional_array_declaration.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "multidimensional array declarations are not supported at line 2, column 16"
    );
}

#[test]
fn rejects_multidimensional_array_parameters_with_context() {
    let program = include_str!("fixtures/invalid/multidimensional_array_parameter.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "multidimensional array parameters are not supported at line 1, column 22"
    );
}

#[test]
fn rejects_multidimensional_array_fields_with_context() {
    let program = include_str!("fixtures/invalid/multidimensional_array_field.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "multidimensional array struct fields are not supported at line 2, column 16"
    );
}

#[test]
fn rejects_parenthesized_pointer_declarations_with_context() {
    let program = include_str!("fixtures/invalid/parenthesized_pointer_declaration.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "parenthesized pointer declarations are not supported at line 2, column 9"
    );
}

#[test]
fn rejects_function_pointer_declarations_with_context() {
    let program = include_str!("fixtures/invalid/function_pointer_declaration.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "function pointer declarations are not supported at line 1, column 5"
    );
}

#[test]
fn rejects_local_function_pointer_declarations_with_context() {
    let program = include_str!("fixtures/invalid/function_pointer_local_declaration.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "function pointer declarations are not supported at line 2, column 9"
    );
}

#[test]
fn rejects_function_pointer_typedef_aliases_with_context() {
    let program = include_str!("fixtures/invalid/function_pointer_typedef_alias.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "function pointer typedef aliases are not supported at line 1, column 13"
    );
}

#[test]
fn rejects_function_typedef_aliases_with_context() {
    let program = include_str!("fixtures/invalid/function_typedef_alias.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "function typedef aliases are not supported at line 1, column 21"
    );
}

#[test]
fn rejects_function_pointer_parameters_with_context() {
    let program = include_str!("fixtures/invalid/function_pointer_parameter.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "function pointer parameters are not supported at line 1, column 15"
    );
}

#[test]
fn rejects_variadic_function_parameters_with_context() {
    let program = include_str!("fixtures/invalid/variadic_function_parameter.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "variadic function parameters are not supported at line 1, column 20"
    );
}

#[test]
fn rejects_goto_statements_with_context() {
    let program = include_str!("fixtures/invalid/goto_statement.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "goto statements are not supported at line 2, column 5"
    );
}

#[test]
fn rejects_label_statements_with_context() {
    let program = include_str!("fixtures/invalid/label_statement.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "labels are not supported at line 2, column 1"
    );
}

#[test]
fn reports_missing_pointer_parameter_names_after_stars() {
    let program = "int identity(int *) { return 0; }\nint main() { return identity(0); }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected parameter name after '*', found RParen at line 1, column 19"
    );
}

#[test]
fn rejects_pointer_to_pointer_parameters_with_context() {
    let program = "int load(int **value) { return **value; }\nint main() { return 0; }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "pointer-to-pointer parameters are not supported at line 1, column 15"
    );
}

#[test]
fn rejects_pointer_to_pointer_declarations_with_context() {
    let program = "int main() {\nint x = 1;\nint **value = &x;\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "pointer-to-pointer declarations are not supported at line 3, column 6"
    );
}

#[test]
fn reports_missing_commas_after_pointer_parameters() {
    let program =
        "int sum(int *values int count) { return values[0] + count; }\nint main() { return 0; }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ',' or ')' after function parameter, found Int at line 1, column 21"
    );
}

#[test]
fn reports_trailing_commas_after_pointer_parameters() {
    let program = "int first(int *values,) { return values[0]; }\nint main() { return 0; }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected function parameter after ',', found RParen at line 1, column 23"
    );
}

#[test]
fn reports_function_parameter_after_comma_before_function_body() {
    let program = "int first(int value, { return value; }\nint main() { return 0; }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected function parameter after ',', found LBrace at line 1, column 22"
    );
}

#[test]
fn reports_function_call_argument_after_comma_before_semicolon() {
    let program = "int first(int value) { return value; }\nint main() { return first(1,; }\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected function call argument after ',', found Semi at line 2, column 29"
    );
}

#[test]
fn reports_missing_assignment_operator_after_variable_declarations() {
    let program = "int main() {\nint x 1;\nreturn x;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '=' after variable declaration, found Number(1) at line 2, column 7"
    );
}

#[test]
fn reports_missing_assignment_operator_after_pointer_declarations() {
    let program = "int main() {\nint x = 1;\nint *p &x;\nreturn *p;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '=' after pointer declaration, found Amp at line 3, column 8"
    );
}

#[test]
fn reports_missing_assignment_operator_after_scalar_assignments() {
    let program = "int main() {\nint x = 1;\nx 2;\nreturn x;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '=' after assignment, found Number(2) at line 3, column 3"
    );
}

#[test]
fn reports_missing_assignment_operator_after_array_assignments() {
    let program = "int main() {\nint values[2];\nvalues[0] 3;\nreturn values[0];\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '=' after assignment, found Number(3) at line 3, column 11"
    );
}

#[test]
fn reports_missing_assignment_operator_after_dereference_assignments() {
    let program = "int main() {\nint x = 1;\nint *p = &x;\n*p 3;\nreturn x;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '=' after assignment, found Number(3) at line 4, column 4"
    );
}

#[test]
fn reports_missing_semicolon_after_variable_declarations() {
    let program = "int main() {\nint x = 1\nreturn x;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ';' after variable declaration, found Return at line 3, column 1"
    );
}

#[test]
fn reports_missing_semicolon_after_array_declarations() {
    let program = "int main() {\nint values[2]\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ';' after array declaration, found Return at line 3, column 1"
    );
}

#[test]
fn reports_missing_semicolon_after_assignments() {
    let program = "int main() {\nint x = 1;\nx = 2\nreturn x;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ';' after assignment, found Return at line 4, column 1"
    );
}

#[test]
fn reports_missing_semicolon_after_array_assignments() {
    let program = "int main() {\nint values[2];\nvalues[0] = 3\nreturn values[0];\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ';' after assignment, found Return at line 4, column 1"
    );
}

#[test]
fn reports_missing_semicolon_after_expression_statements() {
    let program = "int main() {\n1 + 2\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ';' after expression statement, found Return at line 3, column 1"
    );
}

#[test]
fn reports_missing_semicolon_after_return_statements() {
    let program = "int main() {\nreturn 1\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ';' after return statement, found RBrace at line 3, column 1"
    );
}

#[test]
fn supports_block_scope_shadowing_and_outer_assignment() {
    let program = include_str!("fixtures/valid/block_scope_shadowing.c");

    assert_eq!(interpret(program).unwrap(), 43);
}

#[test]
fn supports_logical_operators_short_circuiting_and_unary_plus() {
    let program = include_str!("fixtures/valid/logical_operators.c");

    assert_eq!(interpret(program).unwrap(), 4);
}

#[test]
fn supports_for_loops_with_declaration_assignment_and_empty_clauses() {
    let program = include_str!("fixtures/valid/for_loops.c");

    assert_eq!(interpret(program).unwrap(), 41);
}

#[test]
fn supports_break_and_continue_in_while_and_for_loops() {
    let program = include_str!("fixtures/valid/break_continue.c");

    assert_eq!(interpret(program).unwrap(), 68);
}

#[test]
fn respects_arithmetic_precedence_with_unary_and_remainder() {
    let program = include_str!("fixtures/valid/arithmetic_precedence.c");

    assert_eq!(interpret(program).unwrap(), 22);
}

#[test]
fn for_continue_still_runs_increment_clause() {
    let program = include_str!("fixtures/valid/for_continue_runs_increment.c");

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn break_exits_only_the_innermost_loop() {
    let program = include_str!("fixtures/valid/nested_loop_break.c");

    assert_eq!(interpret(program).unwrap(), 33);
}

#[test]
fn supports_empty_and_expression_statements() {
    let program = include_str!("fixtures/valid/empty_and_expression_statements.c");

    assert_eq!(interpret(program).unwrap(), 16);
}

#[test]
fn supports_function_definitions_calls_and_parameters() {
    let program = include_str!("fixtures/valid/functions_and_parameters.c");

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_void_functions_and_empty_returns_for_side_effects() {
    let program = include_str!("fixtures/valid/void_functions.c");

    assert_eq!(interpret(program).unwrap(), 22);
}

#[test]
fn rejects_return_values_from_void_functions() {
    let program = include_str!("fixtures/invalid/void_function_returns_value.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "void function 'bad' returned a value");
}

#[test]
fn rejects_void_function_calls_used_as_scalar_expressions() {
    let program = include_str!("fixtures/invalid/void_function_used_as_expression.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "void function 'noop' used as scalar expression"
    );
}

#[test]
fn rejects_empty_returns_from_int_functions() {
    let program = include_str!("fixtures/invalid/int_function_empty_return.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "int function 'main' returned without a value"
    );
}

#[test]
fn supports_direct_and_mutual_recursive_calls() {
    let program = include_str!("fixtures/valid/recursive_calls.c");

    assert_eq!(interpret(program).unwrap(), 125);
}

#[test]
fn supports_char_literals_variables_and_parameters() {
    let program = include_str!("fixtures/valid/char_literals_and_variables.c");

    assert_eq!(interpret(program).unwrap(), 206);
}

#[test]
fn supports_one_dimensional_arrays_indexing_and_parameters() {
    let program = include_str!("fixtures/valid/arrays.c");

    assert_eq!(interpret(program).unwrap(), 195);
}

#[test]
fn supports_string_literals_as_read_only_byte_arrays() {
    let program = include_str!("fixtures/valid/string_literals.c");

    assert_eq!(interpret(program).unwrap(), 579);
}

#[test]
fn supports_scalar_pointer_address_dereference_write_and_reassignment() {
    let program = include_str!("fixtures/valid/pointers_scalars.c");

    assert_eq!(interpret(program).unwrap(), 54);
}

#[test]
fn supports_scalar_pointer_parameters() {
    let program = include_str!("fixtures/valid/pointer_parameters.c");

    assert_eq!(interpret(program).unwrap(), 77);
}

#[test]
fn supports_array_decay_to_pointer_parameters_and_pointer_indexing() {
    let program = include_str!("fixtures/valid/pointer_arrays.c");

    assert_eq!(interpret(program).unwrap(), 11);
}

#[test]
fn supports_string_literal_decay_to_pointer_parameters_for_reads() {
    let program = include_str!("fixtures/valid/pointer_string_reads.c");

    assert_eq!(interpret(program).unwrap(), 98);
}

#[test]
fn supports_address_of_string_literal_elements() {
    let program = include_str!("fixtures/valid/string_literal_element_address.c");

    assert_eq!(interpret(program).unwrap(), 0);
}

#[test]
fn supports_array_element_address_of_and_pointer_reads_writes() {
    let program = include_str!("fixtures/valid/pointer_array_elements.c");

    assert_eq!(interpret(program).unwrap(), 116);
}

#[test]
fn supports_mixed_pointer_string_array_conformance_fixture() {
    let program = include_str!("fixtures/compat/valid/mixed_pointer_string_array_conformance.c");

    assert_eq!(interpret(program).unwrap(), 143);
}

#[test]
fn supports_pointer_truthiness_and_equality_comparisons() {
    let program = include_str!("fixtures/valid/pointer_truthiness_and_equality.c");

    assert_eq!(interpret(program).unwrap(), 127);
}

#[test]
fn supports_array_backed_pointer_arithmetic_and_difference() {
    let program = include_str!("fixtures/valid/pointer_arithmetic.c");

    assert_eq!(interpret(program).unwrap(), 70);
}

#[test]
fn supports_assignment_expressions_for_scalar_array_and_deref_lvalues() {
    let program = include_str!("fixtures/valid/assignment_expressions.c");

    assert_eq!(interpret(program).unwrap(), 29);
}

#[test]
fn supports_conditional_operator_with_short_circuiting_and_pointer_truthiness() {
    let program = include_str!("fixtures/valid/conditional_operator.c");

    assert_eq!(interpret(program).unwrap(), 96);
}

#[test]
fn supports_do_while_loops_with_break_continue_and_post_test_execution() {
    let program = include_str!("fixtures/valid/do_while_loops.c");

    assert_eq!(interpret(program).unwrap(), 18);
}

#[test]
fn supports_compound_assignment_expressions_for_scalar_array_and_deref_lvalues() {
    let program = include_str!("fixtures/valid/compound_assignments.c");

    assert_eq!(interpret(program).unwrap(), 26);
}

#[test]
fn supports_increment_decrement_for_scalar_indexed_and_deref_lvalues() {
    let program = include_str!("fixtures/valid/increment_decrement.c");

    assert_eq!(interpret(program).unwrap(), 89);
}

#[test]
fn supports_bitwise_and_shift_operators_with_c_precedence() {
    let program = include_str!("fixtures/valid/bitwise_operators.c");

    assert_eq!(interpret(program).unwrap(), 188);
}

#[test]
fn supports_bitwise_compound_assignments_for_scalar_indexed_and_deref_lvalues() {
    let program = include_str!("fixtures/valid/bitwise_compound_assignments.c");

    assert_eq!(interpret(program).unwrap(), 131);
}

#[test]
fn supports_comma_operator_with_assignments_pointers_loops_and_call_arguments() {
    let program = include_str!("fixtures/valid/comma_operator.c");

    assert_eq!(interpret(program).unwrap(), 233);
}

#[test]
fn supports_switch_statements_with_cases_default_fallthrough_break_and_continue() {
    let program = include_str!("fixtures/valid/switch_statements.c");

    assert_eq!(interpret(program).unwrap(), 48);
}

#[test]
fn supports_enum_constants_as_switch_case_labels() {
    let program = include_str!("fixtures/valid/switch_enum_case_labels.c");

    assert_eq!(interpret(program).unwrap(), 254);
}

#[test]
fn supports_single_statement_control_bodies_else_if_and_dangling_else() {
    let program = include_str!("fixtures/valid/single_statement_control_bodies.c");

    assert_eq!(interpret(program).unwrap(), 116);
}

#[test]
fn rejects_missing_colon_after_switch_case_label() {
    let program = include_str!("fixtures/invalid/switch_case_missing_colon.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ':' after switch case label, found Return at line 5, column 9"
    );
}

#[test]
fn rejects_duplicate_switch_case_labels() {
    let program = include_str!("fixtures/invalid/switch_duplicate_case.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "duplicate switch case label 1 at line 6, column 10"
    );
}

#[test]
fn rejects_duplicate_switch_enum_case_labels() {
    let program = include_str!("fixtures/invalid/switch_duplicate_enum_case.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "duplicate switch case label 3 at line 10, column 10"
    );
}

#[test]
fn rejects_comma_operator_in_enum_constant_expression() {
    let program = include_str!("fixtures/invalid/enum_comma_constant_expression.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "comma operator is not allowed in integer constant expression at line 2, column 15"
    );
}

#[test]
fn rejects_comma_operator_in_switch_case_label() {
    let program = include_str!("fixtures/invalid/switch_comma_case_label.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "comma operator is not allowed in integer constant expression at line 3, column 12"
    );
}

#[test]
fn rejects_unparenthesized_comma_operator_in_switch_case_label() {
    let program = include_str!("fixtures/invalid/switch_unparenthesized_comma_case_label.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "comma operator is not allowed in integer constant expression at line 3, column 11"
    );
}

#[test]
fn rejects_duplicate_switch_default_labels() {
    let program = include_str!("fixtures/invalid/switch_duplicate_default.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "duplicate switch default label at line 8, column 5"
    );
}

#[test]
fn rejects_missing_rhs_after_comma_operator() {
    let program = include_str!("fixtures/invalid/comma_missing_rhs.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected expression, found RParen at line 3, column 19"
    );
}

#[test]
fn rejects_pointer_bitwise_operations() {
    let program = include_str!("fixtures/invalid/pointer_bitwise_operation.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "pointer bitwise operations are not supported"
    );
}

#[test]
fn rejects_pointer_bitwise_compound_assignments() {
    let program = include_str!("fixtures/invalid/pointer_bitwise_compound_assignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "pointer bitwise operations are not supported"
    );
}

#[test]
fn reports_invalid_shift_counts() {
    let negative = include_str!("fixtures/invalid/negative_shift_count.c");
    let too_large = include_str!("fixtures/invalid/shift_count_too_large.c");

    assert_eq!(
        interpret(negative).unwrap_err().to_string(),
        "shift count must be non-negative"
    );
    assert_eq!(
        interpret(too_large).unwrap_err().to_string(),
        "shift count too large"
    );
}

#[test]
fn rejects_non_lvalue_increment_decrement_expressions() {
    let program = include_str!("fixtures/invalid/non_lvalue_increment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "invalid increment/decrement target at line 3, column 12"
    );
}

#[test]
fn rejects_non_lvalue_assignment_expressions() {
    let program = include_str!("fixtures/invalid/non_lvalue_assignment_expression.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "invalid assignment target at line 3, column 20"
    );
}

#[test]
fn rejects_non_lvalue_compound_assignment_expressions() {
    let program = include_str!("fixtures/invalid/non_lvalue_compound_assignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "invalid compound assignment target at line 3, column 20"
    );
}

#[test]
fn reports_missing_colon_in_conditional_operator() {
    let program = include_str!("fixtures/invalid/conditional_missing_colon.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ':' after conditional then-expression, found Semi at line 2, column 17"
    );
}

#[test]
fn reports_missing_semicolon_after_do_while_conditions() {
    let program = include_str!("fixtures/invalid/do_while_missing_semicolon.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ';' after do-while condition, found Return at line 6, column 5"
    );
}

#[test]
fn reports_array_element_pointer_index_out_of_bounds() {
    let program = include_str!("fixtures/invalid/pointer_array_element_out_of_bounds.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array pointer index 2 out of bounds for length 2"
    );
}

#[test]
fn reports_pointer_array_index_out_of_bounds() {
    let program = include_str!("fixtures/invalid/pointer_array_out_of_bounds.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array pointer index 2 out of bounds for length 2"
    );
}

#[test]
fn reports_negative_pointer_array_indices() {
    let program = include_str!("fixtures/invalid/pointer_array_negative_index.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array pointer index -1 out of bounds for length 2"
    );
}

#[test]
fn rejects_pointer_comparison_with_nonzero_integer() {
    let program = include_str!("fixtures/invalid/pointer_nonzero_integer_comparison.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot compare pointer with nonzero integer"
    );
}

#[test]
fn rejects_scalar_pointer_arithmetic() {
    let program = include_str!("fixtures/invalid/scalar_pointer_arithmetic.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "scalar pointer arithmetic is not supported"
    );
}

#[test]
fn reports_pointer_arithmetic_out_of_bounds() {
    let program = include_str!("fixtures/invalid/pointer_arithmetic_out_of_bounds.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array pointer index 3 out of bounds for length 2"
    );
}

#[test]
fn rejects_pointer_ordering_comparisons() {
    let program = include_str!("fixtures/invalid/pointer_ordering_comparison.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "pointer ordering comparisons are not supported"
    );
}

#[test]
fn rejects_writes_through_string_literal_pointer_parameters() {
    let program = include_str!("fixtures/invalid/pointer_string_write.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot modify read-only array through pointer"
    );
}

#[test]
fn rejects_writes_through_string_literal_element_addresses() {
    let program = include_str!("fixtures/invalid/string_literal_element_address_write.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot modify read-only array through pointer"
    );
}

#[test]
fn reports_null_pointer_dereferences() {
    let program = include_str!("fixtures/invalid/null_pointer_dereference.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "null pointer dereference");
}

#[test]
fn reports_pointer_dereferences_after_scalar_scope_ends() {
    let program = include_str!("fixtures/invalid/pointer_out_of_scope.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "pointer to out-of-scope variable 'x'");
}

#[test]
fn rejects_writes_through_string_literal_array_parameters() {
    let program = include_str!("fixtures/invalid/string_literal_assignment.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot modify read-only array through pointer"
    );
}

#[test]
fn rejects_multi_character_char_literals() {
    let program = include_str!("fixtures/invalid/unterminated_char_literal.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "unterminated character literal at line 2, column 12\n    return 'ab';\n           ^"
    );
}

#[test]
fn reports_function_name_when_recursive_calls_exceed_depth_limit() {
    let program = include_str!("fixtures/invalid/recursive_call_depth_limit.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "function call depth limit exceeded while calling 'recurse'"
    );
}

#[test]
fn rejects_calls_to_undefined_functions() {
    let program = include_str!("fixtures/invalid/undefined_function_call.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "undefined function 'missing'");
}

#[test]
fn rejects_function_calls_with_wrong_argument_count() {
    let program = include_str!("fixtures/invalid/function_arity_mismatch.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "function 'add' expected 2 arguments, got 1"
    );
}

#[test]
fn reports_array_index_out_of_bounds() {
    let program = include_str!("fixtures/invalid/array_index_out_of_bounds.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array 'values' index 2 out of bounds for length 2"
    );
}

#[test]
fn reports_negative_array_indices() {
    let program = include_str!("fixtures/invalid/array_negative_index.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array 'values' index -1 out of bounds for length 2"
    );
}

#[test]
fn expression_statements_evaluate_their_expression() {
    let program = include_str!("fixtures/invalid/expression_statement_undefined_variable.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "undefined variable 'missing'");
}

#[test]
fn reports_missing_semicolon_after_break_statements() {
    let program = "int main() {\nwhile (1) {\nbreak\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ';' after break statement, found RBrace at line 4, column 1"
    );
}

#[test]
fn reports_missing_semicolon_after_continue_statements() {
    let program = "int main() {\nwhile (1) {\ncontinue\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ';' after continue statement, found RBrace at line 4, column 1"
    );
}

#[test]
fn reports_missing_semicolon_after_for_conditions() {
    let program = "int main() {\nfor (int i = 0; i < 3 i = i + 1) {\nreturn i;\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected ';' after for condition, found Ident(\"i\") at line 2, column 23"
    );
}

#[test]
fn rejects_break_in_for_initializers() {
    let program = "int main() {\nfor (break; 1; 1) {\nreturn 1;\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "break is not allowed in for initializer at line 2, column 6"
    );
}

#[test]
fn rejects_continue_in_for_increments() {
    let program = "int main() {\nfor (; 1; continue) {\nreturn 1;\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "continue is not allowed in for increment at line 2, column 11"
    );
}

#[test]
fn rejects_break_outside_loops() {
    let program = include_str!("fixtures/invalid/break_outside_loop.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "break outside loop");
}

#[test]
fn rejects_continue_outside_loops() {
    let program = include_str!("fixtures/invalid/continue_outside_loop.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "continue outside loop");
}

#[test]
fn for_loop_initializer_variables_are_scoped_to_the_loop() {
    let program = include_str!("fixtures/invalid/for_loop_initializer_scope.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "undefined variable 'i'");
}

#[test]
fn rejects_variables_after_their_block_scope_ends() {
    let program = include_str!("fixtures/invalid/block_scope_leak.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "undefined variable 'inner'");
}

#[test]
fn rejects_redeclaration_only_in_the_same_block_scope() {
    let program = include_str!("fixtures/invalid/same_scope_redeclaration.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "variable 'x' already declared in this scope"
    );
}
