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
