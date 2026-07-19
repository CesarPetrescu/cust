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
fn rejects_invalid_start_expression_statements_with_context() {
    let program = "int main() {\n[;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected expression statement, found LBracket at line 2, column 1"
    );

    let program = "int main() {\n?;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected expression statement, found Question at line 2, column 1"
    );

    let program = "int main() {\n,;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected expression statement, found Comma at line 2, column 1"
    );

    let program = "int main() {\n.field;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected expression statement, found Dot at line 2, column 1"
    );

    let program = "int main() {\n->field;\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected expression statement, found Arrow at line 2, column 1"
    );
}

#[test]
fn rejects_colon_invalid_starts_in_expression_boundaries_with_context() {
    let cases = [
        (
            "int callee(int value) { return value; }\nint main(void) { return callee(:); }\n",
            "expected function call argument, found Colon at line 2, column 32",
        ),
        (
            "int callee(int left, int right) { return left + right; }\nint main(void) { return callee(1, :); }\n",
            "expected function call argument after ',', found Colon at line 2, column 35",
        ),
        (
            "int main(void) {\nint values[2] = {1, 2};\nreturn values[:];\n}\n",
            "expected array index expression, found Colon at line 3, column 15",
        ),
        (
            "int main(void) {\nint values[2] = {:};\nreturn values[0];\n}\n",
            "expected initializer element in array 'values' initializer, found Colon at line 2, column 18",
        ),
        (
            "int main(void) {\nreturn :;\n}\n",
            "expected expression after return, found Colon at line 2, column 8",
        ),
        (
            "int main(void) {\nif (:) return 1;\nreturn 0;\n}\n",
            "expected expression after if, found Colon at line 2, column 5",
        ),
        (
            "int main(void) {\nswitch (:) { default: return 0; }\n}\n",
            "expected expression after switch, found Colon at line 2, column 9",
        ),
        (
            "int main(void) {\nreturn (:);\n}\n",
            "expected grouped expression, found Colon at line 2, column 9",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_invalid_start_array_designator_indexes_with_keyword_context() {
    let program = "int main() {\nint values[2] = {[int] = 1};\nreturn values[0];\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected array designator index before 'int' at line 2, column 19"
    );

    let program = "int main() {\nint *values = (int[]){[return] = 1};\nreturn values[0];\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected array designator index before 'return' at line 2, column 24"
    );

    let program = r#"
struct Point { int x; int y; };
int main(void) {
    struct Point points[2] = {[struct] = {1, 2}};
    return points[0].x;
}
"#;

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected array designator index before 'struct' at line 4, column 32"
    );

    let program = "int main() {\nint values[2] = {[:] = 1};\nreturn values[0];\n}\n";

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "expected array designator index before ':' at line 2, column 19"
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

    let program = include_str!("fixtures/invalid/enum_forward_declaration.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "forward enum declarations are not supported at line 1, column 11"
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
fn rejects_flexible_array_aggregate_fields_with_context() {
    let program = include_str!("fixtures/invalid/flexible_array_aggregate_field.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "flexible array aggregate fields are not supported at line 3, column 14"
    );
}

#[test]
fn rejects_floating_point_type_specifiers_with_context() {
    let cases = [
        (
            "float global_value;\nint main(void) { return 0; }",
            "floating-point types are not supported at line 1, column 1",
        ),
        (
            "int main(void) {\n    double local_value;\n    return 0;\n}",
            "floating-point types are not supported at line 2, column 5",
        ),
        (
            "int take(float value) { return 0; }\nint main(void) { return take(1); }",
            "floating-point types are not supported at line 1, column 10",
        ),
        (
            "int main(void) {\n    return sizeof(float);\n}",
            "floating-point types are not supported at line 2, column 19",
        ),
        (
            "int main(void) {\n    return (double)1;\n}",
            "floating-point types are not supported at line 2, column 13",
        ),
        (
            "struct Sample {\n    float value;\n};\nint main(void) { return 0; }",
            "floating-point types are not supported at line 2, column 5",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_complex_type_specifiers_with_context() {
    let cases = [
        (
            "_Complex global_value;\nint main(void) { return 0; }",
            "complex types are not supported at line 1, column 1",
        ),
        (
            "int main(void) {\n    _Imaginary local_value;\n    return 0;\n}",
            "complex types are not supported at line 2, column 5",
        ),
        (
            "int take(_Complex value) { return 0; }\nint main(void) { return take(1); }",
            "complex types are not supported at line 1, column 10",
        ),
        (
            "int main(void) {\n    return sizeof(_Complex);\n}",
            "complex types are not supported at line 2, column 19",
        ),
        (
            "int main(void) {\n    return (_Imaginary)1;\n}",
            "complex types are not supported at line 2, column 13",
        ),
        (
            "struct Sample {\n    _Complex value;\n};\nint main(void) { return 0; }",
            "complex types are not supported at line 2, column 5",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_invalid_scalar_type_specifier_combinations_with_context() {
    let program = include_str!("fixtures/invalid/invalid_scalar_type_specifier_combination.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid scalar type specifier combination at line 2, column 12"
    );
}

#[test]
fn rejects_unmatched_top_level_delimiters_with_context() {
    let cases = [
        (
            ")\nint main(void) { return 0; }\n",
            "unmatched ')' at top level at line 1, column 1",
        ),
        (
            "]\nint main(void) { return 0; }\n",
            "unmatched ']' at top level at line 1, column 1",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_invalid_start_top_level_declarations_with_context() {
    let cases = [
        (
            "{ int value; }\nint main(void) { return 0; }\n",
            "expected top-level declaration, found LBrace at line 1, column 1",
        ),
        (
            ",\nint main(void) { return 0; }\n",
            "expected top-level declaration, found Comma at line 1, column 1",
        ),
        (
            "?\nint main(void) { return 0; }\n",
            "expected top-level declaration, found Question at line 1, column 1",
        ),
        (
            ".field\nint main(void) { return 0; }\n",
            "expected top-level declaration, found Dot at line 1, column 1",
        ),
        (
            "->field\nint main(void) { return 0; }\n",
            "expected top-level declaration, found Arrow at line 1, column 1",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_unhandled_control_flow_with_context() {
    let cases = [
        (
            "return 7;\nint main(void) { return 0; }",
            "return outside function at line 1, column 1",
        ),
        (
            "break;\nint main(void) { return 0; }",
            "break outside loop or switch at line 1, column 1",
        ),
        (
            "continue;\nint main(void) { return 0; }",
            "continue outside loop at line 1, column 1",
        ),
        (
            "if (1) { return 1; }\nint main(void) { return 0; }",
            "if statement outside function at line 1, column 1",
        ),
        (
            "else { return 1; }\nint main(void) { return 0; }",
            "else without matching if at line 1, column 1",
        ),
        (
            "while (1) { break; }\nint main(void) { return 0; }",
            "while loop outside function at line 1, column 1",
        ),
        (
            "do { break; } while (0);\nint main(void) { return 0; }",
            "do loop outside function at line 1, column 1",
        ),
        (
            "for (;;) { break; }\nint main(void) { return 0; }",
            "for loop outside function at line 1, column 1",
        ),
        (
            "switch (1) { case 1: break; }\nint main(void) { return 0; }",
            "switch statement outside function at line 1, column 1",
        ),
        (
            "case 1: return 1;\nint main(void) { return 0; }",
            "case label outside switch at line 1, column 1",
        ),
        (
            "default: return 1;\nint main(void) { return 0; }",
            "default label outside switch at line 1, column 1",
        ),
        (
            "int main(void) {\n    else { return 1; }\n    return 0;\n}",
            "else without matching if at line 2, column 5",
        ),
        (
            "int main(void) {\n    break;\n    return 0;\n}",
            "break outside loop or switch at line 2, column 5",
        ),
        (
            "int main(void) {\n    continue;\n    return 0;\n}",
            "continue outside loop at line 2, column 5",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_invalid_start_for_initializer_expressions_with_context() {
    let cases = [
        (
            "int main(void) {\n    for ([; ; ) { }\n    return 0;\n}\n",
            "expected expression after for initializer, found LBracket at line 2, column 10",
        ),
        (
            "int main(void) {\n    for (?; ; ) { }\n    return 0;\n}\n",
            "expected expression after for initializer, found Question at line 2, column 10",
        ),
        (
            "int main(void) {\n    for (,; ; ) { }\n    return 0;\n}\n",
            "expected expression after for initializer, found Comma at line 2, column 10",
        ),
        (
            "int main(void) {\n    for (.field; ; ) { }\n    return 0;\n}\n",
            "expected expression after for initializer, found Dot at line 2, column 10",
        ),
        (
            "int main(void) {\n    for (->field; ; ) { }\n    return 0;\n}\n",
            "expected expression after for initializer, found Arrow at line 2, column 10",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_statement_only_control_flow_in_for_clauses_with_context() {
    let cases = [
        (
            "int main(void) {\n    for (return 1; ; ) { }\n    return 0;\n}",
            "return is not allowed in for initializer at line 2, column 10",
        ),
        (
            "int main(void) {\n    for (if (1) { return 1; }; ; ) { }\n    return 0;\n}",
            "if statement is not allowed in for initializer at line 2, column 10",
        ),
        (
            "int main(void) {\n    for (else { return 1; }; ; ) { }\n    return 0;\n}",
            "else without matching if at line 2, column 10",
        ),
        (
            "int main(void) {\n    for (while (1) { break; }; ; ) { }\n    return 0;\n}",
            "while loop is not allowed in for initializer at line 2, column 10",
        ),
        (
            "int main(void) {\n    for (switch (1) { default: break; }; ; ) { }\n    return 0;\n}",
            "switch statement is not allowed in for initializer at line 2, column 10",
        ),
        (
            "int main(void) {\n    for (case 1: return 1; ; ) { }\n    return 0;\n}",
            "case label outside switch at line 2, column 10",
        ),
        (
            "int main(void) {\n    for (; 1; return 1) { }\n    return 0;\n}",
            "return is not allowed in for increment at line 2, column 15",
        ),
        (
            "int main(void) {\n    for (; 1; if (1) { return 1; }) { }\n    return 0;\n}",
            "if statement is not allowed in for increment at line 2, column 15",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_restrict_on_non_pointer_declarations_with_context() {
    let program = include_str!("fixtures/invalid/restrict_scalar_declaration.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "restrict qualifiers are only supported on pointer declarators at line 2, column 5"
    );

    let program = include_str!("fixtures/invalid/restrict_scalar_parameter.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "restrict qualifiers are only supported on pointer declarators at line 1, column 10"
    );

    let program = include_str!("fixtures/invalid/restrict_scalar_aggregate_field.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "restrict qualifiers are only supported on pointer declarators at line 2, column 5"
    );
}

#[test]
fn rejects_restrict_on_non_pointer_type_names_with_context() {
    let cases = [
        (
            "int main(void) { return sizeof(restrict int); }\n",
            "restrict qualifiers are only supported on pointer declarators at line 1, column 32",
        ),
        (
            "int main(void) { return _Alignof(restrict int); }\n",
            "restrict qualifiers are only supported on pointer declarators at line 1, column 34",
        ),
        (
            "int main(void) { return (restrict int)0; }\n",
            "restrict qualifiers are only supported on pointer declarators at line 1, column 26",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_qualified_pointer_to_pointer_type_names_with_context() {
    let cases = [
        (
            "int main(void) { return sizeof(int * const *); }\n",
            "pointer-to-pointer sizeof types are not supported at line 1, column 44",
        ),
        (
            "int main(void) { return _Alignof(int * const *); }\n",
            "pointer-to-pointer _Alignof types are not supported at line 1, column 46",
        ),
        (
            "int main(void) { return (int * const *)0 != 0; }\n",
            "pointer-to-pointer casts are not supported at line 1, column 38",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_pointer_array_cast_type_names_with_context() {
    let cases = [
        (
            "int main(void) { return (int *[2])0; }\n",
            "pointer array casts are not supported at line 1, column 31",
        ),
        (
            "struct Point { int x; };\nint main(void) { return (struct Point *[2])0; }\n",
            "pointer array casts are not supported at line 2, column 40",
        ),
        (
            "int main(void) { return (struct { int x; } *[2])0; }\n",
            "pointer array casts are not supported at line 1, column 45",
        ),
        (
            "typedef int *IntPtr;\nint main(void) { return (IntPtr[2])0; }\n",
            "pointer array casts are not supported at line 2, column 32",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_pointer_array_sizeof_and_alignof_type_names_with_context() {
    let cases = [
        (
            "int main(void) { return sizeof(int *[2]); }\n",
            "pointer array sizeof types are not supported at line 1, column 37",
        ),
        (
            "int main(void) { return _Alignof(int *[2]); }\n",
            "pointer array _Alignof types are not supported at line 1, column 39",
        ),
        (
            "struct Point { int x; };\nint main(void) { return sizeof(struct Point *[2]); }\n",
            "pointer array sizeof types are not supported at line 2, column 46",
        ),
        (
            "int main(void) { return _Alignof(struct { int x; } *[2]); }\n",
            "pointer array _Alignof types are not supported at line 1, column 53",
        ),
        (
            "typedef int *IntPtr;\nint main(void) { return sizeof(IntPtr[2]); }\n",
            "pointer array sizeof types are not supported at line 2, column 38",
        ),
        (
            "typedef struct Point { int x; } *PointPtr;\nint main(void) { return _Alignof(PointPtr[2]); }\n",
            "pointer array _Alignof types are not supported at line 2, column 42",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_function_type_cast_and_type_query_names_with_context() {
    let cases = [
        (
            "int main(void) { return (int(void))0; }\n",
            "function casts are not supported at line 1, column 29",
        ),
        (
            "int main(void) { return sizeof(int(void)); }\n",
            "function sizeof types are not supported at line 1, column 35",
        ),
        (
            "int main(void) { return _Alignof(int(void)); }\n",
            "function _Alignof types are not supported at line 1, column 37",
        ),
        (
            "int main(void) { return (int *(void))0; }\n",
            "function casts are not supported at line 1, column 31",
        ),
        (
            "int main(void) { return sizeof(int *(void)); }\n",
            "function sizeof types are not supported at line 1, column 37",
        ),
        (
            "int main(void) { return _Alignof(int *(void)); }\n",
            "function _Alignof types are not supported at line 1, column 39",
        ),
        (
            "struct Point { int x; };\nint main(void) { return sizeof(struct Point *(void)); }\n",
            "function sizeof types are not supported at line 2, column 46",
        ),
        (
            "typedef int *IntPtr;\nint main(void) { return _Alignof(IntPtr(void)); }\n",
            "function _Alignof types are not supported at line 2, column 40",
        ),
        (
            "typedef int *IntPtr;\nint main(void) { return (IntPtr(void))0; }\n",
            "function casts are not supported at line 2, column 32",
        ),
        (
            "int main(void) { return (struct { int x; } *(void))0; }\n",
            "function casts are not supported at line 1, column 45",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_function_array_cast_and_type_query_names_with_context() {
    let cases = [
        (
            "int main(void) { return (int[2](void))0; }\n",
            "function casts are not supported at line 1, column 32",
        ),
        (
            "int main(void) { return sizeof(int[2](void)); }\n",
            "function sizeof types are not supported at line 1, column 38",
        ),
        (
            "int main(void) { return _Alignof(int[2](void)); }\n",
            "function _Alignof types are not supported at line 1, column 40",
        ),
        (
            "struct Point { int x; };\nint main(void) { return sizeof(struct Point[2](void)); }\n",
            "function sizeof types are not supported at line 2, column 47",
        ),
        (
            "typedef int Scores[2];\nint main(void) { return _Alignof(Scores(void)); }\n",
            "function _Alignof types are not supported at line 2, column 40",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_multidimensional_array_cast_type_names_with_context() {
    let cases = [
        (
            "int main(void) { return (int[2][3]){1}; }\n",
            "multidimensional array casts are not supported at line 1, column 32",
        ),
        (
            "struct Point { int x; };\nint main(void) { return (struct Point[2][3]){{{1}}}[0][0].x; }\n",
            "multidimensional array casts are not supported at line 2, column 41",
        ),
        (
            "int main(void) { return (struct { int x; }[2][3]){{{1}}}[0][0].x; }\n",
            "multidimensional array casts are not supported at line 1, column 46",
        ),
        (
            "typedef int Scores[2];\nint main(void) { return (Scores[3]){{1, 2}}[0][0]; }\n",
            "multidimensional array casts are not supported at line 2, column 32",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_pointer_to_array_cast_and_type_query_names_with_context() {
    let cases = [
        (
            "int main(void) { return (int[2]*)0 == 0; }\n",
            "pointer-to-array casts are not supported at line 1, column 32",
        ),
        (
            "struct Point { int x; };\nint main(void) { return (struct Point[2]*)0 == 0; }\n",
            "pointer-to-array casts are not supported at line 2, column 41",
        ),
        (
            "int main(void) { return (struct { int x; }[2]*)0 == 0; }\n",
            "pointer-to-array casts are not supported at line 1, column 46",
        ),
        (
            "int main(void) { return sizeof(int[2]*); }\n",
            "pointer-to-array sizeof types are not supported at line 1, column 38",
        ),
        (
            "struct Point { int x; };\nint main(void) { return _Alignof(struct Point[2]*); }\n",
            "pointer-to-array _Alignof types are not supported at line 2, column 49",
        ),
        (
            "int main(void) { return sizeof(struct { int x; }[2]*); }\n",
            "pointer-to-array sizeof types are not supported at line 1, column 52",
        ),
        (
            "typedef int Scores[2];\nint main(void) { return sizeof(Scores*); }\n",
            "pointer-to-array sizeof types are not supported at line 2, column 38",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_void_array_and_function_type_names_with_context() {
    let cases = [
        (
            "int main(void) { return (void(void))0; }\n",
            "function casts are not supported at line 1, column 30",
        ),
        (
            "int main(void) { return sizeof(void(void)); }\n",
            "function sizeof types are not supported at line 1, column 36",
        ),
        (
            "int main(void) { return _Alignof(void(void)); }\n",
            "function _Alignof types are not supported at line 1, column 38",
        ),
        (
            "int main(void) { return (void[2])0; }\n",
            "void array casts are not supported at line 1, column 30",
        ),
        (
            "int main(void) { return sizeof(void[2]); }\n",
            "void array sizeof types are not supported at line 1, column 36",
        ),
        (
            "int main(void) { return _Alignof(void[2]); }\n",
            "void array _Alignof types are not supported at line 1, column 38",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_multidimensional_array_designators_with_context() {
    let cases = [
        (
            "int main(void) { int values[2] = {[0][1] = 3}; return values[0]; }\n",
            "multidimensional array designators are not supported at line 1, column 38",
        ),
        (
            "int main(void) { return (int[2]){[0][1] = 3}[0]; }\n",
            "multidimensional array designators are not supported at line 1, column 37",
        ),
        (
            "struct Point { int x; };\nint main(void) { struct Point points[2] = {[0][1] = {3}}; return 0; }\n",
            "multidimensional array designators are not supported at line 2, column 47",
        ),
        (
            "struct Packet { int values[2]; };\nint main(void) { struct Packet packet = {.values[0][1] = 3}; return 0; }\n",
            "multidimensional array designators are not supported at line 2, column 52",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn supports_post_star_pointer_qualifiers_in_type_names() {
    assert_eq!(
        interpret("int main(void) { return sizeof(int * const); }\n").unwrap(),
        8
    );
    assert_eq!(
        interpret("int main(void) { return _Alignof(int * volatile); }\n").unwrap(),
        8
    );
    assert_eq!(
        interpret("int main(void) { return (int * const)0 == 0; }\n").unwrap(),
        1
    );
}

#[test]
fn rejects_missing_atomic_type_arguments_with_context() {
    let cases = [
        (
            "_Atomic() global_value;\nint main(void) { return 0; }\n",
            "expected _Atomic type, found RParen at line 1, column 9",
        ),
        (
            "int main(void) {\n    _Atomic(,) local_value;\n    return 0;\n}\n",
            "expected _Atomic type, found Comma at line 2, column 13",
        ),
        (
            "int take(_Atomic() value) { return 0; }\nint main(void) { return take(1); }\n",
            "expected _Atomic type, found RParen at line 1, column 18",
        ),
        (
            "int main(void) {\n    return sizeof(_Atomic());\n}\n",
            "expected _Atomic type, found RParen at line 2, column 27",
        ),
        (
            "int main(void) {\n    return (_Atomic())0;\n}\n",
            "expected _Atomic type, found RParen at line 2, column 21",
        ),
        (
            "struct Sample {\n    _Atomic() value;\n};\nint main(void) { return 0; }\n",
            "expected _Atomic type, found RParen at line 2, column 13",
        ),
        (
            "_Atomic([) global_value;\nint main(void) { return 0; }\n",
            "expected _Atomic type before '[' at line 1, column 9",
        ),
        (
            "int main(void) {\n    _Atomic(?) local_value;\n    return 0;\n}\n",
            "expected _Atomic type before '?' at line 2, column 13",
        ),
        (
            "_Atomic({) global_value;\nint main(void) { return 0; }\n",
            "expected _Atomic type before '{' at line 1, column 9",
        ),
        (
            "int main(void) {\n    return sizeof(_Atomic({));\n}\n",
            "expected _Atomic type before '{' at line 2, column 27",
        ),
        (
            "int take(_Atomic(return) value) { return 0; }\nint main(void) { return take(1); }\n",
            "expected _Atomic type before 'return' at line 1, column 18",
        ),
        (
            "int main(void) {\n    return sizeof(_Atomic(:));\n}\n",
            "expected _Atomic type before ':' at line 2, column 27",
        ),
        (
            "int main(void) {\n    return sizeof(_Atomic(.));\n}\n",
            "expected _Atomic type before '.' at line 2, column 27",
        ),
        (
            "struct Sample {\n    _Atomic(->field) value;\n};\nint main(void) { return 0; }\n",
            "expected _Atomic type before '->' at line 2, column 13",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_void_pointer_forms_with_context() {
    let program = include_str!("fixtures/invalid/void_pointer_return.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "void pointers are not supported at line 1, column 6"
    );

    let program = include_str!("fixtures/invalid/void_pointer_parameter.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "void pointers are not supported at line 1, column 18"
    );

    let program = include_str!("fixtures/invalid/void_pointer_local_declaration.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "void pointers are not supported at line 2, column 10"
    );

    let program = include_str!("fixtures/invalid/void_pointer_cast.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "void pointers are not supported at line 2, column 18"
    );

    let program = include_str!("fixtures/invalid/void_pointer_sizeof.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "void pointers are not supported at line 2, column 24"
    );

    let program = include_str!("fixtures/invalid/void_pointer_alignof.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "void pointers are not supported at line 2, column 26"
    );
}

#[test]
fn rejects_void_object_and_typedef_forms_with_context() {
    let cases = [
        (
            include_str!("fixtures/invalid/void_global_object.c"),
            "void object declarations are not supported at line 1, column 1",
        ),
        (
            include_str!("fixtures/invalid/void_local_object.c"),
            "void object declarations are not supported at line 2, column 5",
        ),
        (
            include_str!("fixtures/invalid/void_parameter_name.c"),
            "void parameter lists must be empty at line 1, column 10",
        ),
        (
            include_str!("fixtures/invalid/void_aggregate_field.c"),
            "void object declarations are not supported at line 2, column 5",
        ),
        (
            include_str!("fixtures/invalid/void_typedef_alias.c"),
            "void typedef aliases are not supported at line 1, column 9",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_parenthesized_pointer_cast_and_type_query_forms_with_context() {
    let program = include_str!("fixtures/invalid/parenthesized_pointer_cast.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "parenthesized pointer casts are not supported at line 2, column 18"
    );

    let program = include_str!("fixtures/invalid/parenthesized_pointer_sizeof_type.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "parenthesized pointer sizeof types are not supported at line 2, column 24"
    );

    let program = include_str!("fixtures/invalid/parenthesized_pointer_alignof_type.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "parenthesized pointer _Alignof types are not supported at line 2, column 26"
    );
}

#[test]
fn rejects_missing_cast_operands_with_context() {
    let cases = [
        (
            include_str!("fixtures/invalid/scalar_cast_missing_operand.c"),
            "expected expression after cast, found Semi at line 2, column 17",
        ),
        (
            include_str!("fixtures/invalid/pointer_cast_missing_operand.c"),
            "expected expression after cast, found Semi at line 2, column 20",
        ),
        (
            include_str!("fixtures/invalid/void_cast_missing_operand.c"),
            "expected expression after cast, found Semi at line 2, column 11",
        ),
        (
            "int main(void) {\n    return (int)[;\n}\n",
            "expected expression after cast, found LBracket at line 2, column 17",
        ),
        (
            "int main(void) {\n    return (char)?;\n}\n",
            "expected expression after cast, found Question at line 2, column 18",
        ),
        (
            "int main(void) {\n    return (int)return;\n}\n",
            "expected expression after cast, found Return at line 2, column 17",
        ),
        (
            "int main(void) { return (int).; }",
            "expected expression after cast, found Dot at line 1, column 30",
        ),
        (
            "int main(void) { return (int)->field; }",
            "expected expression after cast, found Arrow at line 1, column 30",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_missing_cast_types_after_qualifiers_with_context() {
    let cases = [
        (
            "int main(void) {\n    return (const);\n}\n",
            "expected cast type after type qualifier 'const', found RParen at line 2, column 18",
        ),
        (
            "int main(void) {\n    return (volatile);\n}\n",
            "expected cast type after type qualifier 'volatile', found RParen at line 2, column 21",
        ),
        (
            "int main(void) {\n    return (_Atomic);\n}\n",
            "expected cast type after type qualifier '_Atomic', found RParen at line 2, column 20",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_qualifier_only_declaration_types_with_context() {
    let cases = [
        (
            "const global;\nint main(void) { return 0; }\n",
            "expected declaration type after type qualifier 'const', found Ident(\"global\") at line 1, column 7",
        ),
        (
            "int main(void) {\n    volatile local;\n    return 0;\n}\n",
            "expected declaration type after type qualifier 'volatile', found Ident(\"local\") at line 2, column 14",
        ),
        (
            "int take(_Atomic value) { return 0; }\nint main(void) { return take(1); }\n",
            "expected parameter type after type qualifier '_Atomic', found Ident(\"value\") at line 1, column 18",
        ),
        (
            "struct Sample {\n    const field;\n};\nint main(void) { return 0; }\n",
            "expected struct field type after type qualifier 'const', found Ident(\"field\") at line 2, column 11",
        ),
        (
            "typedef const Alias;\nint main(void) { return 0; }\n",
            "expected typedef alias type after type qualifier 'const', found Ident(\"Alias\") at line 1, column 15",
        ),
        (
            "const make(void) { return 0; }\nint main(void) { return make(); }\n",
            "expected declaration type after type qualifier 'const', found Ident(\"make\") at line 1, column 7",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_non_constant_array_lengths_with_context() {
    let program = include_str!("fixtures/invalid/array_length_non_constant_identifier.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "array length must be an integer constant expression at line 3, column 16"
    );
}

#[test]
fn rejects_comma_operator_in_array_length_integer_constant_expressions() {
    let program = include_str!("fixtures/invalid/array_length_comma_expression.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "comma operator is not allowed in integer constant expression at line 2, column 17"
    );
}

#[test]
fn rejects_delimiter_only_array_lengths_with_context() {
    let cases = [
        (
            "int main(void) {\n    int values[);\n    return 0;\n}\n",
            "expected array length before ')' at line 2, column 16",
        ),
        (
            "int f(int values[));\nint main(void) { return 0; }\n",
            "expected array length before ')' at line 1, column 18",
        ),
        (
            "struct Packet { int values[); };\nint main(void) { return 0; }\n",
            "expected array length before ')' at line 1, column 28",
        ),
        (
            "typedef int Values[);\nint main(void) { return 0; }\n",
            "expected array length before ')' at line 1, column 20",
        ),
        (
            "int main(void) {\n    return sizeof(int[);\n}\n",
            "expected array length before ')' at line 2, column 23",
        ),
        (
            "int main(void) {\n    int values[;\n    return 0;\n}\n",
            "expected array length before ';' at line 2, column 16",
        ),
        (
            "int main(void) {\n    int values[}\n",
            "expected array length before '}' at line 2, column 16",
        ),
        (
            "int main(void) {\n    int values[?];\n    return 0;\n}\n",
            "expected array length before '?' at line 2, column 16",
        ),
        (
            "int main(void) {\n    return sizeof(int[[);\n}\n",
            "expected array length before '[' at line 2, column 23",
        ),
        (
            "int main(void) {\n    int values[int];\n    return 0;\n}\n",
            "expected array length before 'int' at line 2, column 16",
        ),
        (
            "int f(char text[char]);\nint main(void) { return 0; }\n",
            "expected array length before 'char' at line 1, column 17",
        ),
        (
            "struct Packet { int values[struct]; };\nint main(void) { return 0; }\n",
            "expected array length before 'struct' at line 1, column 28",
        ),
        (
            "typedef int Values[return];\nint main(void) { return 0; }\n",
            "expected array length before 'return' at line 1, column 20",
        ),
        (
            "int main(void) {\n    return sizeof(int[while]);\n}\n",
            "expected array length before 'while' at line 2, column 23",
        ),
        (
            "int main(void) {\n    int values[.field];\n    return 0;\n}\n",
            "expected array length before '.' at line 2, column 16",
        ),
        (
            "struct Packet { int values[->field]; };\nint main(void) { return 0; }\n",
            "expected array length before '->' at line 1, column 28",
        ),
        (
            "int main(void) {\n    int values[{1}];\n    return 0;\n}\n",
            "expected array length before '{' at line 2, column 16",
        ),
        (
            "int main(void) {\n    int values[:];\n    return 0;\n}\n",
            "expected array length before ':' at line 2, column 16",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_star_vla_array_lengths_with_context() {
    let program = include_str!("fixtures/invalid/array_length_star_vla.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "variable length array declarators are not supported at line 1, column 21"
    );

    let program = include_str!("fixtures/invalid/array_length_star_local.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "variable length array declarators are not supported at line 2, column 16"
    );

    let program = include_str!("fixtures/invalid/array_length_star_aggregate_field.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "variable length array declarators are not supported at line 2, column 16"
    );

    let program = include_str!("fixtures/invalid/array_length_star_typedef.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "variable length array declarators are not supported at line 1, column 20"
    );
}

#[test]
fn rejects_parenthesized_pointer_aggregate_fields_with_context() {
    let program = include_str!("fixtures/invalid/function_pointer_aggregate_field.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "function pointer aggregate fields are not supported at line 2, column 9"
    );

    let program = include_str!("fixtures/invalid/parenthesized_pointer_aggregate_field.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "parenthesized pointer aggregate fields are not supported at line 2, column 9"
    );
}

#[test]
fn rejects_anonymous_aggregate_parameters_with_context() {
    let program = include_str!("fixtures/invalid/anonymous_aggregate_parameter.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "anonymous aggregate parameters are not supported at line 1, column 10"
    );
}

#[test]
fn rejects_anonymous_aggregate_return_types_with_context() {
    let program = include_str!("fixtures/invalid/anonymous_aggregate_return_type.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "anonymous aggregate return types are not supported at line 1, column 1"
    );
}

#[test]
fn rejects_old_style_function_parameter_lists_with_context() {
    let program = include_str!("fixtures/invalid/old_style_function_parameters.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "old-style function parameter lists are not supported at line 1, column 9"
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
fn supports_atomic_pointer_type_specifiers() {
    let program = include_str!("fixtures/valid/atomic_pointer_type_specifiers.c");

    assert_eq!(interpret(program).unwrap(), 33);
}

#[test]
fn rejects_unsupported_atomic_pointer_suffixes_with_context() {
    let cases = [
        (
            "int main(void) { return sizeof(_Atomic(int **)); }\n",
            "pointer-to-pointer _Atomic types are not supported at line 1, column 45",
        ),
        (
            "int main(void) { return _Alignof(_Atomic(int *[2])); }\n",
            "pointer array _Atomic types are not supported at line 1, column 47",
        ),
        (
            "int main(void) { return sizeof(_Atomic(int(void))); }\n",
            "function _Atomic types are not supported at line 1, column 43",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_atomic_array_typedef_arguments_with_context() {
    let cases = [
        (
            "typedef int Scores[2];\n_Atomic(Scores) value;\nint main(void) { return 0; }\n",
            "array _Atomic types are not supported at line 2, column 9",
        ),
        (
            "typedef int Scores[2];\nint main(void) { return sizeof(_Atomic(Scores)); }\n",
            "array _Atomic types are not supported at line 2, column 40",
        ),
        (
            "typedef int Scores[2];\nint main(void) { return _Alignof(_Atomic(Scores)); }\n",
            "array _Atomic types are not supported at line 2, column 42",
        ),
        (
            "typedef int Scores[2];\nint main(void) { return ((_Atomic(Scores)){1, 2})[0]; }\n",
            "array _Atomic types are not supported at line 2, column 35",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_nested_and_qualified_atomic_types_with_context() {
    let cases = [
        (
            "_Atomic(_Atomic(int)) value;\nint main(void) { return 0; }\n",
            "nested _Atomic type specifiers are not supported at line 1, column 9",
        ),
        (
            "_Atomic(const int) value;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 1, column 9",
        ),
        (
            "int main(void) { return sizeof(_Atomic(_Atomic(int))); }\n",
            "nested _Atomic type specifiers are not supported at line 1, column 40",
        ),
        (
            "int main(void) { return _Alignof(_Atomic(const int)); }\n",
            "qualified _Atomic types are not supported at line 1, column 42",
        ),
        (
            "int main(void) { return ((_Atomic(_Atomic(int))){1}); }\n",
            "nested _Atomic type specifiers are not supported at line 1, column 35",
        ),
        (
            "int main(void) { return ((_Atomic(const int)){1}); }\n",
            "qualified _Atomic types are not supported at line 1, column 35",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_postfix_qualified_atomic_types_with_context() {
    let cases = [
        (
            "_Atomic(int const) value;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 1, column 13",
        ),
        (
            "int main(void) { return sizeof(_Atomic(int volatile)); }\n",
            "qualified _Atomic types are not supported at line 1, column 44",
        ),
        (
            "int main(void) { return _Alignof(_Atomic(int * const)); }\n",
            "qualified _Atomic types are not supported at line 1, column 48",
        ),
        (
            "_Atomic(_Atomic int) value;\nint main(void) { return 0; }\n",
            "nested _Atomic type specifiers are not supported at line 1, column 9",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_qualified_atomic_typedef_arguments_with_context() {
    let cases = [
        (
            "typedef const int ConstInt;\n_Atomic(ConstInt) value;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 2, column 9",
        ),
        (
            "typedef volatile int VolatileInt;\nint main(void) { return sizeof(_Atomic(VolatileInt)); }\n",
            "qualified _Atomic types are not supported at line 2, column 40",
        ),
        (
            "typedef int * const ConstPtr;\nint main(void) { return _Alignof(_Atomic(ConstPtr)); }\n",
            "qualified _Atomic types are not supported at line 2, column 42",
        ),
        (
            "typedef int * volatile VolatilePtr;\nint main(void) { return ((_Atomic(VolatilePtr)){0}) == 0; }\n",
            "qualified _Atomic types are not supported at line 2, column 35",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn supports_atomic_typedef_pointers_to_qualified_pointees() {
    let program = include_str!("fixtures/valid/atomic_typedef_qualified_pointees.c");

    assert_eq!(interpret(program).unwrap(), 22);
}

#[test]
fn rejects_chained_and_atomic_qualified_typedef_arguments_with_context() {
    let cases = [
        (
            "typedef const int Base;\ntypedef Base Alias;\n_Atomic(Alias) value;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 3, column 9",
        ),
        (
            "typedef volatile int Base;\ntypedef Base Alias;\nint main(void) { return sizeof(_Atomic(Alias)); }\n",
            "qualified _Atomic types are not supported at line 3, column 40",
        ),
        (
            "typedef int * const Slot;\ntypedef Slot Alias;\nint main(void) { return _Alignof(_Atomic(Alias)); }\n",
            "qualified _Atomic types are not supported at line 3, column 42",
        ),
        (
            "typedef const int Value, *View;\nint main(void) { return ((_Atomic(Value)){0}); }\n",
            "qualified _Atomic types are not supported at line 2, column 35",
        ),
        (
            "typedef _Atomic(int) AtomicInt;\n_Atomic(AtomicInt) value;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 2, column 9",
        ),
        (
            "typedef _Atomic(int) AtomicInt;\ntypedef AtomicInt Alias;\nint main(void) { return sizeof(_Atomic(Alias)); }\n",
            "qualified _Atomic types are not supported at line 3, column 40",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn supports_mixed_atomic_typedef_declarators_and_shadowing() {
    let program = include_str!("fixtures/valid/atomic_typedef_chains.c");

    assert_eq!(interpret(program).unwrap(), 19);
}

#[test]
fn rejects_qualified_aggregate_atomic_aliases_in_function_signatures() {
    let cases = [
        (
            "struct Point { int x; };\ntypedef const struct Point ConstPoint;\n_Atomic(ConstPoint) value;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 3, column 9",
        ),
        (
            "struct Point { int x; };\ntypedef const struct Point ConstPoint;\nint take(_Atomic(ConstPoint) value);\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 3, column 18",
        ),
        (
            "struct Point { int x; };\ntypedef const struct Point ConstPoint;\nint take(_Atomic(ConstPoint));\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 3, column 18",
        ),
        (
            "struct Point { int x; };\ntypedef _Atomic(struct Point) AtomicPoint;\nint take(_Atomic(AtomicPoint) value);\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 3, column 18",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn supports_atomic_aggregate_alias_signature_boundaries() {
    let program = include_str!("fixtures/valid/atomic_aggregate_alias_signatures.c");

    assert_eq!(interpret(program).unwrap(), 9);
}

#[test]
fn rejects_qualified_aggregate_atomic_aliases_in_fields_and_returns() {
    let cases = [
        (
            "struct Point { int x; };\ntypedef const struct Point ConstPoint;\nstruct Box {\n    _Atomic(ConstPoint) value;\n};\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 4, column 13",
        ),
        (
            "struct Point { int x; };\ntypedef _Atomic(struct Point) AtomicPoint;\nstruct Box {\n    _Atomic(AtomicPoint) value;\n};\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 4, column 13",
        ),
        (
            "struct Point { int x; };\ntypedef const struct Point ConstPoint;\n_Atomic(ConstPoint) make(void);\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 3, column 9",
        ),
        (
            "struct Point { int x; };\ntypedef _Atomic(struct Point) AtomicPoint;\n_Atomic(AtomicPoint) make(void) { return (struct Point){1}; }\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 3, column 9",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn supports_atomic_aggregate_alias_declaration_boundaries() {
    let program = include_str!("fixtures/valid/atomic_aggregate_alias_declarations.c");

    assert_eq!(interpret(program).unwrap(), 13);
}

#[test]
fn rejects_qualified_and_atomic_enum_aliases_as_atomic_arguments() {
    let cases = [
        (
            "enum State { READY = 1 };\ntypedef const enum State ConstState;\n_Atomic(ConstState) value;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 3, column 9",
        ),
        (
            "enum State { READY = 1 };\ntypedef _Atomic(enum State) AtomicState;\n_Atomic(AtomicState) value;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 3, column 9",
        ),
        (
            "enum State { READY = 1 };\ntypedef const enum State ConstState;\nstruct Box {\n    _Atomic(ConstState) value;\n};\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 4, column 13",
        ),
        (
            "enum State { READY = 1 };\ntypedef const enum State ConstState;\nint take(_Atomic(ConstState) value);\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 3, column 18",
        ),
        (
            "enum State { READY = 1 };\ntypedef const enum State ConstState;\nint main(void) { return sizeof(_Atomic(ConstState)); }\n",
            "qualified _Atomic types are not supported at line 3, column 40",
        ),
        (
            "enum State { READY = 1 };\ntypedef _Atomic(enum State) AtomicState;\nint main(void) { return _Alignof(_Atomic(AtomicState)); }\n",
            "qualified _Atomic types are not supported at line 3, column 42",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn supports_direct_and_typedef_backed_atomic_enum_types() {
    let program = include_str!("fixtures/valid/atomic_enum_types.c");

    assert_eq!(interpret(program).unwrap(), 28);
}

#[test]
fn supports_atomic_inline_enum_type_definitions() {
    let program = include_str!("fixtures/valid/atomic_inline_enum_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 48);
}

#[test]
fn rejects_qualified_and_nested_atomic_inline_enum_types_with_context() {
    let cases = [
        (
            "_Atomic(const enum QualifiedInline { QUALIFIED = 1 }) value;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 1, column 9",
        ),
        (
            "_Atomic(_Atomic(enum NestedInline { NESTED = 1 })) value;\nint main(void) { return 0; }\n",
            "nested _Atomic type specifiers are not supported at line 1, column 9",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn keeps_atomic_inline_enum_constants_lexically_scoped() {
    let cases = [
        (
            "int read(_Atomic(enum ParamAtomic { PARAM_ATOMIC = 6 }) value) {\n    return value + PARAM_ATOMIC;\n}\nint main(void) { return PARAM_ATOMIC; }\n",
            "undefined variable 'PARAM_ATOMIC'",
        ),
        (
            "int main(void) {\n    {\n        _Atomic(enum BlockAtomic { BLOCK_ATOMIC = 7 }) value = BLOCK_ATOMIC;\n    }\n    return BLOCK_ATOMIC;\n}\n",
            "undefined variable 'BLOCK_ATOMIC'",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn supports_atomic_inline_aggregate_type_definitions() {
    let program = include_str!("fixtures/valid/atomic_inline_aggregate_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn rejects_qualified_and_nested_atomic_inline_aggregate_types_with_context() {
    let cases = [
        (
            "_Atomic(const struct QualifiedInline { int value; }) value;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 1, column 9",
        ),
        (
            "_Atomic(_Atomic(union NestedInline { int value; char tag; })) value;\nint main(void) { return 0; }\n",
            "nested _Atomic type specifiers are not supported at line 1, column 9",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn keeps_atomic_inline_aggregate_tags_lexically_scoped() {
    let cases = [
        (
            "int read(_Atomic(struct ParamAtomic { int value; }) value) {\n    return sizeof(value) == sizeof(struct ParamAtomic);\n}\nint main(void) { return sizeof(struct ParamAtomic); }\n",
            "undefined struct type 'ParamAtomic'",
        ),
        (
            "int main(void) {\n    {\n        _Atomic(union BlockAtomic { int value; char tag; }) value;\n    }\n    return sizeof(union BlockAtomic);\n}\n",
            "undefined union type 'BlockAtomic'",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn supports_atomic_anonymous_aggregate_type_definitions() {
    let program = include_str!("fixtures/valid/atomic_anonymous_aggregate_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn supports_atomic_anonymous_aggregate_pointer_type_specifiers() {
    let program =
        include_str!("fixtures/valid/atomic_anonymous_aggregate_pointer_type_specifiers.c");

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn supports_atomic_anonymous_aggregate_pointer_typedef_aliases() {
    let program =
        include_str!("fixtures/valid/atomic_anonymous_aggregate_pointer_typedef_aliases.c");

    assert_eq!(interpret(program).unwrap(), 12);
}

#[test]
fn supports_atomic_anonymous_aggregate_value_typedef_aliases() {
    let program = include_str!("fixtures/valid/atomic_anonymous_aggregate_value_typedef_aliases.c");

    assert_eq!(interpret(program).unwrap(), 11);
}

#[test]
fn supports_derived_declarators_from_atomic_anonymous_aggregate_value_aliases() {
    let program =
        include_str!("fixtures/valid/atomic_anonymous_aggregate_value_alias_derived_declarators.c");

    assert_eq!(interpret(program).unwrap(), 127);
}

#[test]
fn supports_typedef_aliases_of_atomic_anonymous_aggregate_derived_declarators() {
    let program =
        include_str!("fixtures/valid/atomic_anonymous_aggregate_derived_typedef_aliases.c");

    assert_eq!(interpret(program).unwrap(), 255);
}

#[test]
fn supports_atomic_anonymous_aggregate_const_pointer_views() {
    let program = include_str!("fixtures/valid/atomic_anonymous_aggregate_const_pointer_views.c");

    assert_eq!(interpret(program).unwrap(), 255);
}

#[test]
fn supports_atomic_anonymous_aggregate_qualified_value_aliases() {
    let program =
        include_str!("fixtures/valid/atomic_anonymous_aggregate_qualified_value_aliases.c");

    assert_eq!(interpret(program).unwrap(), 255);
}

#[test]
fn supports_atomic_anonymous_aggregate_qualified_array_aliases() {
    let program =
        include_str!("fixtures/valid/atomic_anonymous_aggregate_qualified_array_aliases.c");

    assert_eq!(interpret(program).unwrap(), 255);
}

#[test]
fn rejects_writes_through_atomic_anonymous_aggregate_qualified_array_aliases() {
    let cases = [
        (
            "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
             typedef const AtomicAnonValue ConstAtomicAnonValue;\n\
             typedef ConstAtomicAnonValue ConstAtomicAnonArray[2];\n\
             int main(void) {\n\
                 ConstAtomicAnonArray values;\n\
                 values[0].value = 7;\n\
                 return 0;\n\
             }\n",
            "cannot assign to const variable 'values'",
        ),
        (
            "typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;\n\
             typedef const AtomicAnonUnion ConstAtomicAnonUnion;\n\
             typedef ConstAtomicAnonUnion ConstAtomicAnonUnionArray[2];\n\
             int main(void) {\n\
                 ConstAtomicAnonUnionArray choices;\n\
                 choices[1].tag = 'X';\n\
                 return 0;\n\
             }\n",
            "cannot assign to const variable 'choices'",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_const_discard_from_atomic_anonymous_aggregate_qualified_array_aliases() {
    let cases = [
        "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
         typedef const AtomicAnonValue ConstAtomicAnonValue;\n\
         typedef ConstAtomicAnonValue ConstAtomicAnonArray[2];\n\
         int main(void) {\n\
             ConstAtomicAnonArray values;\n\
             AtomicAnonValue *mutable_view = values;\n\
             return mutable_view == 0;\n\
         }\n",
        "typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;\n\
         typedef const AtomicAnonUnion ConstAtomicAnonUnion;\n\
         typedef ConstAtomicAnonUnion ConstAtomicAnonUnionArray[2];\n\
         int take(AtomicAnonUnion *value) { return value == 0; }\n\
         int main(void) {\n\
             ConstAtomicAnonUnionArray choices;\n\
             return take(choices);\n\
         }\n",
    ];

    for program in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(
            err.to_string(),
            "cannot discard const qualifier from pointer target",
            "program: {program}"
        );
    }
}

#[test]
fn rejects_multidimensional_atomic_anonymous_aggregate_qualified_array_aliases() {
    let program = "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
                   typedef const AtomicAnonValue ConstAtomicAnonValue;\n\
                   typedef ConstAtomicAnonValue ConstAtomicAnonArray[2];\n\
                   typedef ConstAtomicAnonArray Matrix[2];\n\
                   int main(void) { return 0; }\n";

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "multidimensional array typedef aliases are not supported at line 4, column 38"
    );
}

#[test]
fn supports_qualified_named_type_array_alias_parameters() {
    let program = include_str!("fixtures/valid/qualified_named_type_array_alias_parameters.c");

    assert_eq!(interpret(program).unwrap(), 127);
}

#[test]
fn supports_chained_qualified_array_alias_parameters() {
    let program = include_str!("fixtures/valid/chained_qualified_array_alias_parameters.c");

    assert_eq!(interpret(program).unwrap(), 127);
}

#[test]
fn rejects_writes_through_chained_qualified_array_aliases() {
    let cases = [
        (
            "typedef const int ConstInt;\n\
             typedef ConstInt ConstInts[2], OtherInts[3];\n\
             typedef ConstInts ChainedConstInts;\n\
             int main(void) {\n\
                 ChainedConstInts values = {1, 2};\n\
                 values[0] = 7;\n\
                 return 0;\n\
             }\n",
            "cannot modify read-only array 'values'",
        ),
        (
            "struct Point { int x; int y; };\n\
             typedef const struct Point ConstPoint;\n\
             typedef ConstPoint ConstPoints[2], OtherPoints[3];\n\
             typedef ConstPoints ChainedConstPoints;\n\
             int main(void) {\n\
                 ChainedConstPoints points = {{1, 2}, {3, 4}};\n\
                 points[0].x = 7;\n\
                 return 0;\n\
             }\n",
            "cannot assign to const variable 'points'",
        ),
        (
            "enum State { IDLE = 1, READY = 2 };\n\
             typedef const enum State ConstState;\n\
             typedef ConstState ConstStates[2], OtherStates[3];\n\
             typedef ConstStates ChainedConstStates;\n\
             int main(void) {\n\
                 ChainedConstStates states = {IDLE, READY};\n\
                 states[0] = READY;\n\
                 return 0;\n\
             }\n",
            "cannot modify read-only array 'states'",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_writes_through_qualified_named_type_array_aliases() {
    let cases = [
        (
            "struct Point { int x; int y; };\n\
             typedef const struct Point ConstPoint;\n\
             typedef ConstPoint ConstPoints[2];\n\
             int main(void) {\n\
                 ConstPoints points = {{1, 2}, {3, 4}};\n\
                 points[0].x = 7;\n\
                 return 0;\n\
             }\n",
            "cannot assign to const variable 'points'",
        ),
        (
            "union Number { int value; char tag; };\n\
             typedef const union Number ConstNumber;\n\
             typedef ConstNumber ConstNumbers[2];\n\
             int main(void) {\n\
                 ConstNumbers numbers = {{1}, {2}};\n\
                 numbers[1].tag = 'X';\n\
                 return 0;\n\
             }\n",
            "cannot assign to const variable 'numbers'",
        ),
        (
            "enum State { IDLE = 1, READY = 2 };\n\
             typedef const enum State ConstState;\n\
             typedef ConstState ConstStates[2];\n\
             int main(void) {\n\
                 ConstStates states = {IDLE, READY};\n\
                 states[0] = READY;\n\
                 return 0;\n\
             }\n",
            "cannot modify read-only array 'states'",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_const_discard_from_chained_qualified_array_aliases() {
    let cases = [
        "typedef const int ConstInt;\n\
         typedef ConstInt ConstInts[2], OtherInts[3];\n\
         typedef ConstInts ChainedConstInts;\n\
         int main(void) {\n\
             ChainedConstInts values = {1, 2};\n\
             int *mutable_view = values;\n\
             return mutable_view == 0;\n\
         }\n",
        "struct Point { int x; int y; };\n\
         typedef const struct Point ConstPoint;\n\
         typedef ConstPoint ConstPoints[2], OtherPoints[3];\n\
         typedef ConstPoints ChainedConstPoints;\n\
         int take(struct Point *point) { return point == 0; }\n\
         int main(void) {\n\
             ChainedConstPoints points = {{1, 2}, {3, 4}};\n\
             return take(points);\n\
         }\n",
        "enum State { IDLE = 1, READY = 2 };\n\
         typedef const enum State ConstState;\n\
         typedef ConstState ConstStates[2], OtherStates[3];\n\
         typedef ConstStates ChainedConstStates;\n\
         int main(void) {\n\
             ChainedConstStates states = {IDLE, READY};\n\
             enum State *mutable_view = states;\n\
             return mutable_view == 0;\n\
         }\n",
    ];

    for program in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(
            err.to_string(),
            "cannot discard const qualifier from pointer target",
            "program: {program}"
        );
    }
}

#[test]
fn rejects_const_discard_from_qualified_named_type_array_aliases() {
    let cases = [
        "struct Point { int x; int y; };\n\
         typedef const struct Point ConstPoint;\n\
         typedef ConstPoint ConstPoints[2];\n\
         int main(void) {\n\
             ConstPoints points = {{1, 2}, {3, 4}};\n\
             struct Point *mutable_view = points;\n\
             return mutable_view == 0;\n\
         }\n",
        "union Number { int value; char tag; };\n\
         typedef const union Number ConstNumber;\n\
         typedef ConstNumber ConstNumbers[2];\n\
         int take(union Number *number) { return number == 0; }\n\
         int main(void) {\n\
             ConstNumbers numbers = {{1}, {2}};\n\
             return take(numbers);\n\
         }\n",
        "enum State { IDLE = 1, READY = 2 };\n\
         typedef const enum State ConstState;\n\
         typedef ConstState ConstStates[2];\n\
         int main(void) {\n\
             ConstStates states = {IDLE, READY};\n\
             enum State *mutable_view = states;\n\
             return mutable_view == 0;\n\
         }\n",
    ];

    for program in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(
            err.to_string(),
            "cannot discard const qualifier from pointer target",
            "program: {program}"
        );
    }
}

#[test]
fn rejects_writes_to_atomic_anonymous_aggregate_qualified_value_aliases() {
    let cases = [
        (
            "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
             typedef const AtomicAnonValue ConstAtomicAnonValue;\n\
             int main(void) {\n\
                 ConstAtomicAnonValue value = {0};\n\
                 value.value = 7;\n\
                 return 0;\n\
             }\n",
            "cannot assign to const variable 'value'",
        ),
        (
            "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
             typedef const AtomicAnonValue ConstAtomicAnonValue;\n\
             int main(void) {\n\
                 ConstAtomicAnonValue values[1];\n\
                 values[0].value = 7;\n\
                 return 0;\n\
             }\n",
            "cannot assign to const variable 'values'",
        ),
        (
            "typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;\n\
             typedef const AtomicAnonUnion ConstAtomicAnonUnion;\n\
             int main(void) {\n\
                 AtomicAnonUnion values[1];\n\
                 ConstAtomicAnonUnion *view = values;\n\
                 view->tag = 'X';\n\
                 return 0;\n\
             }\n",
            "cannot assign through pointer to const",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_const_discard_from_atomic_anonymous_aggregate_qualified_value_aliases() {
    let cases = [
        "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
         typedef const AtomicAnonValue ConstAtomicAnonValue;\n\
         int main(void) {\n\
             AtomicAnonValue values[1];\n\
             ConstAtomicAnonValue *view = values;\n\
             AtomicAnonValue *mutable_view = view;\n\
             return mutable_view == values;\n\
         }\n",
        "typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;\n\
         typedef const AtomicAnonUnion ConstAtomicAnonUnion;\n\
         int take(AtomicAnonUnion *value) { return value == 0; }\n\
         int main(void) {\n\
             AtomicAnonUnion values[1];\n\
             ConstAtomicAnonUnion *view = values;\n\
             return take(view);\n\
         }\n",
    ];

    for program in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(
            err.to_string(),
            "cannot discard const qualifier from pointer target",
            "program: {program}"
        );
    }
}

#[test]
fn rejects_atomic_wrapping_of_qualified_atomic_anonymous_aggregate_value_aliases() {
    let cases = [
        (
            "typedef _Atomic(struct { int value; }) AtomicAnonValue;\ntypedef const AtomicAnonValue ConstAtomicAnonValue;\n_Atomic(ConstAtomicAnonValue) nested;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 3, column 9",
        ),
        (
            "typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;\ntypedef volatile AtomicAnonUnion VolatileAtomicAnonUnion;\nint main(void) { return sizeof(_Atomic(VolatileAtomicAnonUnion)); }\n",
            "qualified _Atomic types are not supported at line 3, column 40",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_const_discard_from_atomic_anonymous_aggregate_const_pointer_views() {
    let cases = [
        "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
         typedef const AtomicAnonValue *ConstAtomicAnonView;\n\
         int main(void) {\n\
             AtomicAnonValue values[1];\n\
             ConstAtomicAnonView view = values;\n\
             AtomicAnonValue *mutable_view = view;\n\
             return mutable_view == values;\n\
         }\n",
        "typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;\n\
         typedef const AtomicAnonUnion *ConstAtomicAnonUnionView;\n\
         int main(void) {\n\
             AtomicAnonUnion values[1];\n\
             AtomicAnonUnion *mutable_view = values;\n\
             ConstAtomicAnonUnionView view = values;\n\
             mutable_view = view;\n\
             return mutable_view == values;\n\
         }\n",
        "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
         typedef const AtomicAnonValue *ConstAtomicAnonView;\n\
         int read(AtomicAnonValue *value) { return value == 0; }\n\
         int main(void) {\n\
             AtomicAnonValue values[1];\n\
             ConstAtomicAnonView view = values;\n\
             return read(view);\n\
         }\n",
    ];

    for program in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(
            err.to_string(),
            "cannot discard const qualifier from pointer target",
            "program: {program}"
        );
    }
}

#[test]
fn rejects_writes_through_atomic_anonymous_aggregate_const_pointer_views() {
    let cases = [
        "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
         typedef const AtomicAnonValue *ConstAtomicAnonView;\n\
         int main(void) {\n\
             AtomicAnonValue values[1];\n\
             ConstAtomicAnonView view = values;\n\
             view->value = 7;\n\
             return 0;\n\
         }\n",
        "typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;\n\
         typedef const AtomicAnonUnion *ConstAtomicAnonUnionView;\n\
         int main(void) {\n\
             AtomicAnonUnion values[1];\n\
             ConstAtomicAnonUnionView view = values;\n\
             view->tag = 'X';\n\
             return 0;\n\
         }\n",
    ];

    for program in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(
            err.to_string(),
            "cannot assign through pointer to const",
            "program: {program}"
        );
    }
}

#[test]
fn rejects_reassignment_of_atomic_anonymous_aggregate_const_pointer_view_slots() {
    let program = "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
                   typedef const AtomicAnonValue *ConstAtomicAnonView;\n\
                   typedef ConstAtomicAnonView const FixedConstAtomicAnonView;\n\
                   int main(void) {\n\
                       AtomicAnonValue values[2];\n\
                       FixedConstAtomicAnonView view = values;\n\
                       view = values + 1;\n\
                       return 0;\n\
                   }\n";

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign to const variable 'view'");
}

#[test]
fn rejects_deeper_atomic_anonymous_aggregate_const_pointer_view_declarators() {
    let cases = [
        (
            "typedef _Atomic(struct { int value; }) AtomicAnonValue;\ntypedef const AtomicAnonValue *ConstAtomicAnonView;\ntypedef ConstAtomicAnonView *Nested;\nint main(void) { return 0; }\n",
            "pointer-to-pointer typedef aliases are not supported at line 3, column 29",
        ),
        (
            "typedef _Atomic(struct { int value; }) AtomicAnonValue;\ntypedef const AtomicAnonValue *ConstAtomicAnonView;\ntypedef ConstAtomicAnonView Views[2];\nint main(void) { return 0; }\n",
            "pointer array typedef aliases are not supported at line 3, column 36",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_invalid_declarators_from_atomic_anonymous_aggregate_derived_aliases() {
    let cases = [
        (
            "typedef _Atomic(struct { int value; }) AtomicAnonValue;\ntypedef AtomicAnonValue *AtomicAnonPtr, AtomicAnonArray[2];\ntypedef AtomicAnonPtr *Nested;\nint main(void) { return 0; }\n",
            "pointer-to-pointer typedef aliases are not supported at line 3, column 23",
        ),
        (
            "typedef _Atomic(struct { int value; }) AtomicAnonValue;\ntypedef AtomicAnonValue *AtomicAnonPtr, AtomicAnonArray[2];\ntypedef AtomicAnonPtr PointerArray[2];\nint main(void) { return 0; }\n",
            "pointer array typedef aliases are not supported at line 3, column 37",
        ),
        (
            "typedef _Atomic(struct { int value; }) AtomicAnonValue;\ntypedef AtomicAnonValue *AtomicAnonPtr, AtomicAnonArray[2];\ntypedef AtomicAnonArray NestedArray[2];\nint main(void) { return 0; }\n",
            "multidimensional array typedef aliases are not supported at line 3, column 38",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_reassignment_of_const_atomic_anonymous_aggregate_derived_pointer_alias_slots() {
    let program = "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
                   typedef AtomicAnonValue *AtomicAnonPtr;\n\
                   typedef AtomicAnonPtr const FixedAtomicAnonPtr;\n\
                   int main(void) {\n\
                       AtomicAnonValue values[2];\n\
                       FixedAtomicAnonPtr cursor = values;\n\
                       cursor = values + 1;\n\
                       return 0;\n\
                   }\n";

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign to const variable 'cursor'");
}

#[test]
fn rejects_unsupported_declarators_from_atomic_anonymous_aggregate_value_aliases() {
    let cases = [
        (
            "typedef _Atomic(struct { int value; }) AtomicAnonValue;\nAtomicAnonValue **cursor;\nint main(void) { return 0; }\n",
            "pointer-to-pointer declarations are not supported at line 2, column 18",
        ),
        (
            "typedef _Atomic(struct { int value; }) AtomicAnonValue;\nAtomicAnonValue *cursors[2];\nint main(void) { return 0; }\n",
            "pointer array declarations are not supported at line 2, column 25",
        ),
        (
            "typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;\nAtomicAnonUnion values[2][2];\nint main(void) { return 0; }\n",
            "multidimensional array declarations are not supported at line 2, column 26",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_atomic_wrapping_of_atomic_anonymous_aggregate_value_aliases() {
    let program = "typedef _Atomic(struct { int value; }) AtomicAnonValue;\n\
                   _Atomic(AtomicAnonValue) nested;\n\
                   int main(void) { return 0; }\n";

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "qualified _Atomic types are not supported at line 2, column 9"
    );
}

#[test]
fn rejects_reassignment_of_const_atomic_anonymous_aggregate_pointer_alias_slots() {
    let program = "typedef _Atomic(struct { int value; } *) AtomicAnonPtr;\n\
                   typedef AtomicAnonPtr const FixedAtomicAnonPtr;\n\
                   int main(void) {\n\
                       FixedAtomicAnonPtr cursor = 0;\n\
                       cursor = 0;\n\
                       return 0;\n\
                   }\n";

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign to const variable 'cursor'");
}

#[test]
fn rejects_writes_through_const_atomic_anonymous_aggregate_pointer_aliases() {
    let program = "typedef _Atomic(const union { int value; char tag; } *) AtomicConstAnonPtr;\n\
                   int main(void) {\n\
                       AtomicConstAnonPtr cursor = 0;\n\
                       cursor->value = 1;\n\
                       return 0;\n\
                   }\n";

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign through pointer to const");
}

#[test]
fn rejects_atomic_wrapping_of_atomic_anonymous_aggregate_pointer_aliases() {
    let program = "typedef _Atomic(struct { int value; } *) AtomicAnonPtr;\n\
                   _Atomic(AtomicAnonPtr) cursor;\n\
                   int main(void) { return 0; }\n";

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "qualified _Atomic types are not supported at line 2, column 9"
    );
}

#[test]
fn rejects_unsupported_atomic_anonymous_aggregate_pointer_suffixes_with_context() {
    let cases = [
        (
            "int main(void) { return sizeof(_Atomic(struct { int value; } **)); }\n",
            "pointer-to-pointer _Atomic types are not supported at line 1, column 63",
        ),
        (
            "int main(void) { return _Alignof(_Atomic(const union { int value; char tag; } *[2])); }\n",
            "pointer array _Atomic types are not supported at line 1, column 80",
        ),
        (
            "int main(void) { return sizeof(_Atomic(struct { int value; } *(void))); }\n",
            "function _Atomic types are not supported at line 1, column 63",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn keeps_atomic_anonymous_aggregate_pointer_pointee_identities_distinct() {
    let program = "int inspect(_Atomic(struct { int value; } *) cursor);\n\
                   int inspect(_Atomic(struct { int value; } *) cursor);\n\
                   int main(void) { return 0; }\n";

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "function prototype for 'inspect' conflicts with previous declaration"
    );
}

#[test]
fn rejects_writes_through_const_atomic_anonymous_aggregate_pointers() {
    let program = "int main(void) {\n\
                       _Atomic(const union { int value; char tag; } *) cursor = 0;\n\
                       cursor->value = 1;\n\
                       return 0;\n\
                   }\n";

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign through pointer to const");
}

#[test]
fn rejects_qualified_and_nested_atomic_anonymous_aggregate_types_with_context() {
    let cases = [
        (
            "_Atomic(const struct { int value; }) value;\nint main(void) { return 0; }\n",
            "qualified _Atomic types are not supported at line 1, column 9",
        ),
        (
            "_Atomic(_Atomic(union { int value; char tag; })) value;\nint main(void) { return 0; }\n",
            "nested _Atomic type specifiers are not supported at line 1, column 9",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn keeps_atomic_anonymous_aggregate_type_identities_distinct() {
    let program = include_str!("fixtures/invalid/atomic_anonymous_aggregate_distinct_types.c");

    let err = interpret(program).unwrap_err();
    assert!(
        err.to_string().contains("cannot assign struct"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_direct_atomic_array_types_with_context() {
    let cases = [
        (
            "_Atomic(int[2]) value;\nint main(void) { return 0; }\n",
            "array _Atomic types are not supported at line 1, column 12",
        ),
        (
            "struct Point { int x; };\n_Atomic(struct Point[2]) values;\nint main(void) { return 0; }\n",
            "array _Atomic types are not supported at line 2, column 21",
        ),
        (
            "int main(void) { return sizeof(_Atomic(int[2])); }\n",
            "array _Atomic types are not supported at line 1, column 43",
        ),
        (
            "int main(void) { return _Alignof(_Atomic(int[2])); }\n",
            "array _Atomic types are not supported at line 1, column 45",
        ),
        (
            "int main(void) { return ((_Atomic(int[2])){1, 2})[0]; }\n",
            "array _Atomic types are not supported at line 1, column 38",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
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

    assert_eq!(interpret(program).unwrap(), 48);
}

#[test]
fn supports_comma_separated_typedef_aliases() {
    let program = include_str!("fixtures/valid/comma_separated_typedef_aliases.c");

    assert_eq!(interpret(program).unwrap(), 81);
}

#[test]
fn supports_named_aggregate_typedef_declaration_lists() {
    let program = include_str!("fixtures/valid/named_aggregate_typedef_declaration_lists.c");

    assert_eq!(interpret(program).unwrap(), 60);
}

#[test]
fn supports_enum_typedef_declaration_lists() {
    let program = include_str!("fixtures/valid/enum_typedef_declaration_lists.c");

    assert_eq!(interpret(program).unwrap(), 38);
}

#[test]
fn supports_parenthesized_typedef_declarators() {
    let program = include_str!("fixtures/valid/parenthesized_typedef_declarators.c");

    assert_eq!(interpret(program).unwrap(), 43);
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

    assert_eq!(interpret(program).unwrap(), 32);
}

#[test]
fn supports_alignof_type_names() {
    let program = include_str!("fixtures/valid/alignof_type_names.c");

    assert_eq!(interpret(program).unwrap(), 51);
}

#[test]
fn supports_alignas_specifiers() {
    let program = include_str!("fixtures/valid/alignas_specifiers.c");

    assert_eq!(interpret(program).unwrap(), 29);
}

#[test]
fn rejects_missing_alignas_arguments_with_context() {
    let missing_argument = r#"
        _Alignas() int value;
        int main(void) { return 0; }
    "#;
    let err = interpret(missing_argument).unwrap_err();
    assert!(
        err.to_string()
            .contains("expected _Alignas argument, found RParen at line 2, column 18"),
        "unexpected error: {err}"
    );

    let comma_argument = r#"
        int main(void) {
            _Alignas(,) int value;
            return 0;
        }
    "#;
    let err = interpret(comma_argument).unwrap_err();
    assert!(
        err.to_string()
            .contains("expected _Alignas argument, found Comma at line 3, column 22"),
        "unexpected error: {err}"
    );

    let semicolon_argument = r#"
        int main(void) {
            _Alignas(; int value;
            return 0;
        }
    "#;
    let err = interpret(semicolon_argument).unwrap_err();
    assert!(
        err.to_string()
            .contains("expected _Alignas argument, found Semi at line 3, column 22"),
        "unexpected error: {err}"
    );

    let misplaced_bracket_argument = r#"
        int main(void) {
            _Alignas([) int value;
            return 0;
        }
    "#;
    let err = interpret(misplaced_bracket_argument).unwrap_err();
    assert!(
        err.to_string()
            .contains("expected _Alignas argument, found LBracket at line 3, column 22"),
        "unexpected error: {err}"
    );

    let misplaced_question_argument = r#"
        int main(void) {
            _Alignas(?) int value;
            return 0;
        }
    "#;
    let err = interpret(misplaced_question_argument).unwrap_err();
    assert!(
        err.to_string()
            .contains("expected _Alignas argument, found Question at line 3, column 22"),
        "unexpected error: {err}"
    );

    let keyword_argument = r#"
        _Alignas(return) int value;
        int main(void) { return 0; }
    "#;
    let err = interpret(keyword_argument).unwrap_err();
    assert!(
        err.to_string()
            .contains("expected _Alignas argument before 'return' at line 2, column 18"),
        "unexpected error: {err}"
    );

    let selector_argument = "_Alignas(.) int value; int main(void) { return 0; }";
    let err = interpret(selector_argument).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected _Alignas argument, found Dot at line 1, column 10"
    );

    let arrow_argument = "_Alignas(->field) int value; int main(void) { return 0; }";
    let err = interpret(arrow_argument).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected _Alignas argument, found Arrow at line 1, column 10"
    );

    let brace_argument = "_Alignas({1}) int value; int main(void) { return 0; }";
    let err = interpret(brace_argument).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected _Alignas argument, found LBrace at line 1, column 10"
    );

    let colon_argument = "_Alignas(:) int value; int main(void) { return 0; }";
    let err = interpret(colon_argument).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected _Alignas argument before ':' at line 1, column 10"
    );
}

#[test]
fn supports_thread_local_storage_class_specifiers() {
    let program = include_str!("fixtures/valid/thread_local_storage_class.c");

    assert_eq!(interpret(program).unwrap(), 26);
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
fn rejects_missing_static_assert_arguments_with_context() {
    let missing_condition = r#"
        _Static_assert(, "condition required");
        int main(void) { return 0; }
    "#;
    let err = interpret(missing_condition).unwrap_err();
    assert!(
        err.to_string()
            .contains("expected _Static_assert condition, found Comma at line 2, column 24"),
        "unexpected error: {err}"
    );

    let missing_message = r#"
        _Static_assert(1, );
        int main(void) { return 0; }
    "#;
    let err = interpret(missing_message).unwrap_err();
    assert!(
        err.to_string().contains(
            "expected string literal after _Static_assert condition, found RParen at line 2, column 27"
        ),
        "unexpected error: {err}"
    );

    let misplaced_bracket_condition = r#"
        _Static_assert([, "condition required");
        int main(void) { return 0; }
    "#;
    let err = interpret(misplaced_bracket_condition).unwrap_err();
    assert!(
        err.to_string()
            .contains("expected _Static_assert condition, found LBracket at line 2, column 24"),
        "unexpected error: {err}"
    );

    let misplaced_question_condition = r#"
        _Static_assert(?, "condition required");
        int main(void) { return 0; }
    "#;
    let err = interpret(misplaced_question_condition).unwrap_err();
    assert!(
        err.to_string()
            .contains("expected _Static_assert condition, found Question at line 2, column 24"),
        "unexpected error: {err}"
    );

    let keyword_condition = r#"
        _Static_assert(return, "condition required");
        int main(void) { return 0; }
    "#;
    let err = interpret(keyword_condition).unwrap_err();
    assert!(
        err.to_string()
            .contains("expected _Static_assert condition before 'return' at line 2, column 24"),
        "unexpected error: {err}"
    );

    let selector_condition =
        "_Static_assert(., \"condition required\"); int main(void) { return 0; }";
    let err = interpret(selector_condition).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected _Static_assert condition, found Dot at line 1, column 16"
    );

    let arrow_condition =
        "_Static_assert(->field, \"condition required\"); int main(void) { return 0; }";
    let err = interpret(arrow_condition).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected _Static_assert condition, found Arrow at line 1, column 16"
    );

    let brace_condition =
        "_Static_assert({1}, \"condition required\"); int main(void) { return 0; }";
    let err = interpret(brace_condition).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected _Static_assert condition, found LBrace at line 1, column 16"
    );

    let colon_condition = "_Static_assert(:, \"condition required\"); int main(void) { return 0; }";
    let err = interpret(colon_condition).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected _Static_assert condition before ':' at line 1, column 16"
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
fn supports_scalar_variable_reverse_subscript_lvalues() {
    let program = include_str!("fixtures/valid/scalar_variable_reverse_subscript.c");

    assert_eq!(interpret(program).unwrap(), 220);
}

#[test]
fn supports_sizeof_scalar_variable_and_field_reverse_subscripts_without_evaluating_operands() {
    let program = include_str!("fixtures/valid/sizeof_scalar_reverse_subscripts.c");

    assert_eq!(interpret(program).unwrap(), 16);
}

#[test]
fn supports_sizeof_reverse_subscript_lvalue_results_without_evaluating_operands() {
    let program = include_str!("fixtures/valid/sizeof_reverse_subscript_lvalue_results.c");

    assert_eq!(interpret(program).unwrap(), 13);
}

#[test]
fn supports_sizeof_scalar_variable_reverse_subscript_assignment_results_without_evaluation() {
    let program = r#"
        int main(void) {
            int values[2] = {3, 5};
            int index = 1;
            int pointer_marker = 0;
            int rhs_marker = 0;
            int size_matches = sizeof(
                index[(pointer_marker += 1, values)] = (rhs_marker += 1, 9)
            ) == sizeof(values[index]);
            return size_matches + pointer_marker * 10 + rhs_marker * 100;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 1);
}

#[test]
fn supports_sizeof_scalar_variable_reverse_subscript_compound_assignment_results_without_evaluation()
 {
    let program = r#"
        int main(void) {
            char bytes[2] = {'a', 'b'};
            int index = 1;
            int pointer_marker = 0;
            int rhs_marker = 0;
            int size_matches = sizeof(
                index[(pointer_marker += 1, bytes)] += (rhs_marker += 1, 2)
            ) == sizeof(bytes[index]);
            return size_matches + pointer_marker * 10 + rhs_marker * 100;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 1);
}

#[test]
fn supports_sizeof_scalar_field_reverse_subscript_assignment_results_without_evaluation() {
    let program = r#"
        struct Index { int value; };

        int main(void) {
            char bytes[2] = {'a', 'b'};
            struct Index selector = {1};
            int pointer_marker = 0;
            int rhs_marker = 0;
            int size_matches = sizeof(
                selector.value[(pointer_marker += 1, bytes)] = (rhs_marker += 1, 'x')
            ) == sizeof(bytes[selector.value]);
            return size_matches + pointer_marker * 10 + rhs_marker * 100;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 1);
}

#[test]
fn supports_sizeof_reverse_subscript_aggregate_field_assignment_results_without_evaluation() {
    let program = r#"
        struct Point { int x; char tag; };

        int main(void) {
            struct Point points[2] = {{3, 'a'}, {5, 'b'}};
            int index = 1;
            int pointer_marker = 0;
            int rhs_marker = 0;
            int size_matches = sizeof(
                index[(pointer_marker += 1, points)].tag = (rhs_marker += 1, 'x')
            ) == sizeof(points[index].tag);
            return size_matches + pointer_marker * 10 + rhs_marker * 100;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 1);
}

#[test]
fn sizeof_scalar_variable_reverse_subscripts_reject_two_scalar_operands() {
    let program = r#"
        int main(void) {
            int left = 0;
            int right = 1;
            return sizeof(left[right]);
        }
    "#;

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "subscript requires one pointer operand and one scalar operand"
    );
}

#[test]
fn sizeof_scalar_variable_reverse_subscripts_reject_scalar_pointer_field_access() {
    let program = r#"
        int main(void) {
            int values[2] = {3, 5};
            int *pointer = values;
            int index = 1;
            return sizeof(index[pointer].field);
        }
    "#;

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "subscript pointer does not reference a struct"
    );
}

#[test]
fn scalar_variable_reverse_subscripts_preserve_const_diagnostics() {
    let program = r#"
        int main(void) {
            const int values[2] = {3, 5};
            int index = 1;
            index[values] = 8;
            return 0;
        }
    "#;

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "cannot modify read-only array 'values'"
    );
}

#[test]
fn scalar_variable_reverse_subscripts_preserve_direct_array_bounds_diagnostics() {
    let program = r#"
        int main(void) {
            int values[2] = {3, 5};
            int index = 2;
            return index[values];
        }
    "#;

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "array 'values' index 2 out of bounds for length 2"
    );
}

#[test]
fn scalar_variable_reverse_subscripts_reject_two_scalar_operands() {
    let program = r#"
        int main(void) {
            int left = 0;
            int right = 1;
            return left[right];
        }
    "#;

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "subscript requires one pointer operand and one scalar operand"
    );
}

#[test]
fn supports_scalar_field_reverse_subscript_lvalues() {
    let program = r#"
        struct Index {
            int value;
        };

        int main(void) {
            int values[3] = {3, 5, 7};
            struct Index index = {1};
            index.value[values] += 4;
            int *slot = &index.value[values];
            *slot += 3;
            return index.value[values];
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 12);
}

#[test]
fn scalar_variable_reverse_aggregate_subscripts_preserve_const_diagnostics() {
    let program = r#"
        struct Point {
            int x;
        };

        int main(void) {
            const struct Point points[2] = {{3}, {5}};
            int index = 1;
            index[points].x = 8;
            return 0;
        }
    "#;

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "cannot assign to const variable 'points'"
    );
}

#[test]
fn scalar_variable_reverse_aggregate_subscripts_preserve_bounds_diagnostics() {
    let program = r#"
        struct Point {
            int x;
        };

        int main(void) {
            struct Point points[2] = {{3}, {5}};
            int index = 2;
            return index[points].x;
        }
    "#;

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "struct array 'points' index 2 out of bounds for length 2"
    );
}

#[test]
fn scalar_variable_reverse_aggregate_subscripts_reject_scalar_pointers() {
    let program = r#"
        int main(void) {
            int values[2] = {3, 5};
            int *pointer = values;
            int index = 1;
            return index[pointer].field;
        }
    "#;

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "subscript pointer does not reference a struct"
    );
}

#[test]
fn supports_comma_expressions_in_subscript_indices() {
    let program = include_str!("fixtures/valid/subscript_comma_expressions.c");

    assert_eq!(interpret(program).unwrap(), 31);
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
fn supports_scalar_array_field_decay_in_pointer_expressions() {
    let program = include_str!("fixtures/valid/scalar_array_field_pointer_decay.c");

    assert_eq!(interpret(program).unwrap(), 87);
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
fn rejects_pointer_ordering_between_different_scalar_array_fields() {
    let program =
        include_str!("fixtures/invalid/scalar_array_field_pointer_ordering_different_arrays.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot compare pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_difference_between_different_embedded_aggregate_array_fields() {
    let program = include_str!(
        "fixtures/invalid/embedded_aggregate_array_field_pointer_difference_different_fields.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot subtract pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_difference_between_different_scalar_array_fields() {
    let program =
        include_str!("fixtures/invalid/scalar_array_field_pointer_difference_different_arrays.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot subtract pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_difference_between_different_anonymous_aggregate_array_fields() {
    let program = include_str!(
        "fixtures/invalid/anonymous_aggregate_array_field_pointer_difference_different_fields.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot subtract pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_ordering_between_different_anonymous_aggregate_array_fields() {
    let program = include_str!(
        "fixtures/invalid/anonymous_aggregate_array_field_pointer_ordering_different_fields.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot compare pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_difference_between_different_string_literals() {
    let program =
        include_str!("fixtures/invalid/string_literal_pointer_difference_different_literals.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot subtract pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_ordering_between_different_string_literals() {
    let program =
        include_str!("fixtures/invalid/string_literal_pointer_ordering_different_literals.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot compare pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_difference_between_different_array_compound_literals() {
    let program = include_str!(
        "fixtures/invalid/array_compound_literal_pointer_difference_different_literals.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot subtract pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_ordering_between_different_array_compound_literals() {
    let program = include_str!(
        "fixtures/invalid/array_compound_literal_pointer_ordering_different_literals.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot compare pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_difference_between_different_aggregate_array_compound_literals() {
    let program = include_str!(
        "fixtures/invalid/aggregate_array_compound_literal_pointer_difference_different_literals.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot subtract pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_ordering_between_different_aggregate_array_compound_literals() {
    let program = include_str!(
        "fixtures/invalid/aggregate_array_compound_literal_pointer_ordering_different_literals.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot compare pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_difference_between_scalar_array_fields_on_different_aggregate_compound_literals()
{
    let program = include_str!(
        "fixtures/invalid/aggregate_compound_literal_scalar_array_field_pointer_difference_different_literals.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot subtract pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_ordering_between_scalar_array_fields_on_different_aggregate_compound_literals() {
    let program = include_str!(
        "fixtures/invalid/aggregate_compound_literal_scalar_array_field_pointer_ordering_different_literals.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot compare pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_difference_between_aggregate_array_fields_on_different_aggregate_compound_literals()
 {
    let program = include_str!(
        "fixtures/invalid/aggregate_compound_literal_aggregate_array_field_pointer_difference_different_literals.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot subtract pointers to different arrays"
    );
}

#[test]
fn rejects_pointer_ordering_between_aggregate_array_fields_on_different_aggregate_compound_literals()
 {
    let program = include_str!(
        "fixtures/invalid/aggregate_compound_literal_aggregate_array_field_pointer_ordering_different_literals.c",
    );

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
        "initializer string for char array 'short_text' is too long at line 2, column 26"
    );
}

#[test]
fn reports_struct_char_array_string_initializer_too_long() {
    let program = include_str!("fixtures/invalid/struct_char_array_string_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "initializer string for char array 'text' is too long at line 6, column 27"
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
fn supports_c_style_line_comments_as_whitespace() {
    let program = include_str!("fixtures/valid/line_comments.c");

    assert_eq!(interpret(program).unwrap(), 8);
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
fn supports_sizeof_and_alignof_anonymous_aggregate_type_names() {
    let program = include_str!("fixtures/valid/sizeof_anonymous_aggregate_types.c");

    assert_eq!(interpret(program).unwrap(), 62);
}

#[test]
fn supports_sizeof_pointer_expressions_without_evaluating_operands() {
    let program = include_str!("fixtures/valid/sizeof_pointer_expressions.c");

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn rejects_missing_sizeof_and_alignof_operands_with_context() {
    let cases = [
        (
            "int main(void) { sizeof([); return 0; }",
            "expected sizeof operand before '[' at line 1, column 25",
        ),
        (
            "int main(void) { sizeof(?); return 0; }",
            "expected sizeof operand before '?' at line 1, column 25",
        ),
        (
            "int main(void) {\n    return sizeof();\n}\n",
            "expected sizeof operand, found RParen at line 2, column 19",
        ),
        (
            "int main(void) {\n    return sizeof(,);\n}\n",
            "expected sizeof operand, found Comma at line 2, column 19",
        ),
        (
            "int main(void) {\n    return sizeof;\n}\n",
            "expected sizeof operand, found Semi at line 2, column 18",
        ),
        (
            "int main(void) {\n    return sizeof return;\n}\n",
            "expected sizeof operand before 'return' at line 2, column 19",
        ),
        (
            "int main(void) {\n    return sizeof int;\n}\n",
            "expected sizeof operand before 'int' at line 2, column 19",
        ),
        (
            "int main(void) {\n    return sizeof(return);\n}\n",
            "expected sizeof operand before 'return' at line 2, column 19",
        ),
        (
            "int main(void) { return sizeof(:); }",
            "expected sizeof operand before ':' at line 1, column 32",
        ),
        (
            "int main(void) { return sizeof(.); }",
            "expected sizeof operand before '.' at line 1, column 32",
        ),
        (
            "int main(void) { return sizeof(->field); }",
            "expected sizeof operand before '->' at line 1, column 32",
        ),
        (
            "int main(void) { return sizeof({1}); }",
            "expected sizeof operand before '{' at line 1, column 32",
        ),
        (
            "int main(void) {\n    return _Alignof();\n}\n",
            "expected _Alignof type, found RParen at line 2, column 21",
        ),
        (
            "int main(void) {\n    return _Alignof(,);\n}\n",
            "expected _Alignof type, found Comma at line 2, column 21",
        ),
        (
            "int main(void) {\n    return _Alignof([);\n}\n",
            "expected _Alignof type before '[' at line 2, column 21",
        ),
        (
            "int main(void) {\n    return _Alignof(?);\n}\n",
            "expected _Alignof type before '?' at line 2, column 21",
        ),
        (
            "int main(void) {\n    return _Alignof(return);\n}\n",
            "expected _Alignof type before 'return' at line 2, column 21",
        ),
        (
            "int main(void) { return _Alignof(.); }",
            "expected _Alignof type before '.' at line 1, column 34",
        ),
        (
            "int main(void) { return _Alignof(->field); }",
            "expected _Alignof type before '->' at line 1, column 34",
        ),
        (
            "int main(void) { return _Alignof({1}); }",
            "expected _Alignof type before '{' at line 1, column 34",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_qualifier_only_sizeof_and_alignof_types_with_context() {
    let cases = [
        (
            "int main(void) {\n    return sizeof(volatile);\n}\n",
            "expected sizeof type after type qualifier 'volatile', found RParen at line 2, column 27",
        ),
        (
            "int main(void) {\n    return sizeof(_Atomic);\n}\n",
            "expected sizeof type after type qualifier '_Atomic', found RParen at line 2, column 26",
        ),
        (
            "int main(void) {\n    return _Alignof(volatile);\n}\n",
            "expected _Alignof type after type qualifier 'volatile', found RParen at line 2, column 29",
        ),
        (
            "int main(void) {\n    return _Alignof(_Atomic);\n}\n",
            "expected _Alignof type after type qualifier '_Atomic', found RParen at line 2, column 28",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn supports_sizeof_comma_expression_rhs_types_without_evaluating_operands() {
    let program = include_str!("fixtures/valid/sizeof_comma_expression_types.c");

    assert_eq!(interpret(program).unwrap(), 4);
}

#[test]
fn supports_sizeof_aggregate_conditional_expressions_without_evaluating_operands() {
    let program = include_str!("fixtures/valid/sizeof_aggregate_conditional_expressions.c");

    assert_eq!(interpret(program).unwrap(), 6);
}

#[test]
fn supports_sizeof_aggregate_assignment_expressions_without_evaluating_operands() {
    let program = include_str!("fixtures/valid/sizeof_aggregate_assignment_expressions.c");

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn supports_sizeof_aggregate_element_assignment_expressions_without_evaluating_operands() {
    let program = include_str!("fixtures/valid/sizeof_aggregate_element_assignment_expressions.c");

    assert_eq!(interpret(program).unwrap(), 12);
}

#[test]
fn supports_sizeof_embedded_aggregate_array_element_field_types_without_evaluating_operands() {
    let program = include_str!("fixtures/valid/sizeof_embedded_aggregate_array_element_fields.c");

    assert_eq!(interpret(program).unwrap(), 4);
}

#[test]
fn supports_field_access_on_aggregate_element_assignment_results() {
    let program = include_str!("fixtures/valid/aggregate_element_assignment_field_access.c");

    assert_eq!(interpret(program).unwrap(), 11);
}

#[test]
fn supports_field_access_on_aggregate_field_assignment_results() {
    let program = include_str!("fixtures/valid/aggregate_field_assignment_field_access.c");

    assert_eq!(interpret(program).unwrap(), 23);
}

#[test]
fn supports_unparenthesized_sizeof_in_integer_constant_expressions() {
    let program =
        include_str!("fixtures/valid/sizeof_unparenthesized_integer_constant_expressions.c");

    assert_eq!(interpret(program).unwrap(), 20);
}

#[test]
fn supports_integer_constant_expressions_for_array_lengths() {
    let program = include_str!("fixtures/valid/array_lengths_integer_constant_expressions.c");

    assert_eq!(interpret(program).unwrap(), 48);
}

#[test]
fn supports_integer_constant_expressions_for_array_type_lengths() {
    let program = include_str!("fixtures/valid/array_type_integer_constant_expressions.c");

    assert_eq!(interpret(program).unwrap(), 6);
}

#[test]
fn supports_inline_type_definitions_in_array_lengths() {
    let program = include_str!("fixtures/valid/inline_type_definitions_in_array_lengths.c");

    assert_eq!(interpret(program).unwrap(), 15);
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
fn supports_enum_compound_literals() {
    let program = include_str!("fixtures/valid/enum_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 32);
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
fn supports_array_typedef_compound_literals_as_pointer_expressions() {
    let program = include_str!("fixtures/valid/array_typedef_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 99);
}

#[test]
fn supports_const_array_typedef_compound_literals_as_const_pointer_expressions() {
    let program = include_str!("fixtures/valid/const_array_typedef_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 144);
}

#[test]
fn supports_chained_const_array_typedef_compound_literals() {
    let program = include_str!("fixtures/valid/chained_const_array_typedef_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 127);
}

#[test]
fn rejects_const_discard_from_chained_const_array_typedef_compound_literals() {
    let cases = [
        "typedef const int ConstInt;\n\
         typedef ConstInt ConstInts[2];\n\
         typedef ConstInts ChainedConstInts;\n\
         int main(void) {\n\
             int *values = (ChainedConstInts){1, 2};\n\
             return values[0];\n\
         }\n",
        "struct Point { int x; int y; };\n\
         typedef const struct Point ConstPoint;\n\
         typedef ConstPoint ConstPoints[2];\n\
         typedef ConstPoints ChainedConstPoints;\n\
         int main(void) {\n\
             struct Point *points = (ChainedConstPoints){{1, 2}, {3, 4}};\n\
             return points[0].x;\n\
         }\n",
        "enum State { IDLE = 1, READY = 2 };\n\
         typedef const enum State ConstState;\n\
         typedef ConstState ConstStates[2];\n\
         typedef ConstStates ChainedConstStates;\n\
         int main(void) {\n\
             enum State *states = (ChainedConstStates){IDLE, READY};\n\
             return states[0];\n\
         }\n",
    ];

    for program in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(
            err.to_string(),
            "cannot discard const qualifier from pointer target",
            "program: {program}"
        );
    }
}

#[test]
fn rejects_writes_through_chained_const_array_typedef_compound_literals() {
    let cases = [
        "typedef const int ConstInt;\n\
         typedef ConstInt ConstInts[2];\n\
         typedef ConstInts ChainedConstInts;\n\
         int main(void) {\n\
             const int *values = (ChainedConstInts){1, 2};\n\
             values[0] = 7;\n\
             return values[0];\n\
         }\n",
        "struct Point { int x; int y; };\n\
         typedef const struct Point ConstPoint;\n\
         typedef ConstPoint ConstPoints[2];\n\
         typedef ConstPoints ChainedConstPoints;\n\
         int main(void) {\n\
             const struct Point *points = (ChainedConstPoints){{1, 2}, {3, 4}};\n\
             points[0].x = 7;\n\
             return points[0].x;\n\
         }\n",
        "enum State { IDLE = 1, READY = 2 };\n\
         typedef const enum State ConstState;\n\
         typedef ConstState ConstStates[2];\n\
         typedef ConstStates ChainedConstStates;\n\
         int main(void) {\n\
             const enum State *states = (ChainedConstStates){IDLE, READY};\n\
             states[0] = READY;\n\
             return states[0];\n\
         }\n",
    ];

    for program in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(
            err.to_string(),
            "cannot assign through pointer to const",
            "program: {program}"
        );
    }
}

#[test]
fn rejects_const_array_typedef_compound_literal_const_discard() {
    let program = include_str!("fixtures/invalid/const_array_typedef_compound_literal_discard.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_const_array_typedef_compound_literal_writes() {
    let program = include_str!("fixtures/invalid/const_array_typedef_compound_literal_write.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "cannot assign through pointer to const");
}

#[test]
fn supports_aggregate_array_compound_literals_as_pointer_expressions() {
    let program = include_str!("fixtures/valid/aggregate_array_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 48);
}

#[test]
fn supports_anonymous_aggregate_compound_literals() {
    let program = include_str!("fixtures/valid/anonymous_aggregate_compound_literals.c");

    assert_eq!(interpret(program).unwrap(), 21);
}

#[test]
fn supports_anonymous_aggregate_pointer_casts() {
    let program = include_str!("fixtures/valid/anonymous_aggregate_pointer_casts.c");

    assert_eq!(interpret(program).unwrap(), 3);
}

#[test]
fn supports_anonymous_aggregate_fields() {
    let program = include_str!("fixtures/valid/anonymous_aggregate_fields.c");

    assert_eq!(interpret(program).unwrap(), 21);
}

#[test]
fn supports_nested_named_aggregate_fields() {
    let program = include_str!("fixtures/valid/nested_named_aggregate_fields.c");

    assert_eq!(interpret(program).unwrap(), 59);
}

#[test]
fn supports_inline_enum_aggregate_fields() {
    let program = include_str!("fixtures/valid/inline_enum_aggregate_fields.c");

    assert_eq!(interpret(program).unwrap(), 44);
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
        "too many initializers for aggregate array compound literal at line 7, column 54"
    );
}

#[test]
fn rejects_scalar_compound_literals_with_too_many_initializers() {
    let program = include_str!("fixtures/invalid/scalar_compound_literal_too_many_initializers.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "too many initializers for scalar compound literal at line 2, column 21"
    );
}

#[test]
fn rejects_missing_scalar_compound_literal_initializers_with_context() {
    let cases = [
        (
            include_str!("fixtures/invalid/scalar_compound_literal_missing_initializer.c"),
            "expected initializer element in braced scalar initializer for scalar compound literal, found RBrace at line 2, column 18",
        ),
        (
            include_str!("fixtures/invalid/scalar_compound_literal_comma_missing_initializer.c"),
            "expected initializer element in braced scalar initializer for scalar compound literal, found Comma at line 2, column 19",
        ),
        (
            include_str!(
                "fixtures/invalid/scalar_compound_literal_semicolon_missing_initializer.c"
            ),
            "expected initializer element in braced scalar initializer for scalar compound literal, found Semi at line 2, column 18",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_array_designator_starts_in_struct_compound_literal_initializers_with_context() {
    let program =
        "int main(void) { struct Point { int x; }; struct Point p = (struct Point){[}; return 0; }";

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected initializer element in struct 'Point' initializer, found LBracket at line 1, column 75"
    );
}

#[test]
fn rejects_range_array_designators_with_context() {
    let cases = [
        (
            "int main(void) {\n    int values[3] = {[0 ... 2] = 1};\n    return 0;\n}\n",
            "array range designators are not supported at line 2, column 25",
        ),
        (
            "int main(void) {\n    int *values = (int[]){[0 ... 2] = 1};\n    return 0;\n}\n",
            "array range designators are not supported at line 2, column 30",
        ),
        (
            "struct Packet { int values[3]; };\nint main(void) {\n    struct Packet packet = {.values[0 ... 2] = 1};\n    return 0;\n}\n",
            "array range designators are not supported at line 3, column 39",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_array_compound_literals_longer_than_declared_length() {
    let program = include_str!("fixtures/invalid/array_compound_literal_too_many_initializers.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "too many initializers for array compound literal at line 2, column 34"
    );
}

#[test]
fn rejects_array_compound_literal_string_initializers_that_are_too_long() {
    let program = include_str!("fixtures/invalid/array_compound_literal_string_too_long.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "initializer string for char array compound literal is too long at line 2, column 28"
    );
}

#[test]
fn reports_non_char_array_string_initializer_type_mismatches_with_source_locations() {
    let cases = [
        (
            include_str!("fixtures/invalid/non_char_array_string_initializer.c"),
            "string literal initializer requires char array 'values' at line 2, column 21",
        ),
        (
            include_str!("fixtures/invalid/non_char_inferred_array_string_initializer.c"),
            "string literal initializer requires char array 'values' at line 2, column 20",
        ),
        (
            include_str!("fixtures/invalid/non_char_array_compound_literal_string_initializer.c"),
            "string literal initializer requires char array compound literal at line 2, column 27",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn supports_direct_enum_type_declarations_parameters_returns_and_sizeof() {
    let program = include_str!("fixtures/valid/direct_enum_types.c");

    assert_eq!(interpret(program).unwrap(), 24);
}

#[test]
fn supports_direct_enum_pointer_parameters_casts_and_type_queries() {
    let program = include_str!("fixtures/valid/direct_enum_pointer_type_queries.c");

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_direct_enum_aggregate_fields_arrays_and_pointers() {
    let program = include_str!("fixtures/valid/direct_enum_aggregate_fields.c");

    assert_eq!(interpret(program).unwrap(), 39);
}

#[test]
fn rejects_assignment_to_const_enum_aggregate_fields() {
    let program = include_str!("fixtures/invalid/const_enum_aggregate_field_assignment.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'fixed'"
    );
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
fn supports_integer_constant_expressions_for_designator_indexes() {
    let program = include_str!("fixtures/valid/integer_constant_designator_indexes.c");

    assert_eq!(interpret(program).unwrap(), 240);
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
fn supports_anonymous_aggregate_for_initializers() {
    let program = include_str!("fixtures/valid/anonymous_aggregate_for_initializers.c");

    assert_eq!(interpret(program).unwrap(), 17);
}

#[test]
fn supports_qualified_anonymous_aggregate_for_initializers() {
    let program = include_str!("fixtures/valid/qualified_anonymous_aggregate_for_initializers.c");

    assert_eq!(interpret(program).unwrap(), 24);
}

#[test]
fn supports_inline_enum_object_declarations() {
    let program = include_str!("fixtures/valid/inline_enum_object_declarations.c");

    assert_eq!(interpret(program).unwrap(), 48);
}

#[test]
fn supports_inline_enum_object_declarations_in_storage_and_for_contexts() {
    let program = include_str!("fixtures/valid/inline_enum_declaration_contexts.c");

    assert_eq!(interpret(program).unwrap(), 41);
}

#[test]
fn supports_inline_enum_return_type_definitions() {
    let program = include_str!("fixtures/valid/inline_enum_return_types.c");

    assert_eq!(interpret(program).unwrap(), 37);
}

#[test]
fn supports_inline_enum_parameter_definitions_in_function_bodies() {
    let program = include_str!("fixtures/valid/inline_enum_parameter_definitions.c");

    assert_eq!(interpret(program).unwrap(), 12);
}

#[test]
fn supports_inline_enum_cast_type_definitions() {
    let program = include_str!("fixtures/valid/inline_enum_cast_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 19);
}

#[test]
fn supports_inline_enum_sizeof_type_definitions() {
    let program = include_str!("fixtures/valid/inline_enum_sizeof_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 24);
}

#[test]
fn supports_inline_enum_alignof_type_definitions() {
    let program = include_str!("fixtures/valid/inline_enum_alignof_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 22);
}

#[test]
fn supports_inline_enum_call_argument_type_definitions() {
    let program = include_str!("fixtures/valid/inline_enum_call_argument_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 36);
}

#[test]
fn supports_inline_enum_pointer_aggregate_expression_statements() {
    let program =
        include_str!("fixtures/valid/inline_enum_pointer_aggregate_expression_statements.c");

    assert_eq!(interpret(program).unwrap(), 97);
}

#[test]
fn supports_inline_enum_aggregate_initializer_type_definitions() {
    let program =
        include_str!("fixtures/valid/inline_enum_aggregate_initializer_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 122);
}

#[test]
fn supports_inline_enum_conditional_type_definitions() {
    let program = include_str!("fixtures/valid/inline_enum_conditional_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 73);
}

#[test]
fn supports_inline_enum_assignment_lvalue_type_definitions() {
    let program = include_str!("fixtures/valid/inline_enum_assignment_lvalue_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 82);
}

#[test]
fn supports_inline_enum_control_expr_definitions() {
    let program = include_str!("fixtures/valid/inline_enum_control_expr_definitions.c");

    assert_eq!(interpret(program).unwrap(), 44);
}

#[test]
fn supports_inline_enum_switch_case_label_definitions() {
    let program = include_str!("fixtures/valid/inline_enum_switch_case_labels.c");

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn supports_mixed_declaration_context_conformance_fixture() {
    let program = include_str!("fixtures/valid/mixed_declaration_contexts.c");

    assert_eq!(interpret(program).unwrap(), 45);
}

#[test]
fn supports_mixed_aggregate_field_type_query_conformance_fixture() {
    let program = include_str!("fixtures/valid/mixed_aggregate_field_type_queries.c");

    assert_eq!(interpret(program).unwrap(), 52);
}

#[test]
fn rejects_assignment_to_const_inline_enum_object() {
    let program = include_str!("fixtures/invalid/const_inline_enum_assignment.c");

    let err = interpret(program).unwrap_err();
    assert!(
        err.to_string()
            .contains("cannot assign to const variable 'value'"),
        "unexpected error: {err}"
    );
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
fn rejects_assignment_to_nested_fields_of_const_aggregate_fields() {
    let program = include_str!("fixtures/invalid/const_aggregate_field_nested_assignment.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'point'"
    );
}

#[test]
fn rejects_assignment_to_nested_fields_of_const_anonymous_aggregate_fields() {
    let program =
        include_str!("fixtures/invalid/const_anonymous_aggregate_field_nested_assignment.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'point'"
    );
}

#[test]
fn rejects_assignment_to_nested_fields_of_const_aggregate_array_fields() {
    let program =
        include_str!("fixtures/invalid/const_aggregate_array_field_element_nested_assignment.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'points'"
    );
}

#[test]
fn rejects_assignment_to_nested_fields_of_const_aggregate_array_fields_on_compound_literals() {
    let program = include_str!(
        "fixtures/invalid/const_aggregate_compound_literal_array_field_nested_assignment.c"
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'points'"
    );
}

#[test]
fn rejects_assignment_to_const_anonymous_aggregate_array_elements() {
    let program =
        include_str!("fixtures/invalid/const_anonymous_aggregate_array_element_assignment.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign through pointer to const");
}

#[test]
fn rejects_anonymous_aggregate_pointer_to_pointer_declarations_with_targeted_diagnostic() {
    let program =
        include_str!("fixtures/invalid/anonymous_aggregate_pointer_to_pointer_declaration.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "pointer-to-pointer declarations are not supported at line 1, column 37"
    );
}

#[test]
fn rejects_anonymous_aggregate_pointer_array_declarations_with_targeted_diagnostic() {
    let program = include_str!("fixtures/invalid/anonymous_aggregate_pointer_array_declaration.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "pointer array declarations are not supported at line 1, column 42"
    );
}

#[test]
fn rejects_distinct_anonymous_aggregate_pointer_assignments() {
    let program =
        include_str!("fixtures/invalid/anonymous_aggregate_distinct_type_pointer_assignment.c");

    let err = interpret(program).unwrap_err();
    assert!(
        err.to_string().contains("cannot convert pointer to struct"),
        "unexpected error: {err}"
    );
}

#[test]
fn rejects_anonymous_aggregate_parenthesized_pointer_declarations_with_targeted_diagnostic() {
    let program =
        include_str!("fixtures/invalid/anonymous_aggregate_parenthesized_pointer_declaration.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "parenthesized pointer declarations are not supported at line 1, column 36"
    );
}

#[test]
fn rejects_const_anonymous_aggregate_for_initializer_pointer_writes() {
    let program =
        include_str!("fixtures/invalid/const_anonymous_aggregate_for_initializer_pointer_write.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign through pointer to const");
}

#[test]
fn supports_path_designated_struct_initializers() {
    let program = include_str!("fixtures/valid/path_designated_initializers.c");

    assert_eq!(interpret(program).unwrap(), 156);
}

#[test]
fn supports_aggregate_array_field_path_designators() {
    let program = include_str!("fixtures/valid/aggregate_array_field_path_designators.c");

    assert_eq!(interpret(program).unwrap(), 84);
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
fn supports_aggregate_pointer_field_assignment_result_metadata() {
    let program =
        include_str!("fixtures/valid/aggregate_pointer_field_assignment_result_metadata.c");

    assert_eq!(interpret(program).unwrap(), 12);
}

#[test]
fn supports_embedded_aggregate_array_pointer_model_routes() {
    let program = include_str!("fixtures/valid/embedded_aggregate_array_pointer_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 246);
}

#[test]
fn supports_scalar_array_field_pointer_model_routes() {
    let program = include_str!("fixtures/valid/scalar_array_field_pointer_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 56);
}

#[test]
fn supports_hidden_scalar_array_literal_pointer_model_routes() {
    let program = include_str!("fixtures/valid/hidden_scalar_array_literal_pointer_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 65);
}

#[test]
fn supports_hidden_aggregate_array_literal_pointer_model_routes() {
    let program =
        include_str!("fixtures/valid/hidden_aggregate_array_literal_pointer_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 164);
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
fn supports_addresses_of_aggregate_pointer_indexed_elements() {
    let program = include_str!("fixtures/valid/aggregate_pointer_index_addresses.c");

    assert_eq!(interpret(program).unwrap(), 205);
}

#[test]
fn supports_addresses_of_aggregate_pointer_expression_indexed_elements() {
    let program = include_str!("fixtures/valid/aggregate_pointer_expression_index_addresses.c");

    assert_eq!(interpret(program).unwrap(), 110);
}

#[test]
fn rejects_aggregate_pointer_call_index_addresses_that_discard_const() {
    let program =
        include_str!("fixtures/invalid/aggregate_pointer_call_index_address_const_discard.c",);

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn supports_addresses_of_direct_aggregate_pointer_field_indexed_elements() {
    let program = include_str!("fixtures/valid/direct_aggregate_pointer_field_index_address.c");

    assert_eq!(interpret(program).unwrap(), 20);
}

#[test]
fn supports_addresses_of_arrow_aggregate_pointer_field_indexed_elements() {
    let program = include_str!("fixtures/valid/arrow_aggregate_pointer_field_index_address.c");

    assert_eq!(interpret(program).unwrap(), 20);
}

#[test]
fn rejects_direct_aggregate_pointer_field_index_addresses_that_discard_const() {
    let program = include_str!(
        "fixtures/invalid/direct_aggregate_pointer_field_index_address_const_discard.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_arrow_aggregate_pointer_field_index_addresses_that_discard_const() {
    let program = include_str!(
        "fixtures/invalid/arrow_aggregate_pointer_field_index_address_const_discard.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_aggregate_pointer_index_addresses_that_discard_const() {
    let program = include_str!("fixtures/invalid/aggregate_pointer_index_address_const_discard.c",);

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_out_of_bounds_aggregate_pointer_index_addresses() {
    let program = include_str!("fixtures/invalid/aggregate_pointer_index_address_out_of_bounds.c",);

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "struct array pointer index 2 out of bounds for length 2"
    );
}

#[test]
fn rejects_out_of_bounds_aggregate_pointer_field_index_addresses() {
    let cases = [
        include_str!(
            "fixtures/invalid/direct_aggregate_pointer_field_index_address_out_of_bounds.c",
        ),
        include_str!(
            "fixtures/invalid/arrow_aggregate_pointer_field_index_address_out_of_bounds.c",
        ),
    ];

    for program in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(
            err.to_string(),
            "struct array pointer index 2 out of bounds for length 2"
        );
    }
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
fn rejects_const_embedded_aggregate_array_element_copy_assignment() {
    let program =
        include_str!("fixtures/invalid/const_embedded_aggregate_array_element_copy_assignment.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'points'"
    );
}

#[test]
fn rejects_const_embedded_aggregate_array_element_copy_assignment_through_pointer() {
    let program = include_str!(
        "fixtures/invalid/const_embedded_aggregate_array_element_copy_assignment_through_pointer.c"
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'points'"
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
fn supports_pointer_return_function_model_routes() {
    let program = include_str!("fixtures/valid/pointer_return_function_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 114);
}

#[test]
fn supports_pointer_parameter_forwarding_model_routes() {
    let program = include_str!("fixtures/valid/pointer_parameter_forwarding_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 121);
}

#[test]
fn supports_pointer_parameter_mutation_model_routes() {
    let program = include_str!("fixtures/valid/pointer_parameter_mutation_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 35);
}

#[test]
fn supports_pointer_parameter_alias_mutation_model_routes() {
    let program = include_str!("fixtures/valid/pointer_parameter_alias_mutation_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 54);
}

#[test]
fn supports_pointer_parameter_mixed_qualification_alias_model_routes() {
    let program =
        include_str!("fixtures/valid/pointer_parameter_mixed_qualification_alias_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 63);
}

#[test]
fn supports_field_backed_mixed_qualification_parameter_alias_model_routes() {
    let program = include_str!(
        "fixtures/valid/field_backed_mixed_qualification_parameter_alias_model_routes.c",
    );

    assert_eq!(interpret(program).unwrap(), 28);
}

#[test]
fn supports_field_backed_pointer_return_forwarding_model_routes() {
    let program =
        include_str!("fixtures/valid/field_backed_pointer_return_forwarding_model_routes.c",);

    assert_eq!(interpret(program).unwrap(), 160);
}

#[test]
fn supports_field_backed_pointer_return_alias_mutation_model_routes() {
    let program =
        include_str!("fixtures/valid/field_backed_pointer_return_alias_mutation_model_routes.c",);

    assert_eq!(interpret(program).unwrap(), 36);
}

#[test]
fn supports_nested_anonymous_field_backed_pointer_return_alias_mutation_model_routes() {
    let program = include_str!(
        "fixtures/valid/nested_anonymous_field_backed_pointer_return_alias_mutation_model_routes.c",
    );

    assert_eq!(interpret(program).unwrap(), 36);
}

#[test]
fn supports_aggregate_compound_literal_field_pointer_alias_mutation_model_routes() {
    let program = include_str!(
        "fixtures/valid/aggregate_compound_literal_field_pointer_alias_mutation_model_routes.c",
    );

    assert_eq!(interpret(program).unwrap(), 20);
}

#[test]
fn supports_nested_anonymous_aggregate_compound_literal_field_pointer_alias_mutation_model_routes()
{
    let program = include_str!(
        "fixtures/valid/nested_anonymous_aggregate_compound_literal_field_pointer_alias_mutation_model_routes.c",
    );

    assert_eq!(interpret(program).unwrap(), 20);
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
fn supports_block_scoped_named_aggregate_definitions() {
    let program = include_str!("fixtures/valid/block_scoped_named_aggregate_definitions.c");

    assert_eq!(interpret(program).unwrap(), 57);
}

#[test]
fn supports_named_aggregate_definition_declarators() {
    let program = include_str!("fixtures/valid/named_aggregate_definition_declarators.c");

    assert_eq!(interpret(program).unwrap(), 58);
}

#[test]
fn supports_inline_aggregate_return_type_definitions() {
    let program = include_str!("fixtures/valid/inline_aggregate_return_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 84);
}

#[test]
fn supports_inline_aggregate_expression_type_definitions() {
    let program = include_str!("fixtures/valid/inline_aggregate_expression_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 36);
}

#[test]
fn supports_inline_aggregate_control_type_definitions() {
    let program = include_str!("fixtures/valid/inline_aggregate_control_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 29);
}

#[test]
fn supports_inline_aggregate_for_clause_type_definitions() {
    let program = include_str!("fixtures/valid/inline_aggregate_for_clause_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 15);
}

#[test]
fn supports_inline_aggregate_call_argument_type_definitions() {
    let program = include_str!("fixtures/valid/inline_aggregate_call_argument_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 38);
}

#[test]
fn supports_inline_aggregate_static_assert_type_definitions() {
    let program = include_str!("fixtures/valid/inline_aggregate_static_assert_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 21);
}

#[test]
fn supports_inline_aggregate_conditional_type_definitions() {
    let program = include_str!("fixtures/valid/inline_aggregate_conditional_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 28);
}

#[test]
fn supports_inline_aggregate_declaration_assignment_type_definitions() {
    let program =
        include_str!("fixtures/valid/inline_aggregate_declaration_assignment_type_definitions.c",);

    assert_eq!(interpret(program).unwrap(), 34);
}

#[test]
fn supports_inline_aggregate_expression_statement_type_definitions() {
    let program =
        include_str!("fixtures/valid/inline_aggregate_expression_statement_type_definitions.c",);

    assert_eq!(interpret(program).unwrap(), 29);
}

#[test]
fn supports_inline_aggregate_return_expression_type_definitions() {
    let program =
        include_str!("fixtures/valid/inline_aggregate_return_expression_type_definitions.c",);

    assert_eq!(interpret(program).unwrap(), 22);
}

#[test]
fn supports_inline_aggregate_initializer_type_definitions() {
    let program = include_str!("fixtures/valid/inline_aggregate_initializer_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 36);
}

#[test]
fn supports_inline_aggregate_pointer_initializer_type_definitions() {
    let program =
        include_str!("fixtures/valid/inline_aggregate_pointer_initializer_type_definitions.c",);

    assert_eq!(interpret(program).unwrap(), 56);
}

#[test]
fn supports_inline_aggregate_array_compound_literal_type_definitions() {
    let program =
        include_str!("fixtures/valid/inline_aggregate_array_compound_literal_type_definitions.c",);

    assert_eq!(interpret(program).unwrap(), 103);
}

#[test]
fn supports_inline_aggregate_aggregate_array_compound_literal_type_definitions() {
    let program = include_str!(
        "fixtures/valid/inline_aggregate_aggregate_array_compound_literal_type_definitions.c",
    );

    assert_eq!(interpret(program).unwrap(), 105);
}

#[test]
fn supports_inline_aggregate_pointer_arithmetic_type_definitions() {
    let program =
        include_str!("fixtures/valid/inline_aggregate_pointer_arithmetic_type_definitions.c",);

    assert_eq!(interpret(program).unwrap(), 168);
}

#[test]
fn supports_inline_aggregate_pointer_comparison_type_definitions() {
    let program =
        include_str!("fixtures/valid/inline_aggregate_pointer_comparison_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 65);
}

#[test]
fn supports_inline_aggregate_sizeof_pointer_expression_type_definitions() {
    let program = include_str!(
        "fixtures/valid/inline_aggregate_sizeof_pointer_expression_type_definitions.c",
    );

    assert_eq!(interpret(program).unwrap(), 52);
}

#[test]
fn supports_inline_aggregate_parameter_type_definitions() {
    let program = include_str!("fixtures/valid/inline_aggregate_parameter_type_definitions.c");

    assert_eq!(interpret(program).unwrap(), 19);
}

#[test]
fn rejects_inline_aggregate_parameter_tags_after_function_scope() {
    let program = include_str!("fixtures/invalid/inline_aggregate_parameter_tag_leaks.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "undefined struct type 'Pair'");
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
fn rejects_block_scoped_named_aggregate_tags_after_scope_exit() {
    let program = include_str!("fixtures/invalid/block_scoped_named_aggregate_tag_out_of_scope.c");

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

    assert_eq!(
        err.to_string(),
        "too many initializers for union 'Number' at line 7, column 26"
    );
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

    assert_eq!(
        err.to_string(),
        "too many initializers for array 'values' at line 6, column 36"
    );
}

#[test]
fn rejects_struct_array_variable_initializers_longer_than_declared_length() {
    let program = include_str!("fixtures/invalid/struct_array_variable_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "too many initializers for struct array 'points' at line 7, column 39"
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

    assert_eq!(
        err.to_string(),
        "too many initializers for array 'values' at line 2, column 28"
    );
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
    let cases = [
        (
            include_str!("fixtures/invalid/struct_array_path_designator_out_of_bounds.c"),
            "array designator index 2 out of bounds for array field 'values'",
        ),
        (
            include_str!("fixtures/invalid/aggregate_array_field_path_designator_out_of_bounds.c"),
            "array designator index 2 out of bounds for array field 'points'",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_malformed_path_designators_with_context() {
    let cases = [
        (
            "struct Inner { int x; };\nstruct Box { struct Inner inner; };\nint main(void) {\n    struct Box box = { .inner. = 1 };\n    return 0;\n}\n",
            "expected struct field name after '.', found Assign at line 4, column 32",
        ),
        (
            "struct Packet { int values[2]; };\nint main(void) {\n    struct Packet packet = { .values[] = 1 };\n    return 0;\n}\n",
            "expected array designator index before ']' at line 3, column 38",
        ),
        (
            "struct Packet { int values[2]; };\nint main(void) {\n    struct Packet packet = { .values[;] = 1 };\n    return 0;\n}\n",
            "expected array designator index before ';' at line 3, column 38",
        ),
        (
            "struct Packet { int values[2]; };\nint main(void) {\n    struct Packet packet = { .values[,] = 1 };\n    return 0;\n}\n",
            "expected array designator index before ',' at line 3, column 38",
        ),
        (
            "struct Packet { int values[2]; };\nint main(void) {\n    struct Packet packet = { .values[} = 1 };\n    return 0;\n}\n",
            "expected array designator index before '}' at line 3, column 38",
        ),
        (
            "struct Packet { int values[2]; };\nint main(void) {\n    struct Packet packet = { .values[[] = 1 };\n    return 0;\n}\n",
            "expected array designator index before '[' at line 3, column 38",
        ),
        (
            "struct Packet { int values[2]; };\nint main(void) {\n    struct Packet packet = { .values[?] = 1 };\n    return 0;\n}\n",
            "expected array designator index before '?' at line 3, column 38",
        ),
        (
            "int main(void) {\n    int *values = (int[]){[;] = 1};\n    return 0;\n}\n",
            "expected array designator index before ';' at line 2, column 28",
        ),
        (
            "int main(void) {\n    int *values = (int[]){[,] = 1};\n    return 0;\n}\n",
            "expected array designator index before ',' at line 2, column 28",
        ),
        (
            "int main(void) {\n    int *values = (int[]){[} = 1};\n    return 0;\n}\n",
            "expected array designator index before '}' at line 2, column 28",
        ),
        (
            "int main(void) {\n    int *values = (int[]){[[] = 1};\n    return 0;\n}\n",
            "expected array designator index before '[' at line 2, column 28",
        ),
        (
            "int main(void) {\n    int *values = (int[]){[?] = 1};\n    return 0;\n}\n",
            "expected array designator index before '?' at line 2, column 28",
        ),
        (
            "struct Packet { int values[2]; };\nint main(void) {\n    struct Packet packet = { .values[.field] = 1 };\n    return 0;\n}\n",
            "expected array designator index before '.' at line 3, column 38",
        ),
        (
            "int main(void) {\n    int *values = (int[]){[->field] = 1};\n    return 0;\n}\n",
            "expected array designator index before '->' at line 2, column 28",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_struct_initializers_longer_than_declared_fields() {
    let program = include_str!("fixtures/invalid/struct_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "too many initializers for struct 'Point' at line 7, column 33"
    );
}

#[test]
fn rejects_nested_struct_initializers_longer_than_nested_fields() {
    let program = include_str!("fixtures/invalid/nested_struct_initializer_too_long.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "too many initializers for struct 'Point' at line 12, column 32"
    );
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
fn supports_parenthesized_function_declarators() {
    let program = include_str!("fixtures/valid/parenthesized_function_declarators.c");

    assert_eq!(interpret(program).unwrap(), 9);
}

#[test]
fn supports_parenthesized_variable_declarators() {
    let program = include_str!("fixtures/compat/valid/parenthesized_variable_declarators.c");

    assert_eq!(interpret(program).unwrap(), 25);
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
fn supports_aggregate_assignment_expressions_as_function_arguments() {
    let program = "struct Point { int x; };\n\
                   int read_x(struct Point point) { return point.x; }\n\
                   int main(void) {\n\
                   struct Point target = {1};\n\
                   struct Point replacement = {7};\n\
                   return read_x(target = replacement) + target.x;\n\
                   }\n";

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_aggregate_dereference_assignment_expressions_as_function_arguments() {
    let program = "struct Point { int x; };\n\
                   int read_x(struct Point point) { return point.x; }\n\
                   int main(void) {\n\
                   struct Point target = {1};\n\
                   struct Point replacement = {7};\n\
                   struct Point *slot = &target;\n\
                   return read_x(*slot = replacement) + target.x;\n\
                   }\n";

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_aggregate_field_assignment_expressions_as_function_arguments() {
    let program = "struct Point { int x; };\n\
                   struct Box { struct Point point; };\n\
                   int read_x(struct Point point) { return point.x; }\n\
                   int main(void) {\n\
                   struct Box box = {{1}};\n\
                   struct Point replacement = {7};\n\
                   return read_x(box.point = replacement) + box.point.x;\n\
                   }\n";

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_aggregate_pointer_field_assignment_expressions_as_function_arguments() {
    let program = "struct Point { int x; };\n\
                   struct Box { struct Point point; };\n\
                   int read_x(struct Point point) { return point.x; }\n\
                   int main(void) {\n\
                   struct Box box = {{1}};\n\
                   struct Box *view = &box;\n\
                   struct Point replacement = {7};\n\
                   return read_x(view->point = replacement) + box.point.x;\n\
                   }\n";

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_aggregate_array_element_assignment_expressions_as_function_arguments() {
    let program = "struct Point { int x; };\n\
                   int read_x(struct Point point) { return point.x; }\n\
                   int main(void) {\n\
                   struct Point points[1] = {{1}};\n\
                   struct Point replacement = {7};\n\
                   return read_x(points[0] = replacement) + points[0].x;\n\
                   }\n";

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_embedded_aggregate_array_element_assignment_expressions_as_function_arguments() {
    let program = "struct Point { int x; };\n\
                   struct Box { struct Point points[1]; };\n\
                   int read_x(struct Point point) { return point.x; }\n\
                   int main(void) {\n\
                   struct Box box = {{{1}}};\n\
                   struct Point replacement = {7};\n\
                   return read_x(box.points[0] = replacement) + box.points[0].x;\n\
                   }\n";

    assert_eq!(interpret(program).unwrap(), 14);
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
fn rejects_missing_struct_field_names_after_dot_and_arrow_with_context() {
    let cases = [
        (
            "struct Point { int x; };\nint main(void) { struct Point p = {1}; return p.; }\n",
            "expected struct field name after '.', found Semi at line 2, column 49",
        ),
        (
            "struct Point { int x; };\nint main(void) { struct Point p = {1}; return p.]; }\n",
            "expected struct field name after '.', found RBracket at line 2, column 49",
        ),
        (
            "struct Point { int x; };\nint main(void) { struct Point p = {1}; struct Point *q = &p; return q->; }\n",
            "expected struct field name after '->', found Semi at line 2, column 72",
        ),
        (
            "struct Point { int x; };\nint main(void) { struct Point p = {1}; struct Point *q = &p; return q->]; }\n",
            "expected struct field name after '->', found RBracket at line 2, column 72",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
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
fn rejects_missing_integer_constant_cast_operands_with_context() {
    let cases = [
        (
            "enum Bad { VALUE = (int) };\nint main(void) { return VALUE; }\n",
            "expected integer constant after cast, found RBrace at line 1, column 26",
        ),
        (
            "int main(void) {\n    int values[(int)];\n    return 0;\n}\n",
            "expected integer constant after cast, found RBracket at line 2, column 21",
        ),
        (
            "int main(void) {\n    switch (1) { case (int): return 1; default: return 0; }\n}\n",
            "expected integer constant after cast, found Colon at line 2, column 28",
        ),
        (
            "enum Bad { VALUE = (int)return };\nint main(void) { return VALUE; }\n",
            "expected integer constant after cast, found Return at line 1, column 25",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_missing_parenthesized_integer_constant_operands_with_context() {
    let cases = [
        (
            "enum Bad { VALUE = () };\nint main(void) { return VALUE; }\n",
            "expected integer constant in parenthesized integer constant expression before ')' at line 1, column 21",
        ),
        (
            "int main(void) {\n    int values[(,)];\n    return 0;\n}\n",
            "expected integer constant in parenthesized integer constant expression before ',' at line 2, column 17",
        ),
        (
            "int main(void) {\n    switch (1) { case (: return 1; default: return 0; }\n}\n",
            "expected integer constant in parenthesized integer constant expression before ':' at line 2, column 24",
        ),
        (
            "int main(void) {\n    int values[(?)];\n    return 0;\n}\n",
            "expected integer constant in parenthesized integer constant expression before '?' at line 2, column 17",
        ),
        (
            "enum Bad { VALUE = (return) };\nint main(void) { return VALUE; }\n",
            "expected integer constant in parenthesized integer constant expression before 'return' at line 1, column 21",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_missing_integer_constant_unary_operands_with_context() {
    let cases = [
        (
            "enum Bad { VALUE = + };\nint main(void) { return VALUE; }\n",
            "expected integer constant after unary operator '+', found RBrace at line 1, column 22",
        ),
        (
            "int main(void) {\n    int values[-];\n    return 0;\n}\n",
            "expected integer constant after unary operator '-', found RBracket at line 2, column 17",
        ),
        (
            "int main(void) {\n    switch (1) { case !: return 1; default: return 0; }\n}\n",
            "expected integer constant after unary operator '!', found Colon at line 2, column 24",
        ),
        (
            "enum Bad { VALUE = ~return };\nint main(void) { return VALUE; }\n",
            "expected integer constant after unary operator '~', found Return at line 1, column 21",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_invalid_start_enum_constant_values_with_context() {
    let cases = [
        (
            include_str!("fixtures/invalid/enum_invalid_start_value.c"),
            "expected integer constant after enum constant '=' before '[' at line 2, column 29",
        ),
        (
            include_str!("fixtures/invalid/enum_question_value.c"),
            "expected integer constant after enum constant '=' before '?' at line 2, column 29",
        ),
        (
            "int main(void) { enum BadStart { FIRST = int }; return FIRST; }",
            "expected integer constant after enum constant '=' before 'int' at line 1, column 42",
        ),
        (
            "int main(void) { enum BadStart { FIRST = return }; return FIRST; }",
            "expected integer constant after enum constant '=' before 'return' at line 1, column 42",
        ),
        (
            "int main(void) { enum BadStart { FIRST = .field }; return FIRST; }",
            "expected integer constant after enum constant '=' before '.' at line 1, column 42",
        ),
        (
            "int main(void) { enum BadStart { FIRST = ->field }; return FIRST; }",
            "expected integer constant after enum constant '=' before '->' at line 1, column 42",
        ),
        (
            "int main(void) { enum BadStart { FIRST = {1} }; return FIRST; }",
            "expected integer constant after enum constant '=' before '{' at line 1, column 42",
        ),
        (
            "int main(void) { enum BadStart { FIRST = : }; return FIRST; }",
            "expected integer constant after enum constant '=' before ':' at line 1, column 42",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();

        assert_eq!(err.to_string(), expected);
    }
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
        "expected expression after unary operator '+', found Semi at line 2, column 9"
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
fn rejects_missing_initial_function_call_arguments_with_context() {
    let cases = [
        (
            "int first(int value) { return value; }\nint main() { return first(; }\n",
            "expected function call argument, found Semi at line 2, column 27",
        ),
        (
            "int first(int value) { return value; }\nint main() { return first(} }\n",
            "expected function call argument, found RBrace at line 2, column 27",
        ),
        (
            "int first(int value) { return value; }\nint main() { return first(,); }\n",
            "expected function call argument, found Comma at line 2, column 27",
        ),
        (
            "int add(int left, int right) { return left + right; }\nint main() { return add(, 2); }\n",
            "expected function call argument, found Comma at line 2, column 25",
        ),
        (
            "int first(int value) { return value; }\nint main() { return first(]; }\n",
            "expected function call argument, found RBracket at line 2, column 27",
        ),
        (
            "int first(int value) { return value; }\nint main() { return first(.field); }\n",
            "expected function call argument, found Dot at line 2, column 27",
        ),
        (
            "int first(int value) { return value; }\nint main() { return first(->field); }\n",
            "expected function call argument, found Arrow at line 2, column 27",
        ),
        (
            "int add(int left, int right) { return left + right; }\nint main() { return add(1,]); }\n",
            "expected function call argument after ',', found RBracket at line 2, column 27",
        ),
        (
            "int add(int left, int right) { return left + right; }\nint main() { return add(1,.field); }\n",
            "expected function call argument after ',', found Dot at line 2, column 27",
        ),
        (
            "int add(int left, int right) { return left + right; }\nint main() { return add(1,->field); }\n",
            "expected function call argument after ',', found Arrow at line 2, column 27",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
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
fn rejects_missing_function_parameters_around_commas_with_context() {
    let cases = [
        (
            "int add(, int b) { return b; }\nint main(void) { return add(1); }\n",
            "expected function parameter, found Comma at line 1, column 9",
        ),
        (
            "int add(int a,, int b) { return a + b; }\nint main(void) { return add(1, 2); }\n",
            "expected function parameter after ',', found Comma at line 1, column 15",
        ),
        (
            "int add(] int b) { return b; }\nint main(void) { return add(1); }\n",
            "expected function parameter, found RBracket at line 1, column 9",
        ),
        (
            "int add(int a,] int b) { return a + b; }\nint main(void) { return add(1, 2); }\n",
            "expected function parameter after ',', found RBracket at line 1, column 15",
        ),
        (
            "int add([ int b) { return b; }\nint main(void) { return add(1); }\n",
            "expected function parameter, found LBracket at line 1, column 9",
        ),
        (
            "int add(? int b) { return b; }\nint main(void) { return add(1); }\n",
            "expected function parameter, found Question at line 1, column 9",
        ),
        (
            "int add({ int b) { return b; }\nint main(void) { return add(1); }\n",
            "expected function parameter, found LBrace at line 1, column 9",
        ),
        (
            "int add(; int b) { return b; }\nint main(void) { return add(1); }\n",
            "expected function parameter, found Semi at line 1, column 9",
        ),
        (
            "int add(: int b) { return b; }\nint main(void) { return add(1); }\n",
            "expected function parameter, found Colon at line 1, column 9",
        ),
        (
            "int add(. int b) { return b; }\nint main(void) { return add(1); }\n",
            "expected function parameter, found Dot at line 1, column 9",
        ),
        (
            "int add(-> int b) { return b; }\nint main(void) { return add(1); }\n",
            "expected function parameter, found Arrow at line 1, column 9",
        ),
        (
            "int add(int a,[ int b) { return a + b; }\nint main(void) { return add(1, 2); }\n",
            "expected function parameter after ',', found LBracket at line 1, column 15",
        ),
        (
            "int add(int a,? int b) { return a + b; }\nint main(void) { return add(1, 2); }\n",
            "expected function parameter after ',', found Question at line 1, column 15",
        ),
        (
            "int add(return int b) { return b; }\nint main(void) { return add(1); }\n",
            "expected function parameter before 'return' at line 1, column 9",
        ),
        (
            "int add(int a,return int b) { return a; }\nint main(void) { return add(1, 2); }\n",
            "expected function parameter after ',' before 'return' at line 1, column 15",
        ),
        (
            "int add(int a,: int b) { return a; }\nint main(void) { return add(1, 2); }\n",
            "expected function parameter after ',', found Colon at line 1, column 15",
        ),
        (
            "int add(int a,. int b) { return a; }\nint main(void) { return add(1, 2); }\n",
            "expected function parameter after ',', found Dot at line 1, column 15",
        ),
        (
            "int add(int a,-> int b) { return a; }\nint main(void) { return add(1, 2); }\n",
            "expected function parameter after ',', found Arrow at line 1, column 15",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
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
fn rejects_missing_inferred_array_initializer_expressions_with_context() {
    let cases = [
        (
            "int main() {\nint values[] = ;\nreturn 0;\n}\n",
            "expected initializer expression after '=' in inferred array declaration, found Semi at line 2, column 16",
        ),
        (
            "int main() {\nint first[] = {1}, second[] = ;\nreturn 0;\n}\n",
            "expected initializer expression after '=' in inferred array declaration, found Semi at line 2, column 31",
        ),
        (
            "struct Point { int x; };\nint main() {\nstruct Point points[] = ;\nreturn 0;\n}\n",
            "expected initializer expression after '=' in inferred aggregate array declaration, found Semi at line 3, column 25",
        ),
        (
            "struct Point { int x; };\nint main() {\nstruct Point first[] = {{1}}, second[] = ;\nreturn 0;\n}\n",
            "expected initializer expression after '=' in inferred aggregate array declaration, found Semi at line 3, column 42",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
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
fn rejects_missing_grouped_expression_operands_with_context() {
    let cases = [
        (
            "int main(void) {\n    return ();\n}\n",
            "expected grouped expression, found RParen at line 2, column 13",
        ),
        (
            "int main(void) {\n    return (, 1);\n}\n",
            "expected grouped expression, found Comma at line 2, column 13",
        ),
        (
            "int main(void) {\n    return (;\n}\n",
            "expected grouped expression, found Semi at line 2, column 13",
        ),
        (
            "int main(void) {\n    return (];\n}\n",
            "expected grouped expression, found RBracket at line 2, column 13",
        ),
        (
            "int main(void) {\n    return ([);\n}\n",
            "expected grouped expression, found LBracket at line 2, column 13",
        ),
        (
            "int main(void) {\n    return (?);\n}\n",
            "expected grouped expression, found Question at line 2, column 13",
        ),
        (
            "int main(void) {\n    return (typedef);\n}\n",
            "expected grouped expression before 'typedef' at line 2, column 13",
        ),
        (
            "int main(void) {\n    return (.field);\n}\n",
            "expected grouped expression, found Dot at line 2, column 13",
        ),
        (
            "int main(void) {\n    return (->field);\n}\n",
            "expected grouped expression, found Arrow at line 2, column 13",
        ),
        (
            "int main(void) {\n    return ({1});\n}\n",
            "expected grouped expression, found LBrace at line 2, column 13",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
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
fn rejects_misplaced_function_call_arguments_with_context() {
    let cases = [
        (
            "int take(int value) { return value; }\nint main(void) {\n    return take([);\n}\n",
            "expected function call argument, found LBracket at line 3, column 17",
        ),
        (
            "int take(int value) { return value; }\nint main(void) {\n    return take(?);\n}\n",
            "expected function call argument, found Question at line 3, column 17",
        ),
        (
            "int add(int a, int b) { return a + b; }\nint main(void) {\n    return add(1, [);\n}\n",
            "expected function call argument after ',', found LBracket at line 3, column 19",
        ),
        (
            "int add(int a, int b) { return a + b; }\nint main(void) {\n    return add(1, ?);\n}\n",
            "expected function call argument after ',', found Question at line 3, column 19",
        ),
        (
            "int take(int value) { return value; }\nint main(void) {\n    return take(return);\n}\n",
            "expected function call argument before 'return' at line 3, column 17",
        ),
        (
            "int add(int a, int b) { return a + b; }\nint main(void) {\n    return add(1, int);\n}\n",
            "expected function call argument after ',' before 'int' at line 3, column 19",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
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
fn rejects_local_function_definitions_with_context() {
    let program = include_str!("fixtures/invalid/local_function_definition.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "function definitions are not supported inside blocks at line 2, column 5"
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
fn rejects_parenthesized_pointer_typedef_aliases_with_context() {
    let program = include_str!("fixtures/invalid/parenthesized_pointer_typedef_alias.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "parenthesized pointer typedef aliases are not supported at line 1, column 13"
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
fn rejects_array_return_types_with_context() {
    let program = include_str!("fixtures/invalid/array_return_type.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(
        err.to_string(),
        "array return types are not supported at line 1, column 15"
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
fn supports_model_based_scalar_and_pointer_equality_classification() {
    let program = include_str!("fixtures/compat/valid/equality_classification_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 129);
}

#[test]
fn supports_model_based_return_context_classification() {
    let program =
        include_str!("fixtures/compat/valid/return_context_classification_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 199);
}

#[test]
fn supports_model_based_function_argument_context_classification() {
    let program = include_str!(
        "fixtures/compat/valid/function_argument_context_classification_model_routes.c"
    );

    assert_eq!(interpret(program).unwrap(), 173);
}

#[test]
fn supports_model_based_initializer_and_assignment_context_classification() {
    let program =
        include_str!("fixtures/valid/initializer_assignment_context_classification_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 180);
}

#[test]
fn supports_model_based_initializer_element_context_classification() {
    let program =
        include_str!("fixtures/valid/initializer_element_context_classification_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 255);
}

#[test]
fn supports_model_based_scalar_operand_context_classification() {
    let program =
        include_str!("fixtures/valid/scalar_operand_context_classification_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 97);
}

#[test]
fn rejects_wrapped_nonzero_integer_pointer_equality() {
    let program = include_str!("fixtures/invalid/pointer_wrapped_nonzero_equality.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot compare pointer with nonzero integer"
    );
}

#[test]
fn supports_model_based_scalar_and_pointer_ordering_classification() {
    let program = include_str!("fixtures/compat/valid/ordering_classification_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 130);
}

#[test]
fn rejects_pointer_ordering_against_nested_scalar_field_reads() {
    let program = include_str!("fixtures/invalid/pointer_nested_scalar_ordering.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "pointer ordering comparisons are not supported"
    );
}

#[test]
fn supports_model_based_scalar_and_pointer_truthiness_classification() {
    let program = include_str!("fixtures/compat/valid/truthiness_classification_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 227);
}

#[test]
fn supports_model_based_discard_context_classification() {
    let program = include_str!("fixtures/valid/discard_context_classification_model_routes.c");

    assert_eq!(interpret(program).unwrap(), 240);
}

#[test]
fn supports_array_backed_pointer_arithmetic_and_difference() {
    let program = include_str!("fixtures/valid/pointer_arithmetic.c");

    assert_eq!(interpret(program).unwrap(), 70);
}

#[test]
fn treats_pointer_differences_as_scalars_in_larger_expressions() {
    let program = include_str!("fixtures/valid/pointer_difference_scalar_expressions.c");

    assert_eq!(interpret(program).unwrap(), 50);
}

#[test]
fn preserves_different_array_diagnostics_in_nested_pointer_differences() {
    let program = include_str!("fixtures/invalid/nested_pointer_difference_different_arrays.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot subtract pointers to different arrays"
    );
}

#[test]
fn pointer_difference_const_metadata_follows_pointer_valued_base() {
    let program = include_str!("fixtures/valid/pointer_difference_const_metadata.c");

    assert_eq!(interpret(program).unwrap(), 9);
}

#[test]
fn pointer_difference_const_metadata_rejects_genuinely_const_base() {
    let program = include_str!("fixtures/invalid/pointer_difference_const_metadata_discard.c");

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn supports_pointer_difference_offsets_before_pointer_operands() {
    let program = r#"
int main(void) {
    int values[8] = {11, 12, 13, 14, 15, 16, 17, 18};
    int *result = ((values + 5) - (values + 2)) + (values + 1);
    return *result;
}
"#;

    assert_eq!(interpret(program).unwrap(), 15);
}

#[test]
fn preserves_cross_array_errors_in_discarded_pointer_comma_operands() {
    let program = r#"
int main(void) {
    int left[4];
    int right[4];
    const int *result = (((left + 1) - (right + 1)), left + 1)
        + ((left + 2) - (left + 1));
    return *result;
}
"#;

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot subtract pointers to different arrays"
    );
}

#[test]
fn discarded_pointer_comma_expressions_evaluate_left_operand_once() {
    let program = r#"
int main(void) {
    int values[2] = {3, 4};
    int marker = 0;
    (marker++, values + 1);
    return marker;
}
"#;

    assert_eq!(interpret(program).unwrap(), 1);
}

#[test]
fn discarded_aggregate_expressions_do_not_require_scalar_conversion() {
    let program = r#"
struct Point {
    int x;
};

int main(void) {
    struct Point point = {7};
    point;
    (void)point;
    return point.x;
}
"#;

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn discarded_void_comma_expressions_evaluate_each_operand_once() {
    let program = r#"
int marker = 0;

void touch(void) {
    marker++;
}

int main(void) {
    (marker++, touch());
    return marker;
}
"#;

    assert_eq!(interpret(program).unwrap(), 2);
}

#[test]
fn discarded_aggregate_dereference_assignment_evaluates_pointer_once() {
    let program = r#"
struct Point {
    int x;
};

int main(void) {
    struct Point target = {3};
    struct Point replacement = {7};
    struct Point *slot = &target;
    int marker = 0;
    (*(marker++, slot) = replacement);
    return marker * 10 + target.x;
}
"#;

    assert_eq!(interpret(program).unwrap(), 17);
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
fn supports_prefix_increment_for_embedded_aggregate_array_element_fields() {
    let program = r#"
        struct Point { int value; };
        struct Box { struct Point points[2]; };

        int main(void) {
            struct Box box = {{{3}, {5}}};
            return ++box.points[1].value;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 6);
}

#[test]
fn supports_prefix_increment_for_reverse_aggregate_subscript_fields() {
    let program = r#"
        struct Point { int value; };
        struct Index { int value; };

        int main(void) {
            struct Point points[2] = {{3}, {5}};
            struct Index index = {1};
            return ++index.value[points].value;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 6);
}

#[test]
fn supports_embedded_aggregate_array_element_field_increment_decrement() {
    let program = include_str!("fixtures/valid/embedded_aggregate_array_element_field_increment.c");

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_pointer_field_replacement_through_embedded_aggregate_array_elements() {
    let program = r#"
        struct Node { int *cursor; };
        struct Box { struct Node nodes[2]; };

        int main(void) {
            int values[3] = {3, 5, 7};
            struct Box box = {{{values}, {values + 2}}};
            int *result = (box.nodes[0].cursor = values + 1);
            return *result + *box.nodes[0].cursor;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 10);
}

#[test]
fn pointer_field_replacement_through_embedded_aggregate_array_element_evaluates_once() {
    let program = r#"
        struct Node { int *cursor; };
        struct Box { struct Node nodes[1]; };

        int main(void) {
            int values[2] = {3, 5};
            struct Box box = {{{values}}};
            int index_marker = 0;
            int rhs_marker = 0;
            int *result = (
                box.nodes[index_marker++].cursor = (rhs_marker++, values + 1)
            );
            return *result + index_marker + rhs_marker + *box.nodes[0].cursor;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 12);
}

#[test]
fn supports_pointer_field_compound_assignment_through_embedded_aggregate_array_elements() {
    let program = r#"
        struct Node { int *cursor; };
        struct Box { struct Node nodes[1]; };

        int main(void) {
            int values[3] = {3, 5, 7};
            struct Box box = {{{values}}};
            int *result = (box.nodes[0].cursor += 2);
            return *result + *box.nodes[0].cursor;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn supports_pointer_field_increment_through_embedded_aggregate_array_elements() {
    let program = r#"
        struct Node { int *cursor; };
        struct Box { struct Node nodes[1]; };

        int main(void) {
            int values[4] = {2, 4, 6, 8};
            struct Box box = {{{values + 1}}};
            int *post = box.nodes[0].cursor++;
            int *pre = ++box.nodes[0].cursor;
            return *post + *pre + *box.nodes[0].cursor;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 20);
}

#[test]
fn reads_pointer_fields_from_named_aggregate_array_elements() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[1] = {7};
            struct Node nodes[1] = {{values}};
            return *nodes[0].cursor;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn replaces_pointer_fields_in_named_aggregate_array_elements_once() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[3] = {3, 5, 7};
            struct Node nodes[1] = {{values}};
            int index_marker = 0;
            int rhs_marker = 0;
            int *result = (nodes[index_marker++].cursor =
                           (rhs_marker += 1, values + 1));
            return *result + *nodes[0].cursor + index_marker + rhs_marker;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 12);
}

#[test]
fn updates_pointer_fields_in_named_aggregate_array_elements_once() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[4] = {2, 4, 6, 8};
            struct Node nodes[1] = {{values}};
            int index_marker = 0;
            int rhs_marker = 0;
            int *result = (nodes[index_marker++].cursor +=
                           (rhs_marker += 1, 2));
            return *result + *nodes[0].cursor + index_marker + rhs_marker;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 14);
}

#[test]
fn increments_pointer_fields_in_named_aggregate_array_elements_once() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[4] = {2, 4, 6, 8};
            struct Node nodes[1] = {{values}};
            int post_index = 0;
            int pre_index = 0;
            int *post = nodes[post_index++].cursor++;
            int *pre = ++nodes[pre_index++].cursor;
            return *post + *pre + *nodes[0].cursor + post_index + pre_index;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 16);
}

#[test]
fn named_aggregate_array_element_pointer_field_updates_match_fixture() {
    let program =
        include_str!("fixtures/valid/named_aggregate_array_element_pointer_field_updates.c",);

    assert_eq!(interpret(program).unwrap(), 17);
}

#[test]
fn named_aggregate_array_pointer_field_subscript_reads() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[3] = {3, 5, 7};
            struct Node nodes[1] = {{values}};
            return nodes[0].cursor[1] + 1[nodes[0].cursor];
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 10);
}

#[test]
fn named_aggregate_array_pointer_field_subscripts_match_fixture() {
    let program = include_str!("fixtures/valid/named_aggregate_array_pointer_field_subscripts.c");

    assert_eq!(interpret(program).unwrap(), 80);
}

#[test]
fn named_aggregate_array_pointer_field_aggregate_consumers_match_fixture() {
    let program =
        include_str!("fixtures/valid/named_aggregate_array_pointer_field_aggregate_consumers.c",);

    assert_eq!(interpret(program).unwrap(), 201);
}

#[test]
fn nested_holder_array_pointer_field_subscripts_match_fixture() {
    let program = include_str!("fixtures/valid/nested_holder_array_pointer_field_subscripts.c");

    assert_eq!(interpret(program).unwrap(), 32);
}

#[test]
fn nested_named_holder_array_pointer_field_subscript_reads() {
    let program = r#"
        struct Inner { int *cursor; };
        struct Holder { struct Inner nested; };

        int main(void) {
            int values[2] = {3, 4};
            struct Holder holders[1] = {{{values}}};
            return holders[0].nested.cursor[1];
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 4);
}

#[test]
fn aggregate_array_parameter_nested_pointer_field_subscript_reads() {
    let program = r#"
        struct Point { int value; };
        struct Inner { struct Point *points; };
        union Choice { struct Inner nested; int marker; };

        struct Point read(union Choice choices[]) {
            return choices[0].nested.points[0];
        }

        int main(void) {
            struct Point points[1] = {{6}};
            union Choice choices[1] = {{.nested = {points}}};
            struct Point result = read(choices);
            return result.value;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 6);
}

#[test]
fn aggregate_array_parameter_embedded_scalar_array_field_subscript_reads() {
    let program = r#"
        struct Inner { int values[2]; };
        union Choice { struct Inner nested; int marker; };

        int read(union Choice choices[]) {
            return choices[0].nested.values[1];
        }

        int main(void) {
            union Choice choices[1] = {{.nested = {{3, 5}}}};
            return read(choices);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 5);
}

#[test]
fn aggregate_array_parameter_embedded_scalar_array_field_operations_evaluate_once() {
    let program = r#"
        struct Inner { int values[3]; };
        struct Holder { struct Inner nested; };

        int update(struct Holder items[]) {
            int outer = 0;
            int inner = 1;
            int rhs = 0;
            int *slot = &items[outer++].nested.values[inner++];
            *slot += 4;
            int direct = items[0].nested.values[0] = (rhs++, 7);
            int compound = 1[items[0].nested.values] += 3;
            int old = items[0].nested.values[2]++;
            return direct + compound + old + *slot + outer + inner + rhs;
        }

        int main(void) {
            struct Holder items[1] = {{.nested = {{3, 5, 7}}}};
            return update(items);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 42);
}

#[test]
fn aggregate_array_parameter_embedded_aggregate_array_field_subscript_reads() {
    let program = r#"
        struct Point { int value; };
        struct Inner { struct Point points[2]; };
        union Choice { struct Inner nested; int marker; };

        struct Point read(union Choice choices[]) {
            return choices[0].nested.points[1];
        }

        int main(void) {
            union Choice choices[1] = {{.nested = {{{3}, {7}}}}};
            struct Point result = read(choices);
            return result.value;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn aggregate_array_parameter_embedded_aggregate_array_field_copy_assignment_isolated() {
    let program = r#"
        struct Point { int value; };
        struct Inner { struct Point points[2]; };
        union Choice { struct Inner nested; int marker; };

        int replace(union Choice choices[], struct Point replacement) {
            int outer = 0;
            int inner = 1;
            int rhs = 0;
            struct Point result = choices[outer++].nested.points[inner++] =
                (rhs++, replacement);
            result.value = 11;
            return choices[0].nested.points[1].value + result.value
                + outer + inner + rhs;
        }

        int main(void) {
            union Choice choices[1] = {{.nested = {{{3}, {7}}}}};
            struct Point replacement = {9};
            return replace(choices, replacement);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 24);
}

#[test]
fn aggregate_array_parameter_embedded_aggregate_array_field_reverse_operations() {
    let program = r#"
        struct Point { int value; };
        struct Inner { struct Point points[2]; };
        union Choice { struct Inner nested; int marker; };

        int replace(union Choice choices[], struct Point replacement) {
            struct Point result = 1[choices[0].nested.points] = replacement;
            struct Point *slot = &0[choices[0].nested.points];
            slot->value += 2;
            return result.value + choices[0].nested.points[1].value + slot->value;
        }

        int main(void) {
            union Choice choices[1] = {{.nested = {{{3}, {7}}}}};
            struct Point replacement = {9};
            return replace(choices, replacement);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 23);
}

#[test]
fn adjusted_aggregate_parameter_embedded_array_fields_match_fixture() {
    let program =
        include_str!("fixtures/valid/adjusted_aggregate_parameter_embedded_array_fields.c",);

    assert_eq!(interpret(program).unwrap(), 82);
}

#[test]
fn adjusted_aggregate_parameter_recursive_identity_model_routes_match_fixture() {
    let program = include_str!(
        "fixtures/valid/adjusted_aggregate_parameter_recursive_identity_model_routes.c",
    );

    assert_eq!(interpret(program).unwrap(), 35);
}

#[test]
fn adjusted_aggregate_parameter_alias_mutation_model_routes_match_fixture() {
    let program =
        include_str!("fixtures/valid/adjusted_aggregate_parameter_alias_mutation_model_routes.c",);

    assert_eq!(interpret(program).unwrap(), 40);
}

#[test]
fn adjusted_aggregate_parameter_nested_path_alias_mutation_routes_match_fixture() {
    let program = include_str!(
        "fixtures/valid/adjusted_aggregate_parameter_nested_path_alias_mutation_routes.c",
    );

    assert_eq!(interpret(program).unwrap(), 32);
}

#[test]
fn adjusted_aggregate_parameter_compound_literal_outer_alias_routes_match_fixture() {
    let program = include_str!(
        "fixtures/valid/adjusted_aggregate_parameter_compound_literal_outer_alias_routes.c",
    );

    assert_eq!(interpret(program).unwrap(), 67);
}

#[test]
fn adjusted_aggregate_parameter_direct_compound_literal_alias_routes_match_fixture() {
    let program = include_str!(
        "fixtures/valid/adjusted_aggregate_parameter_direct_compound_literal_alias_routes.c",
    );

    assert_eq!(interpret(program).unwrap(), 72);
}

#[test]
fn aggregate_array_field_elements_decay_embedded_arrays_for_direct_access() {
    let program = r#"
        struct Point { int value; };
        struct Inner { int values[3]; struct Point points[2]; };
        struct Item { struct Inner nested; };
        struct Wrapper { struct Item items[2]; };

        int main(void) {
            struct Wrapper wrapper;
            wrapper.items[0].nested.values[1] = 7;
            2[wrapper.items[0].nested.values] = 9;
            wrapper.items[0].nested.points[1].value = 11;
            int *scalar = &wrapper.items[0].nested.values[1];
            struct Point *point = &wrapper.items[0].nested.points[1];
            return *scalar + wrapper.items[0].nested.values[2] + point->value;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 27);
}

#[test]
fn adjusted_aggregate_parameter_alias_helpers_preserve_identity_and_write_order() {
    let program = r#"
        struct Point { int value; };
        struct Inner { int values[3]; struct Point points[3]; };
        struct Item { struct Inner nested; };
        struct Wrapper { struct Item items[2]; };

        int *forward_int(int *slot) { return slot; }
        int *forward_int_twice(int *slot) { return forward_int(slot); }
        const int *forward_const_int(const int *slot) { return slot; }

        int mutate(int *first, int *second, const int *reader) {
            *first = 20;
            int before = *reader;
            *second += 3;
            int after = *reader;
            first = 0;
            second = 0;
            reader = 0;
            return before + after;
        }

        int probe(struct Item items[]) {
            int *first = forward_int(&items[0].nested.values[1]);
            int *second = forward_int_twice(&items[0].nested.values[1]);
            const int *reader = forward_const_int(&items[0].nested.values[1]);
            int observed = mutate(first, second, reader);
            return observed + *first + *second + *reader;
        }

        int main(void) {
            struct Item root[2];
            struct Wrapper wrapper;
            root[0].nested.values[1] = 4;
            wrapper.items[0].nested.values[1] = 5;
            return probe(root) + probe(wrapper.items)
                + root[0].nested.values[1]
                + wrapper.items[0].nested.values[1];
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 270);
}

#[test]
fn sizeof_aggregate_array_parameter_embedded_array_field_operations_is_non_evaluating() {
    let program = r#"
        struct Point { int value; };
        struct Inner { char bytes[2]; struct Point points[2]; };
        union Choice { struct Inner nested; int marker; };

        int inspect(union Choice choices[]) {
            int outer = 0;
            int inner = 0;
            int rhs = 0;
            int ok = 0;
            struct Point replacement = {9};
            ok += sizeof(choices[outer++].nested.bytes[inner++]) == sizeof(char);
            ok += sizeof(choices[outer++].nested.bytes[inner++] = (rhs++, 'x'))
                == sizeof(char);
            ok += sizeof(choices[outer++].nested.bytes[inner++] += (rhs++, 1))
                == sizeof(char);
            ok += sizeof(choices[outer++].nested.bytes[inner++]++) == sizeof(char);
            ok += sizeof(choices[outer++].nested.points[inner++])
                == sizeof(struct Point);
            ok += sizeof(choices[outer++].nested.points[inner++] =
                         (rhs++, replacement)) == sizeof(struct Point);
            return ok + (outer == 0) + (inner == 0) + (rhs == 0)
                + (choices[0].nested.bytes[0] == 'a')
                + (choices[0].nested.points[0].value == 3);
        }

        int main(void) {
            union Choice choices[1] = {
                {.nested = {{'a', 'b'}, {{3}, {7}}}}
            };
            return inspect(choices);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 11);
}

#[test]
fn aggregate_array_parameter_embedded_aggregate_array_fields_support_field_backed_arguments() {
    let program = r#"
        struct Point { int value; };
        struct Item { struct Point points[2]; };
        struct Wrapper { struct Item items[1]; };

        int read(struct Item items[]) {
            return items[0].points[1].value;
        }

        int main(void) {
            struct Wrapper wrapper = {{{{{3}, {7}}}}};
            return read(wrapper.items);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn aggregate_array_parameter_embedded_aggregate_array_addresses_preserve_identity() {
    let program = r#"
        struct Point { int value; };
        struct Item { struct Point points[2]; };
        struct Wrapper { struct Item items[1]; };

        int same_root(struct Item items[]) {
            struct Point *first = &items[0].points[0];
            struct Point *again = &items[0].points[0];
            struct Point *second = &items[0].points[1];
            return (first == again) + (second - first) + (first < second);
        }

        int main(void) {
            struct Item items[1] = {{.points = {{3}, {7}}}};
            struct Wrapper wrapper = {
                .items = {{.points = {{5}, {9}}}}
            };
            return same_root(items) + same_root(wrapper.items);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 6);
}

#[test]
fn aggregate_array_parameter_const_embedded_scalar_array_parent_rejects_writes() {
    let program = r#"
        struct Inner { int values[1]; };
        struct Holder { const struct Inner nested; };

        int update(struct Holder items[]) {
            return items[0].nested.values[0] = 9;
        }

        int main(void) {
            struct Holder items[1] = {{.nested = {{3}}}};
            return update(items);
        }
    "#;

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "cannot assign through pointer to const"
    );
}

#[test]
fn aggregate_array_parameter_const_embedded_aggregate_array_parent_rejects_writes() {
    let program = r#"
        struct Point { int value; };
        struct Inner { struct Point points[1]; };
        struct Holder { const struct Inner nested; };

        int update(struct Holder items[], struct Point replacement) {
            items[0].nested.points[0] = replacement;
            return items[0].nested.points[0].value;
        }

        int main(void) {
            struct Holder items[1] = {{.nested = {{{3}}}}};
            struct Point replacement = {9};
            return update(items, replacement);
        }
    "#;

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "cannot assign through pointer to const"
    );
}

#[test]
fn aggregate_array_parameter_const_embedded_aggregate_array_field_preserves_diagnostic() {
    let program = r#"
        struct Point { int value; };
        struct Item { const struct Point points[1]; };

        int update(struct Item items[], struct Point replacement) {
            items[0].points[0] = replacement;
            return items[0].points[0].value;
        }

        int main(void) {
            struct Item items[1] = {{.points = {{3}}}};
            struct Point replacement = {9};
            return update(items, replacement);
        }
    "#;

    assert_eq!(
        interpret(program).unwrap_err().to_string(),
        "cannot assign to const struct field 'points'"
    );
}

#[test]
fn aggregate_array_parameter_embedded_array_fields_preserve_diagnostics() {
    let cases = [
        (
            r#"
                struct Inner { int values[1]; };
                union Choice { struct Inner nested; int marker; };
                int update(const union Choice choices[]) {
                    return choices[0].nested.values[0] = 9;
                }
                int main(void) {
                    union Choice choices[1] = {{.nested = {{3}}}};
                    return update(choices);
                }
            "#,
            "cannot assign through pointer to const",
        ),
        (
            r#"
                struct Inner { int values[1]; };
                union Choice { struct Inner nested; int marker; };
                int read(union Choice choices[]) {
                    return choices[1].nested.values[0];
                }
                int main(void) {
                    union Choice choices[1] = {{.nested = {{3}}}};
                    return read(choices);
                }
            "#,
            "struct array pointer index 1 out of bounds for length 1",
        ),
        (
            r#"
                struct Inner { int values[1]; };
                union Choice { struct Inner nested; int marker; };
                int read(union Choice choices[]) {
                    return choices[0].nested.values[1];
                }
                int main(void) {
                    union Choice choices[1] = {{.nested = {{3}}}};
                    return read(choices);
                }
            "#,
            "array 'values' index 1 out of bounds for length 1",
        ),
        (
            r#"
                struct Point { int value; };
                struct Inner { struct Point points[1]; };
                union Choice { struct Inner nested; int marker; };
                int read(union Choice choices[]) {
                    return choices[0].nested.points[1].value;
                }
                int main(void) {
                    union Choice choices[1] = {{.nested = {{{3}}}}};
                    return read(choices);
                }
            "#,
            "struct array field 'points' index 1 out of bounds for length 1",
        ),
        (
            r#"
                struct Point { int value; };
                union Number { int value; };
                struct Inner { struct Point points[1]; };
                union Choice { struct Inner nested; int marker; };
                int replace(union Choice choices[], union Number replacement) {
                    choices[0].nested.points[0] = replacement;
                    return 0;
                }
                int main(void) {
                    union Choice choices[1] = {{.nested = {{{3}}}}};
                    union Number replacement = {7};
                    return replace(choices, replacement);
                }
            "#,
            "cannot assign struct 'Number' to struct 'Point'",
        ),
    ];

    for (program, expected) in cases {
        match interpret(program) {
            Ok(value) => panic!("expected '{expected}', got {value}; program: {program}"),
            Err(err) => assert_eq!(err.to_string(), expected, "program: {program}"),
        }
    }
}

#[test]
fn aggregate_array_parameter_nested_pointer_field_subscript_updates() {
    let program = r#"
        struct Inner { int *values; };
        union Choice { struct Inner nested; int marker; };

        int update(union Choice choices[]) {
            choices[0].nested.values[0] = 7;
            choices[0].nested.values[1] += 3;
            return ++choices[0].nested.values[0]
                + choices[0].nested.values[1];
        }

        int main(void) {
            int values[2] = {2, 4};
            union Choice choices[1] = {{.nested = {values}}};
            return update(choices) + values[0] + values[1];
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 30);
}

#[test]
fn aggregate_array_parameter_nested_pointer_field_replacement_updates_slot_once() {
    let program = r#"
        struct Inner { int *cursor; };
        union Choice { struct Inner nested; int marker; };

        int replace(union Choice choices[], int *replacement) {
            int outer = 0;
            int rhs = 0;
            int *result =
                choices[outer++].nested.cursor = replacement + (rhs++, 1);
            return result[1] + choices[0].nested.cursor[0] + outer + rhs;
        }

        int main(void) {
            int original[2] = {2, 3};
            int replacement[3] = {5, 7, 11};
            union Choice choices[1] = {{.nested = {original}}};
            return replace(choices, replacement);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 20);
}

#[test]
fn aggregate_array_parameter_nested_pointer_field_compound_and_increment_updates() {
    let program = r#"
        struct Inner { int *cursor; };
        union Choice { struct Inner nested; int marker; };

        int update(union Choice choices[]) {
            int outer = 0;
            int rhs = 0;
            int *after_compound =
                (choices[outer++].nested.cursor += (rhs++, 1));
            int *old = choices[0].nested.cursor++;
            int *updated = --choices[0].nested.cursor;
            return after_compound[0] + old[0] + updated[1] + outer + rhs;
        }

        int main(void) {
            int values[3] = {2, 3, 5};
            union Choice choices[1] = {{.nested = {values}}};
            return update(choices);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 13);
}

#[test]
fn adjusted_aggregate_parameter_pointer_field_updates_match_fixture() {
    let program =
        include_str!("fixtures/valid/adjusted_aggregate_parameter_pointer_field_updates.c",);

    assert_eq!(interpret(program).unwrap(), 63);
}

#[test]
fn aggregate_array_parameter_pointer_field_updates_distinguish_qualifiers() {
    let program = r#"
        struct MutableInner { int *cursor; };
        union MutableChoice { struct MutableInner nested; int marker; };
        struct ConstViewInner { const int *cursor; };
        union ConstViewChoice { struct ConstViewInner nested; int marker; };

        int replace_through_const_parameter_slot(
            union MutableChoice choices[const 1],
            int *replacement
        ) {
            choices[0].nested.cursor = replacement;
            return choices[0].nested.cursor[1];
        }

        int replace_const_view(
            union ConstViewChoice choices[],
            const int *replacement
        ) {
            const int *result = choices[0].nested.cursor = replacement;
            return result[0] + choices[0].nested.cursor[1];
        }

        int main(void) {
            int original[2] = {2, 3};
            int replacement[2] = {5, 7};
            const int fixed[2] = {11, 13};
            union MutableChoice mutable_choices[1] = {{.nested = {original}}};
            union ConstViewChoice const_view_choices[1] = {{.nested = {original}}};
            return replace_through_const_parameter_slot(mutable_choices, replacement)
                + replace_const_view(const_view_choices, fixed);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 31);
}

#[test]
fn aggregate_array_parameter_pointer_field_updates_preserve_diagnostics() {
    let cases = [
        (
            r#"
                struct Inner { int *cursor; };
                union Choice { struct Inner nested; int marker; };
                int replace(const union Choice choices[], int *replacement) {
                    choices[0].nested.cursor = replacement;
                    return 0;
                }
                int main(void) {
                    int values[1] = {2};
                    int replacement[1] = {3};
                    union Choice choices[1] = {{.nested = {values}}};
                    return replace(choices, replacement);
                }
            "#,
            "cannot assign through pointer to const",
        ),
        (
            r#"
                struct Inner { int * const cursor; };
                union Choice { struct Inner nested; int marker; };
                int replace(union Choice choices[], int *replacement) {
                    choices[0].nested.cursor = replacement;
                    return 0;
                }
                int main(void) {
                    int values[1] = {2};
                    int replacement[1] = {3};
                    union Choice choices[1] = {{.nested = {values}}};
                    return replace(choices, replacement);
                }
            "#,
            "cannot assign to const struct field 'cursor'",
        ),
        (
            r#"
                struct Inner { const int *cursor; };
                union Choice { struct Inner nested; int marker; };
                int *replace(union Choice choices[], const int *replacement) {
                    return choices[0].nested.cursor = replacement;
                }
                int main(void) {
                    int values[1] = {2};
                    const int replacement[1] = {3};
                    union Choice choices[1] = {{.nested = {values}}};
                    return replace(choices, replacement)[0];
                }
            "#,
            "cannot discard const qualifier from pointer target",
        ),
        (
            r#"
                struct Inner { int *cursor; };
                union Choice { struct Inner nested; int marker; };
                int shift(union Choice choices[]) {
                    choices[0].nested.cursor += 2;
                    return 0;
                }
                int main(void) {
                    int values[2] = {2, 3};
                    union Choice choices[1] = {{.nested = {values}}};
                    return shift(choices);
                }
            "#,
            "array pointer index 2 out of bounds for length 2",
        ),
        (
            r#"
                struct Point { int value; };
                union Number { int value; };
                struct Inner { struct Point *cursor; };
                union Choice { struct Inner nested; int marker; };
                int replace(union Choice choices[], union Number *replacement) {
                    choices[0].nested.cursor = replacement;
                    return 0;
                }
                int main(void) {
                    struct Point points[1] = {{2}};
                    union Number replacements[1] = {{3}};
                    union Choice choices[1] = {{.nested = {points}}};
                    return replace(choices, replacements);
                }
            "#,
            "cannot convert pointer to union 'Number' to pointer to struct 'Point'",
        ),
    ];

    for (program, expected) in cases {
        match interpret(program) {
            Ok(value) => panic!("expected '{expected}', got {value}; program: {program}"),
            Err(err) => assert_eq!(err.to_string(), expected, "program: {program}"),
        }
    }
}

#[test]
fn sizeof_aggregate_array_parameter_pointer_field_updates_is_non_evaluating() {
    let program = r#"
        struct Inner { int *cursor; };
        union Choice { struct Inner nested; int marker; };

        int inspect(union Choice choices[], int *replacement) {
            int outer = 0;
            int rhs = 0;
            int ok = 0;
            ok += sizeof(choices[outer++].nested.cursor = (rhs++, replacement))
                == sizeof(int *);
            ok += sizeof(choices[outer++].nested.cursor += (rhs++, 1))
                == sizeof(int *);
            ok += sizeof(choices[outer++].nested.cursor++) == sizeof(int *);
            return ok + (outer == 0) + (rhs == 0);
        }

        int main(void) {
            int values[2] = {2, 3};
            int replacement[2] = {5, 7};
            union Choice choices[1] = {{.nested = {values}}};
            return inspect(choices, replacement)
                + (choices[0].nested.cursor == values);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 6);
}

#[test]
fn aggregate_array_parameter_nested_pointer_field_addresses_and_sizeof_evaluate_once() {
    let program = r#"
        struct Inner { int *values; };
        union Choice { struct Inner nested; int marker; };

        int inspect(union Choice choices[]) {
            int outer = 0;
            int inner = 1;
            int *slot = &choices[outer++].nested.values[inner++];
            *slot += 4;
            int measured = sizeof(choices[outer++].nested.values[inner++]);
            return *slot + outer + inner + (measured == sizeof(int));
        }

        int main(void) {
            int values[3] = {2, 4, 6};
            union Choice choices[1] = {{.nested = {values}}};
            return inspect(choices) + values[1];
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 20);
}

#[test]
fn sizeof_aggregate_array_parameter_nested_pointer_field_updates_is_non_evaluating() {
    let program = r#"
        struct Inner { char *bytes; };
        union Choice { struct Inner nested; int marker; };

        int inspect(union Choice choices[]) {
            int outer = 0;
            int inner = 0;
            int rhs = 0;
            int ok = 0;
            ok += sizeof(choices[outer++].nested.bytes[inner++]) == sizeof(char);
            ok += sizeof(choices[outer++].nested.bytes[inner++] = (rhs++, 'x'))
                == sizeof(char);
            ok += sizeof(choices[outer++].nested.bytes[inner++] += (rhs++, 1))
                == sizeof(char);
            ok += sizeof(choices[outer++].nested.bytes[inner++]++) == sizeof(char);
            return ok + (outer == 0) + (inner == 0) + (rhs == 0);
        }

        int main(void) {
            char bytes[1] = {'a'};
            union Choice choices[1] = {{.nested = {bytes}}};
            return inspect(choices) + (bytes[0] == 'a');
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 8);
}

#[test]
fn aggregate_array_parameter_nested_pointer_field_subscripts_preserve_diagnostics() {
    let cases = [
        (
            r#"
                struct Inner { const int *values; };
                union Choice { struct Inner nested; int marker; };
                int update(union Choice choices[]) {
                    return choices[0].nested.values[0] = 9;
                }
                int main(void) {
                    int values[1] = {3};
                    union Choice choices[1] = {{.nested = {values}}};
                    return update(choices);
                }
            "#,
            "cannot assign through pointer to const",
        ),
        (
            r#"
                struct Inner { int *values; };
                union Choice { struct Inner nested; int marker; };
                int read(union Choice choices[]) {
                    return choices[0].nested.values[2];
                }
                int main(void) {
                    int values[2] = {3, 5};
                    union Choice choices[1] = {{.nested = {values}}};
                    return read(choices);
                }
            "#,
            "array pointer index 2 out of bounds for length 2",
        ),
        (
            r#"
                struct Point { int value; };
                union Number { int value; };
                struct Inner { struct Point *points; };
                union Choice { struct Inner nested; int marker; };
                int convert(union Choice choices[]) {
                    union Number *bad = &choices[0].nested.points[0];
                    return bad->value;
                }
                int main(void) {
                    struct Point points[1] = {{3}};
                    union Choice choices[1] = {{.nested = {points}}};
                    return convert(choices);
                }
            "#,
            "cannot convert pointer to struct 'Point' to pointer to union 'Number'",
        ),
    ];

    for (program, expected) in cases {
        match interpret(program) {
            Ok(value) => panic!("expected '{expected}', got {value}; program: {program}"),
            Err(err) => assert_eq!(err.to_string(), expected, "program: {program}"),
        }
    }
}

#[test]
fn named_aggregate_array_pointer_field_subscript_writes_evaluate_once() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[3] = {3, 5, 7};
            struct Node nodes[1] = {{values}};
            int outer = 0;
            int inner = 0;
            int rhs = 0;
            int result = nodes[outer++].cursor[inner++] = (rhs += 1, 9);
            return result + values[0] + outer + inner + rhs;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 21);
}

#[test]
fn named_aggregate_array_pointer_field_subscript_compound_writes_evaluate_once() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[3] = {3, 5, 7};
            struct Node nodes[1] = {{values}};
            int outer = 0;
            int inner = 0;
            int rhs = 0;
            int result = nodes[outer++].cursor[inner++] += (rhs += 1, 4);
            return result + values[0] + outer + inner + rhs;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 17);
}

#[test]
fn named_aggregate_array_pointer_field_subscript_increments_evaluate_once() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[3] = {3, 5, 7};
            struct Node nodes[1] = {{values}};
            int post_outer = 0;
            int post_inner = 0;
            int pre_outer = 0;
            int pre_inner = 1;
            int post = nodes[post_outer++].cursor[post_inner++]++;
            int pre = ++nodes[pre_outer++].cursor[pre_inner++];
            return post + pre + values[0] + values[1]
                + post_outer + post_inner + pre_outer + pre_inner;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 24);
}

#[test]
fn named_aggregate_array_pointer_field_subscript_addresses_alias_storage_once() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[2] = {3, 5};
            struct Node nodes[1] = {{values}};
            int outer = 0;
            int inner = 1;
            int *slot = &nodes[outer++].cursor[inner++];
            *slot = 9;
            return *slot + values[1] + outer + inner;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 21);
}

#[test]
fn named_aggregate_array_aggregate_pointer_field_subscript_reads() {
    let program = r#"
        struct Point { int value; };
        struct Node { struct Point *cursor; };

        int main(void) {
            struct Point points[2] = {{3}, {5}};
            struct Node nodes[1] = {{points}};
            return nodes[0].cursor[1].value;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 5);
}

#[test]
fn named_aggregate_array_aggregate_pointer_field_subscript_copy_reads() {
    let program = r#"
        struct Point { int value; };
        struct Node { struct Point *cursor; };

        int main(void) {
            struct Point points[2] = {{3}, {5}};
            struct Node nodes[1] = {{points}};
            int outer = 0;
            int inner = 1;
            struct Point copy = nodes[outer++].cursor[inner++];
            copy.value = 9;
            return copy.value + points[1].value + outer + inner;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 17);
}

#[test]
fn named_aggregate_array_aggregate_pointer_field_subscript_reads_bind_as_arguments() {
    let program = r#"
        struct Point { int value; };
        struct Node { struct Point *cursor; };

        int read(struct Point point) {
            point.value = 9;
            return point.value;
        }

        int main(void) {
            struct Point points[1] = {{3}};
            struct Node nodes[1] = {{points}};
            return read(nodes[0].cursor[0]) + points[0].value;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 12);
}

#[test]
fn named_aggregate_array_aggregate_pointer_field_subscript_assignment_results_bind_as_arguments() {
    let program = r#"
        struct Point { int value; };
        struct Node { struct Point *cursor; };

        int read(struct Point point) {
            point.value = 11;
            return point.value;
        }

        int main(void) {
            struct Point points[1] = {{3}};
            struct Point replacement = {7};
            struct Node nodes[1] = {{points}};
            return read(nodes[0].cursor[0] = replacement) + points[0].value;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 18);
}

#[test]
fn named_aggregate_array_aggregate_pointer_field_subscripts_return_copies() {
    let program = r#"
        struct Point { int value; };
        struct Node { struct Point *cursor; };

        struct Point read(void) {
            struct Point points[1] = {{3}};
            struct Node nodes[1] = {{points}};
            return nodes[0].cursor[0];
        }

        struct Point replace(void) {
            struct Point points[1] = {{3}};
            struct Point replacement = {7};
            struct Node nodes[1] = {{points}};
            return nodes[0].cursor[0] = replacement;
        }

        int main(void) {
            struct Point first = read();
            struct Point second = replace();
            first.value = 11;
            second.value = 13;
            return first.value + second.value;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 24);
}

#[test]
fn named_aggregate_array_aggregate_pointer_field_subscript_arguments_preserve_type_diagnostics() {
    let program = r#"
        struct Point { int value; };
        struct Pair { int value; };
        struct Node { struct Point *cursor; };

        int read(struct Pair pair) {
            return pair.value;
        }

        int main(void) {
            struct Point points[1] = {{3}};
            struct Node nodes[1] = {{points}};
            return read(nodes[0].cursor[0]);
        }
    "#;

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "function 'read' struct parameter 'pair' expected struct 'Pair', got struct 'Point'"
    );
}

#[test]
fn named_aggregate_array_aggregate_pointer_field_subscript_lvalues_evaluate_once() {
    let program = r#"
        struct Point { int value; };
        struct Node { struct Point *cursor; };

        int main(void) {
            struct Point points[2] = {{3}, {5}};
            struct Node nodes[1] = {{points}};
            int outer = 0;
            int inner = 0;
            int assigned = (nodes[outer++].cursor[inner++].value = 7);
            int compound = (nodes[outer++ - 1].cursor[inner++].value += 2);
            int post = nodes[outer++ - 2].cursor[inner++ - 2].value++;
            int pre = ++nodes[outer++ - 3].cursor[inner++ - 2].value;
            struct Point *slot = &nodes[outer++ - 4].cursor[inner++ - 4];
            slot->value += 1;
            1[nodes[0].cursor].value += 1;
            return assigned + compound + post + pre + slot->value
                + points[0].value + points[1].value + outer + inner;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 66);
}

#[test]
fn named_aggregate_array_aggregate_pointer_field_subscript_copy_assignment_evaluates_once() {
    let program = r#"
        struct Point { int value; };
        struct Node { struct Point *cursor; };

        int main(void) {
            struct Point points[2] = {{3}, {5}};
            struct Point replacement = {9};
            struct Node nodes[1] = {{points}};
            int outer = 0;
            int inner = 1;
            int rhs = 0;
            struct Point result = (nodes[outer++].cursor[inner++] =
                                   (rhs++, replacement));
            replacement.value = 4;
            return result.value + points[1].value + replacement.value
                + outer + inner + rhs;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 26);
}

#[test]
fn named_aggregate_array_aggregate_pointer_field_subscripts_preserve_const() {
    let program = r#"
        struct Point { int value; };
        struct Node { const struct Point *cursor; };

        int main(void) {
            struct Point points[1] = {{3}};
            struct Node nodes[1] = {{points}};
            return nodes[0].cursor[0].value = 9;
        }
    "#;

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign through pointer to const");
}

#[test]
fn sizeof_named_aggregate_array_pointer_field_subscripts_is_non_evaluating() {
    let program = r#"
        struct Node { char *bytes; };

        int main(void) {
            char values[2] = {'a', 'b'};
            struct Node nodes[1] = {{values}};
            int outer = 0;
            int inner = 0;
            int rhs = 0;
            int ok = 0;
            ok += sizeof(nodes[outer++].bytes[inner++]) == sizeof(char);
            ok += sizeof(nodes[outer++].bytes[inner++] = (rhs++, 'x')) == sizeof(char);
            ok += sizeof(nodes[outer++].bytes[inner++] += (rhs++, 1)) == sizeof(char);
            ok += sizeof(nodes[outer++].bytes[inner++]++) == sizeof(char);
            return ok + (outer == 0) + (inner == 0) + (rhs == 0)
                + (values[0] == 'a');
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 8);
}

#[test]
fn sizeof_named_aggregate_array_aggregate_pointer_field_subscripts_is_non_evaluating() {
    let program = r#"
        struct Point { int value; };
        struct Node { struct Point *points; };

        int main(void) {
            struct Point points[1] = {{3}};
            struct Node nodes[1] = {{points}};
            int outer = 0;
            int inner = 0;
            int rhs = 0;
            int ok = 0;
            ok += sizeof(nodes[outer++].points[inner++]) == sizeof(struct Point);
            ok += sizeof(nodes[outer++].points[inner++].value = (rhs++, 9)) == sizeof(int);
            ok += sizeof(nodes[outer++].points[inner++].value += (rhs++, 2)) == sizeof(int);
            ok += sizeof(nodes[outer++].points[inner++].value++) == sizeof(int);
            return ok + (outer == 0) + (inner == 0) + (rhs == 0)
                + (points[0].value == 3);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 8);
}

#[test]
fn sizeof_named_aggregate_array_aggregate_pointer_field_assignment_result_fields_is_non_evaluating()
{
    let program = r#"
        struct Point { char tag; int value; };
        struct Node { struct Point *points; };

        int main(void) {
            struct Point points[1] = {{'a', 3}};
            struct Point replacement = {'z', 9};
            struct Node nodes[1] = {{points}};
            int outer = 0;
            int inner = 0;
            int rhs = 0;
            int ok = sizeof((nodes[outer++].points[inner++] =
                             (rhs++, replacement)).tag) == sizeof(char);
            return ok + (outer == 0) + (inner == 0) + (rhs == 0)
                + (points[0].tag == 'a');
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 5);
}

#[test]
fn named_aggregate_array_pointer_field_subscript_addresses_preserve_const() {
    let program = r#"
        struct Node { const int *reader; };

        int main(void) {
            int values[1] = {3};
            struct Node nodes[1] = {{values}};
            int *bad = &nodes[0].reader[0];
            return *bad;
        }
    "#;

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn named_aggregate_array_pointer_field_subscripts_preserve_diagnostics() {
    let cases = [
        (
            r#"
                struct Node { const int *cursor; };
                int main(void) {
                    int values[1] = {3};
                    struct Node nodes[1] = {{values}};
                    return nodes[0].cursor[0] = 9;
                }
            "#,
            "cannot assign through pointer to const",
        ),
        (
            r#"
                struct Node { int *cursor; };
                int main(void) {
                    int values[2] = {3, 5};
                    struct Node nodes[1] = {{values}};
                    return nodes[0].cursor[2];
                }
            "#,
            "array pointer index 2 out of bounds for length 2",
        ),
        (
            r#"
                struct Node { int *cursor; };
                int main(void) {
                    int values[1] = {3};
                    struct Node nodes[1] = {{values}};
                    return nodes[1].cursor[0];
                }
            "#,
            "struct array 'nodes' index 1 out of bounds for length 1",
        ),
        (
            r#"
                struct Point { int value; };
                union Number { int value; };
                struct Node { struct Point *cursor; };
                int main(void) {
                    struct Point points[1] = {{3}};
                    struct Node nodes[1] = {{points}};
                    union Number *bad = &nodes[0].cursor[0];
                    return bad->value;
                }
            "#,
            "cannot convert pointer to struct 'Point' to pointer to union 'Number'",
        ),
    ];

    for (program, expected) in cases {
        match interpret(program) {
            Ok(value) => panic!("expected '{expected}', got {value}; program: {program}"),
            Err(err) => assert_eq!(err.to_string(), expected, "program: {program}"),
        }
    }
}

#[test]
fn rejects_const_pointer_field_updates_in_named_aggregate_array_elements() {
    let program = r#"
        struct Node { int * const cursor; };

        int main(void) {
            int values[2] = {3, 5};
            struct Node nodes[1] = {{values}};
            nodes[0].cursor = values + 1;
            return 0;
        }
    "#;

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'cursor'"
    );
}

#[test]
fn rejects_const_discard_from_named_aggregate_array_element_pointer_fields() {
    let program = r#"
        struct Node { const int *reader; };

        int main(void) {
            const int values[1] = {3};
            struct Node nodes[1] = {{values}};
            int *cursor = nodes[0].reader;
            return *cursor;
        }
    "#;

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn reads_mutable_pointer_fields_through_const_named_aggregate_array_elements() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[1] = {3};
            const struct Node nodes[1] = {{values}};
            int *cursor = nodes[0].cursor;
            *cursor = 7;
            return values[0];
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn sizeof_named_aggregate_array_element_pointer_field_updates_is_non_evaluating() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[2] = {3, 5};
            struct Node nodes[1] = {{values}};
            int index_marker = 0;
            int rhs_marker = 0;
            int total = 0;
            total += sizeof(nodes[index_marker++].cursor) == sizeof(int *);
            total += sizeof(nodes[index_marker++].cursor =
                            (rhs_marker++, values + 1)) == sizeof(int *);
            total += sizeof(nodes[index_marker++].cursor +=
                            (rhs_marker++, 1)) == sizeof(int *);
            total += sizeof(nodes[index_marker++].cursor++) == sizeof(int *);
            return total + (index_marker == 0) + (rhs_marker == 0)
                + (*nodes[0].cursor == 3);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn rejects_pointer_type_mismatches_in_named_aggregate_array_element_fields() {
    let program = r#"
        struct Point { int value; };
        union Number { int value; };
        struct Node { struct Point *point; };

        int main(void) {
            struct Point points[1] = {{1}};
            union Number numbers[1] = {{2}};
            struct Node nodes[1] = {{points}};
            nodes[0].point = numbers;
            return 0;
        }
    "#;

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot convert pointer to union 'Number' to pointer to struct 'Point'"
    );
}

#[test]
fn rejects_const_discard_from_named_aggregate_array_pointer_field_increment_results() {
    let program = r#"
        struct Node { const int *cursor; };

        int main(void) {
            const int values[2] = {3, 5};
            struct Node nodes[1] = {{values}};
            int *bad = nodes[0].cursor++;
            return *bad;
        }
    "#;

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_pointer_field_updates_in_const_named_aggregate_arrays() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[2] = {3, 5};
            const struct Node nodes[1] = {{values}};
            nodes[0].cursor = values + 1;
            return 0;
        }
    "#;

    let err = interpret(program).unwrap_err();
    assert_eq!(err.to_string(), "cannot assign to const variable 'nodes'");
}

#[test]
fn preserves_named_aggregate_array_bounds_for_pointer_field_reads() {
    let program = r#"
        struct Node { int *cursor; };

        int main(void) {
            int values[1] = {3};
            struct Node nodes[1] = {{values}};
            return *nodes[1].cursor;
        }
    "#;

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "struct array 'nodes' index 1 out of bounds for length 1"
    );
}

#[test]
fn supports_pointer_field_updates_through_reverse_and_nested_embedded_aggregate_array_elements() {
    let program = r#"
        struct Node { int *cursor; };
        struct Layer { struct Node nodes[1]; };
        struct Box { struct Layer layer; };
        struct Index { int value; };

        int main(void) {
            int values[3] = {3, 5, 7};
            struct Node loose[1] = {{values}};
            struct Index index = {0};
            struct Box box = {{{{values + 1}}}};
            int *reverse = (index.value[loose].cursor = values + 2);
            int *nested = (box.layer.nodes[0].cursor += 1);
            return *reverse + *index.value[loose].cursor + *nested + *box.layer.nodes[0].cursor;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 28);
}

#[test]
fn embedded_aggregate_array_element_pointer_field_updates_match_fixture() {
    let program =
        include_str!("fixtures/valid/embedded_aggregate_array_element_pointer_field_updates.c",);

    assert_eq!(interpret(program).unwrap(), 9);
}

#[test]
fn sizeof_pointer_field_update_through_embedded_aggregate_array_element_is_non_evaluating() {
    let program = r#"
        struct Node { int *cursor; };
        struct Box { struct Node nodes[1]; };

        int main(void) {
            int values[2] = {3, 5};
            struct Box box = {{{values}}};
            int index_marker = 0;
            int rhs_marker = 0;
            int result_size = sizeof(
                (box.nodes[index_marker++].cursor += (rhs_marker++, 1))
            );
            return (result_size == sizeof(int *)) +
                   (index_marker == 0) +
                   (rhs_marker == 0) +
                   (*box.nodes[0].cursor == 3);
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 4);
}

#[test]
fn reads_pointer_fields_through_const_embedded_aggregate_array_elements() {
    let program = r#"
        struct Node { int *cursor; };
        struct Box { struct Node nodes[1]; };

        int main(void) {
            int values[1] = {7};
            const struct Box box = {{{values}}};
            return *box.nodes[0].cursor;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 7);
}

#[test]
fn rejects_const_pointer_field_assignment_through_embedded_aggregate_array_element() {
    let program = include_str!(
        "fixtures/invalid/const_embedded_aggregate_array_element_pointer_field_assignment.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot assign to const struct field 'cursor'"
    );
}

#[test]
fn rejects_const_discard_in_pointer_field_assignment_through_embedded_aggregate_array_element() {
    let program = include_str!(
        "fixtures/invalid/embedded_aggregate_array_element_pointer_field_const_discard.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_const_discard_from_pointer_field_increment_result_through_embedded_array_element() {
    let program = include_str!(
        "fixtures/invalid/embedded_aggregate_array_element_pointer_field_increment_const_discard.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot discard const qualifier from pointer target"
    );
}

#[test]
fn rejects_type_mismatch_in_pointer_field_assignment_through_embedded_aggregate_array_element() {
    let program = include_str!(
        "fixtures/invalid/embedded_aggregate_array_element_pointer_field_type_mismatch.c",
    );

    let err = interpret(program).unwrap_err();
    assert_eq!(
        err.to_string(),
        "cannot convert pointer to char to pointer to int"
    );
}

#[test]
fn supports_reverse_embedded_aggregate_array_element_field_assignment() {
    let program = r#"
        struct Point { int value; };
        struct Index { int value; };

        int main(void) {
            struct Point points[2] = {{3}, {5}};
            struct Index index = {1};
            int result = index.value[points].value = 9;
            return result + points[1].value;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 18);
}

#[test]
fn embedded_aggregate_array_element_field_replacement_assignment_evaluates_once() {
    let program = r#"
        struct Point { int value; };
        struct Box { struct Point points[2]; };
        struct Index { int value; };

        int main(void) {
            struct Box box = {{{3}, {5}}};
            struct Point points[2] = {{6}, {8}};
            struct Index index = {1};
            int direct_index_marker = 0;
            int direct_rhs_marker = 0;
            int reverse_pointer_marker = 0;
            int reverse_rhs_marker = 0;
            int direct_result = box.points[(direct_index_marker += 1, 0)].value =
                (direct_rhs_marker += 1, 7);
            int reverse_result = index.value[(reverse_pointer_marker += 1, points)].value =
                (reverse_rhs_marker += 1, 9);
            return direct_result + reverse_result + box.points[0].value + points[1].value
                + direct_index_marker + direct_rhs_marker
                + reverse_pointer_marker + reverse_rhs_marker;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 36);
}

#[test]
fn direct_embedded_aggregate_array_element_field_compound_assignment_evaluates_once() {
    let program = r#"
        struct Point { int value; };
        struct Box { struct Point points[2]; };

        int main(void) {
            struct Box box = {{{3}, {10}}};
            int index_marker = 0;
            int rhs_marker = 0;
            int result = (box.points[index_marker++].value += (rhs_marker += 1, 2));
            int ok = 0;
            ok += result == 5;
            ok += index_marker == 1;
            ok += rhs_marker == 1;
            ok += box.points[0].value == 5;
            ok += box.points[1].value == 10;
            return ok;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 5);
}

#[test]
fn reverse_embedded_aggregate_array_element_field_compound_assignment_evaluates_once() {
    let program = r#"
        struct Point { int value; };
        struct Index { int value; };

        int main(void) {
            struct Point points[2] = {{3}, {5}};
            struct Index index = {1};
            int pointer_marker = 0;
            int rhs_marker = 0;
            int result = (index.value[(pointer_marker += 1, points)].value +=
                          (rhs_marker += 1, 2));
            int ok = 0;
            ok += result == 7;
            ok += pointer_marker == 1;
            ok += rhs_marker == 1;
            ok += points[0].value == 3;
            ok += points[1].value == 7;
            return ok;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 5);
}

#[test]
fn supports_embedded_aggregate_array_element_field_assignment() {
    let program =
        include_str!("fixtures/valid/embedded_aggregate_array_element_field_assignment.c");

    assert_eq!(interpret(program).unwrap(), 16);
}

#[test]
fn embedded_aggregate_array_element_field_assignment_preserves_diagnostics() {
    let cases = [
        (
            r#"
                struct Point { int value; };
                struct Box { const struct Point points[2]; };
                int main(void) {
                    struct Box box = {{{3}, {5}}};
                    return box.points[0].value = 9;
                }
            "#,
            "cannot assign to const struct field 'points'",
        ),
        (
            r#"
                struct Point { const int value; };
                struct Box { struct Point points[2]; };
                int main(void) {
                    struct Box box = {{{3}, {5}}};
                    return box.points[0].value += 2;
                }
            "#,
            "cannot assign to const struct field 'value'",
        ),
        (
            r#"
                struct Point { int value; };
                struct Index { int value; };
                int main(void) {
                    const struct Point points[2] = {{3}, {5}};
                    struct Index index = {1};
                    return index.value[points].value = 9;
                }
            "#,
            "cannot assign to const variable 'points'",
        ),
        (
            r#"
                struct Point { int value; };
                struct Box { struct Point points[2]; };
                int main(void) {
                    struct Box box = {{{3}, {5}}};
                    return box.points[2].value = 9;
                }
            "#,
            "struct array field 'points' index 2 out of bounds for length 2",
        ),
        (
            r#"
                struct Point { int value; };
                struct Index { int value; };
                int main(void) {
                    struct Point points[2] = {{3}, {5}};
                    struct Index index = {2};
                    return index.value[points].value += 2;
                }
            "#,
            "struct array 'points' index 2 out of bounds for length 2",
        ),
        (
            r#"
                struct Index { int value; };
                int main(void) {
                    int values[2] = {3, 5};
                    struct Index index = {1};
                    return index.value[values].value = 9;
                }
            "#,
            "pointer does not reference a struct",
        ),
    ];

    for (program, expected) in cases {
        match interpret(program) {
            Ok(value) => panic!("expected '{expected}', got {value}; program: {program}"),
            Err(err) => assert_eq!(err.to_string(), expected, "program: {program}"),
        }
    }
}

#[test]
fn aggregate_pointer_field_indexed_assignments_preserve_pointee_constness() {
    let const_pointee = r#"
        struct Point { int value; };
        struct Cursor { const struct Point *points; };
        int main(void) {
            struct Point points[1] = {{1}};
            struct Cursor cursor = {points};
            return cursor.points[0].value += 2;
        }
    "#;
    assert_eq!(
        interpret(const_pointee).unwrap_err().to_string(),
        "cannot assign through pointer to const"
    );

    let const_slot = r#"
        struct Point { int value; };
        struct Cursor { struct Point * const points; };
        int main(void) {
            struct Point points[1] = {{1}};
            const struct Cursor cursor = {points};
            int result = cursor.points[0].value = 9;
            return result + points[0].value;
        }
    "#;
    assert_eq!(interpret(const_slot).unwrap(), 18);
}

#[test]
fn sizeof_embedded_aggregate_array_element_field_assignments_is_non_evaluating() {
    let program = r#"
        struct Point { int value; char tag; };
        struct Box { struct Point points[2]; };
        struct Index { int value; };

        int main(void) {
            struct Box box = {{{3, 'a'}, {5, 'b'}}};
            struct Point points[2] = {{7, 'c'}, {9, 'd'}};
            struct Index index = {1};
            int direct_index_marker = 0;
            int direct_rhs_marker = 0;
            int reverse_pointer_marker = 0;
            int reverse_rhs_marker = 0;
            int ok = 0;

            ok += sizeof(
                box.points[(direct_index_marker += 1, 0)].tag =
                    (direct_rhs_marker += 1, 'x')
            ) == sizeof(char);
            ok += sizeof(
                box.points[(direct_index_marker += 1, 1)].value +=
                    (direct_rhs_marker += 1, 2)
            ) == sizeof(int);
            ok += sizeof(
                index.value[(reverse_pointer_marker += 1, points)].tag =
                    (reverse_rhs_marker += 1, 'y')
            ) == sizeof(char);
            ok += sizeof(
                index.value[(reverse_pointer_marker += 1, points)].value *=
                    (reverse_rhs_marker += 1, 2)
            ) == sizeof(int);
            ok += direct_index_marker == 0;
            ok += direct_rhs_marker == 0;
            ok += reverse_pointer_marker == 0;
            ok += reverse_rhs_marker == 0;
            ok += box.points[0].tag == 'a' && box.points[1].value == 5;
            ok += points[1].tag == 'd' && points[1].value == 9;
            return ok;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 10);
}

#[test]
fn embedded_aggregate_array_element_field_increment_preserves_diagnostics() {
    let cases = [
        (
            r#"
                struct Point { int value; };
                struct Box { const struct Point points[2]; };
                int main(void) {
                    struct Box box = {{{3}, {5}}};
                    return ++box.points[0].value;
                }
            "#,
            "cannot assign to const struct field 'points'",
        ),
        (
            r#"
                struct Point { const int value; };
                struct Box { struct Point points[2]; };
                int main(void) {
                    struct Box box = {{{3}, {5}}};
                    return ++box.points[0].value;
                }
            "#,
            "cannot assign to const struct field 'value'",
        ),
        (
            r#"
                struct Point { int value; };
                struct Index { int value; };
                int main(void) {
                    const struct Point points[2] = {{3}, {5}};
                    struct Index index = {1};
                    return ++index.value[points].value;
                }
            "#,
            "cannot assign to const variable 'points'",
        ),
        (
            r#"
                struct Point { int value; };
                struct Box { struct Point points[2]; };
                int main(void) {
                    struct Box box = {{{3}, {5}}};
                    return box.points[2].value++;
                }
            "#,
            "struct array field 'points' index 2 out of bounds for length 2",
        ),
        (
            r#"
                struct Point { int value; };
                struct Index { int value; };
                int main(void) {
                    struct Point points[2] = {{3}, {5}};
                    struct Index index = {2};
                    return index.value[points].value++;
                }
            "#,
            "struct array 'points' index 2 out of bounds for length 2",
        ),
        (
            r#"
                struct Index { int value; };
                int main(void) {
                    int values[2] = {3, 5};
                    struct Index index = {1};
                    return ++index.value[values].value;
                }
            "#,
            "pointer does not reference a struct",
        ),
    ];

    for (program, expected) in cases {
        match interpret(program) {
            Ok(value) => panic!("expected '{expected}', got {value}; program: {program}"),
            Err(err) => assert_eq!(err.to_string(), expected, "program: {program}"),
        }
    }
}

#[test]
fn sizeof_embedded_aggregate_array_element_field_increments_is_non_evaluating() {
    let program = r#"
        struct Point { int value; char tag; };
        struct Box { struct Point points[2]; };
        struct Index { int value; };

        int main(void) {
            struct Box box = {{{3, 'a'}, {5, 'b'}}};
            struct Point points[2] = {{7, 'c'}, {9, 'd'}};
            struct Index index = {1};
            int direct_marker = 0;
            int reverse_marker = 0;
            int ok = 0;

            ok += sizeof(++box.points[(direct_marker += 1, 0)].tag) == sizeof(char);
            ok += sizeof(box.points[(direct_marker += 1, 1)].value--) == sizeof(int);
            ok += sizeof(++index.value[(reverse_marker += 1, points)].tag) == sizeof(char);
            ok += sizeof(index.value[(reverse_marker += 1, points)].value--) == sizeof(int);
            ok += direct_marker == 0;
            ok += reverse_marker == 0;
            ok += box.points[0].tag == 'a' && box.points[1].value == 5;
            ok += points[1].tag == 'd' && points[1].value == 9;
            return ok;
        }
    "#;

    assert_eq!(interpret(program).unwrap(), 8);
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
fn rejects_invalid_start_switch_case_values_with_context() {
    let cases = [
        (
            "int main(void) { switch (1) { case [: return 1; default: return 0; } }",
            "expected integer constant after switch case before '[' at line 1, column 36",
        ),
        (
            "int main(void) { switch (1) { case ?: return 1; default: return 0; } }",
            "expected integer constant after switch case before '?' at line 1, column 36",
        ),
        (
            "int main(void) { switch (1) { case int: return 1; default: return 0; } }",
            "expected integer constant after switch case before 'int' at line 1, column 36",
        ),
        (
            "int main(void) { switch (1) { case return: return 1; default: return 0; } }",
            "expected integer constant after switch case before 'return' at line 1, column 36",
        ),
        (
            "int main(void) { switch (1) { case .field: return 1; default: return 0; } }",
            "expected integer constant after switch case before '.' at line 1, column 36",
        ),
        (
            "int main(void) { switch (1) { case ->field: return 1; default: return 0; } }",
            "expected integer constant after switch case before '->' at line 1, column 36",
        ),
        (
            "int main(void) { switch (1) { case {1}: return 1; default: return 0; } }",
            "expected integer constant after switch case before '{' at line 1, column 36",
        ),
        (
            "int main(void) { switch (1) { case :: return 1; default: return 0; } }",
            "expected integer constant after switch case before ':' at line 1, column 36",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();

        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_missing_switch_expressions_with_context() {
    let cases = [
        (
            "int main(void) { switch () { default: return 1; } }",
            "expected expression after switch, found RParen at line 1, column 26",
        ),
        (
            "int main(void) { switch (; }",
            "expected expression after switch, found Semi at line 1, column 26",
        ),
        (
            "int main(void) { switch (} }",
            "expected expression after switch, found RBrace at line 1, column 26",
        ),
        (
            "int main(void) { switch ([) { default: return 1; } }",
            "expected expression after switch, found LBracket at line 1, column 26",
        ),
        (
            "int main(void) { switch ({1}) { default: return 1; } }",
            "expected expression after switch, found LBrace at line 1, column 26",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();

        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_keyword_start_return_and_control_expressions_with_context() {
    let cases = [
        (
            "int main(void) { return int; }",
            "expected expression after return before 'int' at line 1, column 25",
        ),
        (
            "int main(void) { return struct; }",
            "expected expression after return before 'struct' at line 1, column 25",
        ),
        (
            "int main(void) { if (return) { return 1; } return 0; }",
            "expected expression after if before 'return' at line 1, column 22",
        ),
        (
            "int main(void) { while (int) { return 1; } return 0; }",
            "expected expression after while before 'int' at line 1, column 25",
        ),
        (
            "int main(void) { do { } while (struct); }",
            "expected expression after do-while before 'struct' at line 1, column 32",
        ),
        (
            "int main(void) { for (int i = 0; return; i = i + 1) { return i; } return 0; }",
            "expected expression after for condition before 'return' at line 1, column 34",
        ),
        (
            "int main(void) { for (int i = 0; i < 3; struct) { return i; } return 0; }",
            "expected expression after for increment before 'struct' at line 1, column 41",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();

        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_missing_control_flow_condition_expressions_with_context() {
    let cases = [
        (
            "int main(void) { if () { return 1; } return 0; }",
            "expected expression after if, found RParen at line 1, column 22",
        ),
        (
            "int main(void) { if ([) { return 1; } return 0; }",
            "expected expression after if, found LBracket at line 1, column 22",
        ),
        (
            "int main(void) { if ({1}) { return 1; } return 0; }",
            "expected expression after if, found LBrace at line 1, column 22",
        ),
        (
            "int main(void) { if (.field) { return 1; } return 0; }",
            "expected expression after if, found Dot at line 1, column 22",
        ),
        (
            "int main(void) { while (; }",
            "expected expression after while, found Semi at line 1, column 25",
        ),
        (
            "int main(void) { while (?) { return 1; } return 0; }",
            "expected expression after while, found Question at line 1, column 25",
        ),
        (
            "int main(void) { while (->field) { return 1; } return 0; }",
            "expected expression after while, found Arrow at line 1, column 25",
        ),
        (
            "int main(void) { do { } while (); }",
            "expected expression after do-while, found RParen at line 1, column 32",
        ),
        (
            "int main(void) { do { } while ([); }",
            "expected expression after do-while, found LBracket at line 1, column 32",
        ),
        (
            "int main(void) { for (int i = 0; [; i = i + 1) { return i; } return 0; }",
            "expected expression after for condition, found LBracket at line 1, column 34",
        ),
        (
            "int main(void) { for (int i = 0; ?; i = i + 1) { return i; } return 0; }",
            "expected expression after for condition, found Question at line 1, column 34",
        ),
        (
            "int main(void) { for (int i = 0; .field; i = i + 1) { return i; } return 0; }",
            "expected expression after for condition, found Dot at line 1, column 34",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();

        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn rejects_misplaced_for_increment_expressions_with_context() {
    let cases = [
        (
            "int main(void) { for (int i = 0; i < 3; [) { return i; } return 0; }",
            "expected expression after for increment, found LBracket at line 1, column 41",
        ),
        (
            "int main(void) { for (int i = 0; i < 3; {1}) { return i; } return 0; }",
            "expected expression after for increment, found LBrace at line 1, column 41",
        ),
        (
            "int main(void) { for (int i = 0; i < 3; ?) { return i; } return 0; }",
            "expected expression after for increment, found Question at line 1, column 41",
        ),
        (
            "int main(void) { for (int i = 0; i < 3; .field) { return i; } return 0; }",
            "expected expression after for increment, found Dot at line 1, column 41",
        ),
        (
            "int main(void) { for (int i = 0; i < 3; ->field) { return i; } return 0; }",
            "expected expression after for increment, found Arrow at line 1, column 41",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();

        assert_eq!(err.to_string(), expected);
    }
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
    let cases = [
        (
            include_str!("fixtures/invalid/comma_missing_rhs.c"),
            "expected expression after comma operator, found RParen at line 3, column 19",
        ),
        (
            "int main(void) {\n    return (1,];\n}\n",
            "expected expression after comma operator, found RBracket at line 2, column 15",
        ),
        (
            "int main(void) {\n    return 1,;\n}\n",
            "expected expression after comma operator, found Semi at line 2, column 14",
        ),
        (
            "int main(void) {\n    return 1,{2};\n}\n",
            "expected expression after comma operator, found LBrace at line 2, column 14",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_missing_array_index_expressions_with_context() {
    let cases = [
        (
            "int main(void) {\n    int values[2] = {1, 2};\n    return values[];\n}\n",
            "expected array index expression, found RBracket at line 3, column 19",
        ),
        (
            "int main(void) {\n    int values[2] = {1, 2};\n    return values[;\n}\n",
            "expected array index expression, found Semi at line 3, column 19",
        ),
        (
            "int main(void) {\n    return \"hi\"[];\n}\n",
            "expected array index expression, found RBracket at line 2, column 17",
        ),
        (
            "int main(void) {\n    int values[2] = {1, 2};\n    return values[[);\n}\n",
            "expected array index expression, found LBracket at line 3, column 19",
        ),
        (
            "int main(void) {\n    return \"hi\"[?];\n}\n",
            "expected array index expression, found Question at line 2, column 17",
        ),
        (
            "int main(void) {\n    int values[2] = {1, 2};\n    return values[};\n",
            "expected array index expression, found RBrace at line 3, column 19",
        ),
        (
            "int main(void) {\n    int values[2] = {1, 2};\n    return values[int];\n}\n",
            "expected array index expression before 'int' at line 3, column 19",
        ),
        (
            "int main(void) {\n    return \"hi\"[return];\n}\n",
            "expected array index expression before 'return' at line 2, column 17",
        ),
        (
            "int main(void) {\n    int values[2] = {1, 2};\n    return values[.field];\n}\n",
            "expected array index expression, found Dot at line 3, column 19",
        ),
        (
            "int main(void) {\n    int values[2] = {1, 2};\n    return values[{1}];\n}\n",
            "expected array index expression, found LBrace at line 3, column 19",
        ),
        (
            "int main(void) {\n    return \"hi\"[->field];\n}\n",
            "expected array index expression, found Arrow at line 2, column 17",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_missing_rhs_after_binary_operators() {
    let cases = [
        (
            "int main(void) {\n    return 1 + ;\n}\n",
            "expected expression after binary operator '+', found Semi at line 2, column 16",
        ),
        (
            "int main(void) {\n    return (1 && );\n}\n",
            "expected expression after binary operator '&&', found RParen at line 2, column 18",
        ),
        (
            "int main(void) {\n    return 1 << ];\n}\n",
            "expected expression after binary operator '<<', found RBracket at line 2, column 17",
        ),
        (
            "int main(void) {\n    if (1 == }) { return 1; }\n    return 0;\n}\n",
            "expected expression after binary operator '==', found RBrace at line 2, column 14",
        ),
        (
            "int main(void) {\n    return 1 + {2};\n}\n",
            "expected expression after binary operator '+', found LBrace at line 2, column 16",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_missing_operands_after_unary_operators() {
    let cases = [
        (
            "int main(void) {\n    return !;\n}\n",
            "expected expression after unary operator '!', found Semi at line 2, column 13",
        ),
        (
            "int main(void) {\n    return ~);\n}\n",
            "expected expression after unary operator '~', found RParen at line 2, column 13",
        ),
        (
            "int main(void) {\n    return ++];\n}\n",
            "expected expression after unary operator '++', found RBracket at line 2, column 14",
        ),
        (
            "int main(void) {\n    return *}\n",
            "expected expression after unary operator '*', found RBrace at line 2, column 13",
        ),
        (
            "int main(void) {\n    return &;\n}\n",
            "expected expression after unary operator '&', found Semi at line 2, column 13",
        ),
        (
            "int main(void) {\n    return ![;\n}\n",
            "expected expression after unary operator '!', found LBracket at line 2, column 13",
        ),
        (
            "int main(void) {\n    return *?;\n}\n",
            "expected expression after unary operator '*', found Question at line 2, column 13",
        ),
        (
            "int main(void) {\n    return &return;\n}\n",
            "expected expression after unary operator '&', found Return at line 2, column 13",
        ),
        (
            "int main(void) {\n    return !.field;\n}\n",
            "expected expression after unary operator '!', found Dot at line 2, column 13",
        ),
        (
            "int main(void) {\n    return *->field;\n}\n",
            "expected expression after unary operator '*', found Arrow at line 2, column 13",
        ),
        (
            "int main(void) {\n    return !{1};\n}\n",
            "expected expression after unary operator '!', found LBrace at line 2, column 13",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
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
fn rejects_missing_rhs_after_assignment_operators() {
    let cases = [
        (
            include_str!("fixtures/invalid/assignment_missing_rhs.c"),
            "expected expression after assignment operator '=', found Semi at line 3, column 13",
        ),
        (
            "int main(void) {\n    int value = 1;\n    value += );\n    return value;\n}\n",
            "expected expression after assignment operator '+=', found RParen at line 3, column 14",
        ),
        (
            "int main(void) {\n    int value = 1;\n    value <<= }\n",
            "expected expression after assignment operator '<<=', found RBrace at line 3, column 15",
        ),
        (
            "int main(void) {\n    int value = 1;\n    value = {2};\n    return value;\n}\n",
            "expected expression after assignment operator '=', found LBrace at line 3, column 13",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_missing_return_expressions_with_context() {
    let cases = [
        (
            "int main(void) {\n    return ,;\n}\n",
            "expected expression after return, found Comma at line 2, column 12",
        ),
        (
            "int main(void) {\n    return );\n}\n",
            "expected expression after return, found RParen at line 2, column 12",
        ),
        (
            "int main(void) {\n    return }\n",
            "expected expression after return, found RBrace at line 2, column 12",
        ),
        (
            "int main(void) {\n    return [;\n}\n",
            "expected expression after return, found LBracket at line 2, column 12",
        ),
        (
            "int main(void) {\n    return ?;\n}\n",
            "expected expression after return, found Question at line 2, column 12",
        ),
        (
            "int main(void) {\n    return .field;\n}\n",
            "expected expression after return, found Dot at line 2, column 12",
        ),
        (
            "int main(void) {\n    return ->field;\n}\n",
            "expected expression after return, found Arrow at line 2, column 12",
        ),
        (
            "int main(void) {\n    return {1};\n}\n",
            "expected expression after return, found LBrace at line 2, column 12",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_missing_declaration_initializer_expressions_with_context() {
    let cases = [
        (
            "int main(void) {\n    int value = ;\n    return value;\n}\n",
            "expected initializer expression after '=' in variable declaration, found Semi at line 2, column 17",
        ),
        (
            "int main(void) {\n    int value = 1, other = ;\n    return value + other;\n}\n",
            "expected initializer expression after '=' in variable declaration, found Semi at line 2, column 28",
        ),
        (
            "int main(void) {\n    int value = [?];\n    return value;\n}\n",
            "expected initializer expression after '=' in variable declaration, found LBracket at line 2, column 17",
        ),
        (
            "int main(void) {\n    int value = typedef;\n    return value;\n}\n",
            "expected initializer expression after '=' in variable declaration before 'typedef' at line 2, column 17",
        ),
        (
            "int main(void) {\n    int value = 1;\n    int *slot = , fallback = 0;\n    return value;\n}\n",
            "expected initializer expression after '=' in pointer declaration, found Comma at line 3, column 17",
        ),
        (
            "int main(void) {\n    int value = 1;\n    int *slot = ?;\n    return value;\n}\n",
            "expected initializer expression after '=' in pointer declaration, found Question at line 3, column 17",
        ),
        (
            "struct Point { int x; };\nint main(void) {\n    struct Point point = ;\n    return 0;\n}\n",
            "expected initializer expression after '=' in struct variable declaration, found Semi at line 3, column 26",
        ),
        (
            "int main(void) {\n    int values[2] = ;\n    return 0;\n}\n",
            "expected initializer expression after '=' in array declaration, found Semi at line 2, column 21",
        ),
        (
            "int main(void) {\n    int first[1] = {1}, second[2] = ;\n    return first[0];\n}\n",
            "expected initializer expression after '=' in array declaration, found Semi at line 2, column 37",
        ),
        (
            "struct Point { int x; };\nint main(void) {\n    struct Point points[1] = ;\n    return 0;\n}\n",
            "expected initializer expression after '=' in struct array declaration, found Semi at line 3, column 30",
        ),
        (
            "struct Point { int x; };\nint main(void) {\n    struct Point first[1] = {{1}}, second[1] = ;\n    return first[0].x;\n}\n",
            "expected initializer expression after '=' in struct array declaration, found Semi at line 3, column 48",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_missing_braced_initializer_elements_with_context() {
    let cases = [
        (
            "int main(void) {\n    int values[2] = {, 1};\n    return 0;\n}\n",
            "expected initializer element in array 'values' initializer, found Comma at line 2, column 22",
        ),
        (
            "int main(void) {\n    int values[2] = { [0] = , 1};\n    return 0;\n}\n",
            "expected initializer element in array 'values' initializer, found Comma at line 2, column 29",
        ),
        (
            "struct Point { int x; int y; };\nint main(void) {\n    struct Point point = {, 1};\n    return 0;\n}\n",
            "expected initializer element in struct 'Point' initializer, found Comma at line 3, column 27",
        ),
        (
            "struct Point { int x; int y; };\nint main(void) {\n    struct Point point = {.x = , .y = 1};\n    return 0;\n}\n",
            "expected initializer element in struct 'Point' initializer, found Comma at line 3, column 32",
        ),
        (
            "struct Point { int x; int y; };\nint main(void) {\n    struct Point point = {.x = [?], .y = 1};\n    return 0;\n}\n",
            "expected initializer element in struct 'Point' initializer, found LBracket at line 3, column 32",
        ),
        (
            "int main(void) {\n    int values[2] = {[0] = ?};\n    return 0;\n}\n",
            "expected initializer element in array 'values' initializer, found Question at line 2, column 28",
        ),
        (
            "int main(void) {\n    int values[2] = {.field};\n    return 0;\n}\n",
            "expected initializer element in array 'values' initializer, found Dot at line 2, column 22",
        ),
        (
            "int main(void) {\n    int values[2] = {->field};\n    return 0;\n}\n",
            "expected initializer element in array 'values' initializer, found Arrow at line 2, column 22",
        ),
        (
            "struct Point { int x; int y; };\nint main(void) {\n    struct Point points[2] = {, {1, 2}};\n    return 0;\n}\n",
            "expected initializer element in struct array 'points' initializer, found Comma at line 3, column 31",
        ),
        (
            "struct Point { int x; int y; };\nint main(void) {\n    struct Point point = {->field = 1};\n    return 0;\n}\n",
            "expected initializer element in struct 'Point' initializer, found Arrow at line 3, column 27",
        ),
        (
            "int main(void) {\n    int values[2] = {return};\n    return 0;\n}\n",
            "expected initializer element in array 'values' initializer before 'return' at line 2, column 22",
        ),
        (
            "struct Point { int x; int y; };\nint main(void) {\n    struct Point point = {return};\n    return 0;\n}\n",
            "expected initializer element in struct 'Point' initializer before 'return' at line 3, column 27",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_missing_braced_scalar_initializer_expressions_with_context() {
    let cases = [
        (
            "int main(void) {\n    int value = {,};\n    return value;\n}\n",
            "expected initializer element in braced scalar initializer for variable 'value', found Comma at line 2, column 18",
        ),
        (
            "int main(void) {\n    int value = {};\n    return value;\n}\n",
            "expected initializer element in braced scalar initializer for variable 'value', found RBrace at line 2, column 18",
        ),
        (
            "struct Point { int x; };\nint main(void) {\n    struct Point point = { .x = {,} };\n    return 0;\n}\n",
            "expected initializer element in braced scalar initializer for struct field 'x', found Comma at line 3, column 34",
        ),
        (
            "int main(void) {\n    int value = {[};\n    return value;\n}\n",
            "expected initializer element in braced scalar initializer for variable 'value', found LBracket at line 2, column 18",
        ),
        (
            "struct Point { int x; };\nint main(void) {\n    struct Point point = { .x = {.bad} };\n    return 0;\n}\n",
            "expected initializer element in braced scalar initializer for struct field 'x', found Dot at line 3, column 34",
        ),
        (
            "int main(void) {\n    return (int){->field};\n}\n",
            "expected initializer element in braced scalar initializer for scalar compound literal, found Arrow at line 2, column 18",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn reports_too_many_braced_initializer_entries_with_source_locations() {
    let cases = [
        (
            "int main(void) {\n    int value = {1, 2};\n    return value;\n}\n",
            "too many initializers for variable 'value' at line 2, column 21",
        ),
        (
            "int main(void) {\n    int values[1] = {1, 2};\n    return values[0];\n}\n",
            "too many initializers for array 'values' at line 2, column 25",
        ),
        (
            "struct Point { int x; };\nint main(void) {\n    struct Point point = {1, 2};\n    return point.x;\n}\n",
            "too many initializers for struct 'Point' at line 3, column 30",
        ),
        (
            "struct Point { int x; };\nint main(void) {\n    struct Point points[1] = {{1}, {2}};\n    return points[0].x;\n}\n",
            "too many initializers for struct array 'points' at line 3, column 36",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
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
fn rejects_missing_conditional_operator_operands_with_context() {
    let cases = [
        (
            "int main(void) {\n    return 1 ? : 2;\n}\n",
            "expected expression after '?' in conditional operator, found Colon at line 2, column 16",
        ),
        (
            "int main(void) {\n    return 1 ? 2 : ;\n}\n",
            "expected expression after ':' in conditional operator, found Semi at line 2, column 20",
        ),
        (
            "int main(void) {\n    return 1 ? 2 : );\n}\n",
            "expected expression after ':' in conditional operator, found RParen at line 2, column 20",
        ),
        (
            "int main(void) {\n    return 1 ? [ : 2;\n}\n",
            "expected expression after '?' in conditional operator, found LBracket at line 2, column 16",
        ),
        (
            "int main(void) {\n    return 1 ? .field : 2;\n}\n",
            "expected expression after '?' in conditional operator, found Dot at line 2, column 16",
        ),
        (
            "int main(void) {\n    return 1 ? {2} : 3;\n}\n",
            "expected expression after '?' in conditional operator, found LBrace at line 2, column 16",
        ),
        (
            "int main(void) {\n    return 1 ? 2 : [;\n}\n",
            "expected expression after ':' in conditional operator, found LBracket at line 2, column 20",
        ),
        (
            "int main(void) {\n    return 1 ? 2 : ->field;\n}\n",
            "expected expression after ':' in conditional operator, found Arrow at line 2, column 20",
        ),
        (
            "int main(void) {\n    return 1 ? 2 : {3};\n}\n",
            "expected expression after ':' in conditional operator, found LBrace at line 2, column 20",
        ),
        (
            "int main(void) {\n    return 1 ? return : 2;\n}\n",
            "expected expression after '?' in conditional operator before 'return' at line 2, column 16",
        ),
        (
            "int main(void) {\n    return 1 ? 2 : return;\n}\n",
            "expected expression after ':' in conditional operator before 'return' at line 2, column 20",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
}

#[test]
fn rejects_misplaced_operator_rhs_operands_with_context() {
    let cases = [
        (
            "int main(void) {\n    return 1 + [;\n}\n",
            "expected expression after binary operator '+', found LBracket at line 2, column 16",
        ),
        (
            "int main(void) {\n    int value = 1;\n    value = [;\n    return value;\n}\n",
            "expected expression after assignment operator '=', found LBracket at line 3, column 13",
        ),
        (
            "int main(void) {\n    int value = 1;\n    value += ?;\n    return value;\n}\n",
            "expected expression after assignment operator '+=', found Question at line 3, column 14",
        ),
        (
            "int main(void) {\n    return 1, [;\n}\n",
            "expected expression after comma operator, found LBracket at line 2, column 15",
        ),
        (
            "int main(void) {\n    return 1 + return;\n}\n",
            "expected expression after binary operator '+' before 'return' at line 2, column 16",
        ),
        (
            "int main(void) {\n    int value = 1;\n    value = return;\n    return value;\n}\n",
            "expected expression after assignment operator '=' before 'return' at line 3, column 13",
        ),
        (
            "int main(void) {\n    return 1, return;\n}\n",
            "expected expression after comma operator before 'return' at line 2, column 15",
        ),
        (
            "int main(void) {\n    return 1 + .field;\n}\n",
            "expected expression after binary operator '+', found Dot at line 2, column 16",
        ),
        (
            "int main(void) {\n    int value = 1;\n    value = ->field;\n    return value;\n}\n",
            "expected expression after assignment operator '=', found Arrow at line 3, column 13",
        ),
        (
            "int main(void) {\n    return 1, .field;\n}\n",
            "expected expression after comma operator, found Dot at line 2, column 15",
        ),
    ];

    for (program, expected) in cases {
        let err = interpret(program).unwrap_err();
        assert_eq!(err.to_string(), expected, "program: {program}");
    }
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

    assert_eq!(
        err.to_string(),
        "break outside loop or switch at line 2, column 5"
    );
}

#[test]
fn rejects_continue_outside_loops() {
    let program = include_str!("fixtures/invalid/continue_outside_loop.c");

    let err = interpret(program).unwrap_err();

    assert_eq!(err.to_string(), "continue outside loop at line 2, column 5");
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
