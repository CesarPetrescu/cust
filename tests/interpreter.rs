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
fn expression_statements_evaluate_their_expression() {
    let program = include_str!("fixtures/invalid/expression_statement_undefined_variable.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "undefined variable 'missing'");
}

#[test]
fn rejects_break_without_required_semicolon() {
    let program = "int main() {\nwhile (1) {\nbreak\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected Semi, found RBrace at line 4, column 1"
    );
}

#[test]
fn rejects_continue_without_required_semicolon() {
    let program = "int main() {\nwhile (1) {\ncontinue\n}\nreturn 0;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected Semi, found RBrace at line 4, column 1"
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
