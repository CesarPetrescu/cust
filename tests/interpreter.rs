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
fn reports_line_and_column_for_parser_expression_errors() {
    let program = "int main() {\nreturn ;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected expression, found Semi at line 2, column 8"
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
fn reports_missing_opening_braces_after_if_conditions() {
    let program = "int main() {\nif (1) return 1;\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '{' after if condition, found Return at line 2, column 8"
    );
}

#[test]
fn reports_missing_opening_braces_after_else_keywords() {
    let program = "int main() {\nif (0) { return 0; } else return 1;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '{' after else, found Return at line 2, column 27"
    );
}

#[test]
fn reports_missing_opening_braces_after_while_conditions() {
    let program = "int main() {\nwhile (1) return 1;\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '{' after while condition, found Return at line 2, column 11"
    );
}

#[test]
fn reports_missing_opening_braces_after_for_clauses() {
    let program = "int main() {\nfor (int i = 0; i < 3; i = i + 1) return i;\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected '{' after for clauses, found Return at line 2, column 35"
    );
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
fn reports_pointer_array_index_out_of_bounds() {
    let program = include_str!("fixtures/invalid/pointer_array_out_of_bounds.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array pointer index 2 out of bounds for length 2"
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

    assert_eq!(err.to_string(), "cannot modify read-only array 'text'");
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
