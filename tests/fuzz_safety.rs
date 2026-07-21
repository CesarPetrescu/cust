use std::panic;

use cust::interpret;

#[test]
fn generated_malformed_programs_do_not_panic() {
    let alphabet: Vec<char> =
        "int main() { return 0; }[]*&+-/%=!<>,;\n\t\"'\\abcdefghijklmnopqrstuvwxyz0123456789_@#$"
            .chars()
            .collect();
    let mut state = 0xC057_F022_u64;

    for case_index in 0..512 {
        let len = (next_u64(&mut state) % 192) as usize;
        let mut source = String::new();
        if case_index % 4 == 0 {
            source.push_str("int main() {\n");
        }
        for _ in 0..len {
            let ch = alphabet[(next_u64(&mut state) as usize) % alphabet.len()];
            source.push(ch);
        }
        if case_index % 4 == 0 {
            source.push_str("\n}\n");
        }

        let result = panic::catch_unwind(|| {
            let _ = interpret(&source);
        });

        assert!(
            result.is_ok(),
            "interpret panicked for generated case {case_index}: {source:?}"
        );
    }
}

#[test]
fn arbitrary_byte_inputs_do_not_panic_after_lossy_utf8_decoding() {
    let mut state = 0xC057_BA7E_u64;

    for case_index in 0..512 {
        let len = (next_u64(&mut state) % 256) as usize;
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push((next_u64(&mut state) & 0xff) as u8);
        }
        let source = String::from_utf8_lossy(&bytes).into_owned();

        let result = panic::catch_unwind(|| {
            let _ = interpret(&source);
        });

        assert!(
            result.is_ok(),
            "interpret panicked for arbitrary byte case {case_index}: {bytes:?}"
        );
    }
}

#[test]
fn generated_pointer_expression_values_match_model_without_panics() {
    const SEEDS: [u64; 3] = [0xC057_5101, 0xC057_5102, 0xC057_5103];
    let mut value_cases = 0;
    let mut cross_array_cases = 0;

    for mut state in SEEDS {
        for case_index in 0..48 {
            let expression = generate_pointer_expr(&mut state, 3);
            let model = expression.evaluate();
            if matches!(&model, Ok(pointer) if !(0..256).contains(&pointer.index)) {
                continue;
            }

            let (setup, expected) = match model {
                Ok(pointer) => {
                    value_cases += 1;
                    let value = match pointer.root {
                        ArrayRoot::Left => 41,
                        ArrayRoot::Right => 73,
                    };
                    (
                        format!("{}[{}] = {value};", pointer.root.name(), pointer.index),
                        ExpectedInterpretation::Value(value),
                    )
                }
                Err(ModelError::CrossArrayDifference) => {
                    cross_array_cases += 1;
                    (
                        String::new(),
                        ExpectedInterpretation::Error(
                            "cannot subtract pointers to different arrays",
                        ),
                    )
                }
            };
            let source = pointer_program("const int *", &expression.render(), &setup);

            assert_interpretation(
                &source,
                expected,
                &format!("seed {state:#x}, value case {case_index}, model {expression:?}"),
            );
        }
    }

    assert!(
        value_cases >= 40,
        "generated only {value_cases} value cases"
    );
    assert!(
        cross_array_cases >= 12,
        "generated only {cross_array_cases} cross-array cases"
    );
}

#[test]
fn generated_pointer_expression_const_diagnostics_match_model_without_panics() {
    const SEEDS: [u64; 3] = [0xC057_C011, 0xC057_C012, 0xC057_C013];
    let mut mutable_cases = 0;
    let mut const_cases = 0;

    for mut state in SEEDS {
        for case_index in 0..64 {
            let expression = generate_pointer_expr(&mut state, 3);
            let Ok(pointer) = expression.evaluate() else {
                continue;
            };
            if !(0..256).contains(&pointer.index) {
                continue;
            }

            let (setup, expected) = if expression.points_to_const() {
                const_cases += 1;
                (
                    String::new(),
                    ExpectedInterpretation::Error(
                        "cannot discard const qualifier from pointer target",
                    ),
                )
            } else {
                mutable_cases += 1;
                let value = match pointer.root {
                    ArrayRoot::Left => 43,
                    ArrayRoot::Right => 79,
                };
                (
                    format!("{}[{}] = {value};", pointer.root.name(), pointer.index),
                    ExpectedInterpretation::Value(value),
                )
            };
            let source = pointer_program("int *", &expression.render(), &setup);

            assert_interpretation(
                &source,
                expected,
                &format!("seed {state:#x}, const case {case_index}, model {expression:?}"),
            );
        }
    }

    assert!(
        mutable_cases >= 24,
        "generated only {mutable_cases} mutable-pointer cases"
    );
    assert!(
        const_cases >= 24,
        "generated only {const_cases} const-pointer cases"
    );
}

#[test]
fn aggregate_pointer_field_assignment_results_preserve_const_metadata() {
    let expressions = [
        "(view->points = points) + 1",
        "(((struct Cursor){points}).points = points) + 1",
    ];

    for expression in expressions {
        let source = format!(
            "struct Point {{ int value; }};\n\
             struct Cursor {{ const struct Point *points; }};\n\
             int main(void) {{\n\
             struct Point points[2] = {{{{3}}, {{7}}}};\n\
             struct Cursor cursor = {{points}};\n\
             struct Cursor *view = &cursor;\n\
             struct Point *result = {expression};\n\
             return result->value;\n\
             }}\n"
        );

        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            expression,
        );
    }
}

#[test]
fn generated_aggregate_pointer_expression_values_match_model_without_panics() {
    let mut state = 0xC057_A661_u64;
    let mut value_cases = 0;
    let mut bounds_cases = 0;
    let mut cross_array_cases = 0;

    for kind in AggregateKind::ALL {
        for route in AggregatePointerRoute::ALL {
            for case_index in 0..16 {
                let expression = generate_aggregate_pointer_expr(&mut state, kind, route, 3);
                let expected = match expression.evaluate() {
                    Ok(pointer) => {
                        value_cases += 1;
                        let value = match pointer.root {
                            AggregateRoot::Left => 41,
                            AggregateRoot::Right => 73,
                        };
                        ExpectedInterpretation::Value(value)
                    }
                    Err(AggregateModelError::Bounds(index)) => {
                        bounds_cases += 1;
                        ExpectedInterpretation::OwnedError(format!(
                            "struct array pointer index {index} out of bounds for length {AGGREGATE_ARRAY_LEN}"
                        ))
                    }
                    Err(AggregateModelError::CrossArrayDifference) => {
                        cross_array_cases += 1;
                        ExpectedInterpretation::Error(
                            "cannot subtract pointers to different arrays",
                        )
                    }
                };
                let source = aggregate_pointer_program(
                    kind.const_pointer_type(),
                    &expression.render(),
                    expression.evaluate().ok(),
                );

                assert_interpretation(
                    &source,
                    expected,
                    &format!(
                        "aggregate value case {case_index}, kind {kind:?}, route {route:?}, model {expression:?}"
                    ),
                );
            }
        }
    }

    assert!(
        value_cases >= 90,
        "generated only {value_cases} value cases"
    );
    assert!(
        bounds_cases >= 20,
        "generated only {bounds_cases} bounds cases"
    );
    assert!(
        cross_array_cases >= 12,
        "generated only {cross_array_cases} cross-array cases"
    );
}

#[test]
fn generated_aggregate_pointer_const_and_type_diagnostics_match_model_without_panics() {
    let mut state = 0xC057_A662_u64;
    let mut mutable_cases = 0;
    let mut const_cases = 0;

    for kind in AggregateKind::ALL {
        for route in AggregatePointerRoute::ALL {
            for case_index in 0..16 {
                let expression = generate_aggregate_pointer_expr(&mut state, kind, route, 2);
                let Ok(pointer) = expression.evaluate() else {
                    continue;
                };
                let expected = if expression.points_to_const() {
                    const_cases += 1;
                    ExpectedInterpretation::Error(
                        "cannot discard const qualifier from pointer target",
                    )
                } else {
                    mutable_cases += 1;
                    let value = match pointer.root {
                        AggregateRoot::Left => 41,
                        AggregateRoot::Right => 73,
                    };
                    ExpectedInterpretation::Value(value)
                };
                let source = aggregate_pointer_program(
                    kind.mutable_pointer_type(),
                    &expression.render(),
                    Some(pointer),
                );

                assert_interpretation(
                    &source,
                    expected,
                    &format!(
                        "aggregate const case {case_index}, kind {kind:?}, route {route:?}, model {expression:?}"
                    ),
                );
            }

            let expression = AggregatePointerExpr::Base(AggregatePointerBase {
                kind,
                root: AggregateRoot::Left,
                index: 1,
                points_to_const: false,
                route,
            });
            let source = aggregate_pointer_program(
                kind.other().mutable_pointer_type(),
                &expression.render(),
                None,
            );
            assert_interpretation(
                &source,
                ExpectedInterpretation::OwnedError(format!(
                    "cannot convert pointer to {} to pointer to {}",
                    kind.pointee_label(),
                    kind.other().pointee_label()
                )),
                &format!("aggregate type diagnostic, kind {kind:?}, route {route:?}"),
            );
        }
    }

    assert!(
        mutable_cases >= 30,
        "generated only {mutable_cases} mutable-pointer cases"
    );
    assert!(
        const_cases >= 40,
        "generated only {const_cases} const-pointer cases"
    );
}

#[test]
fn generated_embedded_aggregate_array_pointer_expressions_match_model_without_panics() {
    let mut state = 0xC057_E661_u64;
    let mut value_cases = 0;
    let mut bounds_cases = 0;
    let mut cross_owner_cases = 0;
    let mut const_cases = 0;

    for kind in AggregateKind::ALL {
        for container in EmbeddedContainerKind::ALL {
            for route in EmbeddedPointerRoute::ALL {
                for case_index in 0..12 {
                    let expression = generate_embedded_pointer_expr(&mut state, kind, route, 3);
                    let model = expression.evaluate();
                    let (result_type, expected) = match model {
                        Ok(pointer) if expression.points_to_const() => {
                            const_cases += 1;
                            (
                                kind.mutable_pointer_type(),
                                ExpectedInterpretation::Error(
                                    "cannot discard const qualifier from pointer target",
                                ),
                            )
                        }
                        Ok(pointer) => {
                            value_cases += 1;
                            (
                                kind.mutable_pointer_type(),
                                ExpectedInterpretation::Value(pointer.value()),
                            )
                        }
                        Err(EmbeddedModelError::Bounds { index, field }) => {
                            bounds_cases += 1;
                            (
                                kind.const_pointer_type(),
                                ExpectedInterpretation::OwnedError(format!(
                                    "struct array{} pointer index {index} out of bounds for length {EMBEDDED_ARRAY_LEN}",
                                    if field { " field" } else { "" }
                                )),
                            )
                        }
                        Err(EmbeddedModelError::CrossOwnerDifference) => {
                            cross_owner_cases += 1;
                            (
                                kind.const_pointer_type(),
                                ExpectedInterpretation::Error(
                                    "cannot subtract pointers to different arrays",
                                ),
                            )
                        }
                    };
                    let source = embedded_pointer_program(
                        kind,
                        container,
                        result_type,
                        &expression.render(container),
                    );

                    assert_interpretation(
                        &source,
                        expected,
                        &format!(
                            "embedded case {case_index}, kind {kind:?}, container {container:?}, route {route:?}, model {expression:?}"
                        ),
                    );
                }

                let expression = EmbeddedPointerExpr::Base(EmbeddedPointerBase {
                    kind,
                    root: EmbeddedRoot::Left,
                    index: 1,
                    points_to_const: false,
                    route,
                    literal_id: 0,
                });
                let source = embedded_pointer_program(
                    kind,
                    container,
                    kind.other().mutable_pointer_type(),
                    &expression.render(container),
                );
                assert_interpretation(
                    &source,
                    ExpectedInterpretation::OwnedError(format!(
                        "cannot convert pointer to {} to pointer to {}",
                        kind.pointee_label(),
                        kind.other().pointee_label()
                    )),
                    &format!(
                        "embedded type case, kind {kind:?}, container {container:?}, route {route:?}"
                    ),
                );
            }
        }
    }

    assert!(
        value_cases >= 70,
        "generated only {value_cases} value cases"
    );
    assert!(
        bounds_cases >= 60,
        "generated only {bounds_cases} bounds cases"
    );
    assert!(
        cross_owner_cases >= 30,
        "generated only {cross_owner_cases} cross-owner cases"
    );
    assert!(
        const_cases >= 100,
        "generated only {const_cases} const cases"
    );
}

#[test]
fn generated_scalar_array_field_pointer_expressions_match_model_without_panics() {
    let mut state = 0xC057_5CA1_u64;
    let mut value_cases = 0;
    let mut bounds_cases = 0;
    let mut cross_owner_cases = 0;
    let mut const_discard_cases = 0;
    let mut read_only_cases = 0;

    for kind in ScalarFieldKind::ALL {
        for container in EmbeddedContainerKind::ALL {
            for route in EmbeddedPointerRoute::ALL {
                for case_index in 0..12 {
                    let expression = generate_scalar_field_pointer_expr(&mut state, kind, route, 3);
                    let model = expression.evaluate();
                    let (result_type, write, expected) = match model {
                        Ok(_) if expression.points_to_const() => {
                            const_discard_cases += 1;
                            (
                                kind.mutable_pointer_type(),
                                None,
                                ExpectedInterpretation::Error(
                                    "cannot discard const qualifier from pointer target",
                                ),
                            )
                        }
                        Ok(pointer) => {
                            value_cases += 1;
                            (
                                kind.mutable_pointer_type(),
                                None,
                                ExpectedInterpretation::Value(pointer.value()),
                            )
                        }
                        Err(ScalarFieldModelError::Bounds(index)) => {
                            bounds_cases += 1;
                            (
                                kind.const_pointer_type(),
                                None,
                                ExpectedInterpretation::OwnedError(format!(
                                    "array pointer index {index} out of bounds for length {EMBEDDED_ARRAY_LEN}"
                                )),
                            )
                        }
                        Err(ScalarFieldModelError::CrossOwnerDifference) => {
                            cross_owner_cases += 1;
                            (
                                kind.const_pointer_type(),
                                None,
                                ExpectedInterpretation::Error(
                                    "cannot subtract pointers to different arrays",
                                ),
                            )
                        }
                    };
                    let source = scalar_field_pointer_program(
                        kind,
                        container,
                        result_type,
                        &expression.render(container),
                        write,
                    );

                    assert_interpretation(
                        &source,
                        expected,
                        &format!(
                            "scalar field case {case_index}, kind {kind:?}, container {container:?}, route {route:?}, model {expression:?}"
                        ),
                    );
                }

                let const_expression = ScalarFieldPointerExpr::Base(ScalarFieldPointerBase {
                    kind,
                    root: EmbeddedRoot::Left,
                    index: 1,
                    points_to_const: true,
                    route,
                    literal_id: 0,
                });
                let source = scalar_field_pointer_program(
                    kind,
                    container,
                    kind.const_pointer_type(),
                    &const_expression.render(container),
                    Some("*result = 1;"),
                );
                assert_interpretation(
                    &source,
                    ExpectedInterpretation::Error("cannot assign through pointer to const"),
                    &format!(
                        "scalar field read-only case, kind {kind:?}, container {container:?}, route {route:?}"
                    ),
                );
                read_only_cases += 1;

                let expression = ScalarFieldPointerExpr::Base(ScalarFieldPointerBase {
                    kind,
                    root: EmbeddedRoot::Left,
                    index: 1,
                    points_to_const: false,
                    route,
                    literal_id: 0,
                });
                let source = scalar_field_pointer_program(
                    kind,
                    container,
                    kind.other().mutable_pointer_type(),
                    &expression.render(container),
                    None,
                );
                assert_interpretation(
                    &source,
                    ExpectedInterpretation::OwnedError(format!(
                        "cannot convert pointer to {} to pointer to {}",
                        kind.pointee_label(),
                        kind.other().pointee_label()
                    )),
                    &format!(
                        "scalar field type case, kind {kind:?}, container {container:?}, route {route:?}"
                    ),
                );
            }
        }
    }

    assert!(
        value_cases >= 70,
        "generated only {value_cases} value cases"
    );
    assert!(
        bounds_cases >= 60,
        "generated only {bounds_cases} bounds cases"
    );
    assert!(
        cross_owner_cases >= 30,
        "generated only {cross_owner_cases} cross-owner cases"
    );
    assert!(
        const_discard_cases >= 100,
        "generated only {const_discard_cases} const-discard cases"
    );
    assert_eq!(read_only_cases, 64);
}

#[test]
fn generated_hidden_scalar_array_literal_pointers_match_model_without_panics() {
    let mut state = 0xC057_117E_u64;
    let mut value_cases = 0;
    let mut bounds_cases = 0;
    let mut difference_cases = 0;
    let mut equality_cases = 0;
    let mut ordering_cases = 0;

    for root in hidden_literal_roots() {
        for case_index in 0..24 {
            let expression = generate_hidden_literal_pointer_expr(&mut state, root, 3);
            let (operation, expected) = match expression.index {
                Err(index) => {
                    bounds_cases += 1;
                    (
                        "return 0;".to_string(),
                        ExpectedInterpretation::OwnedError(format!(
                            "array pointer index {index} out of bounds for length {HIDDEN_LITERAL_LEN}"
                        )),
                    )
                }
                Ok(index) => match next_u64(&mut state) % 4 {
                    0 => {
                        value_cases += 1;
                        (
                            "return *result;".to_string(),
                            ExpectedInterpretation::Value(root.value(index)),
                        )
                    }
                    1 => {
                        difference_cases += 1;
                        (
                            format!("return result - {};", root.name()),
                            ExpectedInterpretation::Value(index),
                        )
                    }
                    2 => {
                        equality_cases += 1;
                        (
                            format!("return result == ({} + {index});", root.name()),
                            ExpectedInterpretation::Value(1),
                        )
                    }
                    _ => {
                        ordering_cases += 1;
                        let compared_index =
                            (next_u64(&mut state) % HIDDEN_LITERAL_LEN as u64) as i64;
                        (
                            format!("return result >= ({} + {compared_index});", root.name()),
                            ExpectedInterpretation::Value(i64::from(index >= compared_index)),
                        )
                    }
                },
            };
            let source = hidden_literal_pointer_program(
                root.pointer_type(),
                &expression.rendered,
                &operation,
            );

            assert_interpretation(
                &source,
                expected,
                &format!(
                    "hidden literal case {case_index}, root {root:?}, expression {expression:?}"
                ),
            );
        }

        let other = root.other();
        for (operation, expected) in [
            (
                format!("return result - {};", other.name()),
                ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            ),
            (
                format!("return result < {};", other.name()),
                ExpectedInterpretation::Error("cannot compare pointers to different arrays"),
            ),
            (
                format!("return result == {};", other.name()),
                ExpectedInterpretation::Value(0),
            ),
        ] {
            let source =
                hidden_literal_pointer_program(root.pointer_type(), root.name(), &operation);
            assert_interpretation(
                &source,
                expected,
                &format!("hidden literal cross-root identity, root {root:?}"),
            );
        }
    }

    for kind in ScalarFieldKind::ALL {
        let const_root = HiddenLiteralRoot {
            kind,
            storage: HiddenLiteralStorage::ConstTypedef,
            side: HiddenLiteralSide::Left,
        };
        let source = hidden_literal_pointer_program(
            kind.mutable_pointer_type(),
            const_root.name(),
            "return *result;",
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("hidden const literal discard, kind {kind:?}"),
        );

        let source = hidden_literal_pointer_program(
            kind.const_pointer_type(),
            const_root.name(),
            "*result = 1; return *result;",
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("hidden const literal write, kind {kind:?}"),
        );

        let mutable_root = HiddenLiteralRoot {
            kind,
            storage: HiddenLiteralStorage::MutableCompound,
            side: HiddenLiteralSide::Left,
        };
        let source = hidden_literal_pointer_program(
            kind.other().mutable_pointer_type(),
            mutable_root.name(),
            "return *result;",
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                kind.other().pointee_label()
            )),
            &format!("hidden literal pointee type, kind {kind:?}"),
        );
    }

    let source = hidden_literal_pointer_program(
        "char *",
        "string_left + 1",
        "*result = 'z'; return *result;",
    );
    assert_interpretation(
        &source,
        ExpectedInterpretation::Error("cannot modify read-only array through pointer"),
        "hidden string literal write",
    );

    assert!(
        value_cases >= 15,
        "generated only {value_cases} value cases"
    );
    assert!(
        bounds_cases >= 25,
        "generated only {bounds_cases} bounds cases"
    );
    assert!(
        difference_cases >= 15,
        "generated only {difference_cases} difference cases"
    );
    assert!(
        equality_cases >= 15,
        "generated only {equality_cases} equality cases"
    );
    assert!(
        ordering_cases >= 15,
        "generated only {ordering_cases} ordering cases"
    );
}

#[test]
fn generated_hidden_aggregate_array_literal_pointers_match_model_without_panics() {
    let mut state = 0xC057_A117_u64;
    let mut value_cases = 0;
    let mut bounds_cases = 0;
    let mut difference_cases = 0;
    let mut equality_cases = 0;
    let mut ordering_cases = 0;

    for root in hidden_aggregate_literal_roots() {
        for case_index in 0..24 {
            let expression = generate_hidden_aggregate_literal_pointer_expr(&mut state, root, 3);
            let (operation, expected) = match expression.index {
                Err(index) => {
                    bounds_cases += 1;
                    (
                        "return 0;".to_string(),
                        ExpectedInterpretation::OwnedError(format!(
                            "struct array pointer index {index} out of bounds for length {HIDDEN_LITERAL_LEN}"
                        )),
                    )
                }
                Ok(index) => match next_u64(&mut state) % 4 {
                    0 => {
                        value_cases += 1;
                        (
                            "return result[0].value;".to_string(),
                            ExpectedInterpretation::Value(root.value(index)),
                        )
                    }
                    1 => {
                        difference_cases += 1;
                        (
                            format!("return result - {};", root.name()),
                            ExpectedInterpretation::Value(index),
                        )
                    }
                    2 => {
                        equality_cases += 1;
                        (
                            format!("return result == ({} + {index});", root.name()),
                            ExpectedInterpretation::Value(1),
                        )
                    }
                    _ => {
                        ordering_cases += 1;
                        let compared_index =
                            (next_u64(&mut state) % HIDDEN_LITERAL_LEN as u64) as i64;
                        (
                            format!("return result >= ({} + {compared_index});", root.name()),
                            ExpectedInterpretation::Value(i64::from(index >= compared_index)),
                        )
                    }
                },
            };
            let source = hidden_aggregate_literal_pointer_program(
                root.pointer_type(),
                &expression.rendered,
                &operation,
            );

            assert_interpretation(
                &source,
                expected,
                &format!(
                    "hidden aggregate literal case {case_index}, root {root:?}, expression {expression:?}"
                ),
            );
        }

        for other in [root.other_side(), root.other_storage()] {
            for (operation, expected) in [
                (
                    format!("return result - {};", other.name()),
                    ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
                ),
                (
                    format!("return result < {};", other.name()),
                    ExpectedInterpretation::Error("cannot compare pointers to different arrays"),
                ),
                (
                    format!("return result == {};", other.name()),
                    ExpectedInterpretation::Value(0),
                ),
            ] {
                let source = hidden_aggregate_literal_pointer_program(
                    root.pointer_type(),
                    root.name(),
                    &operation,
                );
                assert_interpretation(
                    &source,
                    expected,
                    &format!("hidden aggregate literal cross-root identity, root {root:?}"),
                );
            }
        }
    }

    for kind in AggregateKind::ALL {
        let const_root = HiddenAggregateLiteralRoot {
            kind,
            storage: HiddenAggregateLiteralStorage::ConstTypedef,
            side: HiddenLiteralSide::Left,
        };
        let source = hidden_aggregate_literal_pointer_program(
            kind.mutable_pointer_type(),
            const_root.name(),
            "return result->value;",
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("hidden const aggregate literal discard, kind {kind:?}"),
        );

        let source = hidden_aggregate_literal_pointer_program(
            kind.const_pointer_type(),
            const_root.name(),
            "result->value = 1; return result->value;",
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("hidden const aggregate literal write, kind {kind:?}"),
        );

        let mutable_root = HiddenAggregateLiteralRoot {
            kind,
            storage: HiddenAggregateLiteralStorage::MutableCompound,
            side: HiddenLiteralSide::Left,
        };
        let source = hidden_aggregate_literal_pointer_program(
            kind.mutable_pointer_type(),
            mutable_root.name(),
            "result->value = 91; return result->value;",
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Value(91),
            &format!("hidden mutable aggregate literal write, kind {kind:?}"),
        );

        let source = hidden_aggregate_literal_pointer_program(
            kind.other().mutable_pointer_type(),
            mutable_root.name(),
            "return result->value;",
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                kind.other().pointee_label()
            )),
            &format!("hidden aggregate literal pointee type, kind {kind:?}"),
        );
    }

    assert!(
        value_cases >= 12,
        "generated only {value_cases} value cases"
    );
    assert!(
        bounds_cases >= 20,
        "generated only {bounds_cases} bounds cases"
    );
    assert!(
        difference_cases >= 12,
        "generated only {difference_cases} difference cases"
    );
    assert!(
        equality_cases >= 12,
        "generated only {equality_cases} equality cases"
    );
    assert!(
        ordering_cases >= 12,
        "generated only {ordering_cases} ordering cases"
    );
}

#[test]
fn generated_pointer_return_function_results_match_model_without_panics() {
    let mut state = 0xC057_CA11_u64;
    let mut value_cases = 0;
    let mut bounds_cases = 0;
    let mut difference_cases = 0;
    let mut equality_cases = 0;
    let mut ordering_cases = 0;
    let mut indexed_address_cases = 0;

    for kind in ReturnedPointeeKind::ALL {
        for case_index in 0..48 {
            let expression = generate_returned_pointer_expr(&mut state, kind, 3);
            if expression.used_indexed_address {
                indexed_address_cases += 1;
            }
            let (result_type, operation, expected) = match expression.pointer {
                Err(index) => {
                    bounds_cases += 1;
                    (
                        kind.const_pointer_type(),
                        "return 0;".to_string(),
                        ExpectedInterpretation::OwnedError(format!(
                            "{} pointer index {index} out of bounds for length {RETURNED_ARRAY_LEN}",
                            kind.bounds_prefix()
                        )),
                    )
                }
                Ok(pointer) => {
                    let result_type = if pointer.points_to_const {
                        kind.const_pointer_type()
                    } else {
                        kind.mutable_pointer_type()
                    };
                    let (operation, expected) = match next_u64(&mut state) % 4 {
                        0 => {
                            value_cases += 1;
                            (
                                kind.read_result(),
                                ExpectedInterpretation::Value(pointer.value()),
                            )
                        }
                        1 => {
                            difference_cases += 1;
                            (
                                format!(
                                    "return result - {};",
                                    render_return_call(ReturnedModelPointer {
                                        index: 0,
                                        ..pointer
                                    })
                                ),
                                ExpectedInterpretation::Value(pointer.index),
                            )
                        }
                        2 => {
                            equality_cases += 1;
                            (
                                format!("return result == {};", render_return_call(pointer)),
                                ExpectedInterpretation::Value(1),
                            )
                        }
                        _ => {
                            ordering_cases += 1;
                            let compared_index =
                                (next_u64(&mut state) % RETURNED_ARRAY_LEN as u64) as i64;
                            (
                                format!(
                                    "return result >= {};",
                                    render_return_call(ReturnedModelPointer {
                                        index: compared_index,
                                        ..pointer
                                    })
                                ),
                                ExpectedInterpretation::Value(i64::from(
                                    pointer.index >= compared_index,
                                )),
                            )
                        }
                    };
                    (result_type, operation, expected)
                }
            };
            let source = returned_pointer_program(result_type, &expression.rendered, &operation);

            assert_interpretation(
                &source,
                expected,
                &format!(
                    "pointer return case {case_index}, kind {kind:?}, expression {expression:?}"
                ),
            );
        }

        for points_to_const in [false, true] {
            let left = ReturnedModelPointer {
                kind,
                root: ReturnedRoot::Left,
                index: 1,
                points_to_const,
            };
            let right = ReturnedModelPointer {
                root: ReturnedRoot::Right,
                ..left
            };
            for (operation, expected) in [
                (
                    format!("return result - {};", render_return_call(right)),
                    ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
                ),
                (
                    format!("return result < {};", render_return_call(right)),
                    ExpectedInterpretation::Error("cannot compare pointers to different arrays"),
                ),
                (
                    format!("return result == {};", render_return_call(right)),
                    ExpectedInterpretation::Value(0),
                ),
            ] {
                let source = returned_pointer_program(
                    if points_to_const {
                        kind.const_pointer_type()
                    } else {
                        kind.mutable_pointer_type()
                    },
                    &render_return_call(left),
                    &operation,
                );
                assert_interpretation(
                    &source,
                    expected,
                    &format!(
                        "pointer return cross-root identity, kind {kind:?}, const {points_to_const}"
                    ),
                );
            }
        }

        let const_pointer = ReturnedModelPointer {
            kind,
            root: ReturnedRoot::Left,
            index: 1,
            points_to_const: true,
        };
        let source = returned_pointer_program(
            kind.mutable_pointer_type(),
            &render_return_call(const_pointer),
            &kind.read_result(),
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("pointer return const discard, kind {kind:?}"),
        );

        let source = returned_pointer_program(
            kind.const_pointer_type(),
            &render_return_call(const_pointer),
            kind.write_result(),
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("pointer return const write, kind {kind:?}"),
        );

        let mutable_pointer = ReturnedModelPointer {
            points_to_const: false,
            ..const_pointer
        };
        let source = returned_pointer_program(
            kind.other().mutable_pointer_type(),
            &render_return_call(mutable_pointer),
            &kind.other().read_result(),
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                kind.other().pointee_label()
            )),
            &format!("pointer return pointee type, kind {kind:?}"),
        );
    }

    for (kind, expected) in [
        (
            ReturnedPointeeKind::Int,
            "pointer to out-of-scope variable 'local_int'",
        ),
        (
            ReturnedPointeeKind::Point,
            "pointer to out-of-scope variable 'local_point'",
        ),
        (
            ReturnedPointeeKind::Number,
            "pointer to out-of-scope variable 'local_number'",
        ),
    ] {
        let source = dangling_returned_pointer_program(kind);
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error(expected),
            &format!("pointer return out-of-scope diagnostic, kind {kind:?}"),
        );
    }

    assert!(
        value_cases >= 15,
        "generated only {value_cases} value cases"
    );
    assert!(
        bounds_cases >= 30,
        "generated only {bounds_cases} bounds cases"
    );
    assert!(
        difference_cases >= 15,
        "generated only {difference_cases} difference cases"
    );
    assert!(
        equality_cases >= 15,
        "generated only {equality_cases} equality cases"
    );
    assert!(
        ordering_cases >= 15,
        "generated only {ordering_cases} ordering cases"
    );
    assert!(
        indexed_address_cases >= 40,
        "generated only {indexed_address_cases} indexed-address cases"
    );
}

#[test]
fn generated_pointer_parameter_forwarding_results_match_model_without_panics() {
    let mut state = 0xC057_F04D_u64;
    let mut value_cases = 0;
    let mut bounds_cases = 0;
    let mut difference_cases = 0;
    let mut equality_cases = 0;
    let mut ordering_cases = 0;
    let mut indexed_address_cases = 0;
    let mut nested_forwarding_cases = 0;

    for kind in ReturnedPointeeKind::ALL {
        for case_index in 0..48 {
            let expression = generate_forwarded_pointer_expr(&mut state, kind, 3);
            if expression.used_indexed_address {
                indexed_address_cases += 1;
            }
            if expression.used_nested_forwarding {
                nested_forwarding_cases += 1;
            }
            let (result_type, operation, expected) = match expression.pointer {
                Err(index) => {
                    bounds_cases += 1;
                    (
                        kind.const_pointer_type(),
                        "return 0;".to_string(),
                        ExpectedInterpretation::OwnedError(format!(
                            "{} pointer index {index} out of bounds for length {RETURNED_ARRAY_LEN}",
                            kind.bounds_prefix()
                        )),
                    )
                }
                Ok(pointer) => {
                    let result_type = if pointer.points_to_const {
                        kind.const_pointer_type()
                    } else {
                        kind.mutable_pointer_type()
                    };
                    let (operation, expected) = match next_u64(&mut state) % 4 {
                        0 => {
                            value_cases += 1;
                            (
                                kind.read_result(),
                                ExpectedInterpretation::Value(pointer.value()),
                            )
                        }
                        1 => {
                            difference_cases += 1;
                            (
                                format!(
                                    "return result - {};",
                                    render_forwarded_call(
                                        ForwardedModelPointer {
                                            index: 0,
                                            ..pointer
                                        },
                                        false,
                                    )
                                ),
                                ExpectedInterpretation::Value(pointer.index),
                            )
                        }
                        2 => {
                            equality_cases += 1;
                            (
                                format!(
                                    "return result == {};",
                                    render_forwarded_call(pointer, true)
                                ),
                                ExpectedInterpretation::Value(1),
                            )
                        }
                        _ => {
                            ordering_cases += 1;
                            let compared_index =
                                (next_u64(&mut state) % RETURNED_ARRAY_LEN as u64) as i64;
                            (
                                format!(
                                    "return result >= {};",
                                    render_forwarded_call(
                                        ForwardedModelPointer {
                                            index: compared_index,
                                            ..pointer
                                        },
                                        false,
                                    )
                                ),
                                ExpectedInterpretation::Value(i64::from(
                                    pointer.index >= compared_index,
                                )),
                            )
                        }
                    };
                    (result_type, operation, expected)
                }
            };
            let source = forwarded_pointer_program(result_type, &expression.rendered, &operation);

            assert_interpretation(
                &source,
                expected,
                &format!(
                    "pointer forwarding case {case_index}, kind {kind:?}, expression {expression:?}"
                ),
            );
        }

        for storage_const in [false, true] {
            let left = ForwardedModelPointer {
                kind,
                root: ReturnedRoot::Left,
                index: 1,
                storage_const,
                points_to_const: storage_const,
            };
            let right = ForwardedModelPointer {
                root: ReturnedRoot::Right,
                ..left
            };
            for (operation, expected) in [
                (
                    format!("return result - {};", render_forwarded_call(right, true)),
                    ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
                ),
                (
                    format!("return result < {};", render_forwarded_call(right, false)),
                    ExpectedInterpretation::Error("cannot compare pointers to different arrays"),
                ),
                (
                    format!("return result == {};", render_forwarded_call(right, true)),
                    ExpectedInterpretation::Value(0),
                ),
            ] {
                let source = forwarded_pointer_program(
                    if left.points_to_const {
                        kind.const_pointer_type()
                    } else {
                        kind.mutable_pointer_type()
                    },
                    &render_forwarded_call(left, false),
                    &operation,
                );
                assert_interpretation(
                    &source,
                    expected,
                    &format!(
                        "pointer forwarding cross-root identity, kind {kind:?}, storage const {storage_const}"
                    ),
                );
            }
        }

        let promoted = ForwardedModelPointer {
            kind,
            root: ReturnedRoot::Left,
            index: 1,
            storage_const: false,
            points_to_const: true,
        };
        let source = forwarded_pointer_program(
            kind.mutable_pointer_type(),
            &render_forwarded_call(promoted, true),
            &kind.read_result(),
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("pointer forwarding promoted const discard, kind {kind:?}"),
        );

        let source = forwarded_pointer_program(
            kind.const_pointer_type(),
            &render_forwarded_call(promoted, false),
            kind.write_result(),
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("pointer forwarding promoted const write, kind {kind:?}"),
        );

        let const_storage = ForwardedModelPointer {
            storage_const: true,
            points_to_const: true,
            ..promoted
        };
        let invalid_argument = format!(
            "forward_{}({} + 1)",
            kind.function_suffix(),
            const_storage.storage_name()
        );
        let source = forwarded_pointer_program(
            kind.mutable_pointer_type(),
            &invalid_argument,
            &kind.read_result(),
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("pointer forwarding parameter const discard, kind {kind:?}"),
        );

        let mutable = ForwardedModelPointer {
            storage_const: false,
            points_to_const: false,
            ..promoted
        };
        let other = kind.other();
        let invalid_argument = format!(
            "forward_{}({} + 1)",
            other.function_suffix(),
            mutable.storage_name()
        );
        let source = forwarded_pointer_program(
            other.mutable_pointer_type(),
            &invalid_argument,
            &other.read_result(),
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                other.pointee_label()
            )),
            &format!("pointer forwarding parameter pointee type, kind {kind:?}"),
        );

        assert_interpretation(
            &forwarding_return_mismatch_program(kind, false),
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                other.pointee_label()
            )),
            &format!("pointer forwarding return pointee type, kind {kind:?}"),
        );
        assert_interpretation(
            &forwarding_return_mismatch_program(kind, true),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("pointer forwarding return const discard, kind {kind:?}"),
        );
    }

    for (kind, expected) in [
        (
            ReturnedPointeeKind::Int,
            "pointer to out-of-scope variable 'local_int'",
        ),
        (
            ReturnedPointeeKind::Point,
            "pointer to out-of-scope variable 'local_point'",
        ),
        (
            ReturnedPointeeKind::Number,
            "pointer to out-of-scope variable 'local_number'",
        ),
    ] {
        assert_interpretation(
            &dangling_forwarded_pointer_program(kind),
            ExpectedInterpretation::Error(expected),
            &format!("pointer forwarding out-of-scope diagnostic, kind {kind:?}"),
        );
    }

    assert!(
        value_cases >= 15,
        "generated only {value_cases} value cases"
    );
    assert!(
        bounds_cases >= 25,
        "generated only {bounds_cases} bounds cases"
    );
    assert!(
        difference_cases >= 15,
        "generated only {difference_cases} difference cases"
    );
    assert!(
        equality_cases >= 15,
        "generated only {equality_cases} equality cases"
    );
    assert!(
        ordering_cases >= 15,
        "generated only {ordering_cases} ordering cases"
    );
    assert!(
        indexed_address_cases >= 20,
        "generated only {indexed_address_cases} indexed-address cases"
    );
    assert!(
        nested_forwarding_cases >= 20,
        "generated only {nested_forwarding_cases} nested-forwarding cases"
    );
}

#[test]
fn generated_pointer_parameter_mutations_match_model_without_panics() {
    let mut state = 0xC057_5E7A_u64;
    let mut left_root_cases = 0;
    let mut right_root_cases = 0;
    let mut one_hop_cases = 0;
    let mut two_hop_cases = 0;

    for kind in ReturnedPointeeKind::ALL {
        for case_index in 0..40 {
            let root = if (next_u64(&mut state) >> 32) & 1 == 0 {
                left_root_cases += 1;
                ReturnedRoot::Left
            } else {
                right_root_cases += 1;
                ReturnedRoot::Right
            };
            let pointer = ForwardedModelPointer {
                kind,
                root,
                index: (next_u64(&mut state) % RETURNED_ARRAY_LEN as u64) as i64,
                storage_const: false,
                points_to_const: false,
            };
            let replacement = 130 + (next_u64(&mut state) % 80) as i64;
            let twice = (next_u64(&mut state) >> 32) & 1 == 0;
            if twice {
                two_hop_cases += 1;
            } else {
                one_hop_cases += 1;
            }
            let expected = replacement + pointer.index + if twice { 2 } else { 1 };
            let source = pointer_parameter_mutation_program(pointer, replacement, twice);

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "parameter mutation case {case_index}, kind {kind:?}, pointer {pointer:?}, replacement {replacement}, twice {twice}"
                ),
            );
        }
    }

    assert!(
        left_root_cases >= 40,
        "generated only {left_root_cases} left-root cases"
    );
    assert!(
        right_root_cases >= 40,
        "generated only {right_root_cases} right-root cases"
    );
    assert!(
        one_hop_cases >= 40,
        "generated only {one_hop_cases} one-hop cases"
    );
    assert!(
        two_hop_cases >= 40,
        "generated only {two_hop_cases} two-hop cases"
    );
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParameterAliasPattern {
    SameElement,
    SameRootDistinct,
    CrossRoot,
}

#[test]
fn generated_two_pointer_parameter_alias_mutations_match_model_without_panics() {
    let mut state = 0xC057_A11A_51A5_u64;
    let mut same_element_cases = 0;
    let mut same_root_distinct_cases = 0;
    let mut cross_root_cases = 0;
    let mut left_first_cases = 0;
    let mut right_first_cases = 0;

    for kind in ReturnedPointeeKind::ALL {
        for case_index in 0..30 {
            let first_root = if (next_u64(&mut state) >> 32) & 1 == 0 {
                left_first_cases += 1;
                ReturnedRoot::Left
            } else {
                right_first_cases += 1;
                ReturnedRoot::Right
            };
            let first_index = (next_u64(&mut state) % RETURNED_ARRAY_LEN as u64) as i64;
            let pattern = match case_index % 3 {
                0 => {
                    same_element_cases += 1;
                    ParameterAliasPattern::SameElement
                }
                1 => {
                    same_root_distinct_cases += 1;
                    ParameterAliasPattern::SameRootDistinct
                }
                _ => {
                    cross_root_cases += 1;
                    ParameterAliasPattern::CrossRoot
                }
            };
            let (second_root, second_index) = match pattern {
                ParameterAliasPattern::SameElement => (first_root, first_index),
                ParameterAliasPattern::SameRootDistinct => (
                    first_root,
                    (first_index + 1 + (next_u64(&mut state) % 5) as i64) % RETURNED_ARRAY_LEN,
                ),
                ParameterAliasPattern::CrossRoot => (
                    match first_root {
                        ReturnedRoot::Left => ReturnedRoot::Right,
                        ReturnedRoot::Right => ReturnedRoot::Left,
                    },
                    (next_u64(&mut state) % RETURNED_ARRAY_LEN as u64) as i64,
                ),
            };
            let first = ForwardedModelPointer {
                kind,
                root: first_root,
                index: first_index,
                storage_const: false,
                points_to_const: false,
            };
            let second = ForwardedModelPointer {
                root: second_root,
                index: second_index,
                ..first
            };
            let replacement = 130 + (next_u64(&mut state) % 80) as i64;
            let delta = 1 + (next_u64(&mut state) % 9) as i64;
            let expected =
                two_pointer_parameter_alias_mutation_expected(first, second, replacement, delta);
            let source =
                two_pointer_parameter_alias_mutation_program(first, second, replacement, delta);

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "two-parameter alias mutation case {case_index}, kind {kind:?}, pattern {pattern:?}, first {first:?}, second {second:?}, replacement {replacement}, delta {delta}"
                ),
            );
        }
    }

    assert_eq!(same_element_cases, 30);
    assert_eq!(same_root_distinct_cases, 30);
    assert_eq!(cross_root_cases, 30);
    assert!(
        left_first_cases >= 30,
        "generated only {left_first_cases} left-first cases"
    );
    assert!(
        right_first_cases >= 30,
        "generated only {right_first_cases} right-first cases"
    );
}

#[test]
fn generated_mixed_qualification_pointer_parameter_aliases_match_model_without_panics() {
    let mut state = 0xC057_C0A5_7A11_u64;
    let mut same_element_cases = 0;
    let mut same_root_distinct_cases = 0;
    let mut cross_root_cases = 0;
    let mut left_writer_cases = 0;
    let mut right_writer_cases = 0;

    for kind in ReturnedPointeeKind::ALL {
        for case_index in 0..30 {
            let writer_root = if (next_u64(&mut state) >> 32) & 1 == 0 {
                left_writer_cases += 1;
                ReturnedRoot::Left
            } else {
                right_writer_cases += 1;
                ReturnedRoot::Right
            };
            let writer_index = (next_u64(&mut state) % RETURNED_ARRAY_LEN as u64) as i64;
            let pattern = match case_index % 3 {
                0 => {
                    same_element_cases += 1;
                    ParameterAliasPattern::SameElement
                }
                1 => {
                    same_root_distinct_cases += 1;
                    ParameterAliasPattern::SameRootDistinct
                }
                _ => {
                    cross_root_cases += 1;
                    ParameterAliasPattern::CrossRoot
                }
            };
            let (reader_root, reader_index) = match pattern {
                ParameterAliasPattern::SameElement => (writer_root, writer_index),
                ParameterAliasPattern::SameRootDistinct => (
                    writer_root,
                    (writer_index + 1 + (next_u64(&mut state) % 5) as i64) % RETURNED_ARRAY_LEN,
                ),
                ParameterAliasPattern::CrossRoot => (
                    match writer_root {
                        ReturnedRoot::Left => ReturnedRoot::Right,
                        ReturnedRoot::Right => ReturnedRoot::Left,
                    },
                    (next_u64(&mut state) % RETURNED_ARRAY_LEN as u64) as i64,
                ),
            };
            let writer = ForwardedModelPointer {
                kind,
                root: writer_root,
                index: writer_index,
                storage_const: false,
                points_to_const: false,
            };
            let reader = ForwardedModelPointer {
                root: reader_root,
                index: reader_index,
                points_to_const: true,
                ..writer
            };
            let replacement = 130 + (next_u64(&mut state) % 80) as i64;
            let expected = mixed_qualification_alias_expected(writer, reader, replacement);
            let source = mixed_qualification_alias_program(writer, reader, replacement);

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "mixed-qualification alias case {case_index}, kind {kind:?}, pattern {pattern:?}, writer {writer:?}, reader {reader:?}, replacement {replacement}"
                ),
            );
        }
    }

    assert_eq!(same_element_cases, 30);
    assert_eq!(same_root_distinct_cases, 30);
    assert_eq!(cross_root_cases, 30);
    assert!(
        left_writer_cases >= 30,
        "generated only {left_writer_cases} left-writer cases"
    );
    assert!(
        right_writer_cases >= 30,
        "generated only {right_writer_cases} right-writer cases"
    );
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FieldAliasPattern {
    SameElement,
    SameFieldDistinctElement,
    DifferentField,
    DifferentOwner,
}

#[test]
fn generated_field_backed_mixed_qualification_parameter_aliases_match_model_without_panics() {
    let mut state = 0xC057_F13D_A11A_u64;
    let mut pattern_counts = [0; 4];
    let mut direct_writer_cases = 0;
    let mut arrow_writer_cases = 0;
    let mut direct_reader_cases = 0;
    let mut arrow_reader_cases = 0;

    for kind in FieldBackedPointeeKind::ALL {
        for case_index in 0..24 {
            let writer = FieldBackedModelPointer {
                kind,
                owner: if next_u64(&mut state) & 1 == 0 {
                    FieldBackedOwner::Left
                } else {
                    FieldBackedOwner::Right
                },
                field: if next_u64(&mut state) & 1 == 0 {
                    FieldBackedField::Primary
                } else {
                    FieldBackedField::Secondary
                },
                index: (next_u64(&mut state) % EMBEDDED_ARRAY_LEN as u64) as i64,
                route: if next_u64(&mut state) & 1 == 0 {
                    direct_writer_cases += 1;
                    FieldBackedRoute::Direct
                } else {
                    arrow_writer_cases += 1;
                    FieldBackedRoute::Arrow
                },
            };
            let pattern = match case_index % 4 {
                0 => FieldAliasPattern::SameElement,
                1 => FieldAliasPattern::SameFieldDistinctElement,
                2 => FieldAliasPattern::DifferentField,
                _ => FieldAliasPattern::DifferentOwner,
            };
            pattern_counts[case_index % 4] += 1;
            let (reader_owner, reader_field, reader_index) = match pattern {
                FieldAliasPattern::SameElement => (writer.owner, writer.field, writer.index),
                FieldAliasPattern::SameFieldDistinctElement => (
                    writer.owner,
                    writer.field,
                    (writer.index + 1 + (next_u64(&mut state) % 3) as i64) % EMBEDDED_ARRAY_LEN,
                ),
                FieldAliasPattern::DifferentField => {
                    (writer.owner, writer.field.other(), writer.index)
                }
                FieldAliasPattern::DifferentOwner => {
                    (writer.owner.other(), writer.field, writer.index)
                }
            };
            let reader = FieldBackedModelPointer {
                owner: reader_owner,
                field: reader_field,
                index: reader_index,
                route: if next_u64(&mut state) & 1 == 0 {
                    direct_reader_cases += 1;
                    FieldBackedRoute::Direct
                } else {
                    arrow_reader_cases += 1;
                    FieldBackedRoute::Arrow
                },
                ..writer
            };
            let replacement = 90 + (next_u64(&mut state) % 30) as i64;
            let expected = field_backed_mixed_qualification_expected(writer, reader, replacement);
            let source = field_backed_mixed_qualification_program(writer, reader, replacement);

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "field-backed mixed-qualification case {case_index}, kind {kind:?}, pattern {pattern:?}, writer {writer:?}, reader {reader:?}, replacement {replacement}"
                ),
            );
        }

        assert_interpretation(
            &field_backed_const_containing_object_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("field-backed const containing object, kind {kind:?}"),
        );
        assert_interpretation(
            &field_backed_bounds_program(kind, FieldBackedRoute::Arrow),
            ExpectedInterpretation::OwnedError(format!(
                "{} pointer index 5 out of bounds for length {EMBEDDED_ARRAY_LEN}",
                kind.bounds_prefix()
            )),
            &format!("field-backed parameter bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &field_backed_type_mismatch_program(kind),
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                kind.other().pointee_label()
            )),
            &format!("field-backed parameter pointee type, kind {kind:?}"),
        );
    }

    assert_eq!(pattern_counts, [24, 24, 24, 24]);
    assert!(direct_writer_cases >= 30);
    assert!(arrow_writer_cases >= 30);
    assert!(direct_reader_cases >= 30);
    assert!(arrow_reader_cases >= 30);
}

#[test]
fn generated_field_backed_pointer_return_and_forwarding_results_match_model_without_panics() {
    let mut state = 0xC057_F13D_F04D_u64;
    let mut direct_argument_cases = 0;
    let mut arrow_argument_cases = 0;
    let mut mutable_result_cases = 0;
    let mut const_result_cases = 0;
    let mut one_hop_cases = 0;
    let mut two_hop_cases = 0;
    let mut arithmetic_cases = 0;
    let mut conditional_cases = 0;
    let mut comma_cases = 0;
    let mut indexed_address_cases = 0;

    for kind in FieldBackedPointeeKind::ALL {
        for case_index in 0..32 {
            let expression = generate_field_backed_forwarded_expr(
                &mut state,
                kind,
                case_index,
                &mut direct_argument_cases,
                &mut arrow_argument_cases,
            );
            if expression.pointer.points_to_const {
                const_result_cases += 1;
            } else {
                mutable_result_cases += 1;
            }
            if expression.used_two_hop {
                two_hop_cases += 1;
            } else {
                one_hop_cases += 1;
            }
            arithmetic_cases += usize::from(expression.used_arithmetic);
            conditional_cases += usize::from(expression.used_conditional);
            comma_cases += usize::from(expression.used_comma);
            indexed_address_cases += usize::from(expression.used_indexed_address);

            let pointer = expression.pointer;
            let result_type = if pointer.points_to_const {
                kind.const_pointer_type()
            } else {
                kind.mutable_pointer_type()
            };
            let (operation, expected) = match case_index % 4 {
                0 => (format!("return {};", kind.read("result")), pointer.value()),
                1 => (
                    format!("return result - {};", pointer.storage.field_storage()),
                    pointer.storage.index,
                ),
                2 => {
                    let same_storage = FieldBackedModelPointer {
                        route: pointer.storage.route.other(),
                        ..pointer.storage
                    };
                    (
                        format!(
                            "return result == {};",
                            render_field_backed_forward_call(
                                FieldBackedQualifiedPointer {
                                    storage: same_storage,
                                    ..pointer
                                },
                                case_index & 1 == 0,
                            )
                        ),
                        1,
                    )
                }
                _ => {
                    let compared_index = (next_u64(&mut state) % EMBEDDED_ARRAY_LEN as u64) as i64;
                    let compared = FieldBackedQualifiedPointer {
                        storage: FieldBackedModelPointer {
                            index: compared_index,
                            ..pointer.storage
                        },
                        ..pointer
                    };
                    (
                        format!(
                            "return result >= {};",
                            render_field_backed_forward_call(compared, case_index & 1 == 0)
                        ),
                        i64::from(pointer.storage.index >= compared_index),
                    )
                }
            };
            let source = field_backed_forwarding_program(
                kind,
                &result_type,
                &expression.rendered,
                &operation,
            );

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "field-backed forwarding case {case_index}, kind {kind:?}, expression {expression:?}"
                ),
            );
        }

        let left_primary = FieldBackedQualifiedPointer {
            storage: FieldBackedModelPointer {
                kind,
                owner: FieldBackedOwner::Left,
                field: FieldBackedField::Primary,
                index: 1,
                route: FieldBackedRoute::Direct,
            },
            points_to_const: false,
        };
        let left_secondary = FieldBackedQualifiedPointer {
            storage: FieldBackedModelPointer {
                field: FieldBackedField::Secondary,
                route: FieldBackedRoute::Arrow,
                ..left_primary.storage
            },
            ..left_primary
        };
        let right_primary = FieldBackedQualifiedPointer {
            storage: FieldBackedModelPointer {
                owner: FieldBackedOwner::Right,
                route: FieldBackedRoute::Arrow,
                ..left_primary.storage
            },
            ..left_primary
        };
        for (other, operator, expected) in [
            (
                left_secondary,
                "-",
                ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            ),
            (
                right_primary,
                "<",
                ExpectedInterpretation::Error("cannot compare pointers to different arrays"),
            ),
            (left_secondary, "==", ExpectedInterpretation::Value(0)),
        ] {
            let operation = format!(
                "return result {operator} {};",
                render_field_backed_forward_call(other, true)
            );
            let source = field_backed_forwarding_program(
                kind,
                &kind.mutable_pointer_type(),
                &render_field_backed_forward_call(left_primary, false),
                &operation,
            );
            assert_interpretation(
                &source,
                expected,
                &format!(
                    "field-backed forwarding cross-field/root diagnostic, kind {kind:?}, operator {operator}"
                ),
            );
        }

        assert_interpretation(
            &field_backed_forwarding_bounds_program(kind),
            ExpectedInterpretation::OwnedError(format!(
                "{} pointer index 5 out of bounds for length {EMBEDDED_ARRAY_LEN}",
                kind.bounds_prefix()
            )),
            &format!("field-backed forwarding bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &field_backed_forwarding_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("field-backed forwarding const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &field_backed_forwarding_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("field-backed forwarding const write, kind {kind:?}"),
        );
        assert_interpretation(
            &field_backed_forwarding_const_container_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("field-backed forwarding const container, kind {kind:?}"),
        );
        assert_interpretation(
            &field_backed_forwarding_type_mismatch_program(kind),
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                kind.other().pointee_label()
            )),
            &format!("field-backed forwarding pointee type, kind {kind:?}"),
        );
    }

    assert!(direct_argument_cases >= 50);
    assert!(arrow_argument_cases >= 50);
    assert!(mutable_result_cases >= 50);
    assert!(const_result_cases >= 50);
    assert!(one_hop_cases >= 50);
    assert!(two_hop_cases >= 50);
    assert!(arithmetic_cases >= 40);
    assert!(conditional_cases >= 40);
    assert!(comma_cases >= 40);
    assert!(indexed_address_cases >= 40);
}

#[test]
fn generated_field_backed_returned_pointer_alias_mutations_match_model_without_panics() {
    let mut state = 0xC057_F13D_5A11_u64;
    let mut pattern_counts = [0; 4];
    let mut reader_aliases_writer_cases = 0;
    let mut reader_aliases_second_writer_cases = 0;
    let mut one_hop_calls = 0;
    let mut two_hop_calls = 0;
    let mut direct_arguments = 0;
    let mut arrow_arguments = 0;

    for kind in FieldBackedPointeeKind::ALL {
        for case_index in 0..24 {
            let writer = FieldBackedModelPointer {
                kind,
                owner: if next_u64(&mut state) & 1 == 0 {
                    FieldBackedOwner::Left
                } else {
                    FieldBackedOwner::Right
                },
                field: if next_u64(&mut state) & 1 == 0 {
                    FieldBackedField::Primary
                } else {
                    FieldBackedField::Secondary
                },
                index: (next_u64(&mut state) % EMBEDDED_ARRAY_LEN as u64) as i64,
                route: if case_index & 1 == 0 {
                    direct_arguments += 1;
                    FieldBackedRoute::Direct
                } else {
                    arrow_arguments += 1;
                    FieldBackedRoute::Arrow
                },
            };
            let pattern = match case_index % 4 {
                0 => FieldAliasPattern::SameElement,
                1 => FieldAliasPattern::SameFieldDistinctElement,
                2 => FieldAliasPattern::DifferentField,
                _ => FieldAliasPattern::DifferentOwner,
            };
            pattern_counts[case_index % 4] += 1;
            let (second_owner, second_field, second_index) = match pattern {
                FieldAliasPattern::SameElement => (writer.owner, writer.field, writer.index),
                FieldAliasPattern::SameFieldDistinctElement => (
                    writer.owner,
                    writer.field,
                    (writer.index + 1 + (next_u64(&mut state) % 3) as i64) % EMBEDDED_ARRAY_LEN,
                ),
                FieldAliasPattern::DifferentField => {
                    (writer.owner, writer.field.other(), writer.index)
                }
                FieldAliasPattern::DifferentOwner => {
                    (writer.owner.other(), writer.field, writer.index)
                }
            };
            let second_writer = FieldBackedModelPointer {
                owner: second_owner,
                field: second_field,
                index: second_index,
                route: if case_index & 2 == 0 {
                    direct_arguments += 1;
                    FieldBackedRoute::Direct
                } else {
                    arrow_arguments += 1;
                    FieldBackedRoute::Arrow
                },
                ..writer
            };
            let reader_storage = if case_index & 4 == 0 {
                reader_aliases_writer_cases += 1;
                writer
            } else {
                reader_aliases_second_writer_cases += 1;
                second_writer
            };
            let reader = FieldBackedModelPointer {
                route: if case_index & 4 == 0 {
                    direct_arguments += 1;
                    FieldBackedRoute::Direct
                } else {
                    arrow_arguments += 1;
                    FieldBackedRoute::Arrow
                },
                ..reader_storage
            };
            let writer_twice = case_index & 1 == 0;
            let second_writer_twice = case_index & 2 == 0;
            let reader_twice = case_index & 4 == 0;
            one_hop_calls += usize::from(!writer_twice)
                + usize::from(!second_writer_twice)
                + usize::from(!reader_twice);
            two_hop_calls += usize::from(writer_twice)
                + usize::from(second_writer_twice)
                + usize::from(reader_twice);
            let replacement = 90 + (next_u64(&mut state) % 30) as i64;
            let delta = 1 + (next_u64(&mut state) % 9) as i64;
            let expected = field_backed_returned_alias_mutation_expected(
                writer,
                second_writer,
                reader,
                replacement,
                delta,
            );
            let source = field_backed_returned_alias_mutation_program(
                writer,
                second_writer,
                reader,
                replacement,
                delta,
                writer_twice,
                second_writer_twice,
                reader_twice,
            );

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "field-backed returned alias mutation case {case_index}, kind {kind:?}, pattern {pattern:?}, writer {writer:?}, second_writer {second_writer:?}, reader {reader:?}, replacement {replacement}, delta {delta}"
                ),
            );
        }

        assert_interpretation(
            &field_backed_forwarding_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("returned alias mutation const write, kind {kind:?}"),
        );
        assert_interpretation(
            &field_backed_forwarding_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("returned alias mutation const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &field_backed_forwarding_bounds_program(kind),
            ExpectedInterpretation::OwnedError(format!(
                "{} pointer index 5 out of bounds for length {EMBEDDED_ARRAY_LEN}",
                kind.bounds_prefix()
            )),
            &format!("returned alias mutation bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &field_backed_forwarding_type_mismatch_program(kind),
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                kind.other().pointee_label()
            )),
            &format!("returned alias mutation pointee type, kind {kind:?}"),
        );
    }

    assert_eq!(pattern_counts, [24, 24, 24, 24]);
    assert_eq!(reader_aliases_writer_cases, 48);
    assert_eq!(reader_aliases_second_writer_cases, 48);
    assert_eq!(one_hop_calls, 144);
    assert_eq!(two_hop_calls, 144);
    assert_eq!(direct_arguments, 144);
    assert_eq!(arrow_arguments, 144);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExtendedFieldAliasPattern {
    SameElement,
    SameFieldDistinctElement,
    DifferentPath,
    DifferentOwner,
}

#[test]
fn generated_nested_anonymous_field_backed_returned_pointer_alias_mutations_match_model_without_panics()
 {
    let mut state = 0xC057_F13D_AA55_u64;
    let mut pattern_counts = [0; 4];
    let mut nested_arguments = 0;
    let mut anonymous_arguments = 0;
    let mut direct_arguments = 0;
    let mut arrow_arguments = 0;
    let mut one_hop_calls = 0;
    let mut two_hop_calls = 0;

    for kind in FieldBackedPointeeKind::ALL {
        for case_index in 0..24 {
            let writer = ExtendedFieldBackedPointer {
                kind,
                path: if (case_index / 2) & 1 == 0 {
                    nested_arguments += 1;
                    ExtendedFieldBackedPath::Nested
                } else {
                    anonymous_arguments += 1;
                    ExtendedFieldBackedPath::Anonymous
                },
                owner: if next_u64(&mut state) & 1 == 0 {
                    FieldBackedOwner::Left
                } else {
                    FieldBackedOwner::Right
                },
                field: if next_u64(&mut state) & 1 == 0 {
                    FieldBackedField::Primary
                } else {
                    FieldBackedField::Secondary
                },
                index: (next_u64(&mut state) % EMBEDDED_ARRAY_LEN as u64) as i64,
                route: if case_index & 1 == 0 {
                    direct_arguments += 1;
                    FieldBackedRoute::Direct
                } else {
                    arrow_arguments += 1;
                    FieldBackedRoute::Arrow
                },
            };
            let pattern = match case_index % 4 {
                0 => ExtendedFieldAliasPattern::SameElement,
                1 => ExtendedFieldAliasPattern::SameFieldDistinctElement,
                2 => ExtendedFieldAliasPattern::DifferentPath,
                _ => ExtendedFieldAliasPattern::DifferentOwner,
            };
            pattern_counts[case_index % 4] += 1;
            let (second_path, second_owner, second_field, second_index) = match pattern {
                ExtendedFieldAliasPattern::SameElement => {
                    (writer.path, writer.owner, writer.field, writer.index)
                }
                ExtendedFieldAliasPattern::SameFieldDistinctElement => (
                    writer.path,
                    writer.owner,
                    writer.field,
                    (writer.index + 1 + (next_u64(&mut state) % 3) as i64) % EMBEDDED_ARRAY_LEN,
                ),
                ExtendedFieldAliasPattern::DifferentPath => (
                    writer.path.other(),
                    writer.owner,
                    writer.field,
                    writer.index,
                ),
                ExtendedFieldAliasPattern::DifferentOwner => (
                    writer.path,
                    writer.owner.other(),
                    writer.field,
                    writer.index,
                ),
            };
            let second_writer = ExtendedFieldBackedPointer {
                path: second_path,
                owner: second_owner,
                field: second_field,
                index: second_index,
                route: if case_index & 2 == 0 {
                    direct_arguments += 1;
                    FieldBackedRoute::Direct
                } else {
                    arrow_arguments += 1;
                    FieldBackedRoute::Arrow
                },
                ..writer
            };
            let reader_storage = if case_index & 4 == 0 {
                writer
            } else {
                second_writer
            };
            let reader = ExtendedFieldBackedPointer {
                route: if case_index & 4 == 0 {
                    direct_arguments += 1;
                    FieldBackedRoute::Direct
                } else {
                    arrow_arguments += 1;
                    FieldBackedRoute::Arrow
                },
                ..reader_storage
            };
            let writer_twice = case_index & 1 == 0;
            let second_writer_twice = case_index & 2 == 0;
            let reader_twice = case_index & 4 == 0;
            one_hop_calls += usize::from(!writer_twice)
                + usize::from(!second_writer_twice)
                + usize::from(!reader_twice);
            two_hop_calls += usize::from(writer_twice)
                + usize::from(second_writer_twice)
                + usize::from(reader_twice);
            let replacement = 90 + (next_u64(&mut state) % 30) as i64;
            let delta = 1 + (next_u64(&mut state) % 9) as i64;
            let expected = extended_field_backed_alias_mutation_expected(
                writer,
                second_writer,
                reader,
                replacement,
                delta,
            );
            let source = extended_field_backed_alias_mutation_program(
                writer,
                second_writer,
                reader,
                replacement,
                delta,
                writer_twice,
                second_writer_twice,
                reader_twice,
            );

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "nested/anonymous returned alias mutation case {case_index}, kind {kind:?}, pattern {pattern:?}, writer {writer:?}, second_writer {second_writer:?}, reader {reader:?}"
                ),
            );
        }

        for path in ExtendedFieldBackedPath::ALL {
            assert_interpretation(
                &extended_field_backed_const_container_program(kind, path),
                ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
                &format!("nested/anonymous const container, kind {kind:?}, path {path:?}"),
            );
            assert_interpretation(
                &extended_field_backed_bounds_program(kind, path),
                ExpectedInterpretation::OwnedError(format!(
                    "{} pointer index 5 out of bounds for length {EMBEDDED_ARRAY_LEN}",
                    kind.bounds_prefix()
                )),
                &format!("nested/anonymous bounds, kind {kind:?}, path {path:?}"),
            );
        }
        assert_interpretation(
            &extended_field_backed_cross_path_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("nested/anonymous cross-path identity, kind {kind:?}"),
        );
        assert_interpretation(
            &extended_field_backed_type_mismatch_program(kind),
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                kind.other().pointee_label()
            )),
            &format!("nested/anonymous pointee type, kind {kind:?}"),
        );
    }

    assert_eq!(pattern_counts, [24, 24, 24, 24]);
    assert_eq!(nested_arguments, 48);
    assert_eq!(anonymous_arguments, 48);
    assert_eq!(direct_arguments, 144);
    assert_eq!(arrow_arguments, 144);
    assert_eq!(one_hop_calls, 144);
    assert_eq!(two_hop_calls, 144);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExtendedLiteralAliasPattern {
    SameElement,
    SameRootDistinctElement,
    DifferentField,
    DifferentPath,
    DifferentLiteralRoot,
    DifferentPathAndLiteralRoot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AdjustedParameterRelation {
    SameElement,
    SameArrayDistinctElement,
    DifferentOuterElement,
    DifferentFieldPath,
    DifferentOwner,
    RootVersusField,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AdjustedParameterOperation {
    Read,
    WriteThroughAddress,
    Equality,
    Difference,
    Ordering,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LiteralAdjustedParameterRelation {
    SameElement,
    SameArrayDistinctElement,
    DifferentField,
    DifferentLiteralRoot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DirectLiteralAdjustedParameterRelation {
    SameElement,
    SameArrayDistinctElement,
    SeparateLiteralRoot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WrappedDirectLiteralRoute {
    ConditionalTrue,
    ConditionalFalse,
    Comma,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WrappedDirectLiteralOffsetRoute {
    PointerPlusOne,
    OnePlusPointer,
    IndexedAddress,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CapturedOuterForwardRoute {
    OneHopBeforeOffset,
    TwoHopBeforeOffset,
    OneHopAfterOffset,
    TwoHopAfterOffset,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PostForwardWrapperPlacement {
    BeforeOffset,
    AfterOffset,
}

#[test]
fn generated_adjusted_aggregate_parameter_embedded_field_pointers_match_model_without_panics() {
    let mut relation_counts = [0; 6];
    let mut operation_counts = [0; 5];
    let mut root_argument_cases = 0;
    let mut field_argument_cases = 0;
    let mut direct_routes = 0;
    let mut reverse_routes = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for (relation_index, relation) in AdjustedParameterRelation::ALL.into_iter().enumerate() {
            for (operation_index, operation) in
                AdjustedParameterOperation::ALL.into_iter().enumerate()
            {
                relation_counts[relation_index] += 1;
                operation_counts[operation_index] += 1;
                let first = adjusted_parameter_first_pointer(kind, relation_index, operation_index);
                let second = adjusted_parameter_related_pointer(first, relation);
                for pointer in [first, second] {
                    if pointer.storage.is_root() {
                        root_argument_cases += 1;
                    } else {
                        field_argument_cases += 1;
                    }
                    match pointer.route {
                        AdjustedParameterRoute::Direct => direct_routes += 1,
                        AdjustedParameterRoute::Reverse => reverse_routes += 1,
                    }
                }
                let (source, expected) = adjusted_parameter_model_program(first, second, operation);

                assert_interpretation(
                    &source,
                    expected,
                    &format!(
                        "adjusted aggregate parameter case, kind {kind:?}, relation {relation:?}, operation {operation:?}, first {first:?}, second {second:?}"
                    ),
                );
            }
        }

        for storage in [
            AdjustedParameterStorage::RootLeft,
            AdjustedParameterStorage::LeftPrimary,
        ] {
            assert_interpretation(
                &adjusted_parameter_inner_bounds_program(kind, storage),
                ExpectedInterpretation::Error(kind.inner_bounds_error()),
                &format!("adjusted parameter inner bounds, kind {kind:?}, storage {storage:?}"),
            );
            assert_interpretation(
                &adjusted_parameter_outer_bounds_program(kind, storage),
                ExpectedInterpretation::OwnedError(format!(
                    "struct array{} pointer index 2 out of bounds for length 2",
                    if storage.is_root() { "" } else { " field" }
                )),
                &format!("adjusted parameter outer bounds, kind {kind:?}, storage {storage:?}"),
            );
            assert_interpretation(
                &adjusted_parameter_const_ancestor_program(kind, storage),
                ExpectedInterpretation::Error("cannot assign through pointer to const"),
                &format!("adjusted parameter const ancestor, kind {kind:?}, storage {storage:?}"),
            );
        }
    }

    for storage in [
        AdjustedParameterStorage::RootLeft,
        AdjustedParameterStorage::LeftPrimary,
    ] {
        assert_interpretation(
            &adjusted_parameter_aggregate_type_mismatch_program(storage),
            ExpectedInterpretation::Error(
                "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
            ),
            &format!("adjusted parameter aggregate type mismatch, storage {storage:?}"),
        );
    }

    assert_eq!(relation_counts, [10; 6]);
    assert_eq!(operation_counts, [12; 5]);
    assert!(root_argument_cases >= 30);
    assert!(field_argument_cases >= 60);
    assert_eq!(direct_routes, 60);
    assert_eq!(reverse_routes, 60);
}

#[test]
fn generated_adjusted_parameter_alias_mutations_match_model_without_panics() {
    let mut relation_counts = [0; 6];
    let mut path_counts = [0; 5];
    let mut direct_routes = 0;
    let mut reverse_routes = 0;
    let mut one_hop_routes = 0;
    let mut two_hop_routes = 0;
    let mut root_arguments = 0;
    let mut field_arguments = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for case_index in 0..24 {
            let relation_index = case_index % AdjustedParameterRelation::ALL.len();
            let relation = AdjustedParameterRelation::ALL[relation_index];
            relation_counts[relation_index] += 1;
            let first = adjusted_parameter_nested_first_pointer(kind, relation_index, case_index);
            let second = adjusted_parameter_related_pointer(first, relation);
            let reader = if case_index & 1 == 0 { first } else { second };
            let two_hop = [
                case_index & 1 == 0,
                case_index & 2 == 0,
                case_index & 4 == 0,
            ];
            let replacement = 151 + case_index as i64;
            let delta = 1 + (case_index % 7) as i64;

            for pointer in [first, second, reader] {
                path_counts[pointer.storage.path_index()] += 1;
                match pointer.route {
                    AdjustedParameterRoute::Direct => direct_routes += 1,
                    AdjustedParameterRoute::Reverse => reverse_routes += 1,
                }
                if pointer.storage.is_root() {
                    root_arguments += 1;
                } else {
                    field_arguments += 1;
                }
            }
            one_hop_routes += two_hop.iter().filter(|twice| !**twice).count();
            two_hop_routes += two_hop.iter().filter(|twice| **twice).count();

            let source = adjusted_parameter_alias_mutation_program(
                first,
                second,
                reader,
                replacement,
                delta,
                two_hop,
            );
            let expected = adjusted_parameter_alias_mutation_expected(
                first,
                second,
                reader,
                replacement,
                delta,
            );
            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "adjusted parameter alias mutation case {case_index}, kind {kind:?}, relation {relation:?}, first {first:?}, second {second:?}, reader {reader:?}"
                ),
            );
        }
    }

    assert_eq!(relation_counts, [8; 6]);
    assert_eq!(direct_routes, 72);
    assert_eq!(reverse_routes, 72);
    assert_eq!(one_hop_routes, 72);
    assert_eq!(two_hop_routes, 72);
    assert!(root_arguments >= 30);
    assert!(field_arguments >= 90);
    assert!(path_counts.iter().all(|count| *count >= 8));
}

#[test]
fn generated_nested_adjusted_parameter_diagnostics_remain_exact_without_panics() {
    let nested_storages = [
        AdjustedParameterStorage::NamedLeftPrimary,
        AdjustedParameterStorage::AnonymousLeftPrimary,
        AdjustedParameterStorage::UnionLeftPrimary,
    ];

    for kind in AdjustedParameterFieldKind::ALL {
        for storage in nested_storages {
            assert_interpretation(
                &adjusted_parameter_inner_bounds_program(kind, storage),
                ExpectedInterpretation::Error(kind.inner_bounds_error()),
                &format!(
                    "nested adjusted parameter inner bounds, kind {kind:?}, storage {storage:?}"
                ),
            );
            assert_interpretation(
                &adjusted_parameter_outer_bounds_program(kind, storage),
                ExpectedInterpretation::Error(
                    "struct array field pointer index 2 out of bounds for length 2",
                ),
                &format!(
                    "nested adjusted parameter outer bounds, kind {kind:?}, storage {storage:?}"
                ),
            );
            assert_interpretation(
                &adjusted_parameter_nested_const_ancestor_program(kind, storage),
                ExpectedInterpretation::Error("cannot assign through pointer to const"),
                &format!(
                    "nested adjusted parameter const ancestor, kind {kind:?}, storage {storage:?}"
                ),
            );
        }

        let first = AdjustedParameterPointer {
            kind,
            storage: AdjustedParameterStorage::NamedLeftPrimary,
            outer: 0,
            inner: 1,
            route: AdjustedParameterRoute::Direct,
        };
        let second = AdjustedParameterPointer {
            storage: AdjustedParameterStorage::AnonymousLeftPrimary,
            route: AdjustedParameterRoute::Reverse,
            ..first
        };
        let (difference_source, difference_expected) =
            adjusted_parameter_model_program(first, second, AdjustedParameterOperation::Difference);
        assert_interpretation(
            &difference_source,
            difference_expected,
            &format!("nested adjusted parameter cross-path difference, kind {kind:?}"),
        );
    }

    for storage in nested_storages {
        assert_interpretation(
            &adjusted_parameter_aggregate_type_mismatch_program(storage),
            ExpectedInterpretation::Error(
                "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
            ),
            &format!("nested adjusted parameter aggregate type mismatch, storage {storage:?}"),
        );
    }
}

#[test]
fn generated_compound_literal_outer_array_adjusted_parameter_aliases_match_model_without_panics() {
    let mut relation_counts = [0; 4];
    let mut path_counts = [0; 3];
    let mut direct_routes = 0;
    let mut reverse_routes = 0;
    let mut one_hop_routes = 0;
    let mut two_hop_routes = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for case_index in 0..24 {
            let relation_index = case_index % LiteralAdjustedParameterRelation::ALL.len();
            let relation = LiteralAdjustedParameterRelation::ALL[relation_index];
            relation_counts[relation_index] += 1;
            let first = literal_adjusted_parameter_first_pointer(kind, case_index);
            let second = literal_adjusted_parameter_related_pointer(first, relation);
            let reader = if case_index & 2 == 0 { first } else { second };
            let two_hop = [
                case_index & 1 == 0,
                case_index & 2 == 0,
                case_index & 4 == 0,
            ];
            let replacement = 211 + case_index as i64;
            let delta = 1 + (case_index % 5) as i64;

            for pointer in [first, second, reader] {
                path_counts[pointer.storage.literal_path_index()] += 1;
                match pointer.route {
                    AdjustedParameterRoute::Direct => direct_routes += 1,
                    AdjustedParameterRoute::Reverse => reverse_routes += 1,
                }
            }
            one_hop_routes += two_hop.iter().filter(|twice| !**twice).count();
            two_hop_routes += two_hop.iter().filter(|twice| **twice).count();

            let source = literal_adjusted_parameter_alias_mutation_program(
                first,
                second,
                reader,
                replacement,
                delta,
                two_hop,
            );
            let expected = adjusted_parameter_alias_mutation_expected(
                first,
                second,
                reader,
                replacement,
                delta,
            ) + 19;
            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "compound-literal outer adjusted-parameter case {case_index}, kind {kind:?}, relation {relation:?}, first {first:?}, second {second:?}, reader {reader:?}"
                ),
            );
        }

        for storage in [
            AdjustedParameterStorage::NamedLeftPrimary,
            AdjustedParameterStorage::AnonymousLeftPrimary,
            AdjustedParameterStorage::UnionLeftPrimary,
        ] {
            assert_interpretation(
                &literal_adjusted_parameter_inner_bounds_program(kind, storage),
                ExpectedInterpretation::Error(kind.inner_bounds_error()),
                &format!(
                    "compound-literal outer adjusted-parameter inner bounds, kind {kind:?}, storage {storage:?}"
                ),
            );
            assert_interpretation(
                &literal_adjusted_parameter_outer_bounds_program(kind, storage),
                ExpectedInterpretation::Error(
                    "struct array field pointer index 2 out of bounds for length 2",
                ),
                &format!(
                    "compound-literal outer adjusted-parameter outer bounds, kind {kind:?}, storage {storage:?}"
                ),
            );
        }
        assert_interpretation(
            &literal_adjusted_parameter_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("compound-literal outer adjusted-parameter const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &literal_adjusted_parameter_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!(
                "compound-literal outer adjusted-parameter cross-root identity, kind {kind:?}"
            ),
        );
    }

    for storage in [
        AdjustedParameterStorage::NamedLeftPrimary,
        AdjustedParameterStorage::AnonymousLeftPrimary,
        AdjustedParameterStorage::UnionLeftPrimary,
    ] {
        assert_interpretation(
            &literal_adjusted_parameter_type_mismatch_program(storage),
            ExpectedInterpretation::Error(
                "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
            ),
            &format!(
                "compound-literal outer adjusted-parameter aggregate type mismatch, storage {storage:?}"
            ),
        );
    }

    assert_eq!(relation_counts, [12; 4]);
    assert!(path_counts.iter().all(|count| *count >= 36));
    assert_eq!(direct_routes, 72);
    assert_eq!(reverse_routes, 72);
    assert_eq!(one_hop_routes, 72);
    assert_eq!(two_hop_routes, 72);
}

#[test]
fn generated_direct_aggregate_array_literal_adjusted_parameter_aliases_match_model_without_panics()
{
    let mut relation_counts = [0; 3];
    let mut direct_routes = 0;
    let mut reverse_routes = 0;
    let mut one_hop_routes = 0;
    let mut two_hop_routes = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for case_index in 0..24 {
            let relation_index = case_index % DirectLiteralAdjustedParameterRelation::ALL.len();
            let relation = DirectLiteralAdjustedParameterRelation::ALL[relation_index];
            relation_counts[relation_index] += 1;
            let first = direct_literal_adjusted_parameter_first_pointer(kind, case_index);
            let second = direct_literal_adjusted_parameter_related_pointer(first, relation);
            let reader = if case_index & 2 == 0 { first } else { second };
            let two_hop = [
                case_index & 1 == 0,
                case_index & 2 == 0,
                case_index & 4 == 0,
            ];
            let replacement = 251 + case_index as i64;
            let delta = 1 + (case_index % 7) as i64;

            for pointer in [first, second, reader] {
                match pointer.route {
                    AdjustedParameterRoute::Direct => direct_routes += 1,
                    AdjustedParameterRoute::Reverse => reverse_routes += 1,
                }
            }
            one_hop_routes += two_hop.iter().filter(|twice| !**twice).count();
            two_hop_routes += two_hop.iter().filter(|twice| **twice).count();

            let source = direct_literal_adjusted_parameter_alias_mutation_program(
                first,
                second,
                reader,
                replacement,
                delta,
                two_hop,
            );
            let expected = adjusted_parameter_alias_mutation_expected(
                first,
                second,
                reader,
                replacement,
                delta,
            ) + 19;
            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "direct aggregate-array literal adjusted-parameter case {case_index}, kind {kind:?}, relation {relation:?}, first {first:?}, second {second:?}, reader {reader:?}"
                ),
            );
        }

        assert_interpretation(
            &direct_literal_adjusted_parameter_inner_bounds_program(kind),
            ExpectedInterpretation::Error(kind.inner_bounds_error()),
            &format!("direct aggregate-array literal inner bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &direct_literal_adjusted_parameter_outer_bounds_program(kind),
            ExpectedInterpretation::Error(
                "struct array pointer index 2 out of bounds for length 2",
            ),
            &format!("direct aggregate-array literal outer bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &direct_literal_adjusted_parameter_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("direct aggregate-array literal const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &direct_literal_adjusted_parameter_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("direct aggregate-array literal const write, kind {kind:?}"),
        );
        assert_interpretation(
            &direct_literal_adjusted_parameter_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("direct aggregate-array literal cross-root identity, kind {kind:?}"),
        );
    }

    assert_interpretation(
        &direct_literal_adjusted_parameter_type_mismatch_program(),
        ExpectedInterpretation::Error(
            "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
        ),
        "direct aggregate-array literal aggregate type mismatch",
    );

    assert_eq!(relation_counts, [16; 3]);
    assert_eq!(direct_routes, 72);
    assert_eq!(reverse_routes, 72);
    assert_eq!(one_hop_routes, 72);
    assert_eq!(two_hop_routes, 72);
}

#[test]
fn generated_wrapped_direct_aggregate_array_literal_adjusted_parameter_aliases_match_model_without_panics()
 {
    let mut relation_counts = [0; 3];
    let mut route_counts = [0; 3];
    let mut one_hop_routes = 0;
    let mut two_hop_routes = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for case_index in 0..18 {
            let relation_index = case_index % DirectLiteralAdjustedParameterRelation::ALL.len();
            let relation = DirectLiteralAdjustedParameterRelation::ALL[relation_index];
            let route = WrappedDirectLiteralRoute::ALL[(case_index / 3) % 3];
            relation_counts[relation_index] += 1;
            route_counts[route.index()] += 1;
            let first = direct_literal_adjusted_parameter_first_pointer(kind, case_index);
            let second = direct_literal_adjusted_parameter_related_pointer(first, relation);
            let reader = if case_index & 2 == 0 { first } else { second };
            let two_hop = [
                case_index & 1 == 0,
                case_index & 2 == 0,
                case_index & 4 == 0,
            ];
            one_hop_routes += two_hop.iter().filter(|twice| !**twice).count();
            two_hop_routes += two_hop.iter().filter(|twice| **twice).count();
            let replacement = 401 + case_index as i64;
            let delta = 1 + (case_index % 7) as i64;
            let source = wrapped_direct_literal_adjusted_parameter_alias_program(
                first,
                second,
                reader,
                replacement,
                delta,
                two_hop,
                route,
            );
            let expected = adjusted_parameter_alias_mutation_expected(
                first,
                second,
                reader,
                replacement,
                delta,
            ) + 19;

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "wrapped direct literal case {case_index}, kind {kind:?}, relation {relation:?}, route {route:?}"
                ),
            );
        }

        for route in WrappedDirectLiteralRoute::ALL {
            assert_interpretation(
                &wrapped_direct_literal_const_parameter_program(route),
                ExpectedInterpretation::Value(17),
                &format!("wrapped const direct literal route {route:?}"),
            );
        }
        assert_interpretation(
            &wrapped_direct_literal_inner_bounds_program(kind),
            ExpectedInterpretation::Error(kind.inner_bounds_error()),
            &format!("wrapped direct literal inner bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_direct_literal_outer_bounds_program(kind),
            ExpectedInterpretation::Error(
                "struct array pointer index 2 out of bounds for length 2",
            ),
            &format!("wrapped direct literal outer bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_direct_literal_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("wrapped direct literal const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_direct_literal_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("wrapped direct literal const write, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_direct_literal_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("wrapped direct literal cross-root identity, kind {kind:?}"),
        );
    }

    assert_interpretation(
        &wrapped_direct_literal_type_mismatch_program(),
        ExpectedInterpretation::Error(
            "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
        ),
        "wrapped direct literal aggregate type mismatch",
    );

    assert_eq!(relation_counts, [12; 3]);
    assert_eq!(route_counts, [12; 3]);
    assert_eq!(one_hop_routes, 50);
    assert_eq!(two_hop_routes, 58);
}

#[test]
fn generated_wrapped_offset_direct_aggregate_array_literal_adjusted_parameter_aliases_match_model_without_panics()
 {
    let mut relation_counts = [0; 3];
    let mut wrapper_counts = [0; 3];
    let mut offset_counts = [0; 3];
    let mut one_hop_routes = 0;
    let mut two_hop_routes = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for case_index in 0..27 {
            let relation_index = case_index % DirectLiteralAdjustedParameterRelation::ALL.len();
            let relation = DirectLiteralAdjustedParameterRelation::ALL[relation_index];
            let wrapper = WrappedDirectLiteralRoute::ALL[(case_index / 3) % 3];
            let offset = WrappedDirectLiteralOffsetRoute::ALL[(case_index / 9) % 3];
            relation_counts[relation_index] += 1;
            wrapper_counts[wrapper.index()] += 1;
            offset_counts[offset.index()] += 1;
            let first = direct_literal_adjusted_parameter_first_pointer(kind, case_index);
            let second = direct_literal_adjusted_parameter_related_pointer(first, relation);
            let reader = if case_index & 2 == 0 { first } else { second };
            let two_hop = [
                case_index & 1 == 0,
                case_index & 2 == 0,
                case_index & 4 == 0,
            ];
            one_hop_routes += two_hop.iter().filter(|twice| !**twice).count();
            two_hop_routes += two_hop.iter().filter(|twice| **twice).count();
            let replacement = 601 + case_index as i64;
            let delta = 1 + (case_index % 7) as i64;
            let source = wrapped_offset_direct_literal_adjusted_parameter_alias_program(
                first,
                second,
                reader,
                replacement,
                delta,
                two_hop,
                wrapper,
                offset,
            );
            let expected = adjusted_parameter_alias_mutation_expected(
                first,
                second,
                reader,
                replacement,
                delta,
            ) + 19;

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "wrapped offset direct literal case {case_index}, kind {kind:?}, relation {relation:?}, wrapper {wrapper:?}, offset {offset:?}"
                ),
            );
        }

        assert_interpretation(
            &wrapped_offset_direct_literal_inner_bounds_program(kind),
            ExpectedInterpretation::Error(kind.inner_bounds_error()),
            &format!("wrapped offset direct literal inner bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_offset_direct_literal_upper_bounds_program(kind),
            ExpectedInterpretation::Error(
                "struct array pointer index 3 out of bounds for length 3",
            ),
            &format!("wrapped offset direct literal upper bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_offset_direct_literal_lower_bounds_program(kind),
            ExpectedInterpretation::Error(
                "struct array pointer index -1 out of bounds for length 3",
            ),
            &format!("wrapped offset direct literal lower bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_offset_direct_literal_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("wrapped offset direct literal const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_offset_direct_literal_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("wrapped offset direct literal const write, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_offset_direct_literal_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("wrapped offset direct literal cross-root identity, kind {kind:?}"),
        );
    }

    for wrapper in WrappedDirectLiteralRoute::ALL {
        for offset in WrappedDirectLiteralOffsetRoute::ALL {
            assert_interpretation(
                &wrapped_offset_direct_literal_const_parameter_program(wrapper, offset),
                ExpectedInterpretation::Value(17),
                &format!("wrapped offset const direct literal route {wrapper:?}, {offset:?}"),
            );
        }
    }
    assert_interpretation(
        &wrapped_offset_direct_literal_type_mismatch_program(),
        ExpectedInterpretation::Error(
            "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
        ),
        "wrapped offset direct literal aggregate type mismatch",
    );

    assert_eq!(relation_counts, [18; 3]);
    assert_eq!(wrapper_counts, [18; 3]);
    assert_eq!(offset_counts, [18; 3]);
    assert_eq!(one_hop_routes, 76);
    assert_eq!(two_hop_routes, 86);
}

#[test]
fn generated_outer_forwarded_wrapped_direct_aggregate_array_literal_adjusted_parameter_aliases_match_model_without_panics()
 {
    let mut relation_counts = [0; 3];
    let mut wrapper_counts = [0; 3];
    let mut offset_counts = [0; 3];
    let mut outer_forward_counts = [0; 4];
    let mut inner_one_hop_routes = 0;
    let mut inner_two_hop_routes = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for case_index in 0..108 {
            let relation_index = case_index % DirectLiteralAdjustedParameterRelation::ALL.len();
            let relation = DirectLiteralAdjustedParameterRelation::ALL[relation_index];
            let wrapper = WrappedDirectLiteralRoute::ALL[(case_index / 3) % 3];
            let offset = WrappedDirectLiteralOffsetRoute::ALL[(case_index / 9) % 3];
            let outer_forward = CapturedOuterForwardRoute::ALL[(case_index / 27) % 4];
            relation_counts[relation_index] += 1;
            wrapper_counts[wrapper.index()] += 1;
            offset_counts[offset.index()] += 1;
            outer_forward_counts[outer_forward.index()] += 1;

            let first = direct_literal_adjusted_parameter_first_pointer(kind, case_index);
            let second = direct_literal_adjusted_parameter_related_pointer(first, relation);
            let reader = if case_index & 2 == 0 { first } else { second };
            let inner_two_hop = [
                case_index & 1 == 0,
                case_index & 2 == 0,
                case_index & 4 == 0,
            ];
            inner_one_hop_routes += inner_two_hop.iter().filter(|twice| !**twice).count();
            inner_two_hop_routes += inner_two_hop.iter().filter(|twice| **twice).count();
            let replacement = 1001 + case_index as i64;
            let delta = 1 + (case_index % 7) as i64;
            let source =
                outer_forwarded_wrapped_offset_direct_literal_adjusted_parameter_alias_program(
                    first,
                    second,
                    reader,
                    replacement,
                    delta,
                    inner_two_hop,
                    wrapper,
                    offset,
                    outer_forward,
                );
            let expected = adjusted_parameter_alias_mutation_expected(
                first,
                second,
                reader,
                replacement,
                delta,
            ) + 19;

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "outer-forwarded wrapped direct literal case {case_index}, kind {kind:?}, relation {relation:?}, wrapper {wrapper:?}, offset {offset:?}, outer forward {outer_forward:?}"
                ),
            );
        }

        assert_interpretation(
            &outer_forwarded_wrapped_direct_literal_inner_bounds_program(kind),
            ExpectedInterpretation::Error(kind.inner_bounds_error()),
            &format!("outer-forwarded wrapped direct literal inner bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &outer_forwarded_wrapped_direct_literal_outer_bounds_program(kind, 2),
            ExpectedInterpretation::Error(
                "struct array pointer index 3 out of bounds for length 3",
            ),
            &format!("outer-forwarded wrapped direct literal upper bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &outer_forwarded_wrapped_direct_literal_outer_bounds_program(kind, -2),
            ExpectedInterpretation::Error(
                "struct array pointer index -1 out of bounds for length 3",
            ),
            &format!("outer-forwarded wrapped direct literal lower bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &outer_forwarded_wrapped_direct_literal_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("outer-forwarded wrapped direct literal const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &outer_forwarded_wrapped_direct_literal_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("outer-forwarded wrapped direct literal const write, kind {kind:?}"),
        );
        assert_interpretation(
            &outer_forwarded_wrapped_direct_literal_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("outer-forwarded wrapped direct literal cross-root, kind {kind:?}"),
        );
    }

    for wrapper in WrappedDirectLiteralRoute::ALL {
        for offset in WrappedDirectLiteralOffsetRoute::ALL {
            for outer_forward in CapturedOuterForwardRoute::ALL {
                assert_interpretation(
                    &outer_forwarded_wrapped_direct_literal_const_parameter_program(
                        wrapper,
                        offset,
                        outer_forward,
                    ),
                    ExpectedInterpretation::Value(17),
                    &format!(
                        "outer-forwarded wrapped const direct literal route {wrapper:?}, {offset:?}, {outer_forward:?}"
                    ),
                );
            }
        }
    }
    assert_interpretation(
        &outer_forwarded_wrapped_direct_literal_type_mismatch_program(),
        ExpectedInterpretation::Error(
            "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
        ),
        "outer-forwarded wrapped direct literal aggregate type mismatch",
    );
    assert_interpretation(
        &outer_forwarded_wrapped_direct_literal_lifetime_program(),
        ExpectedInterpretation::Error(
            "pointer to out-of-scope variable '__cust_compound_aggregate_array#0'",
        ),
        "outer-forwarded wrapped direct literal lifetime",
    );

    assert_eq!(relation_counts, [72; 3]);
    assert_eq!(wrapper_counts, [72; 3]);
    assert_eq!(offset_counts, [72; 3]);
    assert_eq!(outer_forward_counts, [54; 4]);
    assert_eq!(inner_one_hop_routes + inner_two_hop_routes, 648);
    assert!(inner_one_hop_routes >= 270);
    assert!(inner_two_hop_routes >= 270);
}

#[test]
fn generated_post_forward_wrapped_direct_aggregate_array_literal_adjusted_parameter_aliases_match_model_without_panics()
 {
    let mut relation_counts = [0; 3];
    let mut inner_wrapper_counts = [0; 3];
    let mut post_wrapper_counts = [0; 3];
    let mut offset_counts = [0; 3];
    let mut placement_counts = [0; 2];
    let mut outer_hop_counts = [0; 2];
    let mut inner_one_hop_routes = 0;
    let mut inner_two_hop_routes = 0;
    let mut case_index = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for (relation_index, relation) in DirectLiteralAdjustedParameterRelation::ALL
            .into_iter()
            .enumerate()
        {
            for inner_wrapper in WrappedDirectLiteralRoute::ALL {
                for post_wrapper in WrappedDirectLiteralRoute::ALL {
                    for offset in WrappedDirectLiteralOffsetRoute::ALL {
                        for placement in PostForwardWrapperPlacement::ALL {
                            for outer_two_hop in [false, true] {
                                relation_counts[relation_index] += 1;
                                inner_wrapper_counts[inner_wrapper.index()] += 1;
                                post_wrapper_counts[post_wrapper.index()] += 1;
                                offset_counts[offset.index()] += 1;
                                placement_counts[placement.index()] += 1;
                                outer_hop_counts[usize::from(outer_two_hop)] += 1;

                                let first = direct_literal_adjusted_parameter_first_pointer(
                                    kind, case_index,
                                );
                                let second = direct_literal_adjusted_parameter_related_pointer(
                                    first, relation,
                                );
                                let reader = if case_index & 2 == 0 { first } else { second };
                                let inner_two_hop = [
                                    case_index & 1 == 0,
                                    case_index & 2 == 0,
                                    case_index & 4 == 0,
                                ];
                                inner_one_hop_routes +=
                                    inner_two_hop.iter().filter(|twice| !**twice).count();
                                inner_two_hop_routes +=
                                    inner_two_hop.iter().filter(|twice| **twice).count();
                                let replacement = 1201 + case_index as i64;
                                let delta = 1 + (case_index % 7) as i64;
                                let source = post_forward_wrapped_direct_literal_adjusted_parameter_alias_program(
                                    first,
                                    second,
                                    reader,
                                    replacement,
                                    delta,
                                    inner_two_hop,
                                    inner_wrapper,
                                    post_wrapper,
                                    offset,
                                    placement,
                                    outer_two_hop,
                                );
                                let expected = adjusted_parameter_alias_mutation_expected(
                                    first,
                                    second,
                                    reader,
                                    replacement,
                                    delta,
                                ) + 19;

                                assert_interpretation(
                                    &source,
                                    ExpectedInterpretation::Value(expected),
                                    &format!(
                                        "post-forward wrapped direct literal case {case_index}, kind {kind:?}, relation {relation:?}, inner wrapper {inner_wrapper:?}, post wrapper {post_wrapper:?}, offset {offset:?}, placement {placement:?}, two hop {outer_two_hop}"
                                    ),
                                );
                                case_index += 1;
                            }
                        }
                    }
                }
            }
        }

        assert_interpretation(
            &post_forward_wrapped_direct_literal_inner_bounds_program(kind),
            ExpectedInterpretation::Error(kind.inner_bounds_error()),
            &format!("post-forward wrapped direct literal inner bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_forward_wrapped_direct_literal_outer_bounds_program(kind, 2),
            ExpectedInterpretation::Error(
                "struct array pointer index 3 out of bounds for length 3",
            ),
            &format!("post-forward wrapped direct literal upper bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_forward_wrapped_direct_literal_outer_bounds_program(kind, -2),
            ExpectedInterpretation::Error(
                "struct array pointer index -1 out of bounds for length 3",
            ),
            &format!("post-forward wrapped direct literal lower bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_forward_wrapped_direct_literal_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("post-forward wrapped direct literal const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &post_forward_wrapped_direct_literal_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("post-forward wrapped direct literal const write, kind {kind:?}"),
        );
        assert_interpretation(
            &post_forward_wrapped_direct_literal_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("post-forward wrapped direct literal cross-root, kind {kind:?}"),
        );
    }

    for inner_wrapper in WrappedDirectLiteralRoute::ALL {
        for post_wrapper in WrappedDirectLiteralRoute::ALL {
            for offset in WrappedDirectLiteralOffsetRoute::ALL {
                for placement in PostForwardWrapperPlacement::ALL {
                    for outer_two_hop in [false, true] {
                        assert_interpretation(
                            &post_forward_wrapped_direct_literal_const_parameter_program(
                                inner_wrapper,
                                post_wrapper,
                                offset,
                                placement,
                                outer_two_hop,
                            ),
                            ExpectedInterpretation::Value(17),
                            &format!(
                                "post-forward wrapped const direct literal route {inner_wrapper:?}, {post_wrapper:?}, {offset:?}, {placement:?}, two hop {outer_two_hop}"
                            ),
                        );
                    }
                }
            }
        }
    }
    assert_interpretation(
        &post_forward_wrapped_direct_literal_type_mismatch_program(),
        ExpectedInterpretation::Error(
            "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
        ),
        "post-forward wrapped direct literal aggregate type mismatch",
    );
    assert_interpretation(
        &post_forward_wrapped_direct_literal_lifetime_program(),
        ExpectedInterpretation::Error(
            "pointer to out-of-scope variable '__cust_compound_aggregate_array#0'",
        ),
        "post-forward wrapped direct literal lifetime",
    );

    assert_eq!(case_index, 648);
    assert_eq!(relation_counts, [216; 3]);
    assert_eq!(inner_wrapper_counts, [216; 3]);
    assert_eq!(post_wrapper_counts, [216; 3]);
    assert_eq!(offset_counts, [216; 3]);
    assert_eq!(placement_counts, [324; 2]);
    assert_eq!(outer_hop_counts, [324; 2]);
    assert_eq!(inner_one_hop_routes + inner_two_hop_routes, 1944);
    assert!(inner_one_hop_routes >= 900);
    assert!(inner_two_hop_routes >= 900);
}

#[test]
fn generated_post_wrapper_reforwarded_direct_aggregate_array_literal_adjusted_parameter_aliases_match_model_without_panics()
 {
    let mut relation_counts = [0; 3];
    let mut inner_wrapper_counts = [0; 3];
    let mut post_wrapper_counts = [0; 3];
    let mut offset_counts = [0; 3];
    let mut outer_hop_counts = [0; 2];
    let mut reforward_hop_counts = [0; 2];
    let mut reforward_placement_counts = [0; 2];
    let mut inner_one_hop_routes = 0;
    let mut inner_two_hop_routes = 0;
    let mut case_index = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for (relation_index, relation) in DirectLiteralAdjustedParameterRelation::ALL
            .into_iter()
            .enumerate()
        {
            for inner_wrapper in WrappedDirectLiteralRoute::ALL {
                for post_wrapper in WrappedDirectLiteralRoute::ALL {
                    for offset in WrappedDirectLiteralOffsetRoute::ALL {
                        for outer_two_hop in [false, true] {
                            for reforward_two_hop in [false, true] {
                                for reforward_placement in PostForwardWrapperPlacement::ALL {
                                    relation_counts[relation_index] += 1;
                                    inner_wrapper_counts[inner_wrapper.index()] += 1;
                                    post_wrapper_counts[post_wrapper.index()] += 1;
                                    offset_counts[offset.index()] += 1;
                                    outer_hop_counts[usize::from(outer_two_hop)] += 1;
                                    reforward_hop_counts[usize::from(reforward_two_hop)] += 1;
                                    reforward_placement_counts[reforward_placement.index()] += 1;

                                    let first = direct_literal_adjusted_parameter_first_pointer(
                                        kind, case_index,
                                    );
                                    let second = direct_literal_adjusted_parameter_related_pointer(
                                        first, relation,
                                    );
                                    let reader = if case_index & 2 == 0 { first } else { second };
                                    let inner_two_hop = [
                                        case_index & 1 == 0,
                                        case_index & 2 == 0,
                                        case_index & 4 == 0,
                                    ];
                                    inner_one_hop_routes +=
                                        inner_two_hop.iter().filter(|twice| !**twice).count();
                                    inner_two_hop_routes +=
                                        inner_two_hop.iter().filter(|twice| **twice).count();
                                    let replacement = 1801 + case_index as i64;
                                    let delta = 1 + (case_index % 7) as i64;
                                    let source = post_wrapper_reforwarded_direct_literal_adjusted_parameter_alias_program(
                                        first,
                                        second,
                                        reader,
                                        replacement,
                                        delta,
                                        inner_two_hop,
                                        inner_wrapper,
                                        post_wrapper,
                                        offset,
                                        outer_two_hop,
                                        reforward_two_hop,
                                        reforward_placement,
                                    );
                                    let expected = adjusted_parameter_alias_mutation_expected(
                                        first,
                                        second,
                                        reader,
                                        replacement,
                                        delta,
                                    ) + 19;

                                    assert_interpretation(
                                        &source,
                                        ExpectedInterpretation::Value(expected),
                                        &format!(
                                            "post-wrapper re-forwarded direct literal case {case_index}, kind {kind:?}, relation {relation:?}, inner wrapper {inner_wrapper:?}, post wrapper {post_wrapper:?}, offset {offset:?}, outer two hop {outer_two_hop}, re-forward two hop {reforward_two_hop}, re-forward placement {reforward_placement:?}"
                                        ),
                                    );
                                    case_index += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        assert_interpretation(
            &post_wrapper_reforwarded_direct_literal_inner_bounds_program(kind),
            ExpectedInterpretation::Error(kind.inner_bounds_error()),
            &format!("post-wrapper re-forwarded direct literal inner bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_wrapper_reforwarded_direct_literal_outer_bounds_program(kind, 2),
            ExpectedInterpretation::Error(
                "struct array pointer index 3 out of bounds for length 3",
            ),
            &format!("post-wrapper re-forwarded direct literal upper bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_wrapper_reforwarded_direct_literal_outer_bounds_program(kind, -2),
            ExpectedInterpretation::Error(
                "struct array pointer index -1 out of bounds for length 3",
            ),
            &format!("post-wrapper re-forwarded direct literal lower bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_wrapper_reforwarded_direct_literal_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("post-wrapper re-forwarded direct literal const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &post_wrapper_reforwarded_direct_literal_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("post-wrapper re-forwarded direct literal const write, kind {kind:?}"),
        );
        assert_interpretation(
            &post_wrapper_reforwarded_direct_literal_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("post-wrapper re-forwarded direct literal cross-root, kind {kind:?}"),
        );
    }

    for inner_wrapper in WrappedDirectLiteralRoute::ALL {
        for post_wrapper in WrappedDirectLiteralRoute::ALL {
            for offset in WrappedDirectLiteralOffsetRoute::ALL {
                for outer_two_hop in [false, true] {
                    for reforward_two_hop in [false, true] {
                        for reforward_placement in PostForwardWrapperPlacement::ALL {
                            assert_interpretation(
                                &post_wrapper_reforwarded_direct_literal_const_parameter_program(
                                    inner_wrapper,
                                    post_wrapper,
                                    offset,
                                    outer_two_hop,
                                    reforward_two_hop,
                                    reforward_placement,
                                ),
                                ExpectedInterpretation::Value(17),
                                &format!(
                                    "post-wrapper re-forwarded const direct literal route {inner_wrapper:?}, {post_wrapper:?}, {offset:?}, outer two hop {outer_two_hop}, re-forward two hop {reforward_two_hop}, re-forward placement {reforward_placement:?}"
                                ),
                            );
                        }
                    }
                }
            }
        }
    }
    assert_interpretation(
        &post_wrapper_reforwarded_direct_literal_type_mismatch_program(),
        ExpectedInterpretation::Error(
            "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
        ),
        "post-wrapper re-forwarded direct literal aggregate type mismatch",
    );
    assert_interpretation(
        &post_wrapper_reforwarded_direct_literal_lifetime_program(),
        ExpectedInterpretation::Error(
            "pointer to out-of-scope variable '__cust_compound_aggregate_array#0'",
        ),
        "post-wrapper re-forwarded direct literal lifetime",
    );

    assert_eq!(case_index, 1296);
    assert_eq!(relation_counts, [432; 3]);
    assert_eq!(inner_wrapper_counts, [432; 3]);
    assert_eq!(post_wrapper_counts, [432; 3]);
    assert_eq!(offset_counts, [432; 3]);
    assert_eq!(outer_hop_counts, [648; 2]);
    assert_eq!(reforward_hop_counts, [648; 2]);
    assert_eq!(reforward_placement_counts, [648; 2]);
    assert_eq!(inner_one_hop_routes + inner_two_hop_routes, 3888);
    assert!(inner_one_hop_routes >= 1800);
    assert!(inner_two_hop_routes >= 1800);
}

#[test]
fn generated_mutable_to_const_reforwarded_direct_aggregate_array_literal_adjusted_parameters_preserve_identity_without_panics()
 {
    let mut transition_counts = [0; 2];
    let mut inner_wrapper_counts = [0; 3];
    let mut post_wrapper_counts = [0; 3];
    let mut offset_counts = [0; 3];
    let mut outer_hop_counts = [0; 2];
    let mut reforward_hop_counts = [0; 2];
    let mut reforward_placement_counts = [0; 2];
    let mut case_index = 0;

    for transition_before_post_wrapper in [false, true] {
        for inner_wrapper in WrappedDirectLiteralRoute::ALL {
            for post_wrapper in WrappedDirectLiteralRoute::ALL {
                for offset in WrappedDirectLiteralOffsetRoute::ALL {
                    for outer_two_hop in [false, true] {
                        for reforward_two_hop in [false, true] {
                            for reforward_placement in PostForwardWrapperPlacement::ALL {
                                transition_counts[usize::from(transition_before_post_wrapper)] += 1;
                                inner_wrapper_counts[inner_wrapper.index()] += 1;
                                post_wrapper_counts[post_wrapper.index()] += 1;
                                offset_counts[offset.index()] += 1;
                                outer_hop_counts[usize::from(outer_two_hop)] += 1;
                                reforward_hop_counts[usize::from(reforward_two_hop)] += 1;
                                reforward_placement_counts[reforward_placement.index()] += 1;

                                let source = mutable_to_const_reforwarded_direct_literal_adjusted_parameter_program(
                                    transition_before_post_wrapper,
                                    inner_wrapper,
                                    post_wrapper,
                                    offset,
                                    outer_two_hop,
                                    reforward_two_hop,
                                    reforward_placement,
                                );
                                assert_interpretation(
                                    &source,
                                    ExpectedInterpretation::Value(28),
                                    &format!(
                                        "mutable-to-const direct literal case {case_index}, transition before post wrapper {transition_before_post_wrapper}, inner wrapper {inner_wrapper:?}, post wrapper {post_wrapper:?}, offset {offset:?}, outer two hop {outer_two_hop}, re-forward two hop {reforward_two_hop}, re-forward placement {reforward_placement:?}"
                                    ),
                                );
                                case_index += 1;
                            }
                        }
                    }
                }
            }
        }

        assert_interpretation(
            &mutable_to_const_reforwarded_direct_literal_const_discard_program(
                transition_before_post_wrapper,
            ),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!(
                "mutable-to-const direct literal mutable rebinding, transition before post wrapper {transition_before_post_wrapper}"
            ),
        );
        assert_interpretation(
            &mutable_to_const_reforwarded_direct_literal_const_write_program(
                transition_before_post_wrapper,
            ),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!(
                "mutable-to-const direct literal write, transition before post wrapper {transition_before_post_wrapper}"
            ),
        );
    }

    assert_eq!(case_index, 432);
    assert_eq!(transition_counts, [216; 2]);
    assert_eq!(inner_wrapper_counts, [144; 3]);
    assert_eq!(post_wrapper_counts, [144; 3]);
    assert_eq!(offset_counts, [144; 3]);
    assert_eq!(outer_hop_counts, [216; 2]);
    assert_eq!(reforward_hop_counts, [216; 2]);
    assert_eq!(reforward_placement_counts, [216; 2]);
}

#[test]
fn generated_mutable_to_const_reforwarded_captured_literal_field_adjusted_parameters_preserve_identity_without_panics()
 {
    const PATHS: [AdjustedParameterStorage; 3] = [
        AdjustedParameterStorage::NamedLeftPrimary,
        AdjustedParameterStorage::AnonymousLeftPrimary,
        AdjustedParameterStorage::UnionLeftPrimary,
    ];

    let mut transition_counts = [0; 2];
    let mut path_counts = [0; 3];
    let mut inner_wrapper_counts = [0; 3];
    let mut post_wrapper_counts = [0; 3];
    let mut offset_counts = [0; 3];
    let mut outer_hop_counts = [0; 2];
    let mut reforward_hop_counts = [0; 2];
    let mut reforward_placement_counts = [0; 2];
    let mut case_index = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for (path_index, storage) in PATHS.into_iter().enumerate() {
            for transition_before_post_wrapper in [false, true] {
                for inner_wrapper in WrappedDirectLiteralRoute::ALL {
                    for post_wrapper in WrappedDirectLiteralRoute::ALL {
                        for offset in WrappedDirectLiteralOffsetRoute::ALL {
                            for outer_two_hop in [false, true] {
                                for reforward_two_hop in [false, true] {
                                    for reforward_placement in PostForwardWrapperPlacement::ALL {
                                        transition_counts
                                            [usize::from(transition_before_post_wrapper)] += 1;
                                        path_counts[path_index] += 1;
                                        inner_wrapper_counts[inner_wrapper.index()] += 1;
                                        post_wrapper_counts[post_wrapper.index()] += 1;
                                        offset_counts[offset.index()] += 1;
                                        outer_hop_counts[usize::from(outer_two_hop)] += 1;
                                        reforward_hop_counts[usize::from(reforward_two_hop)] += 1;
                                        reforward_placement_counts[reforward_placement.index()] +=
                                            1;

                                        let source = mutable_to_const_reforwarded_captured_literal_field_adjusted_parameter_program(
                                            kind,
                                            storage,
                                            transition_before_post_wrapper,
                                            inner_wrapper,
                                            post_wrapper,
                                            offset,
                                            outer_two_hop,
                                            reforward_two_hop,
                                            reforward_placement,
                                        );
                                        assert_interpretation(
                                            &source,
                                            ExpectedInterpretation::Value(48),
                                            &format!(
                                                "mutable-to-const captured field case {case_index}, kind {kind:?}, path {storage:?}, transition before post wrapper {transition_before_post_wrapper}, inner wrapper {inner_wrapper:?}, post wrapper {post_wrapper:?}, offset {offset:?}, outer two hop {outer_two_hop}, re-forward two hop {reforward_two_hop}, re-forward placement {reforward_placement:?}"
                                            ),
                                        );
                                        case_index += 1;
                                    }
                                }
                            }
                        }
                    }
                }

                assert_interpretation(
                    &mutable_to_const_reforwarded_captured_literal_field_const_discard_program(
                        storage,
                        transition_before_post_wrapper,
                    ),
                    ExpectedInterpretation::Error(
                        "cannot discard const qualifier from pointer target",
                    ),
                    &format!(
                        "mutable-to-const captured field mutable rebinding, path {storage:?}, transition before post wrapper {transition_before_post_wrapper}"
                    ),
                );
                assert_interpretation(
                    &mutable_to_const_reforwarded_captured_literal_field_const_write_program(
                        storage,
                        transition_before_post_wrapper,
                    ),
                    ExpectedInterpretation::Error("cannot assign through pointer to const"),
                    &format!(
                        "mutable-to-const captured field write, path {storage:?}, transition before post wrapper {transition_before_post_wrapper}"
                    ),
                );
            }
        }
    }

    assert_eq!(case_index, 2592);
    assert_eq!(transition_counts, [1296; 2]);
    assert_eq!(path_counts, [864; 3]);
    assert_eq!(inner_wrapper_counts, [864; 3]);
    assert_eq!(post_wrapper_counts, [864; 3]);
    assert_eq!(offset_counts, [864; 3]);
    assert_eq!(outer_hop_counts, [1296; 2]);
    assert_eq!(reforward_hop_counts, [1296; 2]);
    assert_eq!(reforward_placement_counts, [1296; 2]);
}

#[test]
fn generated_captured_literal_field_offset_adjusted_parameter_aliases_match_model_without_panics() {
    const PATHS: [AdjustedParameterStorage; 3] = [
        AdjustedParameterStorage::NamedLeftPrimary,
        AdjustedParameterStorage::AnonymousLeftPrimary,
        AdjustedParameterStorage::UnionLeftPrimary,
    ];
    const RELATIONS: [LiteralAdjustedParameterRelation; 3] = [
        LiteralAdjustedParameterRelation::SameElement,
        LiteralAdjustedParameterRelation::SameArrayDistinctElement,
        LiteralAdjustedParameterRelation::DifferentField,
    ];

    let mut path_counts = [0; 3];
    let mut relation_counts = [0; 3];
    let mut offset_counts = [0; 3];
    let mut one_hop_routes = 0;
    let mut two_hop_routes = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for (path_index, storage) in PATHS.into_iter().enumerate() {
            for (relation_index, relation) in RELATIONS.into_iter().enumerate() {
                for offset in WrappedDirectLiteralOffsetRoute::ALL {
                    path_counts[path_index] += 1;
                    relation_counts[relation_index] += 1;
                    offset_counts[offset.index()] += 1;
                    let case_index = path_index * 9 + relation_index * 3 + offset.index();
                    let first = AdjustedParameterPointer {
                        kind,
                        storage,
                        outer: ((path_index + relation_index) % 2) as i64,
                        inner: ((path_index * 2 + offset.index()) % 3) as i64,
                        route: if case_index & 1 == 0 {
                            AdjustedParameterRoute::Direct
                        } else {
                            AdjustedParameterRoute::Reverse
                        },
                    };
                    let second = literal_adjusted_parameter_related_pointer(first, relation);
                    let reader = if case_index & 2 == 0 { first } else { second };
                    let two_hop = [
                        case_index & 1 == 0,
                        case_index & 2 == 0,
                        case_index & 4 == 0,
                    ];
                    one_hop_routes += two_hop.iter().filter(|twice| !**twice).count();
                    two_hop_routes += two_hop.iter().filter(|twice| **twice).count();
                    let replacement = 701 + case_index as i64;
                    let delta = 1 + (case_index % 7) as i64;
                    let source = captured_literal_field_offset_alias_program(
                        first,
                        second,
                        reader,
                        replacement,
                        delta,
                        two_hop,
                        offset,
                    );
                    let expected = adjusted_parameter_alias_mutation_expected(
                        first,
                        second,
                        reader,
                        replacement,
                        delta,
                    ) + 19;

                    assert_interpretation(
                        &source,
                        ExpectedInterpretation::Value(expected),
                        &format!(
                            "captured literal field offset case {case_index}, kind {kind:?}, path {storage:?}, relation {relation:?}, offset {offset:?}"
                        ),
                    );
                }
            }
        }

        assert_interpretation(
            &captured_literal_field_offset_inner_bounds_program(kind),
            ExpectedInterpretation::Error(kind.inner_bounds_error()),
            &format!("captured literal field offset inner bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &captured_literal_field_offset_outer_bounds_program(kind, 2),
            ExpectedInterpretation::Error(
                "struct array field pointer index 3 out of bounds for length 3",
            ),
            &format!("captured literal field offset upper bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &captured_literal_field_offset_outer_bounds_program(kind, -2),
            ExpectedInterpretation::Error(
                "struct array field pointer index -1 out of bounds for length 3",
            ),
            &format!("captured literal field offset lower bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &captured_literal_field_offset_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("captured literal field offset const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &captured_literal_field_offset_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("captured literal field offset const write, kind {kind:?}"),
        );
        assert_interpretation(
            &captured_literal_field_offset_cross_path_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("captured literal field offset cross-path identity, kind {kind:?}"),
        );
    }

    assert_interpretation(
        &captured_literal_field_offset_type_mismatch_program(),
        ExpectedInterpretation::Error(
            "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
        ),
        "captured literal field offset aggregate type mismatch",
    );

    assert_eq!(path_counts, [18; 3]);
    assert_eq!(relation_counts, [18; 3]);
    assert_eq!(offset_counts, [18; 3]);
    assert_eq!(one_hop_routes + two_hop_routes, 162);
    assert!(one_hop_routes >= 70);
    assert!(two_hop_routes >= 70);
}

#[test]
fn generated_wrapped_captured_literal_field_offset_adjusted_parameter_aliases_match_model_without_panics()
 {
    const PATHS: [AdjustedParameterStorage; 3] = [
        AdjustedParameterStorage::NamedLeftPrimary,
        AdjustedParameterStorage::AnonymousLeftPrimary,
        AdjustedParameterStorage::UnionLeftPrimary,
    ];

    let mut path_counts = [0; 3];
    let mut relation_counts = [0; 4];
    let mut wrapper_counts = [0; 3];
    let mut offset_counts = [0; 3];
    let mut one_hop_routes = 0;
    let mut two_hop_routes = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for case_index in 0..36 {
            let path_index = case_index % PATHS.len();
            let relation_index = (case_index / 3) % LiteralAdjustedParameterRelation::ALL.len();
            let relation = LiteralAdjustedParameterRelation::ALL[relation_index];
            let wrapper = WrappedDirectLiteralRoute::ALL[(case_index / 12) % 3];
            let offset = WrappedDirectLiteralOffsetRoute::ALL[(case_index / 4) % 3];
            path_counts[path_index] += 1;
            relation_counts[relation_index] += 1;
            wrapper_counts[wrapper.index()] += 1;
            offset_counts[offset.index()] += 1;

            let first = AdjustedParameterPointer {
                kind,
                storage: PATHS[path_index],
                outer: ((case_index / 2) % 2) as i64,
                inner: ((case_index * 2 + offset.index()) % 3) as i64,
                route: if case_index & 1 == 0 {
                    AdjustedParameterRoute::Direct
                } else {
                    AdjustedParameterRoute::Reverse
                },
            };
            let second = literal_adjusted_parameter_related_pointer(first, relation);
            let reader = if case_index & 2 == 0 { first } else { second };
            let two_hop = [
                case_index & 1 == 0,
                case_index & 2 == 0,
                case_index & 4 == 0,
            ];
            one_hop_routes += two_hop.iter().filter(|twice| !**twice).count();
            two_hop_routes += two_hop.iter().filter(|twice| **twice).count();
            let replacement = 801 + case_index as i64;
            let delta = 1 + (case_index % 7) as i64;
            let source = wrapped_captured_literal_field_offset_alias_program(
                first,
                second,
                reader,
                replacement,
                delta,
                two_hop,
                wrapper,
                offset,
                None,
            );
            let expected = adjusted_parameter_alias_mutation_expected(
                first,
                second,
                reader,
                replacement,
                delta,
            ) + 42;

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "wrapped captured field offset case {case_index}, kind {kind:?}, path {:?}, relation {relation:?}, wrapper {wrapper:?}, offset {offset:?}",
                    PATHS[path_index]
                ),
            );
        }

        assert_interpretation(
            &wrapped_captured_literal_field_offset_inner_bounds_program(kind),
            ExpectedInterpretation::Error(kind.inner_bounds_error()),
            &format!("wrapped captured field offset inner bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_captured_literal_field_offset_outer_bounds_program(kind, 2),
            ExpectedInterpretation::Error(
                "struct array field pointer index 3 out of bounds for length 3",
            ),
            &format!("wrapped captured field offset upper bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_captured_literal_field_offset_outer_bounds_program(kind, -2),
            ExpectedInterpretation::Error(
                "struct array field pointer index -1 out of bounds for length 3",
            ),
            &format!("wrapped captured field offset lower bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_captured_literal_field_offset_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("wrapped captured field offset const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_captured_literal_field_offset_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("wrapped captured field offset const write, kind {kind:?}"),
        );
        assert_interpretation(
            &wrapped_captured_literal_field_offset_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("wrapped captured field offset cross-root identity, kind {kind:?}"),
        );
    }

    assert_interpretation(
        &wrapped_captured_literal_field_offset_type_mismatch_program(),
        ExpectedInterpretation::Error(
            "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
        ),
        "wrapped captured field offset aggregate type mismatch",
    );

    assert_eq!(path_counts, [24; 3]);
    assert_eq!(relation_counts, [18; 4]);
    assert_eq!(wrapper_counts, [24; 3]);
    assert_eq!(offset_counts, [24; 3]);
    assert_eq!(one_hop_routes + two_hop_routes, 216);
    assert!(one_hop_routes >= 90);
    assert!(two_hop_routes >= 90);
}

#[test]
fn generated_outer_forwarded_wrapped_captured_literal_field_offset_adjusted_parameter_aliases_match_model_without_panics()
 {
    const PATHS: [AdjustedParameterStorage; 3] = [
        AdjustedParameterStorage::NamedLeftPrimary,
        AdjustedParameterStorage::AnonymousLeftPrimary,
        AdjustedParameterStorage::UnionLeftPrimary,
    ];

    let mut path_counts = [0; 3];
    let mut relation_counts = [0; 4];
    let mut wrapper_counts = [0; 3];
    let mut offset_counts = [0; 3];
    let mut outer_forward_counts = [0; 4];
    let mut inner_one_hop_routes = 0;
    let mut inner_two_hop_routes = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for case_index in 0..72 {
            let path_index = case_index % PATHS.len();
            let relation_index = (case_index / 3) % LiteralAdjustedParameterRelation::ALL.len();
            let relation = LiteralAdjustedParameterRelation::ALL[relation_index];
            let wrapper = WrappedDirectLiteralRoute::ALL[(case_index / 4) % 3];
            let offset = WrappedDirectLiteralOffsetRoute::ALL[(case_index / 8) % 3];
            let outer_forward = CapturedOuterForwardRoute::ALL[(case_index / 18) % 4];
            path_counts[path_index] += 1;
            relation_counts[relation_index] += 1;
            wrapper_counts[wrapper.index()] += 1;
            offset_counts[offset.index()] += 1;
            outer_forward_counts[outer_forward.index()] += 1;

            let first = AdjustedParameterPointer {
                kind,
                storage: PATHS[path_index],
                outer: ((case_index / 2) % 2) as i64,
                inner: ((case_index * 2 + offset.index()) % 3) as i64,
                route: if case_index & 1 == 0 {
                    AdjustedParameterRoute::Direct
                } else {
                    AdjustedParameterRoute::Reverse
                },
            };
            let second = literal_adjusted_parameter_related_pointer(first, relation);
            let reader = if case_index & 2 == 0 { first } else { second };
            let inner_two_hop = [
                case_index & 1 == 0,
                case_index & 2 == 0,
                case_index & 4 == 0,
            ];
            inner_one_hop_routes += inner_two_hop.iter().filter(|twice| !**twice).count();
            inner_two_hop_routes += inner_two_hop.iter().filter(|twice| **twice).count();
            let replacement = 901 + case_index as i64;
            let delta = 1 + (case_index % 7) as i64;
            let source = wrapped_captured_literal_field_offset_alias_program(
                first,
                second,
                reader,
                replacement,
                delta,
                inner_two_hop,
                wrapper,
                offset,
                Some(outer_forward),
            );
            let expected = adjusted_parameter_alias_mutation_expected(
                first,
                second,
                reader,
                replacement,
                delta,
            ) + 42;

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "outer-forwarded wrapped captured field offset case {case_index}, kind {kind:?}, path {:?}, relation {relation:?}, wrapper {wrapper:?}, offset {offset:?}, outer forward {outer_forward:?}",
                    PATHS[path_index]
                ),
            );
        }

        assert_interpretation(
            &outer_forwarded_wrapped_captured_field_offset_inner_bounds_program(kind),
            ExpectedInterpretation::Error(kind.inner_bounds_error()),
            &format!("outer-forwarded wrapped captured field offset inner bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &outer_forwarded_wrapped_captured_field_offset_outer_bounds_program(kind, 2),
            ExpectedInterpretation::Error(
                "struct array field pointer index 3 out of bounds for length 3",
            ),
            &format!("outer-forwarded wrapped captured field offset upper bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &outer_forwarded_wrapped_captured_field_offset_outer_bounds_program(kind, -2),
            ExpectedInterpretation::Error(
                "struct array field pointer index -1 out of bounds for length 3",
            ),
            &format!("outer-forwarded wrapped captured field offset lower bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &outer_forwarded_wrapped_captured_field_offset_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("outer-forwarded wrapped captured field offset const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &outer_forwarded_wrapped_captured_field_offset_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("outer-forwarded wrapped captured field offset const write, kind {kind:?}"),
        );
        assert_interpretation(
            &outer_forwarded_wrapped_captured_field_offset_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("outer-forwarded wrapped captured field offset cross-root, kind {kind:?}"),
        );
    }

    assert_interpretation(
        &outer_forwarded_wrapped_captured_field_offset_type_mismatch_program(),
        ExpectedInterpretation::Error(
            "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
        ),
        "outer-forwarded wrapped captured field offset aggregate type mismatch",
    );
    assert_interpretation(
        &outer_forwarded_wrapped_captured_field_offset_lifetime_program(),
        ExpectedInterpretation::Error(
            "pointer to out-of-scope variable '__cust_compound_aggregate#0'",
        ),
        "outer-forwarded wrapped captured field offset lifetime",
    );

    assert_eq!(path_counts, [48; 3]);
    assert_eq!(relation_counts, [36; 4]);
    assert_eq!(wrapper_counts, [48; 3]);
    assert_eq!(offset_counts, [48; 3]);
    assert_eq!(outer_forward_counts, [36; 4]);
    assert_eq!(inner_one_hop_routes + inner_two_hop_routes, 432);
    assert!(inner_one_hop_routes >= 180);
    assert!(inner_two_hop_routes >= 180);
}

#[test]
fn generated_post_forward_wrapped_captured_literal_field_offset_adjusted_parameter_aliases_match_model_without_panics()
 {
    const PATHS: [AdjustedParameterStorage; 3] = [
        AdjustedParameterStorage::NamedLeftPrimary,
        AdjustedParameterStorage::AnonymousLeftPrimary,
        AdjustedParameterStorage::UnionLeftPrimary,
    ];

    let mut path_counts = [0; 3];
    let mut relation_counts = [0; 4];
    let mut inner_wrapper_counts = [0; 3];
    let mut post_wrapper_counts = [0; 3];
    let mut offset_counts = [0; 3];
    let mut placement_counts = [0; 2];
    let mut outer_hop_counts = [0; 2];
    let mut inner_one_hop_routes = 0;
    let mut inner_two_hop_routes = 0;
    let mut case_index = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for (path_index, storage) in PATHS.into_iter().enumerate() {
            for (relation_index, relation) in LiteralAdjustedParameterRelation::ALL
                .into_iter()
                .enumerate()
            {
                for inner_wrapper in WrappedDirectLiteralRoute::ALL {
                    for post_wrapper in WrappedDirectLiteralRoute::ALL {
                        for offset in WrappedDirectLiteralOffsetRoute::ALL {
                            for placement in PostForwardWrapperPlacement::ALL {
                                for outer_two_hop in [false, true] {
                                    path_counts[path_index] += 1;
                                    relation_counts[relation_index] += 1;
                                    inner_wrapper_counts[inner_wrapper.index()] += 1;
                                    post_wrapper_counts[post_wrapper.index()] += 1;
                                    offset_counts[offset.index()] += 1;
                                    placement_counts[placement.index()] += 1;
                                    outer_hop_counts[usize::from(outer_two_hop)] += 1;

                                    let first = AdjustedParameterPointer {
                                        kind,
                                        storage,
                                        outer: ((case_index / 2) % 2) as i64,
                                        inner: ((case_index * 2 + offset.index()) % 3) as i64,
                                        route: if case_index & 1 == 0 {
                                            AdjustedParameterRoute::Direct
                                        } else {
                                            AdjustedParameterRoute::Reverse
                                        },
                                    };
                                    let second =
                                        literal_adjusted_parameter_related_pointer(first, relation);
                                    let reader = if case_index & 2 == 0 { first } else { second };
                                    let inner_two_hop = [
                                        case_index & 1 == 0,
                                        case_index & 2 == 0,
                                        case_index & 4 == 0,
                                    ];
                                    inner_one_hop_routes +=
                                        inner_two_hop.iter().filter(|twice| !**twice).count();
                                    inner_two_hop_routes +=
                                        inner_two_hop.iter().filter(|twice| **twice).count();
                                    let replacement = 1401 + case_index as i64;
                                    let delta = 1 + (case_index % 7) as i64;
                                    let source = post_forward_wrapped_captured_literal_field_offset_alias_program(
                                        first,
                                        second,
                                        reader,
                                        replacement,
                                        delta,
                                        inner_two_hop,
                                        inner_wrapper,
                                        post_wrapper,
                                        offset,
                                        placement,
                                        outer_two_hop,
                                    );
                                    let expected = adjusted_parameter_alias_mutation_expected(
                                        first,
                                        second,
                                        reader,
                                        replacement,
                                        delta,
                                    ) + 42;

                                    assert_interpretation(
                                        &source,
                                        ExpectedInterpretation::Value(expected),
                                        &format!(
                                            "post-forward wrapped captured field case {case_index}, kind {kind:?}, path {storage:?}, relation {relation:?}, inner wrapper {inner_wrapper:?}, post wrapper {post_wrapper:?}, offset {offset:?}, placement {placement:?}, two hop {outer_two_hop}"
                                        ),
                                    );
                                    case_index += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        assert_interpretation(
            &post_forward_wrapped_captured_field_offset_inner_bounds_program(kind),
            ExpectedInterpretation::Error(kind.inner_bounds_error()),
            &format!("post-forward wrapped captured field inner bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_forward_wrapped_captured_field_offset_outer_bounds_program(kind, 2),
            ExpectedInterpretation::Error(
                "struct array field pointer index 3 out of bounds for length 3",
            ),
            &format!("post-forward wrapped captured field upper bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_forward_wrapped_captured_field_offset_outer_bounds_program(kind, -2),
            ExpectedInterpretation::Error(
                "struct array field pointer index -1 out of bounds for length 3",
            ),
            &format!("post-forward wrapped captured field lower bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_forward_wrapped_captured_field_offset_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("post-forward wrapped captured field const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &post_forward_wrapped_captured_field_offset_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("post-forward wrapped captured field const write, kind {kind:?}"),
        );
        assert_interpretation(
            &post_forward_wrapped_captured_field_offset_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("post-forward wrapped captured field cross-root, kind {kind:?}"),
        );
    }

    assert_interpretation(
        &post_forward_wrapped_captured_field_offset_type_mismatch_program(),
        ExpectedInterpretation::Error(
            "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
        ),
        "post-forward wrapped captured field aggregate type mismatch",
    );
    assert_interpretation(
        &post_forward_wrapped_captured_field_offset_lifetime_program(),
        ExpectedInterpretation::Error(
            "pointer to out-of-scope variable '__cust_compound_aggregate#0'",
        ),
        "post-forward wrapped captured field lifetime",
    );

    assert_eq!(case_index, 2592);
    assert_eq!(path_counts, [864; 3]);
    assert_eq!(relation_counts, [648; 4]);
    assert_eq!(inner_wrapper_counts, [864; 3]);
    assert_eq!(post_wrapper_counts, [864; 3]);
    assert_eq!(offset_counts, [864; 3]);
    assert_eq!(placement_counts, [1296; 2]);
    assert_eq!(outer_hop_counts, [1296; 2]);
    assert_eq!(inner_one_hop_routes + inner_two_hop_routes, 7776);
    assert!(inner_one_hop_routes >= 3600);
    assert!(inner_two_hop_routes >= 3600);
}

#[test]
fn generated_post_wrapper_reforwarded_captured_literal_field_offset_adjusted_parameter_aliases_match_model_without_panics()
 {
    const PATHS: [AdjustedParameterStorage; 3] = [
        AdjustedParameterStorage::NamedLeftPrimary,
        AdjustedParameterStorage::AnonymousLeftPrimary,
        AdjustedParameterStorage::UnionLeftPrimary,
    ];

    let mut path_counts = [0; 3];
    let mut relation_counts = [0; 4];
    let mut inner_wrapper_counts = [0; 3];
    let mut post_wrapper_counts = [0; 3];
    let mut offset_counts = [0; 3];
    let mut outer_hop_counts = [0; 2];
    let mut reforward_hop_counts = [0; 2];
    let mut reforward_placement_counts = [0; 2];
    let mut inner_one_hop_routes = 0;
    let mut inner_two_hop_routes = 0;
    let mut case_index = 0;

    for kind in AdjustedParameterFieldKind::ALL {
        for (path_index, storage) in PATHS.into_iter().enumerate() {
            for (relation_index, relation) in LiteralAdjustedParameterRelation::ALL
                .into_iter()
                .enumerate()
            {
                for inner_wrapper in WrappedDirectLiteralRoute::ALL {
                    for post_wrapper in WrappedDirectLiteralRoute::ALL {
                        for offset in WrappedDirectLiteralOffsetRoute::ALL {
                            for outer_two_hop in [false, true] {
                                for reforward_two_hop in [false, true] {
                                    for reforward_placement in PostForwardWrapperPlacement::ALL {
                                        path_counts[path_index] += 1;
                                        relation_counts[relation_index] += 1;
                                        inner_wrapper_counts[inner_wrapper.index()] += 1;
                                        post_wrapper_counts[post_wrapper.index()] += 1;
                                        offset_counts[offset.index()] += 1;
                                        outer_hop_counts[usize::from(outer_two_hop)] += 1;
                                        reforward_hop_counts[usize::from(reforward_two_hop)] += 1;
                                        reforward_placement_counts[reforward_placement.index()] +=
                                            1;

                                        let first = AdjustedParameterPointer {
                                            kind,
                                            storage,
                                            outer: ((case_index / 2) % 2) as i64,
                                            inner: ((case_index * 2 + offset.index()) % 3) as i64,
                                            route: if case_index & 1 == 0 {
                                                AdjustedParameterRoute::Direct
                                            } else {
                                                AdjustedParameterRoute::Reverse
                                            },
                                        };
                                        let second = literal_adjusted_parameter_related_pointer(
                                            first, relation,
                                        );
                                        let reader =
                                            if case_index & 2 == 0 { first } else { second };
                                        let inner_two_hop = [
                                            case_index & 1 == 0,
                                            case_index & 2 == 0,
                                            case_index & 4 == 0,
                                        ];
                                        inner_one_hop_routes +=
                                            inner_two_hop.iter().filter(|twice| !**twice).count();
                                        inner_two_hop_routes +=
                                            inner_two_hop.iter().filter(|twice| **twice).count();
                                        let replacement = 1601 + case_index as i64;
                                        let delta = 1 + (case_index % 7) as i64;
                                        let source = post_wrapper_reforwarded_captured_literal_field_offset_alias_program(
                                            first,
                                            second,
                                            reader,
                                            replacement,
                                            delta,
                                            inner_two_hop,
                                            inner_wrapper,
                                            post_wrapper,
                                            offset,
                                            outer_two_hop,
                                            reforward_two_hop,
                                            reforward_placement,
                                        );
                                        let expected = adjusted_parameter_alias_mutation_expected(
                                            first,
                                            second,
                                            reader,
                                            replacement,
                                            delta,
                                        ) + 42;

                                        assert_interpretation(
                                            &source,
                                            ExpectedInterpretation::Value(expected),
                                            &format!(
                                                "post-wrapper re-forwarded captured field case {case_index}, kind {kind:?}, path {storage:?}, relation {relation:?}, inner wrapper {inner_wrapper:?}, post wrapper {post_wrapper:?}, offset {offset:?}, outer two hop {outer_two_hop}, re-forward two hop {reforward_two_hop}, re-forward placement {reforward_placement:?}"
                                            ),
                                        );
                                        case_index += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        assert_interpretation(
            &post_wrapper_reforwarded_captured_field_offset_inner_bounds_program(kind),
            ExpectedInterpretation::Error(kind.inner_bounds_error()),
            &format!("post-wrapper re-forwarded captured field inner bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_wrapper_reforwarded_captured_field_offset_outer_bounds_program(kind, 2),
            ExpectedInterpretation::Error(
                "struct array field pointer index 3 out of bounds for length 3",
            ),
            &format!("post-wrapper re-forwarded captured field upper bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_wrapper_reforwarded_captured_field_offset_outer_bounds_program(kind, -2),
            ExpectedInterpretation::Error(
                "struct array field pointer index -1 out of bounds for length 3",
            ),
            &format!("post-wrapper re-forwarded captured field lower bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &post_wrapper_reforwarded_captured_field_offset_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("post-wrapper re-forwarded captured field const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &post_wrapper_reforwarded_captured_field_offset_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("post-wrapper re-forwarded captured field const write, kind {kind:?}"),
        );
        assert_interpretation(
            &post_wrapper_reforwarded_captured_field_offset_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("post-wrapper re-forwarded captured field cross-root, kind {kind:?}"),
        );
    }

    assert_interpretation(
        &post_wrapper_reforwarded_captured_field_offset_type_mismatch_program(),
        ExpectedInterpretation::Error(
            "cannot convert pointer to struct 'Point' to pointer to struct 'Other'",
        ),
        "post-wrapper re-forwarded captured field aggregate type mismatch",
    );
    assert_interpretation(
        &post_wrapper_reforwarded_captured_field_offset_lifetime_program(),
        ExpectedInterpretation::Error(
            "pointer to out-of-scope variable '__cust_compound_aggregate#0'",
        ),
        "post-wrapper re-forwarded captured field lifetime",
    );

    assert_eq!(case_index, 5184);
    assert_eq!(path_counts, [1728; 3]);
    assert_eq!(relation_counts, [1296; 4]);
    assert_eq!(inner_wrapper_counts, [1728; 3]);
    assert_eq!(post_wrapper_counts, [1728; 3]);
    assert_eq!(offset_counts, [1728; 3]);
    assert_eq!(outer_hop_counts, [2592; 2]);
    assert_eq!(reforward_hop_counts, [2592; 2]);
    assert_eq!(reforward_placement_counts, [2592; 2]);
    assert_eq!(inner_one_hop_routes + inner_two_hop_routes, 15552);
    assert!(inner_one_hop_routes >= 7200);
    assert!(inner_two_hop_routes >= 7200);
}

#[test]
fn generated_nested_anonymous_aggregate_compound_literal_field_pointer_alias_mutations_match_model_without_panics()
 {
    let mut state = 0xC057_117E_AA55_u64;
    let mut pattern_counts = [0; 6];
    let mut path_counts = [0; 2];
    let mut wrapper_counts = [0; 4];
    let mut one_hop_calls = 0;
    let mut two_hop_calls = 0;

    for kind in FieldBackedPointeeKind::ALL {
        for case_index in 0..24 {
            let writer = ExtendedFieldBackedPointer {
                kind,
                path: if (case_index / 2) & 1 == 0 {
                    path_counts[0] += 1;
                    ExtendedFieldBackedPath::Nested
                } else {
                    path_counts[1] += 1;
                    ExtendedFieldBackedPath::Anonymous
                },
                owner: if next_u64(&mut state) & 1 == 0 {
                    FieldBackedOwner::Left
                } else {
                    FieldBackedOwner::Right
                },
                field: if next_u64(&mut state) & 1 == 0 {
                    FieldBackedField::Primary
                } else {
                    FieldBackedField::Secondary
                },
                index: (next_u64(&mut state) % EMBEDDED_ARRAY_LEN as u64) as i64,
                route: FieldBackedRoute::Direct,
            };
            let pattern = match case_index % 6 {
                0 => ExtendedLiteralAliasPattern::SameElement,
                1 => ExtendedLiteralAliasPattern::SameRootDistinctElement,
                2 => ExtendedLiteralAliasPattern::DifferentField,
                3 => ExtendedLiteralAliasPattern::DifferentPath,
                4 => ExtendedLiteralAliasPattern::DifferentLiteralRoot,
                _ => ExtendedLiteralAliasPattern::DifferentPathAndLiteralRoot,
            };
            pattern_counts[case_index % 6] += 1;
            let (second_path, second_owner, second_field, second_index) = match pattern {
                ExtendedLiteralAliasPattern::SameElement => {
                    (writer.path, writer.owner, writer.field, writer.index)
                }
                ExtendedLiteralAliasPattern::SameRootDistinctElement => (
                    writer.path,
                    writer.owner,
                    writer.field,
                    (writer.index + 1 + (next_u64(&mut state) % 3) as i64) % EMBEDDED_ARRAY_LEN,
                ),
                ExtendedLiteralAliasPattern::DifferentField => (
                    writer.path,
                    writer.owner,
                    writer.field.other(),
                    writer.index,
                ),
                ExtendedLiteralAliasPattern::DifferentPath => (
                    writer.path.other(),
                    writer.owner,
                    writer.field,
                    writer.index,
                ),
                ExtendedLiteralAliasPattern::DifferentLiteralRoot => (
                    writer.path,
                    writer.owner.other(),
                    writer.field,
                    writer.index,
                ),
                ExtendedLiteralAliasPattern::DifferentPathAndLiteralRoot => (
                    writer.path.other(),
                    writer.owner.other(),
                    writer.field,
                    writer.index,
                ),
            };
            let second_writer = ExtendedFieldBackedPointer {
                path: second_path,
                owner: second_owner,
                field: second_field,
                index: second_index,
                ..writer
            };
            let reader = if case_index & 4 == 0 {
                writer
            } else {
                second_writer
            };
            let wrappers = [
                literal_pointer_wrapper(case_index),
                literal_pointer_wrapper(case_index + 1),
                literal_pointer_wrapper(case_index + 2),
            ];
            for wrapper in wrappers {
                wrapper_counts[literal_pointer_wrapper_index(wrapper)] += 1;
            }
            let twice = [
                case_index & 1 == 0,
                case_index & 2 == 0,
                case_index & 4 == 0,
            ];
            one_hop_calls += twice.iter().filter(|value| !**value).count();
            two_hop_calls += twice.iter().filter(|value| **value).count();
            let replacement = 90 + (next_u64(&mut state) % 30) as i64;
            let delta = 1 + (next_u64(&mut state) % 9) as i64;
            let expected = extended_literal_field_alias_mutation_expected(
                writer,
                second_writer,
                reader,
                replacement,
                delta,
            );
            let source = extended_literal_field_alias_mutation_program(
                writer,
                second_writer,
                reader,
                replacement,
                delta,
                wrappers,
                twice,
            );

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "nested/anonymous aggregate-literal field alias case {case_index}, kind {kind:?}, pattern {pattern:?}, writer {writer:?}, second_writer {second_writer:?}, reader {reader:?}"
                ),
            );
        }

        for path in ExtendedFieldBackedPath::ALL {
            assert_interpretation(
                &extended_literal_field_const_discard_program(kind, path),
                ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
                &format!(
                    "nested/anonymous aggregate-literal recursive const, kind {kind:?}, path {path:?}"
                ),
            );
            assert_interpretation(
                &extended_literal_field_bounds_program(kind, path),
                ExpectedInterpretation::OwnedError(format!(
                    "{} pointer index 5 out of bounds for length {EMBEDDED_ARRAY_LEN}",
                    kind.bounds_prefix()
                )),
                &format!("nested/anonymous aggregate-literal bounds, kind {kind:?}, path {path:?}"),
            );
        }
        assert_interpretation(
            &extended_literal_field_cross_path_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("nested/anonymous aggregate-literal cross-path, kind {kind:?}"),
        );
        assert_interpretation(
            &extended_literal_field_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("nested/anonymous aggregate-literal cross-root, kind {kind:?}"),
        );
        assert_interpretation(
            &extended_literal_field_type_mismatch_program(kind),
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                kind.other().pointee_label()
            )),
            &format!("nested/anonymous aggregate-literal pointee type, kind {kind:?}"),
        );
    }

    assert_eq!(pattern_counts, [16, 16, 16, 16, 16, 16]);
    assert_eq!(path_counts, [48, 48]);
    assert_eq!(wrapper_counts, [72, 72, 72, 72]);
    assert_eq!(one_hop_calls, 144);
    assert_eq!(two_hop_calls, 144);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EqualityRoot {
    Left,
    Right,
    Null,
}

#[derive(Clone, Debug)]
struct EqualityOperand {
    rendered: String,
    root: EqualityRoot,
    index: i64,
    marker_increments: i64,
}

#[test]
fn generated_scalar_and_pointer_equality_classification_matches_model_without_panics() {
    let mut state = 0xC057_E9A1_17E5_u64;
    let mut scalar_cases = 0;
    let mut pointer_cases = 0;
    let mut null_cases = 0;
    let mut nonzero_cases = 0;
    let mut side_effect_cases = 0;
    let mut pointer_route_counts = [0; 8];
    let mut scalar_route_counts = [0; 8];

    for case_index in 0..32 {
        let left_value = (next_u64(&mut state) % 7) as i64 - 3;
        let right_value = if case_index & 1 == 0 {
            left_value
        } else {
            (left_value + 1).min(3)
        };
        let left_route = case_index % 8;
        let right_route = (case_index * 3 + 1) % 8;
        scalar_route_counts[left_route] += 1;
        scalar_route_counts[right_route] += 1;
        let left = equality_scalar_operand(left_route, left_value);
        let right = equality_scalar_operand(right_route, right_value);
        let equal = left_value == right_value;
        let use_equal = case_index & 2 == 0;
        let comparison = if use_equal { equal } else { !equal };
        let expected = i64::from(comparison) * 10 + left.1 + right.1;
        side_effect_cases += usize::from(left.1 + right.1 > 0);

        assert_interpretation(
            &equality_program(&left.0, if use_equal { "==" } else { "!=" }, &right.0),
            ExpectedInterpretation::Value(expected),
            &format!(
                "scalar equality case {case_index}, routes {left_route}/{right_route}, values {left_value}/{right_value}"
            ),
        );
        scalar_cases += 1;
    }

    for case_index in 0..48 {
        let left_root = if next_u64(&mut state) & 1 == 0 {
            EqualityRoot::Left
        } else {
            EqualityRoot::Right
        };
        let left_index = (next_u64(&mut state) % 4) as i64;
        let pattern = case_index % 4;
        let (right_root, right_index) = match pattern {
            0 => (left_root, left_index),
            1 => (left_root, (left_index + 1) % 4),
            2 => (equality_other_root(left_root), left_index),
            _ => (EqualityRoot::Null, 0),
        };
        let left_route = case_index % 8;
        let right_route = (case_index * 5 + 3) % 8;
        pointer_route_counts[left_route] += 1;
        if right_root != EqualityRoot::Null {
            pointer_route_counts[right_route] += 1;
        }
        let left = equality_pointer_operand(left_route, left_root, left_index);
        let right = equality_pointer_operand(right_route, right_root, right_index);
        let equal = left.root == right.root
            && (left.root == EqualityRoot::Null || left.index == right.index);
        let use_equal = case_index & 4 == 0;
        let comparison = if use_equal { equal } else { !equal };
        let expected =
            i64::from(comparison) * 10 + left.marker_increments + right.marker_increments;
        side_effect_cases += usize::from(left.marker_increments + right.marker_increments > 0);

        assert_interpretation(
            &equality_program(
                &left.rendered,
                if use_equal { "==" } else { "!=" },
                &right.rendered,
            ),
            ExpectedInterpretation::Value(expected),
            &format!(
                "pointer equality case {case_index}, pattern {pattern}, operands {left:?}/{right:?}"
            ),
        );
        pointer_cases += 1;
    }

    let scalar_wrappers = [
        ("+0", 0, 0),
        ("(int)0", 0, 0),
        ("(1 ? 0 : 3)", 0, 0),
        ("(marker++, +0)", 0, 1),
        ("+1", 1, 0),
        ("(int)-2", -2, 0),
        ("(0 ? 0 : 3)", 3, 0),
        ("(marker++, +4)", 4, 1),
    ];
    for case_index in 0..16 {
        let (scalar, value, marker_increments) = scalar_wrappers[case_index % 8];
        let pointer =
            equality_pointer_operand(case_index % 8, EqualityRoot::Left, (case_index % 4) as i64);
        let scalar_on_left = case_index & 1 == 0;
        let use_equal = case_index & 2 == 0;
        let (left, right) = if scalar_on_left {
            (scalar, pointer.rendered.as_str())
        } else {
            (pointer.rendered.as_str(), scalar)
        };

        if value == 0 {
            null_cases += 1;
            let expected =
                i64::from(!use_equal) * 10 + marker_increments + pointer.marker_increments;
            side_effect_cases += usize::from(marker_increments + pointer.marker_increments > 0);
            assert_interpretation(
                &equality_program(left, if use_equal { "==" } else { "!=" }, right),
                ExpectedInterpretation::Value(expected),
                &format!(
                    "mixed null equality case {case_index}, scalar {scalar}, pointer {pointer:?}"
                ),
            );
        } else {
            nonzero_cases += 1;
            assert_interpretation(
                &equality_program(left, if use_equal { "==" } else { "!=" }, right),
                ExpectedInterpretation::Error("cannot compare pointer with nonzero integer"),
                &format!(
                    "mixed nonzero equality case {case_index}, scalar {scalar}, pointer {pointer:?}"
                ),
            );
        }
    }

    assert_interpretation(
        &equality_program("left", "==", "point"),
        ExpectedInterpretation::Error("struct variable 'point' used as scalar"),
        "pointer/aggregate equality type diagnostic",
    );

    assert_eq!(scalar_cases, 32);
    assert_eq!(pointer_cases, 48);
    assert_eq!(null_cases, 8);
    assert_eq!(nonzero_cases, 8);
    assert!(side_effect_cases >= 20);
    assert!(pointer_route_counts.iter().all(|count| *count >= 6));
    assert_eq!(scalar_route_counts, [8; 8]);
}

#[test]
fn generated_scalar_and_pointer_ordering_classification_matches_model_without_panics() {
    let mut state = 0xC057_0ADE_17E5_u64;
    let operators = ["<", "<=", ">", ">="];
    let mut scalar_cases = 0;
    let mut same_root_pointer_cases = 0;
    let mut cross_root_pointer_cases = 0;
    let mut null_pointer_cases = 0;
    let mut mixed_cases = 0;
    let mut side_effect_cases = 0;
    let mut pointer_route_counts = [0; 8];
    let mut scalar_route_counts = [0; 8];

    for case_index in 0..32 {
        let left_value = (next_u64(&mut state) % 7) as i64 - 3;
        let right_value = (next_u64(&mut state) % 7) as i64 - 3;
        let left_route = case_index % 8;
        let right_route = (case_index * 3 + 1) % 8;
        let operator = operators[case_index % operators.len()];
        scalar_route_counts[left_route] += 1;
        scalar_route_counts[right_route] += 1;
        let left = equality_scalar_operand(left_route, left_value);
        let right = equality_scalar_operand(right_route, right_value);
        let comparison = ordering_model(left_value, operator, right_value);
        let expected = i64::from(comparison) * 10 + left.1 + right.1;
        side_effect_cases += usize::from(left.1 + right.1 > 0);

        assert_interpretation(
            &equality_program(&left.0, operator, &right.0),
            ExpectedInterpretation::Value(expected),
            &format!(
                "scalar ordering case {case_index}, routes {left_route}/{right_route}, values {left_value}/{right_value}, operator {operator}"
            ),
        );
        scalar_cases += 1;
    }

    for case_index in 0..48 {
        let left_root = if next_u64(&mut state) & 1 == 0 {
            EqualityRoot::Left
        } else {
            EqualityRoot::Right
        };
        let left_index = (next_u64(&mut state) % 4) as i64;
        let pattern = case_index % 4;
        let (right_root, right_index) = match pattern {
            0 => (left_root, left_index),
            1 => (left_root, (left_index + 1) % 4),
            2 => (equality_other_root(left_root), left_index),
            _ => (EqualityRoot::Null, 0),
        };
        let left_route = case_index % 8;
        let right_route = (case_index * 5 + 3) % 8;
        let operator = operators[(case_index / 4) % operators.len()];
        pointer_route_counts[left_route] += 1;
        if right_root != EqualityRoot::Null {
            pointer_route_counts[right_route] += 1;
        }
        let left = equality_pointer_operand(left_route, left_root, left_index);
        let right = equality_pointer_operand(right_route, right_root, right_index);
        let source = equality_program(&left.rendered, operator, &right.rendered);
        let context = format!(
            "pointer ordering case {case_index}, pattern {pattern}, operator {operator}, operands {left:?}/{right:?}"
        );

        if right_root == EqualityRoot::Null {
            null_pointer_cases += 1;
            assert_interpretation(
                &source,
                ExpectedInterpretation::Error("pointer ordering comparisons are not supported"),
                &context,
            );
        } else if left_root != right_root {
            cross_root_pointer_cases += 1;
            assert_interpretation(
                &source,
                ExpectedInterpretation::Error("cannot compare pointers to different arrays"),
                &context,
            );
        } else {
            same_root_pointer_cases += 1;
            let comparison = ordering_model(left_index, operator, right_index);
            let expected =
                i64::from(comparison) * 10 + left.marker_increments + right.marker_increments;
            side_effect_cases += usize::from(left.marker_increments + right.marker_increments > 0);
            assert_interpretation(&source, ExpectedInterpretation::Value(expected), &context);
        }
    }

    for case_index in 0..16 {
        let scalar_value = (case_index as i64 % 7) - 3;
        let scalar_route = case_index % 8;
        let pointer_route = (case_index * 3 + 1) % 8;
        let operator = operators[case_index % operators.len()];
        let scalar = if scalar_route == 5 {
            (format!("view->nested.values[{}]", case_index % 4), 0)
        } else {
            equality_scalar_operand(scalar_route, scalar_value)
        };
        let pointer =
            equality_pointer_operand(pointer_route, EqualityRoot::Left, (case_index % 4) as i64);
        let scalar_on_left = case_index & 1 == 0;
        let (left, right) = if scalar_on_left {
            (scalar.0.as_str(), pointer.rendered.as_str())
        } else {
            (pointer.rendered.as_str(), scalar.0.as_str())
        };

        assert_interpretation(
            &equality_program(left, operator, right),
            ExpectedInterpretation::Error("pointer ordering comparisons are not supported"),
            &format!(
                "mixed ordering case {case_index}, scalar route {scalar_route}, pointer route {pointer_route}, operator {operator}"
            ),
        );
        mixed_cases += 1;
    }

    assert_eq!(scalar_cases, 32);
    assert_eq!(same_root_pointer_cases, 24);
    assert_eq!(cross_root_pointer_cases, 12);
    assert_eq!(null_pointer_cases, 12);
    assert_eq!(mixed_cases, 16);
    assert!(side_effect_cases >= 12);
    assert!(pointer_route_counts.iter().all(|count| *count >= 6));
    assert_eq!(scalar_route_counts, [8; 8]);
}

#[test]
fn generated_scalar_and_pointer_truthiness_classification_matches_model_without_panics() {
    let mut state = 0xC057_7A17_17E5_u64;
    let mut scalar_cases = 0;
    let mut pointer_cases = 0;
    let mut true_cases = 0;
    let mut false_cases = 0;
    let mut short_circuit_cases = 0;
    let mut side_effect_cases = 0;
    let mut scalar_route_counts = [0; 8];
    let mut pointer_route_counts = [0; 8];
    let mut context_counts = [0; 6];

    for case_index in 0..48 {
        let route = case_index % 8;
        let context = case_index % 6;
        let value = if (case_index / 8) & 1 == 0 {
            0
        } else {
            1 + (next_u64(&mut state) % 3) as i64
        };
        let operand = equality_scalar_operand(route, value);
        let expected = truthiness_expected(value != 0, operand.1, context);

        assert_interpretation(
            &truthiness_program(&operand.0, context),
            ExpectedInterpretation::Value(expected),
            &format!(
                "scalar truthiness case {case_index}, route {route}, context {context}, value {value}"
            ),
        );
        scalar_cases += 1;
        true_cases += usize::from(value != 0);
        false_cases += usize::from(value == 0);
        short_circuit_cases += usize::from(matches!(context, 2 | 3));
        side_effect_cases += usize::from(operand.1 > 0 || matches!(context, 2 | 3));
        scalar_route_counts[route] += 1;
        context_counts[context] += 1;
    }

    for case_index in 0..96 {
        let route = case_index % 8;
        let context = case_index % 6;
        let truthy = (case_index / 8) & 1 == 0;
        let operand = truthiness_pointer_operand(route, truthy, (case_index % 4) as i64);
        let expected = truthiness_expected(truthy, operand.marker_increments, context);

        assert_interpretation(
            &truthiness_program(&operand.rendered, context),
            ExpectedInterpretation::Value(expected),
            &format!(
                "pointer truthiness case {case_index}, route {route}, context {context}, truthy {truthy}, operand {operand:?}"
            ),
        );
        pointer_cases += 1;
        true_cases += usize::from(truthy);
        false_cases += usize::from(!truthy);
        short_circuit_cases += usize::from(matches!(context, 2 | 3));
        side_effect_cases += usize::from(operand.marker_increments > 0 || matches!(context, 2 | 3));
        pointer_route_counts[route] += 1;
        context_counts[context] += 1;
    }

    assert_interpretation(
        &truthiness_program("point", 0),
        ExpectedInterpretation::Error("struct variable 'point' used as scalar"),
        "aggregate truthiness diagnostic",
    );
    assert_interpretation(
        &truthiness_program("noop()", 0),
        ExpectedInterpretation::Error("void function 'noop' used as scalar expression"),
        "void truthiness diagnostic",
    );

    assert_eq!(scalar_cases, 48);
    assert_eq!(pointer_cases, 96);
    assert_eq!(true_cases, 72);
    assert_eq!(false_cases, 72);
    assert_eq!(short_circuit_cases, 48);
    assert!(side_effect_cases >= 64);
    assert_eq!(scalar_route_counts, [6; 8]);
    assert_eq!(pointer_route_counts, [12; 8]);
    assert_eq!(context_counts, [24; 6]);
}

#[test]
fn generated_discard_context_classification_matches_model_without_panics() {
    let routes = [
        ("scalar increment", "marker++", 1),
        ("scalar assignment", "scalar = (marker++, 7)", 1),
        ("scalar compound assignment", "scalar += (marker++, 2)", 1),
        ("scalar call", "scalar_call()", 1),
        ("scalar conditional", "1 ? (marker++, scalar) : 0", 1),
        ("scalar comma", "(marker++, scalar)", 1),
        ("array assignment", "cursor[0] = (marker++, 8)", 1),
        ("dereference assignment", "*cursor = (marker++, 9)", 1),
        ("pointer assignment", "cursor = (marker++, values + 1)", 1),
        ("pointer compound assignment", "cursor += (marker++, 1)", 1),
        (
            "direct pointer field assignment",
            "holder.primary = (marker++, values + 1)",
            1,
        ),
        (
            "arrow pointer field assignment",
            "holder_view->primary = (marker++, values + 1)",
            1,
        ),
        (
            "aggregate literal pointer field assignment",
            "((struct Holder){values}).primary = (marker++, values + 1)",
            1,
        ),
        ("pointer call", "(marker++, forward(values + 1))", 1),
        (
            "pointer conditional",
            "(marker++, (1 ? values + 1 : cursor))",
            1,
        ),
        ("pointer cast", "(marker++, (int *)(values + 1))", 1),
        ("aggregate variable", "(marker++, point)", 1),
        ("aggregate assignment", "point = (marker++, replacement)", 1),
        (
            "aggregate conditional",
            "(marker++, (1 ? point : replacement))",
            1,
        ),
        ("aggregate call", "(marker++, make_point())", 1),
        (
            "aggregate compound literal",
            "(marker++, (struct Point){9})",
            1,
        ),
        ("aggregate field", "(marker++, box.point)", 1),
        (
            "aggregate dereference assignment",
            "(*(marker++, point_slot) = replacement)",
            1,
        ),
        ("void call", "touch()", 1),
        ("void comma", "(marker++, touch_noop())", 1),
        ("void conditional", "1 ? touch() : touch_noop()", 1),
        ("void cast", "(marker++, (void)scalar)", 1),
    ];
    let mut context_counts = [0; 3];
    let mut family_counts = [0; 4];

    for (route_index, (label, expression, marker_increments)) in routes.iter().enumerate() {
        for context in 0..3 {
            let expected = marker_increments * 10 + i64::from(context == 2);
            assert_interpretation(
                &discard_context_program(expression, context),
                ExpectedInterpretation::Value(expected),
                &format!("discard route {route_index} ({label}), context {context}"),
            );
            context_counts[context] += 1;
        }
        family_counts[match route_index {
            0..=7 => 0,
            8..=15 => 1,
            16..=22 => 2,
            _ => 3,
        }] += 1;
    }

    assert_interpretation(
        &discard_scalar_use_program("point"),
        ExpectedInterpretation::Error("struct variable 'point' used as scalar"),
        "aggregate scalar-use diagnostic remains exact",
    );
    assert_interpretation(
        &discard_scalar_use_program("touch()"),
        ExpectedInterpretation::Error("void function 'touch' used as scalar expression"),
        "void scalar-use diagnostic remains exact",
    );
    assert_interpretation(
        &discard_scalar_use_program("cursor"),
        ExpectedInterpretation::Error("pointer 'cursor' used as scalar"),
        "pointer scalar-use diagnostic remains exact",
    );

    assert_eq!(context_counts, [27; 3]);
    assert_eq!(family_counts, [8, 8, 7, 4]);
}

#[test]
fn generated_void_return_context_rejects_every_value_shape_without_panics() {
    let expressions = [
        ("scalar", "marker++"),
        ("pointer", "(marker++, cursor)"),
        ("aggregate", "(marker++, point)"),
        ("void", "(marker++, touch())"),
    ];

    for (case_index, (label, expression)) in expressions.iter().enumerate() {
        assert_interpretation(
            &void_return_context_program(expression),
            ExpectedInterpretation::Error("void function 'reject_value' returned a value"),
            &format!("void return case {case_index} ({label})"),
        );
    }
}

#[test]
fn generated_return_context_classification_matches_model_without_panics() {
    let scalar_routes = [
        ("direct", "scalar", 0, 3),
        ("comma", "(marker++, scalar)", 1, 3),
        ("assignment", "scalar = (marker++, 5)", 1, 5),
        ("compound assignment", "scalar += (marker++, 2)", 1, 5),
        ("call", "scalar_call()", 1, 7),
        ("conditional", "1 ? (marker++, 8) : (marker += 20, 9)", 1, 8),
        ("cast", "(int)(marker++, 9)", 1, 9),
        ("nested comma", "((marker++, 1), (marker++, 4))", 2, 4),
    ];
    let pointer_routes = [
        ("direct", "cursor", 0, 1),
        ("arithmetic", "(marker++, cursor + 1)", 1, 2),
        ("assignment", "cursor = (marker++, values + 2)", 1, 3),
        ("call", "(marker++, forward(values + 3))", 1, 4),
        (
            "conditional",
            "1 ? (marker++, values + 1) : (marker += 20, cursor)",
            1,
            2,
        ),
        ("comma", "(marker++, values + 2)", 1, 3),
        ("cast", "(marker++, (int *)(values + 3))", 1, 4),
        (
            "field assignment",
            "holder.primary = (marker++, values + 1)",
            1,
            2,
        ),
    ];
    let aggregate_routes = [
        ("direct", "point", 0, 3),
        ("comma", "(marker++, point)", 1, 3),
        ("assignment", "point = (marker++, replacement)", 1, 5),
        (
            "conditional",
            "1 ? (marker++, replacement) : (marker += 20, point)",
            1,
            5,
        ),
        ("call", "make_point()", 1, 7),
        ("compound literal", "(marker++, (struct Point){8})", 1, 8),
        ("dereference", "(marker++, *slot)", 1, 3),
        (
            "dereference assignment",
            "*slot = (marker++, replacement)",
            1,
            5,
        ),
    ];

    for (case_index, (label, expression, marker_increments, value)) in
        scalar_routes.iter().enumerate()
    {
        assert_interpretation(
            &scalar_return_context_program(expression),
            ExpectedInterpretation::Value(marker_increments * 10 + value),
            &format!("scalar return case {case_index} ({label})"),
        );
    }
    for (case_index, (label, expression, marker_increments, value)) in
        pointer_routes.iter().enumerate()
    {
        assert_interpretation(
            &pointer_return_context_program(expression),
            ExpectedInterpretation::Value(marker_increments * 10 + value),
            &format!("pointer return case {case_index} ({label})"),
        );
    }
    for (case_index, (label, expression, marker_increments, value)) in
        aggregate_routes.iter().enumerate()
    {
        assert_interpretation(
            &aggregate_return_context_program(expression),
            ExpectedInterpretation::Value(marker_increments * 10 + value),
            &format!("aggregate return case {case_index} ({label})"),
        );
    }

    assert_eq!(scalar_routes.len(), 8);
    assert_eq!(pointer_routes.len(), 8);
    assert_eq!(aggregate_routes.len(), 8);
}

#[test]
fn generated_function_argument_context_classification_matches_model_without_panics() {
    let scalar_routes = [
        ("direct", "scalar", 0, 3),
        ("comma", "(marker++, scalar)", 1, 3),
        ("assignment", "scalar = (marker++, 5)", 1, 5),
        ("compound assignment", "scalar += (marker++, 2)", 1, 5),
        ("call", "scalar_call()", 1, 7),
        ("conditional", "1 ? (marker++, 8) : (marker += 20, 9)", 1, 8),
        ("cast", "(int)(marker++, 9)", 1, 9),
        ("nested comma", "((marker++, 1), (marker++, 4))", 2, 4),
    ];
    let pointer_routes = [
        ("direct", "cursor", 0, 1),
        ("arithmetic", "(marker++, cursor + 1)", 1, 2),
        ("assignment", "cursor = (marker++, values + 2)", 1, 3),
        ("call", "(marker++, forward(values + 3))", 1, 4),
        (
            "conditional",
            "1 ? (marker++, values + 1) : (marker += 20, cursor)",
            1,
            2,
        ),
        ("comma", "(marker++, values + 2)", 1, 3),
        ("cast", "(marker++, (int *)(values + 3))", 1, 4),
        (
            "field assignment",
            "holder.primary = (marker++, values + 1)",
            1,
            2,
        ),
    ];
    let aggregate_routes = [
        ("direct", "point", 0, 3),
        ("comma", "(marker++, point)", 1, 3),
        ("assignment", "target = (marker++, replacement)", 1, 5),
        (
            "conditional",
            "1 ? (marker++, replacement) : (marker += 20, point)",
            1,
            5,
        ),
        ("call", "make_point()", 1, 7),
        ("compound literal", "(marker++, (struct Point){8})", 1, 8),
        ("dereference", "(marker++, *slot)", 1, 3),
        (
            "dereference assignment",
            "*slot = (marker++, replacement)",
            1,
            5,
        ),
        (
            "direct field assignment",
            "box.point = (marker++, replacement)",
            1,
            5,
        ),
        (
            "arrow field assignment",
            "view->point = (marker++, replacement)",
            1,
            5,
        ),
        (
            "array element assignment",
            "points[0] = (marker++, replacement)",
            1,
            5,
        ),
        (
            "embedded array element assignment",
            "box.points[0] = (marker++, replacement)",
            1,
            5,
        ),
    ];

    for (case_index, (label, expression, marker_increments, value)) in
        scalar_routes.iter().enumerate()
    {
        assert_interpretation(
            &scalar_argument_context_program(expression),
            ExpectedInterpretation::Value(marker_increments * 10 + value),
            &format!("scalar argument case {case_index} ({label})"),
        );
    }
    for (case_index, (label, expression, marker_increments, value)) in
        pointer_routes.iter().enumerate()
    {
        assert_interpretation(
            &pointer_argument_context_program(expression),
            ExpectedInterpretation::Value(marker_increments * 10 + value),
            &format!("pointer argument case {case_index} ({label})"),
        );
    }
    for (case_index, (label, expression, marker_increments, value)) in
        aggregate_routes.iter().enumerate()
    {
        assert_interpretation(
            &aggregate_argument_context_program(expression),
            ExpectedInterpretation::Value(marker_increments * 10 + value),
            &format!("aggregate argument case {case_index} ({label})"),
        );
    }

    assert_eq!(scalar_routes.len(), 8);
    assert_eq!(pointer_routes.len(), 8);
    assert_eq!(aggregate_routes.len(), 12);
}

#[test]
fn generated_function_argument_context_reports_shape_and_type_mismatches_without_panics() {
    let cases = [
        (
            "scalar from pointer",
            "int reject(int value) { return value; } int main(void) { int values[1] = {1}; int *cursor = values; return reject(cursor); }",
            "pointer 'cursor' used as scalar",
        ),
        (
            "scalar from aggregate",
            "int reject(int value) { return value; } int main(void) { struct Point point = {1}; return reject(point); }",
            "struct variable 'point' used as scalar",
        ),
        (
            "scalar from void",
            "void touch(void) { return; } int reject(int value) { return value; } int main(void) { return reject(touch()); }",
            "void function 'touch' used as scalar expression",
        ),
        (
            "pointer from scalar",
            "int reject(int *value) { return *value; } int main(void) { return reject(7); }",
            "expected pointer expression",
        ),
        (
            "pointer from aggregate",
            "int reject(int *value) { return *value; } int main(void) { struct Point point = {1}; return reject(point); }",
            "struct variable 'point' used as pointer",
        ),
        (
            "pointer from void",
            "void touch(void) { return; } int reject(int *value) { return *value; } int main(void) { return reject(touch()); }",
            "void function 'touch' used as pointer expression",
        ),
        (
            "aggregate from scalar",
            "int reject(struct Point point) { return point.x; } int main(void) { return reject(7); }",
            "function 'reject' struct parameter 'point' requires a struct argument",
        ),
        (
            "aggregate from pointer",
            "int reject(struct Point point) { return point.x; } int main(void) { struct Point point = {1}; struct Point *slot = &point; return reject(slot); }",
            "function 'reject' struct parameter 'point' requires a struct argument",
        ),
        (
            "aggregate from void",
            "void touch(void) { return; } int reject(struct Point point) { return point.x; } int main(void) { return reject(touch()); }",
            "void function 'touch' used as struct expression",
        ),
        (
            "pointer const discard",
            "int reject(int *value) { return *value; } int main(void) { const int values[1] = {1}; return reject(values); }",
            "cannot discard const qualifier from pointer target",
        ),
        (
            "pointer type mismatch",
            "int reject(char *value) { return *value; } int main(void) { int values[1] = {1}; return reject(values); }",
            "cannot convert pointer to int to pointer to char",
        ),
        (
            "aggregate type mismatch",
            "int reject(struct Point point) { return point.x; } int main(void) { struct Pair pair = {1}; return reject(pair); }",
            "function 'reject' struct parameter 'point' expected struct 'Point', got struct 'Pair'",
        ),
    ];

    for (case_index, (label, body, expected)) in cases.iter().enumerate() {
        let source = format!("struct Point {{ int x; }};\nstruct Pair {{ int x; }};\n{body}\n");
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error(expected),
            &format!("argument mismatch case {case_index} ({label})"),
        );
    }
}

#[test]
fn generated_declaration_initializer_and_assignment_rhs_contexts_match_model_without_panics() {
    let scalar_routes = [
        ("direct", "scalar", 0, 3),
        ("comma", "(marker++, scalar)", 1, 3),
        ("assignment", "scalar = (marker++, 5)", 1, 5),
        ("compound assignment", "scalar += (marker++, 2)", 1, 5),
        ("call", "scalar_call()", 1, 7),
        ("conditional", "1 ? (marker++, 8) : (marker += 20, 9)", 1, 8),
        ("cast", "(int)(marker++, 9)", 1, 9),
        ("nested comma", "((marker++, 1), (marker++, 4))", 2, 4),
    ];
    let pointer_routes = [
        ("direct", "cursor", 0, 1),
        ("arithmetic", "(marker++, cursor + 1)", 1, 2),
        ("assignment", "cursor = (marker++, values + 2)", 1, 3),
        ("call", "(marker++, forward(values + 3))", 1, 4),
        (
            "conditional",
            "1 ? (marker++, values + 1) : (marker += 20, cursor)",
            1,
            2,
        ),
        ("comma", "(marker++, values + 2)", 1, 3),
        ("cast", "(marker++, (int *)(values + 3))", 1, 4),
        (
            "direct field assignment",
            "holder.primary = (marker++, values + 1)",
            1,
            2,
        ),
        (
            "arrow field assignment",
            "holder_view->primary = (marker++, values + 2)",
            1,
            3,
        ),
    ];
    let aggregate_routes = [
        ("direct", "point", 0, 3),
        ("comma", "(marker++, point)", 1, 3),
        ("assignment", "target = (marker++, replacement)", 1, 5),
        (
            "conditional",
            "1 ? (marker++, replacement) : (marker += 20, point)",
            1,
            5,
        ),
        ("call", "make_point()", 1, 7),
        ("compound literal", "(marker++, (struct Point){8})", 1, 8),
        ("dereference", "(marker++, *slot)", 1, 3),
        (
            "dereference assignment",
            "*slot = (marker++, replacement)",
            1,
            5,
        ),
        (
            "direct field assignment",
            "box.point = (marker++, replacement)",
            1,
            5,
        ),
        (
            "arrow field assignment",
            "view->point = (marker++, replacement)",
            1,
            5,
        ),
        (
            "array element assignment",
            "points[0] = (marker++, replacement)",
            1,
            5,
        ),
        (
            "embedded array element assignment",
            "box.points[0] = (marker++, replacement)",
            1,
            5,
        ),
    ];

    for context in 0..2 {
        for (case_index, (label, expression, marker_increments, value)) in
            scalar_routes.iter().enumerate()
        {
            assert_interpretation(
                &scalar_initializer_or_rhs_program(expression, context),
                ExpectedInterpretation::Value(marker_increments * 10 + value),
                &format!("scalar context {context} case {case_index} ({label})"),
            );
        }
        for (case_index, (label, expression, marker_increments, value)) in
            pointer_routes.iter().enumerate()
        {
            assert_interpretation(
                &pointer_initializer_or_rhs_program(expression, context),
                ExpectedInterpretation::Value(marker_increments * 10 + value),
                &format!("pointer context {context} case {case_index} ({label})"),
            );
        }
        for (case_index, (label, expression, marker_increments, value)) in
            aggregate_routes.iter().enumerate()
        {
            assert_interpretation(
                &aggregate_initializer_or_rhs_program(expression, context),
                ExpectedInterpretation::Value(marker_increments * 10 + value),
                &format!("aggregate context {context} case {case_index} ({label})"),
            );
        }
    }

    assert_eq!(scalar_routes.len() * 2, 16);
    assert_eq!(pointer_routes.len() * 2, 18);
    assert_eq!(aggregate_routes.len() * 2, 24);
}

#[test]
fn generated_declaration_initializer_and_assignment_rhs_mismatches_do_not_panic() {
    let cases = [
        (
            "scalar initializer from pointer",
            "int values[1] = {1}; int *cursor = values; int result = cursor;",
            "pointer 'cursor' used as scalar",
        ),
        (
            "scalar assignment from pointer",
            "int values[1] = {1}; int *cursor = values; int result = 0; result = cursor;",
            "pointer 'cursor' used as scalar",
        ),
        (
            "scalar initializer from aggregate",
            "struct Point point = {1}; int result = point;",
            "struct variable 'point' used as scalar",
        ),
        (
            "scalar assignment from aggregate",
            "struct Point point = {1}; int result = 0; result = point;",
            "struct variable 'point' used as scalar",
        ),
        (
            "scalar initializer from void",
            "int result = touch();",
            "void function 'touch' used as scalar expression",
        ),
        (
            "scalar assignment from void",
            "int result = 0; result = touch();",
            "void function 'touch' used as scalar expression",
        ),
        (
            "pointer initializer from scalar",
            "int *result = 7;",
            "expected pointer expression",
        ),
        (
            "pointer assignment from scalar",
            "int values[1] = {1}; int *result = values; result = 7;",
            "expected pointer expression",
        ),
        (
            "pointer initializer from aggregate",
            "struct Point point = {1}; int *result = point;",
            "struct variable 'point' used as pointer",
        ),
        (
            "pointer assignment from aggregate",
            "struct Point point = {1}; int values[1] = {1}; int *result = values; result = point;",
            "struct variable 'point' used as pointer",
        ),
        (
            "pointer initializer from void",
            "int *result = touch();",
            "void function 'touch' used as pointer expression",
        ),
        (
            "pointer assignment from void",
            "int values[1] = {1}; int *result = values; result = touch();",
            "void function 'touch' used as pointer expression",
        ),
        (
            "aggregate initializer from scalar",
            "struct Point result = 7;",
            "expected struct expression",
        ),
        (
            "aggregate assignment from scalar",
            "struct Point result = {1}; result = 7;",
            "expected struct expression",
        ),
        (
            "aggregate initializer from pointer",
            "struct Point point = {1}; struct Point *slot = &point; struct Point result = slot;",
            "variable 'slot' is not a struct",
        ),
        (
            "aggregate assignment from pointer",
            "struct Point result = {1}; struct Point *slot = &result; result = slot;",
            "variable 'slot' is not a struct",
        ),
        (
            "aggregate initializer from void",
            "struct Point result = touch();",
            "void function 'touch' used as struct expression",
        ),
        (
            "aggregate assignment from void",
            "struct Point result = {1}; result = touch();",
            "void function 'touch' used as struct expression",
        ),
        (
            "pointer initializer const discard",
            "const int values[1] = {1}; int *result = values;",
            "cannot discard const qualifier from pointer target",
        ),
        (
            "pointer assignment const discard",
            "const int values[1] = {1}; int mutable[1] = {1}; int *result = mutable; result = values;",
            "cannot discard const qualifier from pointer target",
        ),
        (
            "pointer initializer type mismatch",
            "int values[1] = {1}; char *result = values;",
            "cannot convert pointer to int to pointer to char",
        ),
        (
            "pointer assignment type mismatch",
            "int values[1] = {1}; char chars[1] = {'a'}; char *result = chars; result = values;",
            "cannot convert pointer to int to pointer to char",
        ),
        (
            "aggregate initializer type mismatch",
            "struct Pair pair = {1}; struct Point result = pair;",
            "cannot assign struct 'Pair' to struct 'Point'",
        ),
        (
            "aggregate assignment type mismatch",
            "struct Pair pair = {1}; struct Point result = {1}; result = pair;",
            "cannot assign struct 'Pair' to struct 'Point'",
        ),
    ];

    for (case_index, (label, body, expected)) in cases.iter().enumerate() {
        let source = format!(
            "struct Point {{ int x; }};\nstruct Pair {{ int x; }};\nvoid touch(void) {{ return; }}\nint main(void) {{ {body} return 0; }}\n"
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error(expected),
            &format!("initializer/RHS mismatch case {case_index} ({label})"),
        );
    }
}

#[test]
fn generated_static_assert_conditions_reject_non_integer_shapes_without_panics() {
    let cases = [
        (
            "pointer",
            "int values[1] = {1}; int *cursor = values; _Static_assert(cursor, \"pointer\");",
            "pointer 'cursor' used as scalar",
        ),
        (
            "aggregate",
            "struct Point point = {1}; _Static_assert(point, \"aggregate\");",
            "struct variable 'point' used as scalar",
        ),
        (
            "void",
            "_Static_assert(touch(), \"void\");",
            "void function 'touch' used as scalar expression",
        ),
    ];

    for (case_index, (label, body, expected)) in cases.iter().enumerate() {
        let source = format!(
            "struct Point {{ int x; }};\nvoid touch(void) {{ return; }}\nint main(void) {{ {body} return 0; }}\n"
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error(expected),
            &format!("static assertion shape case {case_index} ({label})"),
        );
    }
}

#[test]
fn generated_scalar_only_operand_contexts_match_model_without_panics() {
    let scalar_routes = [
        ("direct", "1", 0),
        ("conditional", "1 ? (marker++, 1) : (marker += 20, 0)", 1),
        ("comma", "(marker++, 1)", 1),
        ("assignment", "scalar = (marker++, 1)", 1),
        ("call", "scalar_call()", 1),
        ("cast", "(int)(marker++, 1)", 1),
        (
            "pointer difference",
            "(marker++, (right_ptr + 1) - right_ptr)",
            1,
        ),
    ];
    let context_values = [4, 4, 20, 20, 4, 4, 2, 7, 9];

    for (context, expected_value) in context_values.into_iter().enumerate() {
        for (case_index, (label, expression, marker_increments)) in scalar_routes.iter().enumerate()
        {
            assert_interpretation(
                &scalar_only_operand_context_program(expression, context),
                ExpectedInterpretation::Value(marker_increments * 100 + expected_value),
                &format!("scalar-only context {context} case {case_index} ({label})"),
            );
        }
    }

    assert_eq!(scalar_routes.len() * context_values.len(), 63);
}

#[test]
fn generated_scalar_only_operand_shape_mismatches_do_not_panic() {
    let shapes = [
        ("pointer", "cursor", "pointer 'cursor' used as scalar"),
        (
            "aggregate",
            "point",
            "struct variable 'point' used as scalar",
        ),
        (
            "void",
            "touch()",
            "void function 'touch' used as scalar expression",
        ),
    ];

    for context in 0..5 {
        for (case_index, (label, expression, scalar_error)) in shapes.iter().enumerate() {
            let expected = if context == 2 && *label == "pointer" {
                "cannot add two pointers"
            } else {
                scalar_error
            };
            assert_interpretation(
                &scalar_only_shape_mismatch_program(expression, context),
                ExpectedInterpretation::Error(expected),
                &format!("scalar-only mismatch context {context} case {case_index} ({label})"),
            );
        }
    }
}

#[test]
fn generated_array_and_aggregate_initializer_elements_match_model_without_panics() {
    let scalar_routes = [
        ("direct", "scalar", 0, 3),
        ("comma", "(marker++, scalar)", 1, 3),
        ("assignment", "scalar = (marker++, 5)", 1, 5),
        ("compound assignment", "scalar += (marker++, 2)", 1, 5),
        ("call", "scalar_call()", 1, 7),
        ("conditional", "1 ? (marker++, 8) : (marker += 20, 9)", 1, 8),
        ("cast", "(int)(marker++, 9)", 1, 9),
        ("nested comma", "((marker++, 1), (marker++, 4))", 2, 4),
    ];
    let pointer_routes = [
        ("direct", "cursor", 0, 1),
        ("arithmetic", "(marker++, cursor + 1)", 1, 2),
        ("assignment", "cursor = (marker++, values + 2)", 1, 3),
        ("call", "(marker++, forward(values + 3))", 1, 4),
        (
            "conditional",
            "1 ? (marker++, values + 1) : (marker += 20, cursor)",
            1,
            2,
        ),
        ("comma", "(marker++, values + 2)", 1, 3),
        ("cast", "(marker++, (int *)(values + 3))", 1, 4),
        (
            "direct field assignment",
            "source.primary = (marker++, values + 1)",
            1,
            2,
        ),
        (
            "arrow field assignment",
            "source_view->primary = (marker++, values + 2)",
            1,
            3,
        ),
    ];
    let aggregate_routes = [
        ("direct", "point", 0, 3),
        ("comma", "(marker++, point)", 1, 3),
        ("assignment", "target = (marker++, replacement)", 1, 5),
        (
            "conditional",
            "1 ? (marker++, replacement) : (marker += 20, point)",
            1,
            5,
        ),
        ("call", "make_point()", 1, 7),
        ("compound literal", "(marker++, (struct Point){8})", 1, 8),
        ("dereference", "(marker++, *slot)", 1, 3),
        (
            "dereference assignment",
            "*slot = (marker++, replacement)",
            1,
            5,
        ),
        (
            "direct field assignment",
            "source.point = (marker++, replacement)",
            1,
            5,
        ),
        (
            "arrow field assignment",
            "source_view->point = (marker++, replacement)",
            1,
            5,
        ),
        (
            "array element assignment",
            "points[0] = (marker++, replacement)",
            1,
            5,
        ),
        (
            "embedded array element assignment",
            "source.points[0] = (marker++, replacement)",
            1,
            5,
        ),
    ];

    for context in 0..10 {
        for (case_index, (label, expression, marker_increments, value)) in
            scalar_routes.iter().enumerate()
        {
            assert_interpretation(
                &scalar_initializer_element_program(expression, context),
                ExpectedInterpretation::Value(marker_increments * 10 + value),
                &format!(
                    "scalar initializer element context {context} case {case_index} ({label})"
                ),
            );
        }
    }
    for context in 0..2 {
        for (case_index, (label, expression, marker_increments, value)) in
            pointer_routes.iter().enumerate()
        {
            assert_interpretation(
                &pointer_field_initializer_program(expression, context),
                ExpectedInterpretation::Value(marker_increments * 10 + value),
                &format!("pointer field initializer context {context} case {case_index} ({label})"),
            );
        }
        for (case_index, (label, expression, marker_increments, value)) in
            aggregate_routes.iter().enumerate()
        {
            assert_interpretation(
                &aggregate_field_initializer_program(expression, context),
                ExpectedInterpretation::Value(marker_increments * 10 + value),
                &format!(
                    "aggregate field initializer context {context} case {case_index} ({label})"
                ),
            );
        }
    }

    assert_interpretation(
        "int marker = 0;\n\
         int stamp(int digit) { marker = marker * 10 + digit; return digit; }\n\
         struct Triple { int first; int second; int third; };\n\
         int main(void) {\n\
         int values[3] = {[2] = stamp(1), [0] = stamp(2), stamp(3)};\n\
         struct Triple triple = {.third = stamp(4), .first = stamp(5), .second = stamp(6)};\n\
         return (marker == 123456) * 100 + values[0] * 10 + values[1] + values[2]\
             + triple.first + triple.second + triple.third;\n\
         }\n",
        ExpectedInterpretation::Value(139),
        "positional and designated initializer elements execute once in source order",
    );

    assert_eq!(scalar_routes.len() * 10, 80);
    assert_eq!(pointer_routes.len() * 2, 18);
    assert_eq!(aggregate_routes.len() * 2, 24);
}

#[test]
fn generated_array_and_aggregate_initializer_element_mismatches_do_not_panic() {
    let cases = [
        (
            "root scalar array element from pointer",
            "int values[1] = {1}; int *cursor = values; int result[1] = {cursor};",
            "pointer 'cursor' used as scalar",
        ),
        (
            "designated scalar array element from aggregate",
            "struct Point point = {1}; int result[1] = {[0] = point};",
            "struct variable 'point' used as scalar",
        ),
        (
            "embedded scalar array element from void",
            "struct ScalarArray box = {{touch()}};",
            "void function 'touch' used as scalar expression",
        ),
        (
            "aggregate scalar field from pointer",
            "int values[1] = {1}; int *cursor = values; struct ScalarHolder result = {.scalar = cursor};",
            "pointer 'cursor' used as scalar",
        ),
        (
            "aggregate scalar field from aggregate",
            "struct Point point = {1}; struct ScalarHolder result = {point};",
            "struct variable 'point' used as scalar",
        ),
        (
            "aggregate scalar field from void",
            "struct ScalarHolder result = {.scalar = touch()};",
            "void function 'touch' used as scalar expression",
        ),
        (
            "aggregate array element field from pointer",
            "int values[1] = {1}; int *cursor = values; struct Point result[1] = {{cursor}};",
            "pointer 'cursor' used as scalar",
        ),
        (
            "designated embedded aggregate array element field from aggregate",
            "struct Point point = {1}; struct Container result = {.points = {[0] = {point}}};",
            "struct variable 'point' used as scalar",
        ),
        (
            "pointer field from scalar",
            "struct Container result = {.pointer = 7};",
            "expected pointer expression",
        ),
        (
            "pointer field from aggregate",
            "struct Point point = {1}; struct Container result = {.pointer = point};",
            "struct variable 'point' used as pointer",
        ),
        (
            "pointer field from void",
            "struct Container result = {.pointer = touch()};",
            "void function 'touch' used as pointer expression",
        ),
        (
            "pointer field const discard",
            "const int values[1] = {1}; struct Container result = {.pointer = values};",
            "cannot discard const qualifier from pointer target",
        ),
        (
            "pointer field type mismatch",
            "char chars[1] = {'a'}; struct Container result = {.pointer = chars};",
            "cannot convert pointer to char to pointer to int",
        ),
        (
            "aggregate field from scalar",
            "struct Container result = {.point = 7};",
            "expected struct expression",
        ),
        (
            "aggregate field from pointer",
            "struct Point point = {1}; struct Point *slot = &point; struct Container result = {.point = slot};",
            "variable 'slot' is not a struct",
        ),
        (
            "aggregate field from void",
            "struct Container result = {.point = touch()};",
            "void function 'touch' used as struct expression",
        ),
        (
            "aggregate field type mismatch",
            "struct Pair pair = {1}; struct Container result = {.point = pair};",
            "cannot initialize struct field 'point' of type 'Point' with struct 'Pair'",
        ),
    ];

    for (case_index, (label, body, expected)) in cases.iter().enumerate() {
        let source = format!(
            "struct Point {{ int x; }};\n\
             struct Pair {{ int x; }};\n\
             struct ScalarArray {{ int values[1]; }};\n\
             struct ScalarHolder {{ int scalar; }};\n\
             struct Container {{ int scalar; int *pointer; struct Point point; struct Point points[1]; }};\n\
             void touch(void) {{ return; }}\n\
             int main(void) {{ {body} return 0; }}\n"
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error(expected),
            &format!("initializer element mismatch case {case_index} ({label})"),
        );
    }
}

#[test]
fn generated_nonvoid_return_context_reports_result_shape_mismatches_without_panics() {
    let cases = [
        (
            "scalar from pointer",
            "int reject_value(void) { int values[1] = {1}; int *cursor = values; return cursor; }",
            "pointer 'cursor' used as scalar",
        ),
        (
            "scalar from aggregate",
            "int reject_value(void) { struct Point point = {1}; return point; }",
            "struct variable 'point' used as scalar",
        ),
        (
            "pointer from scalar",
            "int *reject_value(void) { return 7; }",
            "expected pointer expression",
        ),
        (
            "pointer from aggregate",
            "int *reject_value(void) { struct Point point = {1}; return point; }",
            "struct variable 'point' used as pointer",
        ),
        (
            "aggregate from scalar",
            "struct Point reject_value(void) { return 7; }",
            "expected struct expression",
        ),
        (
            "aggregate from pointer",
            "struct Point reject_value(void) { int values[1] = {1}; int *cursor = values; return cursor; }",
            "variable 'cursor' is not a struct",
        ),
        (
            "pointer const discard",
            "int *reject_value(void) { const int values[1] = {1}; return values; }",
            "cannot discard const qualifier from pointer target",
        ),
        (
            "pointer type mismatch",
            "char *reject_value(void) { int values[1] = {1}; return values; }",
            "cannot convert pointer to int to pointer to char",
        ),
    ];

    for (case_index, (label, function, expected)) in cases.iter().enumerate() {
        let source = format!(
            "struct Point {{ int x; }};\n{function}\nint main(void) {{ reject_value(); return 0; }}\n"
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error(expected),
            &format!("nonvoid return case {case_index} ({label})"),
        );
    }
}

#[test]
fn generated_empty_return_context_reports_function_shape_mismatches_without_panics() {
    let cases = [
        (
            "scalar",
            "int reject_value(void) { return; }",
            "int function 'reject_value' returned without a value",
        ),
        (
            "pointer",
            "int *reject_value(void) { return; }",
            "pointer function 'reject_value' returned without a value",
        ),
        (
            "aggregate",
            "struct Point reject_value(void) { return; }",
            "struct function 'reject_value' returned without a value",
        ),
    ];

    for (case_index, (label, function, expected)) in cases.iter().enumerate() {
        let source = format!(
            "struct Point {{ int x; }};\n{function}\nint main(void) {{ reject_value(); return 0; }}\n"
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error(expected),
            &format!("empty return case {case_index} ({label})"),
        );
    }
}

fn scalar_return_context_program(expression: &str) -> String {
    format!(
        "int marker = 0;\n\
         int scalar_call(void) {{ marker++; return 7; }}\n\
         int produce(void) {{ int scalar = 3; return {expression}; }}\n\
         int main(void) {{ int result = produce(); return marker * 10 + result; }}\n"
    )
}

fn pointer_return_context_program(expression: &str) -> String {
    format!(
        "struct Holder {{ int *primary; }};\n\
         int marker = 0;\n\
         int values[4] = {{1, 2, 3, 4}};\n\
         int *forward(int *value) {{ return value; }}\n\
         int *produce(void) {{\n\
         int *cursor = values;\n\
         struct Holder holder = {{values}};\n\
         return {expression};\n\
         }}\n\
         int main(void) {{ int *result = produce(); return marker * 10 + *result; }}\n"
    )
}

fn aggregate_return_context_program(expression: &str) -> String {
    format!(
        "struct Point {{ int x; }};\n\
         int marker = 0;\n\
         struct Point make_point(void) {{ marker++; return (struct Point){{7}}; }}\n\
         struct Point produce(void) {{\n\
         struct Point point = {{3}};\n\
         struct Point replacement = {{5}};\n\
         struct Point *slot = &point;\n\
         return {expression};\n\
         }}\n\
         int main(void) {{ struct Point result = produce(); return marker * 10 + result.x; }}\n"
    )
}

fn void_return_context_program(expression: &str) -> String {
    format!(
        "struct Point {{ int x; }};\n\
         int marker = 0;\n\
         void touch(void) {{ marker++; }}\n\
         void reject_value(void) {{\n\
         int values[2] = {{1, 2}};\n\
         int *cursor = values;\n\
         struct Point point = {{3}};\n\
         return {expression};\n\
         }}\n\
         int main(void) {{ reject_value(); return marker; }}\n"
    )
}

fn scalar_argument_context_program(expression: &str) -> String {
    format!(
        "int marker = 0;\n\
         int scalar_call(void) {{ marker++; return 7; }}\n\
         int consume_scalar(int value) {{ return value; }}\n\
         int main(void) {{\n\
         int scalar = 3;\n\
         int result = consume_scalar({expression});\n\
         return marker * 10 + result;\n\
         }}\n"
    )
}

fn pointer_argument_context_program(expression: &str) -> String {
    format!(
        "struct Holder {{ int *primary; }};\n\
         int marker = 0;\n\
         int *forward(int *value) {{ return value; }}\n\
         int consume_pointer(int *value) {{ return *value; }}\n\
         int main(void) {{\n\
         int values[4] = {{1, 2, 3, 4}};\n\
         int *cursor = values;\n\
         struct Holder holder = {{values}};\n\
         int result = consume_pointer({expression});\n\
         return marker * 10 + result;\n\
         }}\n"
    )
}

fn aggregate_argument_context_program(expression: &str) -> String {
    format!(
        "struct Point {{ int x; }};\n\
         struct Box {{ struct Point point; struct Point points[1]; }};\n\
         int marker = 0;\n\
         struct Point make_point(void) {{ marker++; return (struct Point){{7}}; }}\n\
         int consume_point(struct Point value) {{ return value.x; }}\n\
         int main(void) {{\n\
         struct Point point = {{3}};\n\
         struct Point target = {{3}};\n\
         struct Point replacement = {{5}};\n\
         struct Point *slot = &target;\n\
         struct Point points[1] = {{{{4}}}};\n\
         struct Box box = {{{{4}}, {{{{6}}}}}};\n\
         struct Box *view = &box;\n\
         int result = consume_point({expression});\n\
         return marker * 10 + result;\n\
         }}\n"
    )
}

fn scalar_initializer_or_rhs_program(expression: &str, context: usize) -> String {
    let statement = match context {
        0 => format!("int result = {expression};"),
        1 => format!("int result = 0; result = {expression};"),
        _ => unreachable!(),
    };
    format!(
        "int marker = 0;\n\
         int scalar_call(void) {{ marker++; return 7; }}\n\
         int main(void) {{ int scalar = 3; {statement} return marker * 10 + result; }}\n"
    )
}

fn pointer_initializer_or_rhs_program(expression: &str, context: usize) -> String {
    let statement = match context {
        0 => format!("int *result = {expression};"),
        1 => format!("int *result = values; result = {expression};"),
        _ => unreachable!(),
    };
    format!(
        "struct Holder {{ int *primary; }};\n\
         int marker = 0;\n\
         int *forward(int *value) {{ return value; }}\n\
         int main(void) {{\n\
         int values[4] = {{1, 2, 3, 4}};\n\
         int *cursor = values;\n\
         struct Holder holder = {{values}};\n\
         struct Holder *holder_view = &holder;\n\
         {statement}\n\
         return marker * 10 + *result;\n\
         }}\n"
    )
}

fn aggregate_initializer_or_rhs_program(expression: &str, context: usize) -> String {
    let statement = match context {
        0 => format!("struct Point result = {expression};"),
        1 => format!("struct Point result = {{0}}; result = {expression};"),
        _ => unreachable!(),
    };
    format!(
        "struct Point {{ int x; }};\n\
         struct Box {{ struct Point point; struct Point points[1]; }};\n\
         int marker = 0;\n\
         struct Point make_point(void) {{ marker++; return (struct Point){{7}}; }}\n\
         int main(void) {{\n\
         struct Point point = {{3}};\n\
         struct Point target = {{3}};\n\
         struct Point replacement = {{5}};\n\
         struct Point *slot = &target;\n\
         struct Point points[1] = {{{{4}}}};\n\
         struct Box box = {{{{4}}, {{{{6}}}}}};\n\
         struct Box *view = &box;\n\
         {statement}\n\
         return marker * 10 + result.x;\n\
         }}\n"
    )
}

fn scalar_only_operand_context_program(expression: &str, context: usize) -> String {
    let statement = match context {
        0 => format!("int result = left[{expression}];"),
        1 => format!("int result = ({expression})[left];"),
        2 => format!("int result = point_ptr[{expression}].x;"),
        3 => format!("int result = ({expression})[point_ptr].x;"),
        4 => format!("int *selected = left_ptr + ({expression}); int result = *selected;"),
        5 => format!("int *selected = ({expression}) + left_ptr; int result = *selected;"),
        6 => format!("int *selected = (left_ptr + 1) - ({expression}); int result = *selected;"),
        7 => format!(
            "int result = 0; switch ({expression}) {{ case 0: result = 3; break; case 1: result = 7; break; default: result = 9; }}"
        ),
        8 => format!("_Static_assert({expression}, \"scalar route\"); int result = 9;"),
        _ => unreachable!(),
    };
    format!(
        "struct Point {{ int x; }};\n\
         int marker = 0;\n\
         int scalar_call(void) {{ marker++; return 1; }}\n\
         int main(void) {{\n\
         int scalar = 1;\n\
         int left[4] = {{2, 4, 6, 8}};\n\
         int right[4] = {{1, 3, 5, 7}};\n\
         int *left_ptr = left;\n\
         int *right_ptr = right;\n\
         struct Point points[4] = {{{{10}}, {{20}}, {{30}}, {{40}}}};\n\
         struct Point *point_ptr = points;\n\
         {statement}\n\
         return marker * 100 + result;\n\
         }}\n"
    )
}

fn scalar_only_shape_mismatch_program(expression: &str, context: usize) -> String {
    let statement = match context {
        0 => format!("int result = left[{expression}];"),
        1 => format!("int result = point_ptr[{expression}].x;"),
        2 => format!("int *selected = left_ptr + ({expression}); int result = *selected;"),
        3 => format!("int result = 0; switch ({expression}) {{ default: result = 1; break; }}"),
        4 => format!("_Static_assert({expression}, \"shape\"); int result = 0;"),
        _ => unreachable!(),
    };
    format!(
        "struct Point {{ int x; }};\n\
         void touch(void) {{ return; }}\n\
         int main(void) {{\n\
         int left[2] = {{1, 2}};\n\
         int *left_ptr = left;\n\
         int *cursor = left;\n\
         struct Point points[2] = {{{{1}}, {{2}}}};\n\
         struct Point *point_ptr = points;\n\
         struct Point point = {{1}};\n\
         {statement}\n\
         return result;\n\
         }}\n"
    )
}

fn scalar_initializer_element_program(expression: &str, context: usize) -> String {
    let statement = match context {
        0 => format!("int result[2] = {{{expression}, 4}};"),
        1 => format!("int result[2] = {{[1] = 4, [0] = {expression}}};"),
        2 => format!("struct ScalarArray result = {{{{{expression}, 4}}}};"),
        3 => format!("struct ScalarArray result = {{.values = {{[1] = 4, [0] = {expression}}}}};"),
        4 => format!("struct ScalarHolder result = {{{expression}}};"),
        5 => format!("struct ScalarHolder result = {{.scalar = {expression}}};"),
        6 => format!("struct Point result[2] = {{{{{expression}}}, {{4}}}};"),
        7 => format!("struct Point result[2] = {{[1] = {{4}}, [0] = {{{expression}}}}};"),
        8 => format!("struct AggregateArray result = {{{{{{{expression}}}, {{4}}}}}};"),
        9 => format!(
            "struct AggregateArray result = {{.points = {{[1] = {{4}}, [0] = {{{expression}}}}}}};"
        ),
        _ => unreachable!(),
    };
    let result = match context {
        0 | 1 => "result[0]",
        2 | 3 => "result.values[0]",
        4 | 5 => "result.scalar",
        6 | 7 => "result[0].x",
        8 | 9 => "result.points[0].x",
        _ => unreachable!(),
    };
    format!(
        "struct ScalarArray {{ int values[2]; }};\n\
         struct ScalarHolder {{ int scalar; }};\n\
         struct Point {{ int x; }};\n\
         struct AggregateArray {{ struct Point points[2]; }};\n\
         int marker = 0;\n\
         int scalar_call(void) {{ marker++; return 7; }}\n\
         int main(void) {{ int scalar = 3; {statement} return marker * 10 + {result}; }}\n"
    )
}

fn pointer_field_initializer_program(expression: &str, context: usize) -> String {
    let statement = match context {
        0 => format!("struct PointerHolder result = {{{expression}}};"),
        1 => format!("struct PointerHolder result = {{.pointer = {expression}}};"),
        _ => unreachable!(),
    };
    format!(
        "struct Source {{ int *primary; }};\n\
         struct PointerHolder {{ int *pointer; }};\n\
         int marker = 0;\n\
         int *forward(int *value) {{ return value; }}\n\
         int main(void) {{\n\
         int values[4] = {{1, 2, 3, 4}};\n\
         int *cursor = values;\n\
         struct Source source = {{values}};\n\
         struct Source *source_view = &source;\n\
         {statement}\n\
         return marker * 10 + *result.pointer;\n\
         }}\n"
    )
}

fn aggregate_field_initializer_program(expression: &str, context: usize) -> String {
    let statement = match context {
        0 => format!("struct Result result = {{{expression}}};"),
        1 => format!("struct Result result = {{.point = {expression}}};"),
        _ => unreachable!(),
    };
    format!(
        "struct Point {{ int x; }};\n\
         struct Source {{ struct Point point; struct Point points[1]; }};\n\
         struct Result {{ struct Point point; }};\n\
         int marker = 0;\n\
         struct Point make_point(void) {{ marker++; return (struct Point){{7}}; }}\n\
         int main(void) {{\n\
         struct Point point = {{3}};\n\
         struct Point target = {{3}};\n\
         struct Point replacement = {{5}};\n\
         struct Point *slot = &target;\n\
         struct Point points[1] = {{{{4}}}};\n\
         struct Source source = {{{{4}}, {{{{6}}}}}};\n\
         struct Source *source_view = &source;\n\
         {statement}\n\
         return marker * 10 + result.point.x;\n\
         }}\n"
    )
}

fn discard_context_program(expression: &str, context: usize) -> String {
    let statement = match context {
        0 => format!("{expression};"),
        1 => format!("(void)({expression});"),
        2 => format!("(({expression}), context_marker++);"),
        _ => unreachable!(),
    };
    format!(
        "struct Point {{ int x; }};\n\
         struct Holder {{ int *primary; struct Point point; }};\n\
         int marker = 0;\n\
         int scalar_call(void) {{ marker++; return 7; }}\n\
         void touch(void) {{ marker++; }}\n\
         void touch_noop(void) {{ return; }}\n\
         int *forward(int *value) {{ return value; }}\n\
         struct Point make_point(void) {{ struct Point result = {{5}}; return result; }}\n\
         int main(void) {{\n\
         int values[4] = {{1, 2, 3, 4}};\n\
         int *cursor = values;\n\
         int scalar = 3;\n\
         int context_marker = 0;\n\
         struct Point point = {{5}};\n\
         struct Point replacement = {{7}};\n\
         struct Point *point_slot = &point;\n\
         struct Holder holder = {{values, {{4}}}};\n\
         struct Holder *holder_view = &holder;\n\
         struct Holder box = {{values, {{6}}}};\n\
         {statement}\n\
         return marker * 10 + context_marker;\n\
         }}\n"
    )
}

fn discard_scalar_use_program(expression: &str) -> String {
    format!(
        "struct Point {{ int x; }};\n\
         void touch(void) {{ return; }}\n\
         int main(void) {{\n\
         int values[2] = {{1, 2}};\n\
         int *cursor = values;\n\
         struct Point point = {{3}};\n\
         return {expression};\n\
         }}\n"
    )
}

fn truthiness_pointer_operand(route: usize, truthy: bool, index: i64) -> EqualityOperand {
    if truthy {
        return equality_pointer_operand(route, EqualityRoot::Left, index);
    }

    let (rendered, marker_increments) = match route % 8 {
        0 => ("(int *)0".to_string(), 0),
        1 => ("forward((int *)0)".to_string(), 0),
        2 => ("(1 ? (int *)0 : left)".to_string(), 0),
        3 => ("(marker++, (int *)0)".to_string(), 1),
        4 => ("(cursor.primary = (int *)0)".to_string(), 0),
        5 => ("(view->primary = (int *)0)".to_string(), 0),
        6 => ("(int *)(int)0".to_string(), 0),
        _ => ("((int *)0)".to_string(), 0),
    };
    EqualityOperand {
        rendered,
        root: EqualityRoot::Null,
        index: 0,
        marker_increments,
    }
}

fn truthiness_expected(truthy: bool, operand_marker_increments: i64, context: usize) -> i64 {
    let score = match context % 6 {
        0 | 2 | 3 | 4 | 5 => i64::from(truthy) * 10,
        1 => i64::from(!truthy) * 10,
        _ => unreachable!(),
    };
    let context_marker_increments = match context % 6 {
        2 if truthy => 1,
        3 if !truthy => 1,
        _ => 0,
    };
    score + operand_marker_increments + context_marker_increments
}

fn truthiness_program(operand: &str, context: usize) -> String {
    let statement = match context % 6 {
        0 => format!("if ({operand}) {{ score = 10; }}"),
        1 => format!("score = !({operand}) * 10;"),
        2 => format!("score = (({operand}) && (marker++, 1)) * 10;"),
        3 => format!("score = (({operand}) || (marker++, 0)) * 10;"),
        4 => format!("score = (({operand}) ? 1 : 0) * 10;"),
        5 => format!("while (score == 0 && ({operand})) {{ score = 10; }}"),
        _ => unreachable!(),
    };
    format!(
        "struct Point {{ int value; }};\n\
         struct Inner {{ int values[4]; }};\n\
         struct Cursor {{ int *primary; int *secondary; struct Inner nested; }};\n\
         int *forward(int *value) {{ return value; }}\n\
         void noop(void) {{ return; }}\n\
         int main(void) {{\n\
         int left[4] = {{4, 7, 9, 12}};\n\
         int right[4] = {{4, 7, 9, 12}};\n\
         struct Point point = {{1}};\n\
         struct Cursor cursor = {{left, right, {{{{0, 1, 2, 3}}}}}};\n\
         struct Cursor *view = &cursor;\n\
         int marker = 0;\n\
         int score = 0;\n\
         {statement}\n\
         return score + marker;\n\
         }}\n"
    )
}

fn ordering_model(left: i64, operator: &str, right: i64) -> bool {
    match operator {
        "<" => left < right,
        "<=" => left <= right,
        ">" => left > right,
        ">=" => left >= right,
        _ => unreachable!("ordering model received unsupported operator"),
    }
}

fn equality_other_root(root: EqualityRoot) -> EqualityRoot {
    match root {
        EqualityRoot::Left => EqualityRoot::Right,
        EqualityRoot::Right => EqualityRoot::Left,
        EqualityRoot::Null => EqualityRoot::Left,
    }
}

fn equality_pointer_operand(route: usize, root: EqualityRoot, index: i64) -> EqualityOperand {
    if root == EqualityRoot::Null {
        return EqualityOperand {
            rendered: "0".to_string(),
            root,
            index: 0,
            marker_increments: 0,
        };
    }
    let name = root.name();
    let field = match root {
        EqualityRoot::Left => "primary",
        EqualityRoot::Right => "secondary",
        EqualityRoot::Null => unreachable!(),
    };
    let (rendered, marker_increments) = match route % 8 {
        0 => (format!("({name} + {index})"), 0),
        1 => (format!("&{name}[{index}]"), 0),
        2 => (format!("forward({name} + {index})"), 0),
        3 => (format!("cursor.{field} + {index}"), 0),
        4 => (format!("view->{field} + {index}"), 0),
        5 => (
            format!(
                "(1 ? {name} + {index} : {} + 3)",
                equality_other_root(root).name()
            ),
            0,
        ),
        6 => (format!("(marker++, {name} + {index})"), 1),
        _ => (format!("(int *)({name} + {index})"), 0),
    };
    EqualityOperand {
        rendered,
        root,
        index,
        marker_increments,
    }
}

impl EqualityRoot {
    fn name(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Null => "0",
        }
    }
}

fn equality_scalar_operand(route: usize, value: i64) -> (String, i64) {
    let index = value.rem_euclid(4);
    match route % 8 {
        0 => (value.to_string(), 0),
        1 => (format!("+({value})"), 0),
        2 => (format!("(int)({value})"), 0),
        3 => (format!("(1 ? {value} : {})", value + 1), 0),
        4 => (format!("(marker++, {value})"), 1),
        5 => (
            format!("view->nested.values[{index}] + ({})", value - index),
            0,
        ),
        6 => (
            format!("((left + {index}) - left) + ({})", value - index),
            0,
        ),
        _ => (format!("(marker++, +({value}))"), 1),
    }
}

fn equality_program(left: &str, op: &str, right: &str) -> String {
    format!(
        "struct Point {{ int value; }};\n\
         struct Inner {{ int values[4]; }};\n\
         struct Cursor {{ int *primary; int *secondary; struct Inner nested; }};\n\
         int *forward(int *value) {{ return value; }}\n\
         int main(void) {{\n\
         int left[4] = {{4, 7, 9, 12}};\n\
         int right[4] = {{4, 7, 9, 12}};\n\
         struct Point point = {{1}};\n\
         struct Cursor cursor = {{left, right, {{{{0, 1, 2, 3}}}}}};\n\
         struct Cursor *view = &cursor;\n\
         int marker = 0;\n\
         return (({left}) {op} ({right})) * 10 + marker;\n\
         }}\n"
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LiteralFieldAliasPattern {
    SameElement,
    SameRootDistinctElement,
    DifferentFieldRoot,
    DifferentLiteralRoot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LiteralPointerWrapper {
    Arithmetic,
    IndexedAddress,
    Conditional,
    Comma,
}

#[test]
fn generated_aggregate_compound_literal_field_pointer_alias_mutations_match_model_without_panics() {
    let mut state = 0xC057_117E_A11A_u64;
    let mut pattern_counts = [0; 4];
    let mut wrapper_counts = [0; 4];
    let mut one_hop_calls = 0;
    let mut two_hop_calls = 0;

    for kind in FieldBackedPointeeKind::ALL {
        for case_index in 0..24 {
            let writer = FieldBackedModelPointer {
                kind,
                owner: if next_u64(&mut state) & 1 == 0 {
                    FieldBackedOwner::Left
                } else {
                    FieldBackedOwner::Right
                },
                field: if next_u64(&mut state) & 1 == 0 {
                    FieldBackedField::Primary
                } else {
                    FieldBackedField::Secondary
                },
                index: (next_u64(&mut state) % EMBEDDED_ARRAY_LEN as u64) as i64,
                route: FieldBackedRoute::Direct,
            };
            let pattern = match case_index % 4 {
                0 => LiteralFieldAliasPattern::SameElement,
                1 => LiteralFieldAliasPattern::SameRootDistinctElement,
                2 => LiteralFieldAliasPattern::DifferentFieldRoot,
                _ => LiteralFieldAliasPattern::DifferentLiteralRoot,
            };
            pattern_counts[case_index % 4] += 1;
            let (second_owner, second_field, second_index) = match pattern {
                LiteralFieldAliasPattern::SameElement => (writer.owner, writer.field, writer.index),
                LiteralFieldAliasPattern::SameRootDistinctElement => (
                    writer.owner,
                    writer.field,
                    (writer.index + 1 + (next_u64(&mut state) % 3) as i64) % EMBEDDED_ARRAY_LEN,
                ),
                LiteralFieldAliasPattern::DifferentFieldRoot => {
                    (writer.owner, writer.field.other(), writer.index)
                }
                LiteralFieldAliasPattern::DifferentLiteralRoot => {
                    (writer.owner.other(), writer.field, writer.index)
                }
            };
            let second_writer = FieldBackedModelPointer {
                owner: second_owner,
                field: second_field,
                index: second_index,
                ..writer
            };
            let reader = if case_index & 4 == 0 {
                writer
            } else {
                second_writer
            };
            let wrappers = [
                literal_pointer_wrapper(case_index),
                literal_pointer_wrapper(case_index + 1),
                literal_pointer_wrapper(case_index + 2),
            ];
            for wrapper in wrappers {
                wrapper_counts[literal_pointer_wrapper_index(wrapper)] += 1;
            }
            let twice = [
                case_index & 1 == 0,
                case_index & 2 == 0,
                case_index & 4 == 0,
            ];
            one_hop_calls += twice.iter().filter(|value| !**value).count();
            two_hop_calls += twice.iter().filter(|value| **value).count();
            let replacement = 90 + (next_u64(&mut state) % 30) as i64;
            let delta = 1 + (next_u64(&mut state) % 9) as i64;
            let expected = literal_field_alias_mutation_expected(
                writer,
                second_writer,
                reader,
                replacement,
                delta,
            );
            let source = literal_field_alias_mutation_program(
                writer,
                second_writer,
                reader,
                replacement,
                delta,
                wrappers,
                twice,
            );

            assert_interpretation(
                &source,
                ExpectedInterpretation::Value(expected),
                &format!(
                    "aggregate-literal field alias mutation case {case_index}, kind {kind:?}, pattern {pattern:?}, writer {writer:?}, second_writer {second_writer:?}, reader {reader:?}"
                ),
            );
        }

        assert_interpretation(
            &literal_field_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("aggregate-literal const field discard, kind {kind:?}"),
        );
        assert_interpretation(
            &literal_address_const_discard_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("aggregate-literal address const discard, kind {kind:?}"),
        );
        assert_interpretation(
            &literal_field_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("aggregate-literal const field write, kind {kind:?}"),
        );
        assert_interpretation(
            &literal_field_cross_root_program(kind),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
            &format!("aggregate-literal field cross-root identity, kind {kind:?}"),
        );
        assert_interpretation(
            &literal_field_bounds_program(kind),
            ExpectedInterpretation::OwnedError(format!(
                "{} pointer index 5 out of bounds for length {EMBEDDED_ARRAY_LEN}",
                literal_field_bounds_prefix(kind)
            )),
            &format!("aggregate-literal field bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &literal_field_type_mismatch_program(kind),
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                kind.other().pointee_label()
            )),
            &format!("aggregate-literal field pointee type, kind {kind:?}"),
        );
    }

    assert_eq!(pattern_counts, [24, 24, 24, 24]);
    assert_eq!(wrapper_counts, [72, 72, 72, 72]);
    assert_eq!(one_hop_calls, 144);
    assert_eq!(two_hop_calls, 144);
}

#[test]
fn pointer_parameter_mutation_diagnostics_match_model_without_panics() {
    for kind in ReturnedPointeeKind::ALL {
        assert_interpretation(
            &pointer_parameter_const_write_program(kind),
            ExpectedInterpretation::Error("cannot assign through pointer to const"),
            &format!("const pointer parameter write, kind {kind:?}"),
        );
        assert_interpretation(
            &pointer_parameter_mutation_bounds_program(kind),
            ExpectedInterpretation::OwnedError(format!(
                "{} pointer index 7 out of bounds for length {RETURNED_ARRAY_LEN}",
                kind.bounds_prefix()
            )),
            &format!("pointer parameter mutation bounds, kind {kind:?}"),
        );
        assert_interpretation(
            &pointer_parameter_mutation_type_mismatch_program(kind),
            ExpectedInterpretation::OwnedError(format!(
                "cannot convert pointer to {} to pointer to {}",
                kind.pointee_label(),
                kind.other().pointee_label()
            )),
            &format!("pointer parameter mutation type mismatch, kind {kind:?}"),
        );
        assert_interpretation(
            &mixed_qualification_const_storage_writer_program(kind),
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("mixed-qualification const storage writer, kind {kind:?}"),
        );
    }
}

const RETURNED_ARRAY_LEN: i64 = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReturnedPointeeKind {
    Int,
    Point,
    Number,
}

impl ReturnedPointeeKind {
    const ALL: [Self; 3] = [Self::Int, Self::Point, Self::Number];

    fn function_suffix(self) -> &'static str {
        match self {
            Self::Int => "int",
            Self::Point => "point",
            Self::Number => "number",
        }
    }

    fn mutable_pointer_type(self) -> &'static str {
        match self {
            Self::Int => "int *",
            Self::Point => "struct Point *",
            Self::Number => "union Number *",
        }
    }

    fn const_pointer_type(self) -> &'static str {
        match self {
            Self::Int => "const int *",
            Self::Point => "const struct Point *",
            Self::Number => "const union Number *",
        }
    }

    fn pointee_label(self) -> &'static str {
        match self {
            Self::Int => "int",
            Self::Point => "struct 'Point'",
            Self::Number => "union 'Number'",
        }
    }

    fn bounds_prefix(self) -> &'static str {
        match self {
            Self::Int => "array",
            Self::Point | Self::Number => "struct array",
        }
    }

    fn read_result(self) -> String {
        match self {
            Self::Int => "return *result;".to_string(),
            Self::Point | Self::Number => "return result->value;".to_string(),
        }
    }

    fn write_result(self) -> &'static str {
        match self {
            Self::Int => "*result = 1; return *result;",
            Self::Point | Self::Number => "result->value = 1; return result->value;",
        }
    }

    fn other(self) -> Self {
        match self {
            Self::Int => Self::Point,
            Self::Point => Self::Number,
            Self::Number => Self::Int,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReturnedRoot {
    Left,
    Right,
}

impl ReturnedRoot {
    fn argument(self) -> i64 {
        match self {
            Self::Left => 0,
            Self::Right => 1,
        }
    }

    fn base_value(self, kind: ReturnedPointeeKind, points_to_const: bool) -> i64 {
        match (kind, points_to_const, self) {
            (ReturnedPointeeKind::Int, false, Self::Left) => 11,
            (ReturnedPointeeKind::Int, false, Self::Right) => 21,
            (ReturnedPointeeKind::Int, true, Self::Left) => 31,
            (ReturnedPointeeKind::Int, true, Self::Right) => 41,
            (ReturnedPointeeKind::Point, false, Self::Left) => 51,
            (ReturnedPointeeKind::Point, false, Self::Right) => 61,
            (ReturnedPointeeKind::Point, true, Self::Left) => 71,
            (ReturnedPointeeKind::Point, true, Self::Right) => 81,
            (ReturnedPointeeKind::Number, false, Self::Left) => 91,
            (ReturnedPointeeKind::Number, false, Self::Right) => 101,
            (ReturnedPointeeKind::Number, true, Self::Left) => 111,
            (ReturnedPointeeKind::Number, true, Self::Right) => 121,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ReturnedModelPointer {
    kind: ReturnedPointeeKind,
    root: ReturnedRoot,
    index: i64,
    points_to_const: bool,
}

impl ReturnedModelPointer {
    fn value(self) -> i64 {
        self.root.base_value(self.kind, self.points_to_const) + self.index
    }
}

#[derive(Debug)]
struct ReturnedPointerExpr {
    rendered: String,
    pointer: Result<ReturnedModelPointer, i64>,
    used_indexed_address: bool,
}

fn generate_returned_pointer_expr(
    state: &mut u64,
    kind: ReturnedPointeeKind,
    depth: usize,
) -> ReturnedPointerExpr {
    let mut pointer = ReturnedModelPointer {
        kind,
        root: if next_u64(state) & 1 == 0 {
            ReturnedRoot::Left
        } else {
            ReturnedRoot::Right
        },
        index: (next_u64(state) % RETURNED_ARRAY_LEN as u64) as i64,
        points_to_const: next_u64(state) & 1 == 0,
    };
    let mut expression = ReturnedPointerExpr {
        rendered: render_return_call(pointer),
        pointer: Ok(pointer),
        used_indexed_address: false,
    };

    for _ in 0..depth {
        let current = expression.rendered;
        let current_pointer = expression.pointer;
        let used_indexed_address = expression.used_indexed_address;
        let (rendered, next_pointer, next_used_indexed_address) = match next_u64(state) % 6 {
            0 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                (
                    format!("({current} + {offset})"),
                    offset_returned_pointer(current_pointer, offset),
                    used_indexed_address,
                )
            }
            1 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                (
                    format!("({offset} + {current})"),
                    offset_returned_pointer(current_pointer, offset),
                    used_indexed_address,
                )
            }
            2 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                (
                    format!("({current} - {offset})"),
                    offset_returned_pointer(current_pointer, -offset),
                    used_indexed_address,
                )
            }
            3 => {
                pointer = ReturnedModelPointer {
                    root: if next_u64(state) & 1 == 0 {
                        ReturnedRoot::Left
                    } else {
                        ReturnedRoot::Right
                    },
                    index: (next_u64(state) % RETURNED_ARRAY_LEN as u64) as i64,
                    points_to_const: next_u64(state) & 1 == 0,
                    ..pointer
                };
                let alternate = render_return_call(pointer);
                let condition = next_u64(state) & 1 == 0;
                let selected = if condition {
                    current_pointer
                } else {
                    Ok(pointer)
                };
                let merged_const = current_pointer
                    .ok()
                    .is_some_and(|value| value.points_to_const)
                    || pointer.points_to_const;
                let selected = selected.map(|mut value| {
                    value.points_to_const = merged_const;
                    value
                });
                (
                    format!("({} ? {current} : {alternate})", i64::from(condition)),
                    selected,
                    used_indexed_address,
                )
            }
            4 => (format!("&({current})[0]"), current_pointer, true),
            _ => {
                let base = current_pointer.ok().unwrap_or(pointer);
                let left = render_return_call(ReturnedModelPointer { index: 2, ..base });
                let right = render_return_call(ReturnedModelPointer { index: 1, ..base });
                (
                    format!("(({left} - {right}), {current})"),
                    current_pointer,
                    used_indexed_address,
                )
            }
        };
        expression = ReturnedPointerExpr {
            rendered,
            pointer: next_pointer,
            used_indexed_address: next_used_indexed_address,
        };
    }
    expression
}

fn offset_returned_pointer(
    pointer: Result<ReturnedModelPointer, i64>,
    offset: i64,
) -> Result<ReturnedModelPointer, i64> {
    let mut pointer = pointer?;
    pointer.index += offset;
    if (0..RETURNED_ARRAY_LEN).contains(&pointer.index) {
        Ok(pointer)
    } else {
        Err(pointer.index)
    }
}

fn render_return_call(pointer: ReturnedModelPointer) -> String {
    format!(
        "pick_{}{}({}, {})",
        if pointer.points_to_const {
            "const_"
        } else {
            ""
        },
        pointer.kind.function_suffix(),
        pointer.root.argument(),
        pointer.index
    )
}

fn returned_pointer_program(result_type: &str, expression: &str, operation: &str) -> String {
    format!(
        "struct Point {{ int value; }};\n\
         union Number {{ int value; char tag; }};\n\
         int int_left[6] = {{11, 12, 13, 14, 15, 16}};\n\
         static int int_right[6] = {{21, 22, 23, 24, 25, 26}};\n\
         const int const_int_left[6] = {{31, 32, 33, 34, 35, 36}};\n\
         static const int const_int_right[6] = {{41, 42, 43, 44, 45, 46}};\n\
         struct Point point_left[6] = {{{{51}}, {{52}}, {{53}}, {{54}}, {{55}}, {{56}}}};\n\
         static struct Point point_right[6] = {{{{61}}, {{62}}, {{63}}, {{64}}, {{65}}, {{66}}}};\n\
         const struct Point const_point_left[6] = {{{{71}}, {{72}}, {{73}}, {{74}}, {{75}}, {{76}}}};\n\
         static const struct Point const_point_right[6] = {{{{81}}, {{82}}, {{83}}, {{84}}, {{85}}, {{86}}}};\n\
         union Number number_left[6] = {{{{91}}, {{92}}, {{93}}, {{94}}, {{95}}, {{96}}}};\n\
         static union Number number_right[6] = {{{{101}}, {{102}}, {{103}}, {{104}}, {{105}}, {{106}}}};\n\
         const union Number const_number_left[6] = {{{{111}}, {{112}}, {{113}}, {{114}}, {{115}}, {{116}}}};\n\
         static const union Number const_number_right[6] = {{{{121}}, {{122}}, {{123}}, {{124}}, {{125}}, {{126}}}};\n\
         int *pick_int(int right, int index) {{ return (right ? int_right : int_left) + index; }}\n\
         const int *pick_const_int(int right, int index) {{ return (right ? const_int_right : const_int_left) + index; }}\n\
         struct Point *pick_point(int right, int index) {{ return (right ? point_right : point_left) + index; }}\n\
         const struct Point *pick_const_point(int right, int index) {{ return (right ? const_point_right : const_point_left) + index; }}\n\
         union Number *pick_number(int right, int index) {{ return (right ? number_right : number_left) + index; }}\n\
         const union Number *pick_const_number(int right, int index) {{ return (right ? const_number_right : const_number_left) + index; }}\n\
         int main(void) {{\n\
         {result_type} result = {expression};\n\
         {operation}\n\
         }}\n"
    )
}

fn dangling_returned_pointer_program(kind: ReturnedPointeeKind) -> String {
    let (declaration, function_type, local_name, read) = match kind {
        ReturnedPointeeKind::Int => ("int local_int = 7;", "int", "local_int", "return *result;"),
        ReturnedPointeeKind::Point => (
            "struct Point local_point = {7};",
            "struct Point",
            "local_point",
            "return result->value;",
        ),
        ReturnedPointeeKind::Number => (
            "union Number local_number = {7};",
            "union Number",
            "local_number",
            "return result->value;",
        ),
    };
    format!(
        "struct Point {{ int value; }};\n\
         union Number {{ int value; char tag; }};\n\
         {function_type} *dangling(void) {{ {declaration} return &{local_name}; }}\n\
         int main(void) {{ {function_type} *result = dangling(); {read} }}\n"
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ForwardedModelPointer {
    kind: ReturnedPointeeKind,
    root: ReturnedRoot,
    index: i64,
    storage_const: bool,
    points_to_const: bool,
}

impl ForwardedModelPointer {
    fn value(self) -> i64 {
        self.root
            .base_value(self.kind, self.storage_const)
            .saturating_add(self.index)
    }

    fn storage_name(self) -> &'static str {
        match (self.kind, self.storage_const, self.root) {
            (ReturnedPointeeKind::Int, false, ReturnedRoot::Left) => "int_left",
            (ReturnedPointeeKind::Int, false, ReturnedRoot::Right) => "int_right",
            (ReturnedPointeeKind::Int, true, ReturnedRoot::Left) => "const_int_left",
            (ReturnedPointeeKind::Int, true, ReturnedRoot::Right) => "const_int_right",
            (ReturnedPointeeKind::Point, false, ReturnedRoot::Left) => "point_left",
            (ReturnedPointeeKind::Point, false, ReturnedRoot::Right) => "point_right",
            (ReturnedPointeeKind::Point, true, ReturnedRoot::Left) => "const_point_left",
            (ReturnedPointeeKind::Point, true, ReturnedRoot::Right) => "const_point_right",
            (ReturnedPointeeKind::Number, false, ReturnedRoot::Left) => "number_left",
            (ReturnedPointeeKind::Number, false, ReturnedRoot::Right) => "number_right",
            (ReturnedPointeeKind::Number, true, ReturnedRoot::Left) => "const_number_left",
            (ReturnedPointeeKind::Number, true, ReturnedRoot::Right) => "const_number_right",
        }
    }
}

#[derive(Debug)]
struct ForwardedPointerExpr {
    rendered: String,
    pointer: Result<ForwardedModelPointer, i64>,
    declared_const: bool,
    used_indexed_address: bool,
    used_nested_forwarding: bool,
}

fn generate_forwarded_pointer_expr(
    state: &mut u64,
    kind: ReturnedPointeeKind,
    depth: usize,
) -> ForwardedPointerExpr {
    let storage_const = next_u64(state) & 1 == 0;
    let pointer = ForwardedModelPointer {
        kind,
        root: if next_u64(state) & 1 == 0 {
            ReturnedRoot::Left
        } else {
            ReturnedRoot::Right
        },
        index: (next_u64(state) % RETURNED_ARRAY_LEN as u64) as i64,
        storage_const,
        points_to_const: storage_const || next_u64(state) & 1 == 0,
    };
    let mut expression = ForwardedPointerExpr {
        rendered: render_forwarded_call(pointer, next_u64(state) & 1 == 0),
        pointer: Ok(pointer),
        declared_const: pointer.points_to_const,
        used_indexed_address: false,
        used_nested_forwarding: false,
    };

    for _ in 0..depth {
        let current = expression.rendered;
        let current_pointer = expression.pointer;
        let current_const = expression.declared_const;
        let used_indexed_address = expression.used_indexed_address;
        let used_nested_forwarding = expression.used_nested_forwarding;
        expression = match next_u64(state) % 7 {
            0 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                ForwardedPointerExpr {
                    rendered: format!("({current} + {offset})"),
                    pointer: offset_forwarded_pointer(current_pointer, offset),
                    declared_const: current_const,
                    used_indexed_address,
                    used_nested_forwarding,
                }
            }
            1 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                ForwardedPointerExpr {
                    rendered: format!("({offset} + {current})"),
                    pointer: offset_forwarded_pointer(current_pointer, offset),
                    declared_const: current_const,
                    used_indexed_address,
                    used_nested_forwarding,
                }
            }
            2 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                ForwardedPointerExpr {
                    rendered: format!("({current} - {offset})"),
                    pointer: offset_forwarded_pointer(current_pointer, -offset),
                    declared_const: current_const,
                    used_indexed_address,
                    used_nested_forwarding,
                }
            }
            3 => {
                let alternate_storage_const = next_u64(state) & 1 == 0;
                let alternate = ForwardedModelPointer {
                    kind,
                    root: if next_u64(state) & 1 == 0 {
                        ReturnedRoot::Left
                    } else {
                        ReturnedRoot::Right
                    },
                    index: (next_u64(state) % RETURNED_ARRAY_LEN as u64) as i64,
                    storage_const: alternate_storage_const,
                    points_to_const: alternate_storage_const || next_u64(state) & 1 == 0,
                };
                let alternate_rendered = render_forwarded_call(alternate, next_u64(state) & 1 == 0);
                let condition = next_u64(state) & 1 == 0;
                let merged_const = current_const || alternate.points_to_const;
                let selected = if condition {
                    current_pointer
                } else {
                    Ok(alternate)
                }
                .map(|mut pointer| {
                    pointer.points_to_const = merged_const;
                    pointer
                });
                ForwardedPointerExpr {
                    rendered: format!(
                        "({} ? {current} : {alternate_rendered})",
                        i64::from(condition)
                    ),
                    pointer: selected,
                    declared_const: merged_const,
                    used_indexed_address,
                    used_nested_forwarding,
                }
            }
            4 => ForwardedPointerExpr {
                rendered: format!("&({current})[0]"),
                pointer: current_pointer,
                declared_const: current_const,
                used_indexed_address: true,
                used_nested_forwarding,
            },
            5 => {
                let base = current_pointer.ok().unwrap_or(pointer);
                let left = render_forwarded_call(ForwardedModelPointer { index: 2, ..base }, false);
                let right = render_forwarded_call(ForwardedModelPointer { index: 1, ..base }, true);
                ForwardedPointerExpr {
                    rendered: format!("(({left} - {right}), {current})"),
                    pointer: current_pointer,
                    declared_const: current_const,
                    used_indexed_address,
                    used_nested_forwarding,
                }
            }
            _ => {
                let target_const = current_const || next_u64(state) & 1 == 0;
                let rendered = render_forwarding_wrapper(
                    kind,
                    target_const,
                    next_u64(state) & 1 == 0,
                    &current,
                );
                let pointer = current_pointer.map(|mut pointer| {
                    pointer.points_to_const = target_const;
                    pointer
                });
                ForwardedPointerExpr {
                    rendered,
                    pointer,
                    declared_const: target_const,
                    used_indexed_address,
                    used_nested_forwarding: true,
                }
            }
        };
    }
    expression
}

fn offset_forwarded_pointer(
    pointer: Result<ForwardedModelPointer, i64>,
    offset: i64,
) -> Result<ForwardedModelPointer, i64> {
    let mut pointer = pointer?;
    pointer.index += offset;
    if (0..RETURNED_ARRAY_LEN).contains(&pointer.index) {
        Ok(pointer)
    } else {
        Err(pointer.index)
    }
}

fn render_forwarded_call(pointer: ForwardedModelPointer, twice: bool) -> String {
    debug_assert!(!pointer.storage_const || pointer.points_to_const);
    format!(
        "forward_{}{}{}({} + {})",
        if pointer.points_to_const {
            "const_"
        } else {
            ""
        },
        pointer.kind.function_suffix(),
        if twice { "_twice" } else { "" },
        pointer.storage_name(),
        pointer.index
    )
}

fn render_forwarding_wrapper(
    kind: ReturnedPointeeKind,
    points_to_const: bool,
    twice: bool,
    expression: &str,
) -> String {
    format!(
        "forward_{}{}{}({expression})",
        if points_to_const { "const_" } else { "" },
        kind.function_suffix(),
        if twice { "_twice" } else { "" },
    )
}

const FORWARDING_PROGRAM_PRELUDE: &str = r#"
struct Point { int value; };
union Number { int value; char tag; };
int int_left[6] = {11, 12, 13, 14, 15, 16};
static int int_right[6] = {21, 22, 23, 24, 25, 26};
const int const_int_left[6] = {31, 32, 33, 34, 35, 36};
static const int const_int_right[6] = {41, 42, 43, 44, 45, 46};
struct Point point_left[6] = {{51}, {52}, {53}, {54}, {55}, {56}};
static struct Point point_right[6] = {{61}, {62}, {63}, {64}, {65}, {66}};
const struct Point const_point_left[6] = {{71}, {72}, {73}, {74}, {75}, {76}};
static const struct Point const_point_right[6] = {{81}, {82}, {83}, {84}, {85}, {86}};
union Number number_left[6] = {{91}, {92}, {93}, {94}, {95}, {96}};
static union Number number_right[6] = {{101}, {102}, {103}, {104}, {105}, {106}};
const union Number const_number_left[6] = {{111}, {112}, {113}, {114}, {115}, {116}};
static const union Number const_number_right[6] = {{121}, {122}, {123}, {124}, {125}, {126}};
int *forward_int(int *value) { return value; }
int *forward_int_twice(int *value) { return forward_int(value); }
const int *forward_const_int(const int *value) { return value; }
const int *forward_const_int_twice(const int *value) { return forward_const_int(value); }
struct Point *forward_point(struct Point *value) { return value; }
struct Point *forward_point_twice(struct Point *value) { return forward_point(value); }
const struct Point *forward_const_point(const struct Point *value) { return value; }
const struct Point *forward_const_point_twice(const struct Point *value) { return forward_const_point(value); }
union Number *forward_number(union Number *value) { return value; }
union Number *forward_number_twice(union Number *value) { return forward_number(value); }
const union Number *forward_const_number(const union Number *value) { return value; }
const union Number *forward_const_number_twice(const union Number *value) { return forward_const_number(value); }
"#;

fn forwarded_pointer_program(result_type: &str, expression: &str, operation: &str) -> String {
    format!(
        "{FORWARDING_PROGRAM_PRELUDE}\nint main(void) {{ {result_type} result = {expression}; {operation} }}\n"
    )
}

fn pointer_parameter_mutation_program(
    pointer: ForwardedModelPointer,
    replacement: i64,
    twice: bool,
) -> String {
    let suffix = pointer.kind.function_suffix();
    let pointer_type = pointer.kind.mutable_pointer_type();
    let left_storage = ForwardedModelPointer {
        root: ReturnedRoot::Left,
        index: 0,
        ..pointer
    }
    .storage_name();
    let right_storage = ForwardedModelPointer {
        root: ReturnedRoot::Right,
        index: 0,
        ..pointer
    }
    .storage_name();
    let write = match pointer.kind {
        ReturnedPointeeKind::Int => "*value = replacement;",
        ReturnedPointeeKind::Point | ReturnedPointeeKind::Number => "value->value = replacement;",
    };
    let read = match pointer.kind {
        ReturnedPointeeKind::Int => "*result",
        ReturnedPointeeKind::Point | ReturnedPointeeKind::Number => "result->value",
    };
    let helper = if twice {
        format!("mutate_{suffix}_twice")
    } else {
        format!("mutate_{suffix}")
    };

    format!(
        "{FORWARDING_PROGRAM_PRELUDE}\n\
         int mutate_{suffix}({pointer_type}value, int replacement) {{\n\
             {write}\n\
             value = {right_storage} + 5;\n\
             return value == {right_storage} + 5;\n\
         }}\n\
         int mutate_{suffix}_twice({pointer_type}value, int replacement) {{\n\
             int checks = mutate_{suffix}(value, replacement);\n\
             value = {left_storage} + 5;\n\
             return checks + (value == {left_storage} + 5);\n\
         }}\n\
         int main(void) {{\n\
             {pointer_type}result = {storage} + {index};\n\
             int checks = {helper}(result, {replacement});\n\
             return {read} + (result - {storage}) + checks;\n\
         }}\n",
        storage = pointer.storage_name(),
        index = pointer.index,
    )
}

fn two_pointer_parameter_alias_mutation_expected(
    first: ForwardedModelPointer,
    second: ForwardedModelPointer,
    replacement: i64,
    delta: i64,
) -> i64 {
    debug_assert_eq!(first.kind, second.kind);
    debug_assert!(!first.storage_const && !second.storage_const);
    let mut left = std::array::from_fn::<_, { RETURNED_ARRAY_LEN as usize }, _>(|index| {
        ReturnedRoot::Left.base_value(first.kind, false) + index as i64
    });
    let mut right = std::array::from_fn::<_, { RETURNED_ARRAY_LEN as usize }, _>(|index| {
        ReturnedRoot::Right.base_value(first.kind, false) + index as i64
    });

    match first.root {
        ReturnedRoot::Left => left[first.index as usize] = replacement,
        ReturnedRoot::Right => right[first.index as usize] = replacement,
    }
    match second.root {
        ReturnedRoot::Left => left[second.index as usize] += delta,
        ReturnedRoot::Right => right[second.index as usize] += delta,
    }

    left.into_iter()
        .chain(right)
        .enumerate()
        .map(|(index, value)| value * (index as i64 + 1))
        .sum::<i64>()
        + first.index * 17
        + second.index * 19
        + 2
}

fn two_pointer_parameter_alias_mutation_program(
    first: ForwardedModelPointer,
    second: ForwardedModelPointer,
    replacement: i64,
    delta: i64,
) -> String {
    debug_assert_eq!(first.kind, second.kind);
    let suffix = first.kind.function_suffix();
    let pointer_type = first.kind.mutable_pointer_type();
    let left_storage = ForwardedModelPointer {
        root: ReturnedRoot::Left,
        index: 0,
        ..first
    }
    .storage_name();
    let right_storage = ForwardedModelPointer {
        root: ReturnedRoot::Right,
        index: 0,
        ..first
    }
    .storage_name();
    let (first_write, second_update, element_value): (&str, &str, fn(&str, i64) -> String) =
        match first.kind {
            ReturnedPointeeKind::Int => (
                "*first = replacement;",
                "*second += delta;",
                |storage: &str, index| format!("{storage}[{index}]"),
            ),
            ReturnedPointeeKind::Point | ReturnedPointeeKind::Number => (
                "first->value = replacement;",
                "second->value += delta;",
                |storage: &str, index| format!("{storage}[{index}].value"),
            ),
        };
    let storage_checksum = [left_storage, right_storage]
        .into_iter()
        .flat_map(|storage| (0..RETURNED_ARRAY_LEN).map(move |index| element_value(storage, index)))
        .enumerate()
        .map(|(index, element)| format!("{element} * {}", index + 1))
        .collect::<Vec<_>>()
        .join(" + ");

    format!(
        "{FORWARDING_PROGRAM_PRELUDE}\n\
         int mutate_pair_{suffix}({pointer_type}first, {pointer_type}second, int replacement, int delta) {{\n\
             {first_write}\n\
             {second_update}\n\
             first = {right_storage} + 5;\n\
             second = {left_storage} + 4;\n\
             return (first == {right_storage} + 5) + (second == {left_storage} + 4);\n\
         }}\n\
         int main(void) {{\n\
             {pointer_type}first = {first_storage} + {first_index};\n\
             {pointer_type}second = {second_storage} + {second_index};\n\
             int checks = mutate_pair_{suffix}(first, second, {replacement}, {delta});\n\
             return {storage_checksum} + (first - {first_storage}) * 17 +\n\
                    (second - {second_storage}) * 19 + checks;\n\
         }}\n",
        first_storage = first.storage_name(),
        first_index = first.index,
        second_storage = second.storage_name(),
        second_index = second.index,
    )
}

fn mixed_qualification_alias_expected(
    writer: ForwardedModelPointer,
    reader: ForwardedModelPointer,
    replacement: i64,
) -> i64 {
    debug_assert_eq!(writer.kind, reader.kind);
    debug_assert!(!writer.storage_const && !writer.points_to_const);
    debug_assert!(!reader.storage_const && reader.points_to_const);
    let mut left = std::array::from_fn::<_, { RETURNED_ARRAY_LEN as usize }, _>(|index| {
        ReturnedRoot::Left.base_value(writer.kind, false) + index as i64
    });
    let mut right = std::array::from_fn::<_, { RETURNED_ARRAY_LEN as usize }, _>(|index| {
        ReturnedRoot::Right.base_value(writer.kind, false) + index as i64
    });

    match writer.root {
        ReturnedRoot::Left => left[writer.index as usize] = replacement,
        ReturnedRoot::Right => right[writer.index as usize] = replacement,
    }
    let observed = match reader.root {
        ReturnedRoot::Left => left[reader.index as usize],
        ReturnedRoot::Right => right[reader.index as usize],
    };

    left.into_iter()
        .chain(right)
        .enumerate()
        .map(|(index, value)| value * (index as i64 + 1))
        .sum::<i64>()
        + writer.index * 17
        + reader.index * 19
        + observed
        + 2
}

fn mixed_qualification_alias_program(
    writer: ForwardedModelPointer,
    reader: ForwardedModelPointer,
    replacement: i64,
) -> String {
    debug_assert_eq!(writer.kind, reader.kind);
    let suffix = writer.kind.function_suffix();
    let writer_type = writer.kind.mutable_pointer_type();
    let reader_type = writer.kind.const_pointer_type();
    let left_storage = ForwardedModelPointer {
        root: ReturnedRoot::Left,
        index: 0,
        ..writer
    }
    .storage_name();
    let right_storage = ForwardedModelPointer {
        root: ReturnedRoot::Right,
        index: 0,
        ..writer
    }
    .storage_name();
    let (write, read, element_value): (&str, &str, fn(&str, i64) -> String) = match writer.kind {
        ReturnedPointeeKind::Int => (
            "*writer = replacement;",
            "*reader",
            |storage: &str, index| format!("{storage}[{index}]"),
        ),
        ReturnedPointeeKind::Point | ReturnedPointeeKind::Number => (
            "writer->value = replacement;",
            "reader->value",
            |storage: &str, index| format!("{storage}[{index}].value"),
        ),
    };
    let storage_checksum = [left_storage, right_storage]
        .into_iter()
        .flat_map(|storage| (0..RETURNED_ARRAY_LEN).map(move |index| element_value(storage, index)))
        .enumerate()
        .map(|(index, element)| format!("{element} * {}", index + 1))
        .collect::<Vec<_>>()
        .join(" + ");

    format!(
        "{FORWARDING_PROGRAM_PRELUDE}\n\
         int observe_pair_{suffix}({writer_type}writer, {reader_type}reader, int replacement) {{\n\
             {write}\n\
             int observed = {read};\n\
             writer = {right_storage} + 5;\n\
             reader = {left_storage} + 4;\n\
             return observed + (writer == {right_storage} + 5) +\n\
                    (reader == {left_storage} + 4);\n\
         }}\n\
         int main(void) {{\n\
             {writer_type}writer = {writer_storage} + {writer_index};\n\
             {reader_type}reader = {reader_storage} + {reader_index};\n\
             int checks = observe_pair_{suffix}(writer, reader, {replacement});\n\
             return {storage_checksum} + (writer - {writer_storage}) * 17 +\n\
                    (reader - {reader_storage}) * 19 + checks;\n\
         }}\n",
        writer_storage = writer.storage_name(),
        writer_index = writer.index,
        reader_storage = reader.storage_name(),
        reader_index = reader.index,
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FieldBackedPointeeKind {
    Int,
    Char,
    Point,
    Number,
}

impl FieldBackedPointeeKind {
    const ALL: [Self; 4] = [Self::Int, Self::Char, Self::Point, Self::Number];

    fn suffix(self) -> &'static str {
        match self {
            Self::Int => "int",
            Self::Char => "char",
            Self::Point => "point",
            Self::Number => "number",
        }
    }

    fn field_type(self) -> &'static str {
        match self {
            Self::Int => "int",
            Self::Char => "char",
            Self::Point => "struct Point",
            Self::Number => "union Number",
        }
    }

    fn mutable_pointer_type(self) -> String {
        format!("{} *", self.field_type())
    }

    fn const_pointer_type(self) -> String {
        format!("const {} *", self.field_type())
    }

    fn holder_name(self) -> &'static str {
        match self {
            Self::Int => "IntFieldHolder",
            Self::Char => "CharFieldHolder",
            Self::Point => "PointFieldHolder",
            Self::Number => "NumberFieldHolder",
        }
    }

    fn pointee_label(self) -> &'static str {
        match self {
            Self::Int => "int",
            Self::Char => "char",
            Self::Point => "struct 'Point'",
            Self::Number => "union 'Number'",
        }
    }

    fn bounds_prefix(self) -> &'static str {
        match self {
            Self::Int | Self::Char => "array",
            Self::Point | Self::Number => "struct array field",
        }
    }

    fn other(self) -> Self {
        match self {
            Self::Int => Self::Char,
            Self::Char => Self::Point,
            Self::Point => Self::Number,
            Self::Number => Self::Int,
        }
    }

    fn base_value(self) -> i64 {
        match self {
            Self::Int => 10,
            Self::Char => 30,
            Self::Point => 50,
            Self::Number => 70,
        }
    }

    fn read(self, expression: &str) -> String {
        match self {
            Self::Int | Self::Char => format!("*{expression}"),
            Self::Point | Self::Number => format!("{expression}->value"),
        }
    }

    fn write(self, expression: &str, value: &str) -> String {
        match self {
            Self::Int | Self::Char => format!("*{expression} = {value};"),
            Self::Point | Self::Number => format!("{expression}->value = {value};"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FieldBackedOwner {
    Left,
    Right,
}

impl FieldBackedOwner {
    const ALL: [Self; 2] = [Self::Left, Self::Right];

    fn name(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    fn offset(self) -> i64 {
        match self {
            Self::Left => 0,
            Self::Right => 10,
        }
    }

    fn other(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FieldBackedField {
    Primary,
    Secondary,
}

impl FieldBackedField {
    const ALL: [Self; 2] = [Self::Primary, Self::Secondary];

    fn name(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }

    fn offset(self) -> i64 {
        match self {
            Self::Primary => 0,
            Self::Secondary => 5,
        }
    }

    fn other(self) -> Self {
        match self {
            Self::Primary => Self::Secondary,
            Self::Secondary => Self::Primary,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FieldBackedRoute {
    Direct,
    Arrow,
}

impl FieldBackedRoute {
    fn other(self) -> Self {
        match self {
            Self::Direct => Self::Arrow,
            Self::Arrow => Self::Direct,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FieldBackedModelPointer {
    kind: FieldBackedPointeeKind,
    owner: FieldBackedOwner,
    field: FieldBackedField,
    index: i64,
    route: FieldBackedRoute,
}

impl FieldBackedModelPointer {
    fn field_storage(self) -> String {
        format!("{}.{}", self.owner.name(), self.field.name())
    }

    fn render(self) -> String {
        let owner = self.owner.name();
        let field = self.field.name();
        match self.route {
            FieldBackedRoute::Direct => format!("({owner}.{field} + {})", self.index),
            FieldBackedRoute::Arrow => format!("({owner}_view->{field} + {})", self.index),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FieldBackedQualifiedPointer {
    storage: FieldBackedModelPointer,
    points_to_const: bool,
}

impl FieldBackedQualifiedPointer {
    fn value(self) -> i64 {
        self.storage.kind.base_value()
            + self.storage.owner.offset()
            + self.storage.field.offset()
            + self.storage.index
    }
}

#[derive(Debug)]
struct FieldBackedForwardedExpr {
    rendered: String,
    pointer: FieldBackedQualifiedPointer,
    used_two_hop: bool,
    used_arithmetic: bool,
    used_conditional: bool,
    used_comma: bool,
    used_indexed_address: bool,
}

fn random_field_backed_storage(
    state: &mut u64,
    kind: FieldBackedPointeeKind,
    direct_cases: &mut usize,
    arrow_cases: &mut usize,
) -> FieldBackedModelPointer {
    let route = if next_u64(state) & 1 == 0 {
        *direct_cases += 1;
        FieldBackedRoute::Direct
    } else {
        *arrow_cases += 1;
        FieldBackedRoute::Arrow
    };
    FieldBackedModelPointer {
        kind,
        owner: if next_u64(state) & 1 == 0 {
            FieldBackedOwner::Left
        } else {
            FieldBackedOwner::Right
        },
        field: if next_u64(state) & 1 == 0 {
            FieldBackedField::Primary
        } else {
            FieldBackedField::Secondary
        },
        index: (next_u64(state) % EMBEDDED_ARRAY_LEN as u64) as i64,
        route,
    }
}

fn generate_field_backed_forwarded_expr(
    state: &mut u64,
    kind: FieldBackedPointeeKind,
    case_index: usize,
    direct_cases: &mut usize,
    arrow_cases: &mut usize,
) -> FieldBackedForwardedExpr {
    let mut pointer = FieldBackedQualifiedPointer {
        storage: random_field_backed_storage(state, kind, direct_cases, arrow_cases),
        points_to_const: (case_index / 2) & 1 == 0,
    };
    let used_two_hop = case_index & 1 == 0;
    let mut rendered = render_field_backed_forward_call(pointer, used_two_hop);
    let mut used_arithmetic = false;
    let mut used_conditional = false;
    let mut used_comma = false;
    let mut used_indexed_address = false;

    for step in 0..2 {
        match (case_index + step) % 4 {
            0 => {
                let offset = if pointer.storage.index == 0 { 1 } else { -1 };
                rendered = format!("({rendered} + {offset})");
                pointer.storage.index += offset;
                used_arithmetic = true;
            }
            1 => {
                let alternate = FieldBackedQualifiedPointer {
                    storage: random_field_backed_storage(state, kind, direct_cases, arrow_cases),
                    points_to_const: pointer.points_to_const,
                };
                let condition = next_u64(state) & 1 == 0;
                let alternate_rendered = render_field_backed_forward_call(alternate, !used_two_hop);
                let mut selected = if condition { pointer } else { alternate };
                selected.points_to_const = pointer.points_to_const || alternate.points_to_const;
                rendered = format!(
                    "({} ? {rendered} : {alternate_rendered})",
                    i64::from(condition)
                );
                pointer = selected;
                used_conditional = true;
            }
            2 => {
                let left = FieldBackedQualifiedPointer {
                    storage: FieldBackedModelPointer {
                        index: 2,
                        ..pointer.storage
                    },
                    ..pointer
                };
                let right = FieldBackedQualifiedPointer {
                    storage: FieldBackedModelPointer {
                        index: 1,
                        ..pointer.storage
                    },
                    ..pointer
                };
                rendered = format!(
                    "(({} - {}), {rendered})",
                    render_field_backed_forward_call(left, false),
                    render_field_backed_forward_call(right, true)
                );
                used_comma = true;
            }
            _ => {
                rendered = format!("&({rendered})[0]");
                used_indexed_address = true;
            }
        }
    }

    FieldBackedForwardedExpr {
        rendered,
        pointer,
        used_two_hop,
        used_arithmetic,
        used_conditional,
        used_comma,
        used_indexed_address,
    }
}

fn render_field_backed_forward_call(pointer: FieldBackedQualifiedPointer, twice: bool) -> String {
    format!(
        "forward_{}field_{}{}({})",
        if pointer.points_to_const {
            "const_"
        } else {
            ""
        },
        pointer.storage.kind.suffix(),
        if twice { "_twice" } else { "" },
        pointer.storage.render(),
    )
}

fn field_backed_initializer(kind: FieldBackedPointeeKind, owner: FieldBackedOwner) -> String {
    let field_values = |field: FieldBackedField| {
        let base = kind.base_value() + owner.offset() + field.offset();
        if matches!(
            kind,
            FieldBackedPointeeKind::Int | FieldBackedPointeeKind::Char
        ) {
            format!("{{{base}, {}, {}, {}}}", base + 1, base + 2, base + 3)
        } else {
            format!(
                "{{{{{base}}}, {{{}}}, {{{}}}, {{{}}}}}",
                base + 1,
                base + 2,
                base + 3
            )
        }
    };
    format!(
        "{{.primary = {}, .secondary = {}}}",
        field_values(FieldBackedField::Primary),
        field_values(FieldBackedField::Secondary)
    )
}

fn field_backed_definitions(kind: FieldBackedPointeeKind) -> String {
    format!(
        "struct Point {{ int value; }};\n\
         union Number {{ int value; char tag; }};\n\
         struct {holder} {{ {field_type} primary[4]; {field_type} secondary[4]; }};",
        holder = kind.holder_name(),
        field_type = kind.field_type(),
    )
}

fn field_backed_element(kind: FieldBackedPointeeKind, storage: &str, index: i64) -> String {
    let expression = format!("{storage}[{index}]");
    match kind {
        FieldBackedPointeeKind::Int | FieldBackedPointeeKind::Char => expression,
        FieldBackedPointeeKind::Point | FieldBackedPointeeKind::Number => {
            format!("{expression}.value")
        }
    }
}

fn field_backed_mixed_qualification_expected(
    writer: FieldBackedModelPointer,
    reader: FieldBackedModelPointer,
    replacement: i64,
) -> i64 {
    debug_assert_eq!(writer.kind, reader.kind);
    let mut cells = Vec::new();
    for owner in FieldBackedOwner::ALL {
        for field in FieldBackedField::ALL {
            for index in 0..EMBEDDED_ARRAY_LEN {
                cells.push((
                    owner,
                    field,
                    index,
                    writer.kind.base_value() + owner.offset() + field.offset() + index,
                ));
            }
        }
    }
    let writer_cell = cells
        .iter_mut()
        .find(|(owner, field, index, _)| {
            *owner == writer.owner && *field == writer.field && *index == writer.index
        })
        .expect("writer cell must exist");
    writer_cell.3 = replacement;
    let observed = cells
        .iter()
        .find(|(owner, field, index, _)| {
            *owner == reader.owner && *field == reader.field && *index == reader.index
        })
        .expect("reader cell must exist")
        .3;

    cells
        .into_iter()
        .enumerate()
        .map(|(index, (_, _, _, value))| value * (index as i64 + 1))
        .sum::<i64>()
        + writer.index * 17
        + reader.index * 19
        + observed
        + 2
}

fn field_backed_mixed_qualification_program(
    writer: FieldBackedModelPointer,
    reader: FieldBackedModelPointer,
    replacement: i64,
) -> String {
    debug_assert_eq!(writer.kind, reader.kind);
    let kind = writer.kind;
    let left_initializer = field_backed_initializer(kind, FieldBackedOwner::Left);
    let right_initializer = field_backed_initializer(kind, FieldBackedOwner::Right);
    let mut elements = Vec::new();
    for owner in FieldBackedOwner::ALL {
        for field in FieldBackedField::ALL {
            let storage = format!("{}.{}", owner.name(), field.name());
            for index in 0..EMBEDDED_ARRAY_LEN {
                elements.push(field_backed_element(kind, &storage, index));
            }
        }
    }
    let checksum = elements
        .into_iter()
        .enumerate()
        .map(|(index, element)| format!("{element} * {}", index + 1))
        .collect::<Vec<_>>()
        .join(" + ");
    let write = kind.write("writer", "replacement");
    let read = kind.read("reader");
    let writer_type = kind.mutable_pointer_type();
    let reader_type = kind.const_pointer_type();

    format!(
        "{definitions}\n\
         struct {holder} left = {left_initializer};\n\
         static struct {holder} right = {right_initializer};\n\
         int observe_field_{suffix}({writer_type}writer, {reader_type}reader, int replacement) {{\n\
             {write}\n\
             int observed = {read};\n\
             writer = right.secondary + 3;\n\
             reader = left.primary + 2;\n\
             return observed + (writer == right.secondary + 3) +\n\
                    (reader == left.primary + 2);\n\
         }}\n\
         int main(void) {{\n\
             struct {holder} *left_view = &left;\n\
             struct {holder} *right_view = &right;\n\
             {writer_type}writer = {writer_expression};\n\
             {reader_type}reader = {reader_expression};\n\
             int checks = observe_field_{suffix}(writer, reader, {replacement});\n\
             return {checksum} + (writer - {writer_storage}) * 17 +\n\
                    (reader - {reader_storage}) * 19 + checks;\n\
         }}\n",
        definitions = field_backed_definitions(kind),
        holder = kind.holder_name(),
        suffix = kind.suffix(),
        writer_expression = writer.render(),
        reader_expression = reader.render(),
        writer_storage = writer.field_storage(),
        reader_storage = reader.field_storage(),
    )
}

fn field_backed_const_containing_object_program(kind: FieldBackedPointeeKind) -> String {
    let initializer = field_backed_initializer(kind, FieldBackedOwner::Left);
    let writer_type = kind.mutable_pointer_type();
    let reader_type = kind.const_pointer_type();
    format!(
        "{definitions}\n\
         int observe({writer_type}writer, {reader_type}reader) {{ return (writer == 0) + (reader == 0); }}\n\
         int main(void) {{\n\
             const struct {holder} locked = {initializer};\n\
             return observe(locked.primary, locked.primary);\n\
         }}\n",
        definitions = field_backed_definitions(kind),
        holder = kind.holder_name(),
    )
}

fn field_backed_bounds_program(kind: FieldBackedPointeeKind, route: FieldBackedRoute) -> String {
    let initializer = field_backed_initializer(kind, FieldBackedOwner::Left);
    let pointer_type = kind.mutable_pointer_type();
    let expression = match route {
        FieldBackedRoute::Direct => "left.primary + 5",
        FieldBackedRoute::Arrow => "left_view->primary + 5",
    };
    let write = kind.write("value", "1");
    format!(
        "{definitions}\n\
         void mutate({pointer_type}value) {{ {write} }}\n\
         int main(void) {{\n\
             struct {holder} left = {initializer};\n\
             struct {holder} *left_view = &left;\n\
             mutate({expression});\n\
             return 0;\n\
         }}\n",
        definitions = field_backed_definitions(kind),
        holder = kind.holder_name(),
    )
}

fn field_backed_type_mismatch_program(kind: FieldBackedPointeeKind) -> String {
    let initializer = field_backed_initializer(kind, FieldBackedOwner::Left);
    let other_type = kind.other().mutable_pointer_type();
    format!(
        "{definitions}\n\
         int accept_other({other_type}value) {{ return value == 0; }}\n\
         int main(void) {{\n\
             struct {holder} left = {initializer};\n\
             return accept_other(left.primary);\n\
         }}\n",
        definitions = field_backed_definitions(kind),
        holder = kind.holder_name(),
    )
}

fn field_backed_forwarding_prelude(kind: FieldBackedPointeeKind) -> String {
    let left_initializer = field_backed_initializer(kind, FieldBackedOwner::Left);
    let right_initializer = field_backed_initializer(kind, FieldBackedOwner::Right);
    let mutable_type = kind.mutable_pointer_type();
    let const_type = kind.const_pointer_type();
    format!(
        "{definitions}\n\
         struct {holder} left = {left_initializer};\n\
         static struct {holder} right = {right_initializer};\n\
         {mutable_type}forward_field_{suffix}({mutable_type}value) {{ return value; }}\n\
         {mutable_type}forward_field_{suffix}_twice({mutable_type}value) {{ return forward_field_{suffix}(value); }}\n\
         {const_type}forward_const_field_{suffix}({const_type}value) {{ return value; }}\n\
         {const_type}forward_const_field_{suffix}_twice({const_type}value) {{ return forward_const_field_{suffix}(value); }}",
        definitions = field_backed_definitions(kind),
        holder = kind.holder_name(),
        suffix = kind.suffix(),
    )
}

fn field_backed_forwarding_program(
    kind: FieldBackedPointeeKind,
    result_type: &str,
    expression: &str,
    operation: &str,
) -> String {
    format!(
        "{prelude}\n\
         int main(void) {{\n\
             struct {holder} *left_view = &left;\n\
             struct {holder} *right_view = &right;\n\
             {result_type}result = {expression};\n\
             {operation}\n\
         }}\n",
        prelude = field_backed_forwarding_prelude(kind),
        holder = kind.holder_name(),
    )
}

fn field_backed_forwarding_bounds_program(kind: FieldBackedPointeeKind) -> String {
    let expression = format!("forward_field_{}(left_view->primary + 5)", kind.suffix());
    field_backed_forwarding_program(kind, &kind.mutable_pointer_type(), &expression, "return 0;")
}

fn field_backed_forwarding_const_discard_program(kind: FieldBackedPointeeKind) -> String {
    let expression = format!(
        "forward_const_field_{}_twice(left.primary + 1)",
        kind.suffix()
    );
    field_backed_forwarding_program(kind, &kind.mutable_pointer_type(), &expression, "return 0;")
}

fn field_backed_forwarding_const_write_program(kind: FieldBackedPointeeKind) -> String {
    let expression = format!(
        "forward_const_field_{}(right_view->secondary + 2)",
        kind.suffix()
    );
    let operation = format!("{} return 0;", kind.write("result", "1"));
    field_backed_forwarding_program(kind, &kind.const_pointer_type(), &expression, &operation)
}

fn field_backed_forwarding_const_container_program(kind: FieldBackedPointeeKind) -> String {
    let initializer = field_backed_initializer(kind, FieldBackedOwner::Left);
    format!(
        "{prelude}\n\
         int main(void) {{\n\
             const struct {holder} locked = {initializer};\n\
             {mutable_type}result = forward_field_{suffix}(locked.primary + 1);\n\
             return result == 0;\n\
         }}\n",
        prelude = field_backed_forwarding_prelude(kind),
        holder = kind.holder_name(),
        mutable_type = kind.mutable_pointer_type(),
        suffix = kind.suffix(),
    )
}

fn field_backed_forwarding_type_mismatch_program(kind: FieldBackedPointeeKind) -> String {
    let expression = format!("forward_field_{}(left.primary + 1)", kind.suffix());
    field_backed_forwarding_program(
        kind,
        &kind.other().mutable_pointer_type(),
        &expression,
        "return result == 0;",
    )
}

fn field_backed_returned_alias_mutation_expected(
    writer: FieldBackedModelPointer,
    second_writer: FieldBackedModelPointer,
    reader: FieldBackedModelPointer,
    replacement: i64,
    delta: i64,
) -> i64 {
    debug_assert_eq!(writer.kind, second_writer.kind);
    debug_assert_eq!(writer.kind, reader.kind);
    let mut cells = Vec::new();
    for owner in FieldBackedOwner::ALL {
        for field in FieldBackedField::ALL {
            for index in 0..EMBEDDED_ARRAY_LEN {
                cells.push((
                    owner,
                    field,
                    index,
                    writer.kind.base_value() + owner.offset() + field.offset() + index,
                ));
            }
        }
    }
    cells
        .iter_mut()
        .find(|(owner, field, index, _)| {
            *owner == writer.owner && *field == writer.field && *index == writer.index
        })
        .expect("writer cell must exist")
        .3 = replacement;
    let observed_after_first = cells
        .iter()
        .find(|(owner, field, index, _)| {
            *owner == reader.owner && *field == reader.field && *index == reader.index
        })
        .expect("reader cell must exist")
        .3;
    cells
        .iter_mut()
        .find(|(owner, field, index, _)| {
            *owner == second_writer.owner
                && *field == second_writer.field
                && *index == second_writer.index
        })
        .expect("second writer cell must exist")
        .3 += delta;
    let observed_after_second = cells
        .iter()
        .find(|(owner, field, index, _)| {
            *owner == reader.owner && *field == reader.field && *index == reader.index
        })
        .expect("reader cell must exist")
        .3;

    cells
        .into_iter()
        .enumerate()
        .map(|(index, (_, _, _, value))| value * (index as i64 + 1))
        .sum::<i64>()
        + writer.index * 17
        + second_writer.index * 19
        + reader.index * 23
        + observed_after_first
        + observed_after_second
        + 6
}

#[allow(clippy::too_many_arguments)]
fn field_backed_returned_alias_mutation_program(
    writer: FieldBackedModelPointer,
    second_writer: FieldBackedModelPointer,
    reader: FieldBackedModelPointer,
    replacement: i64,
    delta: i64,
    writer_twice: bool,
    second_writer_twice: bool,
    reader_twice: bool,
) -> String {
    debug_assert_eq!(writer.kind, second_writer.kind);
    debug_assert_eq!(writer.kind, reader.kind);
    let kind = writer.kind;
    let writer_pointer = FieldBackedQualifiedPointer {
        storage: writer,
        points_to_const: false,
    };
    let second_writer_pointer = FieldBackedQualifiedPointer {
        storage: second_writer,
        points_to_const: false,
    };
    let reader_pointer = FieldBackedQualifiedPointer {
        storage: reader,
        points_to_const: true,
    };
    let mutable_type = kind.mutable_pointer_type();
    let const_type = kind.const_pointer_type();
    let write_first = kind.write("writer", "replacement");
    let read_first = kind.read("reader");
    let read_second = kind.read("reader");
    let update_second = match kind {
        FieldBackedPointeeKind::Int | FieldBackedPointeeKind::Char => {
            "*second_writer += delta;".to_string()
        }
        FieldBackedPointeeKind::Point | FieldBackedPointeeKind::Number => {
            "second_writer->value += delta;".to_string()
        }
    };
    let mut elements = Vec::new();
    for owner in FieldBackedOwner::ALL {
        for field in FieldBackedField::ALL {
            let storage = format!("{}.{}", owner.name(), field.name());
            for index in 0..EMBEDDED_ARRAY_LEN {
                elements.push(field_backed_element(kind, &storage, index));
            }
        }
    }
    let checksum = elements
        .into_iter()
        .enumerate()
        .map(|(index, element)| format!("{element} * {}", index + 1))
        .collect::<Vec<_>>()
        .join(" + ");

    format!(
        "{prelude}\n\
         int mutate_returned_{suffix}({mutable_type}writer, {mutable_type}second_writer,\n\
                                      {const_type}reader, int replacement, int delta) {{\n\
             {write_first}\n\
             int observed_after_first = {read_first};\n\
             {update_second}\n\
             int observed_after_second = {read_second};\n\
             writer = right.secondary + 3;\n\
             second_writer = left.primary + 2;\n\
             reader = right.primary + 1;\n\
             return observed_after_first + observed_after_second +\n\
                    (writer == right.secondary + 3) +\n\
                    (second_writer == left.primary + 2) +\n\
                    (reader == right.primary + 1);\n\
         }}\n\
         int main(void) {{\n\
             struct {holder} *left_view = &left;\n\
             struct {holder} *right_view = &right;\n\
             {mutable_type}writer = {writer_expression};\n\
             {mutable_type}second_writer = {second_writer_expression};\n\
             {const_type}reader = {reader_expression};\n\
             int observations = mutate_returned_{suffix}(writer, second_writer, reader,\n\
                                                          {replacement}, {delta});\n\
             return {checksum} +\n\
                    (writer - {writer_storage}) * 17 +\n\
                    (second_writer - {second_writer_storage}) * 19 +\n\
                    (reader - {reader_storage}) * 23 + observations +\n\
                    (writer == {writer_raw}) +\n\
                    (second_writer == {second_writer_raw}) +\n\
                    (reader == {reader_raw});\n\
         }}\n",
        prelude = field_backed_forwarding_prelude(kind),
        suffix = kind.suffix(),
        holder = kind.holder_name(),
        writer_expression = render_field_backed_forward_call(writer_pointer, writer_twice),
        second_writer_expression =
            render_field_backed_forward_call(second_writer_pointer, second_writer_twice),
        reader_expression = render_field_backed_forward_call(reader_pointer, reader_twice),
        writer_storage = writer.field_storage(),
        second_writer_storage = second_writer.field_storage(),
        reader_storage = reader.field_storage(),
        writer_raw = writer.render(),
        second_writer_raw = second_writer.render(),
        reader_raw = reader.render(),
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExtendedFieldBackedPath {
    Nested,
    Anonymous,
}

impl ExtendedFieldBackedPath {
    const ALL: [Self; 2] = [Self::Nested, Self::Anonymous];

    fn prefix(self) -> &'static str {
        match self {
            Self::Nested => "nested",
            Self::Anonymous => "anonymous",
        }
    }

    fn offset(self) -> i64 {
        match self {
            Self::Nested => 0,
            Self::Anonymous => 20,
        }
    }

    fn other(self) -> Self {
        match self {
            Self::Nested => Self::Anonymous,
            Self::Anonymous => Self::Nested,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExtendedFieldBackedPointer {
    kind: FieldBackedPointeeKind,
    path: ExtendedFieldBackedPath,
    owner: FieldBackedOwner,
    field: FieldBackedField,
    index: i64,
    route: FieldBackedRoute,
}

impl ExtendedFieldBackedPointer {
    fn object_name(self) -> String {
        format!("{}_{}", self.path.prefix(), self.owner.name())
    }

    fn field_storage(self) -> String {
        let object = self.object_name();
        let inner = if self.path == ExtendedFieldBackedPath::Nested {
            ".inner"
        } else {
            ""
        };
        format!("{object}{inner}.{}", self.field.name())
    }

    fn render(self) -> String {
        let object = self.object_name();
        let field = self.field.name();
        match (self.path, self.route) {
            (ExtendedFieldBackedPath::Nested, FieldBackedRoute::Direct) => {
                format!("({object}.inner.{field} + {})", self.index)
            }
            (ExtendedFieldBackedPath::Nested, FieldBackedRoute::Arrow) => {
                format!("({object}_view->inner.{field} + {})", self.index)
            }
            (ExtendedFieldBackedPath::Anonymous, FieldBackedRoute::Direct) => {
                format!("({object}.{field} + {})", self.index)
            }
            (ExtendedFieldBackedPath::Anonymous, FieldBackedRoute::Arrow) => {
                format!("({object}_view->{field} + {})", self.index)
            }
        }
    }

    fn initial_value(self) -> i64 {
        self.kind.base_value()
            + self.path.offset()
            + self.owner.offset()
            + self.field.offset()
            + self.index
    }
}

fn extended_field_backed_prelude(kind: FieldBackedPointeeKind) -> String {
    format!(
        "{forwarding}\n\
         struct Nested{holder} {{ struct {holder} inner; }};",
        forwarding = field_backed_forwarding_prelude(kind),
        holder = kind.holder_name(),
    )
}

fn extended_field_backed_declarations(kind: FieldBackedPointeeKind) -> String {
    let nested_left = field_backed_initializer(kind, FieldBackedOwner::Left);
    let nested_right = field_backed_initializer(kind, FieldBackedOwner::Right);
    let anonymous_left = extended_field_backed_initializer(
        kind,
        ExtendedFieldBackedPath::Anonymous,
        FieldBackedOwner::Left,
    );
    let anonymous_right = extended_field_backed_initializer(
        kind,
        ExtendedFieldBackedPath::Anonymous,
        FieldBackedOwner::Right,
    );
    format!(
        "struct Nested{holder} nested_left = {{.inner = {nested_left}}},\n\
                                  nested_right = {{.inner = {nested_right}}};\n\
         struct Nested{holder} *nested_left_view = &nested_left;\n\
         struct Nested{holder} *nested_right_view = &nested_right;\n\
         struct {{ {field_type} primary[4]; {field_type} secondary[4]; }}\n\
             anonymous_left = {anonymous_left}, anonymous_right = {anonymous_right},\n\
             *anonymous_left_view = &anonymous_left,\n\
             *anonymous_right_view = &anonymous_right;",
        holder = kind.holder_name(),
        field_type = kind.field_type(),
    )
}

fn extended_field_backed_initializer(
    kind: FieldBackedPointeeKind,
    path: ExtendedFieldBackedPath,
    owner: FieldBackedOwner,
) -> String {
    let field_values = |field: FieldBackedField| {
        let base = kind.base_value() + path.offset() + owner.offset() + field.offset();
        if matches!(
            kind,
            FieldBackedPointeeKind::Int | FieldBackedPointeeKind::Char
        ) {
            format!("{{{base}, {}, {}, {}}}", base + 1, base + 2, base + 3)
        } else {
            format!(
                "{{{{{base}}}, {{{}}}, {{{}}}, {{{}}}}}",
                base + 1,
                base + 2,
                base + 3
            )
        }
    };
    format!(
        "{{.primary = {}, .secondary = {}}}",
        field_values(FieldBackedField::Primary),
        field_values(FieldBackedField::Secondary)
    )
}

fn render_extended_field_backed_forward_call(
    pointer: ExtendedFieldBackedPointer,
    points_to_const: bool,
    twice: bool,
) -> String {
    format!(
        "forward_{}field_{}{}({})",
        if points_to_const { "const_" } else { "" },
        pointer.kind.suffix(),
        if twice { "_twice" } else { "" },
        pointer.render(),
    )
}

fn extended_field_backed_alias_mutation_expected(
    writer: ExtendedFieldBackedPointer,
    second_writer: ExtendedFieldBackedPointer,
    reader: ExtendedFieldBackedPointer,
    replacement: i64,
    delta: i64,
) -> i64 {
    debug_assert_eq!(writer.kind, second_writer.kind);
    debug_assert_eq!(writer.kind, reader.kind);
    let mut cells = Vec::new();
    for path in ExtendedFieldBackedPath::ALL {
        for owner in FieldBackedOwner::ALL {
            for field in FieldBackedField::ALL {
                for index in 0..EMBEDDED_ARRAY_LEN {
                    let pointer = ExtendedFieldBackedPointer {
                        kind: writer.kind,
                        path,
                        owner,
                        field,
                        index,
                        route: FieldBackedRoute::Direct,
                    };
                    cells.push((path, owner, field, index, pointer.initial_value()));
                }
            }
        }
    }
    let cell_matches = |cell: &(
        ExtendedFieldBackedPath,
        FieldBackedOwner,
        FieldBackedField,
        i64,
        i64,
    ),
                        pointer: ExtendedFieldBackedPointer| {
        cell.0 == pointer.path
            && cell.1 == pointer.owner
            && cell.2 == pointer.field
            && cell.3 == pointer.index
    };
    cells
        .iter_mut()
        .find(|cell| cell_matches(cell, writer))
        .expect("writer cell must exist")
        .4 = replacement;
    let observed_after_first = cells
        .iter()
        .find(|cell| cell_matches(cell, reader))
        .expect("reader cell must exist")
        .4;
    cells
        .iter_mut()
        .find(|cell| cell_matches(cell, second_writer))
        .expect("second writer cell must exist")
        .4 += delta;
    let observed_after_second = cells
        .iter()
        .find(|cell| cell_matches(cell, reader))
        .expect("reader cell must exist")
        .4;

    cells
        .into_iter()
        .enumerate()
        .map(|(index, (_, _, _, _, value))| value * (index as i64 + 1))
        .sum::<i64>()
        + writer.index * 17
        + second_writer.index * 19
        + reader.index * 23
        + observed_after_first
        + observed_after_second
        + 5
}

#[allow(clippy::too_many_arguments)]
fn extended_field_backed_alias_mutation_program(
    writer: ExtendedFieldBackedPointer,
    second_writer: ExtendedFieldBackedPointer,
    reader: ExtendedFieldBackedPointer,
    replacement: i64,
    delta: i64,
    writer_twice: bool,
    second_writer_twice: bool,
    reader_twice: bool,
) -> String {
    let kind = writer.kind;
    let mutable_type = kind.mutable_pointer_type();
    let const_type = kind.const_pointer_type();
    let write_first = kind.write("writer", "replacement");
    let read = kind.read("reader");
    let update_second = match kind {
        FieldBackedPointeeKind::Int | FieldBackedPointeeKind::Char => {
            "*second_writer += delta;".to_string()
        }
        FieldBackedPointeeKind::Point | FieldBackedPointeeKind::Number => {
            "second_writer->value += delta;".to_string()
        }
    };
    let mut elements = Vec::new();
    for path in ExtendedFieldBackedPath::ALL {
        for owner in FieldBackedOwner::ALL {
            for field in FieldBackedField::ALL {
                let pointer = ExtendedFieldBackedPointer {
                    kind,
                    path,
                    owner,
                    field,
                    index: 0,
                    route: FieldBackedRoute::Direct,
                };
                let storage = pointer.field_storage();
                for index in 0..EMBEDDED_ARRAY_LEN {
                    elements.push(field_backed_element(kind, &storage, index));
                }
            }
        }
    }
    let checksum = elements
        .into_iter()
        .enumerate()
        .map(|(index, element)| format!("{element} * {}", index + 1))
        .collect::<Vec<_>>()
        .join(" + ");

    format!(
        "{prelude}\n\
         int mutate_extended_{suffix}({mutable_type}writer, {mutable_type}second_writer,\n\
                                      {const_type}reader, int replacement, int delta) {{\n\
             {write_first}\n\
             int observed_after_first = {read};\n\
             {update_second}\n\
             int observed_after_second = {read};\n\
             writer = second_writer;\n\
             reader = writer;\n\
             second_writer = 0;\n\
             return observed_after_first + observed_after_second +\n\
                    (writer == reader) + (second_writer == 0);\n\
         }}\n\
         int main(void) {{\n\
             {declarations}\n\
             {mutable_type}writer = {writer_expression};\n\
             {mutable_type}second_writer = {second_writer_expression};\n\
             {const_type}reader = {reader_expression};\n\
             int observations = mutate_extended_{suffix}(writer, second_writer, reader,\n\
                                                           {replacement}, {delta});\n\
             return {checksum} +\n\
                    (writer - {writer_storage}) * 17 +\n\
                    (second_writer - {second_writer_storage}) * 19 +\n\
                    (reader - {reader_storage}) * 23 + observations +\n\
                    (writer == {writer_raw}) +\n\
                    (second_writer == {second_writer_raw}) +\n\
                    (reader == {reader_raw});\n\
         }}\n",
        prelude = extended_field_backed_prelude(kind),
        suffix = kind.suffix(),
        declarations = extended_field_backed_declarations(kind),
        writer_expression = render_extended_field_backed_forward_call(writer, false, writer_twice),
        second_writer_expression =
            render_extended_field_backed_forward_call(second_writer, false, second_writer_twice),
        reader_expression = render_extended_field_backed_forward_call(reader, true, reader_twice),
        writer_storage = writer.field_storage(),
        second_writer_storage = second_writer.field_storage(),
        reader_storage = reader.field_storage(),
        writer_raw = writer.render(),
        second_writer_raw = second_writer.render(),
        reader_raw = reader.render(),
    )
}

fn extended_field_backed_diagnostic_program(
    kind: FieldBackedPointeeKind,
    operation: &str,
) -> String {
    format!(
        "{prelude}\n\
         int main(void) {{\n\
             {declarations}\n\
             {operation}\n\
         }}\n",
        prelude = extended_field_backed_prelude(kind),
        declarations = extended_field_backed_declarations(kind),
    )
}

fn extended_field_backed_const_container_program(
    kind: FieldBackedPointeeKind,
    path: ExtendedFieldBackedPath,
) -> String {
    let initializer = extended_field_backed_initializer(kind, path, FieldBackedOwner::Left);
    let (declaration, expression) = match path {
        ExtendedFieldBackedPath::Nested => (
            format!(
                "const struct Nested{} locked = {{.inner = {initializer}}};",
                kind.holder_name()
            ),
            "locked.inner.primary + 1",
        ),
        ExtendedFieldBackedPath::Anonymous => (
            format!(
                "const struct {{ {ty} primary[4]; {ty} secondary[4]; }} locked = {initializer};",
                ty = kind.field_type()
            ),
            "locked.primary + 1",
        ),
    };
    extended_field_backed_diagnostic_program(
        kind,
        &format!(
            "{declaration} {ty}result = forward_field_{suffix}({expression}); return result == 0;",
            ty = kind.mutable_pointer_type(),
            suffix = kind.suffix(),
        ),
    )
}

fn extended_field_backed_bounds_program(
    kind: FieldBackedPointeeKind,
    path: ExtendedFieldBackedPath,
) -> String {
    let pointer = ExtendedFieldBackedPointer {
        kind,
        path,
        owner: FieldBackedOwner::Left,
        field: FieldBackedField::Primary,
        index: 5,
        route: FieldBackedRoute::Arrow,
    };
    extended_field_backed_diagnostic_program(
        kind,
        &format!(
            "{ty}result = forward_field_{suffix}({expression}); return result == 0;",
            ty = kind.mutable_pointer_type(),
            suffix = kind.suffix(),
            expression = pointer.render(),
        ),
    )
}

fn extended_field_backed_cross_path_program(kind: FieldBackedPointeeKind) -> String {
    let nested = ExtendedFieldBackedPointer {
        kind,
        path: ExtendedFieldBackedPath::Nested,
        owner: FieldBackedOwner::Left,
        field: FieldBackedField::Primary,
        index: 1,
        route: FieldBackedRoute::Direct,
    };
    let anonymous = ExtendedFieldBackedPointer {
        path: ExtendedFieldBackedPath::Anonymous,
        route: FieldBackedRoute::Arrow,
        ..nested
    };
    extended_field_backed_diagnostic_program(
        kind,
        &format!("return {} - {};", nested.render(), anonymous.render()),
    )
}

fn extended_field_backed_type_mismatch_program(kind: FieldBackedPointeeKind) -> String {
    let pointer = ExtendedFieldBackedPointer {
        kind,
        path: ExtendedFieldBackedPath::Nested,
        owner: FieldBackedOwner::Right,
        field: FieldBackedField::Secondary,
        index: 1,
        route: FieldBackedRoute::Arrow,
    };
    extended_field_backed_diagnostic_program(
        kind,
        &format!(
            "{other_type}result = forward_field_{suffix}({expression}); return result == 0;",
            other_type = kind.other().mutable_pointer_type(),
            suffix = kind.suffix(),
            expression = pointer.render(),
        ),
    )
}

fn extended_literal_holder_name(
    kind: FieldBackedPointeeKind,
    path: ExtendedFieldBackedPath,
) -> String {
    let prefix = match path {
        ExtendedFieldBackedPath::Nested => "NestedLiteral",
        ExtendedFieldBackedPath::Anonymous => "AnonymousLiteral",
    };
    format!("{prefix}{}", kind.holder_name())
}

fn extended_literal_root_name(pointer: ExtendedFieldBackedPointer) -> String {
    format!("literal_{}_{}", pointer.path.prefix(), pointer.owner.name())
}

fn extended_literal_field_storage(pointer: ExtendedFieldBackedPointer) -> String {
    format!(
        "{}->inner.{}",
        extended_literal_root_name(pointer),
        pointer.field.name()
    )
}

fn extended_literal_field_prelude(kind: FieldBackedPointeeKind) -> String {
    let nested = extended_literal_holder_name(kind, ExtendedFieldBackedPath::Nested);
    let anonymous = extended_literal_holder_name(kind, ExtendedFieldBackedPath::Anonymous);
    format!(
        "{forwarding}\n\
         struct {nested} {{ struct {holder} inner; }};\n\
         struct {anonymous} {{ struct {{ {field_type} primary[4]; {field_type} secondary[4]; }} inner; }};",
        forwarding = field_backed_forwarding_prelude(kind),
        holder = kind.holder_name(),
        field_type = kind.field_type(),
    )
}

fn extended_literal_field_declarations(kind: FieldBackedPointeeKind) -> String {
    let mut declarations = Vec::new();
    for path in ExtendedFieldBackedPath::ALL {
        for owner in FieldBackedOwner::ALL {
            let pointer = ExtendedFieldBackedPointer {
                kind,
                path,
                owner,
                field: FieldBackedField::Primary,
                index: 0,
                route: FieldBackedRoute::Direct,
            };
            let root = extended_literal_root_name(pointer);
            let holder = extended_literal_holder_name(kind, path);
            let initializer = extended_literal_field_initializer(kind, path, owner);
            declarations.push(format!(
                "struct {holder} *{root} = &(struct {holder}){{.inner = {initializer}}};"
            ));
        }
    }
    declarations.join("\n")
}

fn extended_literal_field_initializer(
    kind: FieldBackedPointeeKind,
    path: ExtendedFieldBackedPath,
    owner: FieldBackedOwner,
) -> String {
    let initializer = extended_field_backed_initializer(kind, path, owner);
    let first_value = kind.base_value() + path.offset() + owner.offset();
    initializer.replacen(
        &first_value.to_string(),
        &format!("(marker += 1, {first_value})"),
        1,
    )
}

fn render_extended_literal_field_forward_call(
    pointer: ExtendedFieldBackedPointer,
    points_to_const: bool,
    twice: bool,
    wrapper: LiteralPointerWrapper,
) -> String {
    let root = extended_literal_field_storage(pointer);
    let suffix = pointer.kind.suffix();
    let function = format!(
        "forward_{}field_{suffix}{}",
        if points_to_const { "const_" } else { "" },
        if twice { "_twice" } else { "" },
    );
    let alternate = extended_literal_field_storage(ExtendedFieldBackedPointer {
        owner: pointer.owner.other(),
        ..pointer
    });
    match wrapper {
        LiteralPointerWrapper::Arithmetic => {
            format!("({function}({root}) + {})", pointer.index)
        }
        LiteralPointerWrapper::IndexedAddress => {
            format!("&{function}({root})[{}]", pointer.index)
        }
        LiteralPointerWrapper::Conditional => format!(
            "(1 ? {function}({root} + {index}) : {function}({alternate}))",
            index = pointer.index,
        ),
        LiteralPointerWrapper::Comma => {
            format!("(marker += 0, {function}({root} + {}))", pointer.index)
        }
    }
}

fn extended_literal_field_alias_mutation_expected(
    writer: ExtendedFieldBackedPointer,
    second_writer: ExtendedFieldBackedPointer,
    reader: ExtendedFieldBackedPointer,
    replacement: i64,
    delta: i64,
) -> i64 {
    extended_field_backed_alias_mutation_expected(writer, second_writer, reader, replacement, delta)
        + 1
}

#[allow(clippy::too_many_arguments)]
fn extended_literal_field_alias_mutation_program(
    writer: ExtendedFieldBackedPointer,
    second_writer: ExtendedFieldBackedPointer,
    reader: ExtendedFieldBackedPointer,
    replacement: i64,
    delta: i64,
    wrappers: [LiteralPointerWrapper; 3],
    twice: [bool; 3],
) -> String {
    let kind = writer.kind;
    let mutable_type = kind.mutable_pointer_type();
    let const_type = kind.const_pointer_type();
    let write_first = kind.write("writer", "replacement");
    let read = kind.read("reader");
    let update_second = match kind {
        FieldBackedPointeeKind::Int | FieldBackedPointeeKind::Char => {
            "*second_writer += delta;".to_string()
        }
        FieldBackedPointeeKind::Point | FieldBackedPointeeKind::Number => {
            "second_writer->value += delta;".to_string()
        }
    };
    let mut elements = Vec::new();
    for path in ExtendedFieldBackedPath::ALL {
        for owner in FieldBackedOwner::ALL {
            for field in FieldBackedField::ALL {
                let storage = extended_literal_field_storage(ExtendedFieldBackedPointer {
                    kind,
                    path,
                    owner,
                    field,
                    index: 0,
                    route: FieldBackedRoute::Direct,
                });
                for index in 0..EMBEDDED_ARRAY_LEN {
                    elements.push(field_backed_element(kind, &storage, index));
                }
            }
        }
    }
    let checksum = elements
        .into_iter()
        .enumerate()
        .map(|(index, element)| format!("{element} * {}", index + 1))
        .collect::<Vec<_>>()
        .join(" + ");
    let writer_storage = extended_literal_field_storage(writer);
    let second_writer_storage = extended_literal_field_storage(second_writer);
    let reader_storage = extended_literal_field_storage(reader);

    format!(
        "{prelude}\n\
         int mutate_extended_literal_{suffix}({mutable_type}writer, {mutable_type}second_writer,\n\
                                               {const_type}reader, int replacement, int delta) {{\n\
             {write_first}\n\
             int observed_after_first = {read};\n\
             {update_second}\n\
             int observed_after_second = {read};\n\
             writer = second_writer;\n\
             reader = writer;\n\
             second_writer = 0;\n\
             return observed_after_first + observed_after_second +\n\
                    (writer == reader) + (second_writer == 0);\n\
         }}\n\
         int main(void) {{\n\
             int marker = 0;\n\
             {declarations}\n\
             {mutable_type}writer = {writer_expression};\n\
             {mutable_type}second_writer = {second_writer_expression};\n\
             {const_type}reader = {reader_expression};\n\
             int observations = mutate_extended_literal_{suffix}(writer, second_writer, reader,\n\
                                                                  {replacement}, {delta});\n\
             return {checksum} +\n\
                    (writer - {writer_storage}) * 17 +\n\
                    (second_writer - {second_writer_storage}) * 19 +\n\
                    (reader - {reader_storage}) * 23 + observations +\n\
                    (writer == {writer_storage} + {writer_index}) +\n\
                    (second_writer == {second_writer_storage} + {second_writer_index}) +\n\
                    (reader == {reader_storage} + {reader_index}) + (marker == 4);\n\
         }}\n",
        prelude = extended_literal_field_prelude(kind),
        suffix = kind.suffix(),
        declarations = extended_literal_field_declarations(kind),
        writer_expression =
            render_extended_literal_field_forward_call(writer, false, twice[0], wrappers[0]),
        second_writer_expression =
            render_extended_literal_field_forward_call(second_writer, false, twice[1], wrappers[1]),
        reader_expression =
            render_extended_literal_field_forward_call(reader, true, twice[2], wrappers[2]),
        writer_index = writer.index,
        second_writer_index = second_writer.index,
        reader_index = reader.index,
    )
}

fn extended_literal_field_diagnostic_program(
    kind: FieldBackedPointeeKind,
    operation: &str,
) -> String {
    format!(
        "{prelude}\n\
         int main(void) {{\n\
             int marker = 0;\n\
             {declarations}\n\
             {operation}\n\
         }}\n",
        prelude = extended_literal_field_prelude(kind),
        declarations = extended_literal_field_declarations(kind),
    )
}

fn extended_literal_field_const_discard_program(
    kind: FieldBackedPointeeKind,
    path: ExtendedFieldBackedPath,
) -> String {
    let holder = extended_literal_holder_name(kind, path);
    let initializer = extended_field_backed_initializer(kind, path, FieldBackedOwner::Left);
    extended_literal_field_diagnostic_program(
        kind,
        &format!(
            "const struct {holder} *locked = &(const struct {holder}){{.inner = {initializer}}};\n\
             {mutable_type}result = locked->inner.primary; return result == 0;",
            mutable_type = kind.mutable_pointer_type(),
        ),
    )
}

fn extended_literal_field_bounds_program(
    kind: FieldBackedPointeeKind,
    path: ExtendedFieldBackedPath,
) -> String {
    let storage = extended_literal_field_storage(ExtendedFieldBackedPointer {
        kind,
        path,
        owner: FieldBackedOwner::Left,
        field: FieldBackedField::Primary,
        index: 0,
        route: FieldBackedRoute::Direct,
    });
    extended_literal_field_diagnostic_program(
        kind,
        &format!(
            "{ty}result = forward_field_{suffix}({storage} + 5); return result == 0;",
            ty = kind.mutable_pointer_type(),
            suffix = kind.suffix(),
        ),
    )
}

fn extended_literal_field_cross_path_program(kind: FieldBackedPointeeKind) -> String {
    let pointer = ExtendedFieldBackedPointer {
        kind,
        path: ExtendedFieldBackedPath::Nested,
        owner: FieldBackedOwner::Left,
        field: FieldBackedField::Primary,
        index: 0,
        route: FieldBackedRoute::Direct,
    };
    let nested = extended_literal_field_storage(pointer);
    let anonymous = extended_literal_field_storage(ExtendedFieldBackedPointer {
        path: ExtendedFieldBackedPath::Anonymous,
        ..pointer
    });
    extended_literal_field_diagnostic_program(kind, &format!("return {nested} - {anonymous};"))
}

fn extended_literal_field_cross_root_program(kind: FieldBackedPointeeKind) -> String {
    let pointer = ExtendedFieldBackedPointer {
        kind,
        path: ExtendedFieldBackedPath::Nested,
        owner: FieldBackedOwner::Left,
        field: FieldBackedField::Primary,
        index: 0,
        route: FieldBackedRoute::Direct,
    };
    let left = extended_literal_field_storage(pointer);
    let right = extended_literal_field_storage(ExtendedFieldBackedPointer {
        owner: FieldBackedOwner::Right,
        ..pointer
    });
    extended_literal_field_diagnostic_program(kind, &format!("return {left} - {right};"))
}

fn extended_literal_field_type_mismatch_program(kind: FieldBackedPointeeKind) -> String {
    let storage = extended_literal_field_storage(ExtendedFieldBackedPointer {
        kind,
        path: ExtendedFieldBackedPath::Anonymous,
        owner: FieldBackedOwner::Right,
        field: FieldBackedField::Secondary,
        index: 0,
        route: FieldBackedRoute::Direct,
    });
    extended_literal_field_diagnostic_program(
        kind,
        &format!(
            "{other_type}result = forward_field_{suffix}({storage} + 1); return result == 0;",
            other_type = kind.other().mutable_pointer_type(),
            suffix = kind.suffix(),
        ),
    )
}

fn literal_pointer_wrapper(case_index: usize) -> LiteralPointerWrapper {
    match case_index % 4 {
        0 => LiteralPointerWrapper::Arithmetic,
        1 => LiteralPointerWrapper::IndexedAddress,
        2 => LiteralPointerWrapper::Conditional,
        _ => LiteralPointerWrapper::Comma,
    }
}

fn literal_pointer_wrapper_index(wrapper: LiteralPointerWrapper) -> usize {
    match wrapper {
        LiteralPointerWrapper::Arithmetic => 0,
        LiteralPointerWrapper::IndexedAddress => 1,
        LiteralPointerWrapper::Conditional => 2,
        LiteralPointerWrapper::Comma => 3,
    }
}

fn literal_field_root_name(pointer: FieldBackedModelPointer) -> String {
    format!("literal_{}_{}", pointer.owner.name(), pointer.field.name())
}

fn render_literal_field_forward_call(
    pointer: FieldBackedModelPointer,
    points_to_const: bool,
    twice: bool,
    wrapper: LiteralPointerWrapper,
) -> String {
    let root = literal_field_root_name(pointer);
    let suffix = pointer.kind.suffix();
    let function = format!(
        "forward_{}literal_{suffix}{}",
        if points_to_const { "const_" } else { "" },
        if twice { "_twice" } else { "" },
    );
    let alternate_owner = pointer.owner.other().name();
    let alternate = format!("literal_{alternate_owner}_{}", pointer.field.name());
    match wrapper {
        LiteralPointerWrapper::Arithmetic => {
            format!("({function}({root}) + {})", pointer.index)
        }
        LiteralPointerWrapper::IndexedAddress => {
            format!("&{function}({root})[{}]", pointer.index)
        }
        LiteralPointerWrapper::Conditional => format!(
            "(1 ? {function}({root} + {index}) : {function}({alternate}))",
            index = pointer.index,
        ),
        LiteralPointerWrapper::Comma => {
            format!("(marker += 0, {function}({root} + {}))", pointer.index)
        }
    }
}

fn literal_field_initializer(
    kind: FieldBackedPointeeKind,
    owner: FieldBackedOwner,
    selected_field: FieldBackedField,
) -> String {
    let field_values = |field: FieldBackedField| {
        let base = kind.base_value() + owner.offset() + field.offset();
        let first = if field == selected_field {
            format!("(marker += 1, {base})")
        } else {
            base.to_string()
        };
        if matches!(
            kind,
            FieldBackedPointeeKind::Int | FieldBackedPointeeKind::Char
        ) {
            format!("{{{first}, {}, {}, {}}}", base + 1, base + 2, base + 3)
        } else {
            format!(
                "{{{{{first}}}, {{{}}}, {{{}}}, {{{}}}}}",
                base + 1,
                base + 2,
                base + 3
            )
        }
    };
    format!(
        "{{.primary = {}, .secondary = {}}}",
        field_values(FieldBackedField::Primary),
        field_values(FieldBackedField::Secondary)
    )
}

fn literal_field_declarations(kind: FieldBackedPointeeKind) -> String {
    let mut declarations = Vec::new();
    for owner in FieldBackedOwner::ALL {
        for field in FieldBackedField::ALL {
            let root = literal_field_root_name(FieldBackedModelPointer {
                kind,
                owner,
                field,
                index: 0,
                route: FieldBackedRoute::Direct,
            });
            let initializer = literal_field_initializer(kind, owner, field);
            declarations.push(format!(
                "{ty}{root} = ((struct {holder}){initializer}).{field_name};",
                ty = kind.mutable_pointer_type(),
                holder = kind.holder_name(),
                field_name = field.name(),
            ));
        }
    }
    declarations.join("\n")
}

fn literal_field_prelude(kind: FieldBackedPointeeKind) -> String {
    let mutable_type = kind.mutable_pointer_type();
    let const_type = kind.const_pointer_type();
    let suffix = kind.suffix();
    format!(
        "{definitions}\n\
         {mutable_type}forward_literal_{suffix}({mutable_type}value) {{ return value; }}\n\
         {mutable_type}forward_literal_{suffix}_twice({mutable_type}value) {{ return forward_literal_{suffix}(value); }}\n\
         {const_type}forward_const_literal_{suffix}({const_type}value) {{ return value; }}\n\
         {const_type}forward_const_literal_{suffix}_twice({const_type}value) {{ return forward_const_literal_{suffix}(value); }}",
        definitions = field_backed_definitions(kind),
    )
}

fn literal_field_alias_mutation_expected(
    writer: FieldBackedModelPointer,
    second_writer: FieldBackedModelPointer,
    reader: FieldBackedModelPointer,
    replacement: i64,
    delta: i64,
) -> i64 {
    let mut cells = Vec::new();
    for owner in FieldBackedOwner::ALL {
        for field in FieldBackedField::ALL {
            for index in 0..EMBEDDED_ARRAY_LEN {
                cells.push((
                    owner,
                    field,
                    index,
                    writer.kind.base_value() + owner.offset() + field.offset() + index,
                ));
            }
        }
    }
    let matches = |cell: &(FieldBackedOwner, FieldBackedField, i64, i64),
                   pointer: FieldBackedModelPointer| {
        cell.0 == pointer.owner && cell.1 == pointer.field && cell.2 == pointer.index
    };
    cells
        .iter_mut()
        .find(|cell| matches(cell, writer))
        .expect("literal writer cell must exist")
        .3 = replacement;
    let observed_after_first = cells
        .iter()
        .find(|cell| matches(cell, reader))
        .expect("literal reader cell must exist")
        .3;
    cells
        .iter_mut()
        .find(|cell| matches(cell, second_writer))
        .expect("literal second writer cell must exist")
        .3 += delta;
    let observed_after_second = cells
        .iter()
        .find(|cell| matches(cell, reader))
        .expect("literal reader cell must exist")
        .3;

    cells
        .into_iter()
        .enumerate()
        .map(|(index, (_, _, _, value))| value * (index as i64 + 1))
        .sum::<i64>()
        + writer.index * 17
        + second_writer.index * 19
        + reader.index * 23
        + observed_after_first
        + observed_after_second
        + 7
}

#[allow(clippy::too_many_arguments)]
fn literal_field_alias_mutation_program(
    writer: FieldBackedModelPointer,
    second_writer: FieldBackedModelPointer,
    reader: FieldBackedModelPointer,
    replacement: i64,
    delta: i64,
    wrappers: [LiteralPointerWrapper; 3],
    twice: [bool; 3],
) -> String {
    let kind = writer.kind;
    let mutable_type = kind.mutable_pointer_type();
    let const_type = kind.const_pointer_type();
    let write_first = kind.write("writer", "replacement");
    let read = kind.read("reader");
    let update_second = match kind {
        FieldBackedPointeeKind::Int | FieldBackedPointeeKind::Char => {
            "*second_writer += delta;".to_string()
        }
        FieldBackedPointeeKind::Point | FieldBackedPointeeKind::Number => {
            "second_writer->value += delta;".to_string()
        }
    };
    let mut elements = Vec::new();
    for owner in FieldBackedOwner::ALL {
        for field in FieldBackedField::ALL {
            let root = literal_field_root_name(FieldBackedModelPointer {
                kind,
                owner,
                field,
                index: 0,
                route: FieldBackedRoute::Direct,
            });
            for index in 0..EMBEDDED_ARRAY_LEN {
                elements.push(field_backed_element(kind, &root, index));
            }
        }
    }
    let checksum = elements
        .into_iter()
        .enumerate()
        .map(|(index, element)| format!("{element} * {}", index + 1))
        .collect::<Vec<_>>()
        .join(" + ");

    format!(
        "{prelude}\n\
         int mutate_literal_{suffix}({mutable_type}writer, {mutable_type}second_writer,\n\
                                     {const_type}reader, int replacement, int delta) {{\n\
             {write_first}\n\
             int observed_after_first = {read};\n\
             {update_second}\n\
             int observed_after_second = {read};\n\
             writer = second_writer;\n\
             reader = writer;\n\
             second_writer = 0;\n\
             return observed_after_first + observed_after_second +\n\
                    (writer == reader) + (second_writer == 0) + (reader == writer);\n\
         }}\n\
         int main(void) {{\n\
             int marker = 0;\n\
             {declarations}\n\
             {mutable_type}writer = {writer_expression};\n\
             {mutable_type}second_writer = {second_writer_expression};\n\
             {const_type}reader = {reader_expression};\n\
             int observations = mutate_literal_{suffix}(writer, second_writer, reader,\n\
                                                         {replacement}, {delta});\n\
             return {checksum} +\n\
                    (writer - {writer_root}) * 17 +\n\
                    (second_writer - {second_writer_root}) * 19 +\n\
                    (reader - {reader_root}) * 23 + observations +\n\
                    (writer == {writer_root} + {writer_index}) +\n\
                    (second_writer == {second_writer_root} + {second_writer_index}) +\n\
                    (reader == {reader_root} + {reader_index}) + (marker == 4);\n\
         }}\n",
        prelude = literal_field_prelude(kind),
        suffix = kind.suffix(),
        declarations = literal_field_declarations(kind),
        writer_expression = render_literal_field_forward_call(writer, false, twice[0], wrappers[0]),
        second_writer_expression =
            render_literal_field_forward_call(second_writer, false, twice[1], wrappers[1]),
        reader_expression = render_literal_field_forward_call(reader, true, twice[2], wrappers[2]),
        writer_root = literal_field_root_name(writer),
        second_writer_root = literal_field_root_name(second_writer),
        reader_root = literal_field_root_name(reader),
        writer_index = writer.index,
        second_writer_index = second_writer.index,
        reader_index = reader.index,
    )
}

fn literal_field_diagnostic_program(kind: FieldBackedPointeeKind, operation: &str) -> String {
    format!(
        "{prelude}\n\
         int main(void) {{\n\
             int marker = 0;\n\
             {declarations}\n\
             {operation}\n\
         }}\n",
        prelude = literal_field_prelude(kind),
        declarations = literal_field_declarations(kind),
    )
}

fn literal_field_const_discard_program(kind: FieldBackedPointeeKind) -> String {
    let initializer = field_backed_initializer(kind, FieldBackedOwner::Left);
    literal_field_diagnostic_program(
        kind,
        &format!(
            "{mutable_type}result = ((const struct {holder}){initializer}).primary; return result == 0;",
            mutable_type = kind.mutable_pointer_type(),
            holder = kind.holder_name(),
        ),
    )
}

fn literal_field_const_write_program(kind: FieldBackedPointeeKind) -> String {
    let initializer = field_backed_initializer(kind, FieldBackedOwner::Left);
    let write = kind.write("result", "1");
    literal_field_diagnostic_program(
        kind,
        &format!(
            "{const_type}result = ((const struct {holder}){initializer}).primary; {write} return 0;",
            const_type = kind.const_pointer_type(),
            holder = kind.holder_name(),
        ),
    )
}

fn literal_address_const_discard_program(kind: FieldBackedPointeeKind) -> String {
    let initializer = field_backed_initializer(kind, FieldBackedOwner::Left);
    literal_field_diagnostic_program(
        kind,
        &format!(
            "struct {holder} *result = &(const struct {holder}){initializer}; return result == 0;",
            holder = kind.holder_name(),
        ),
    )
}

fn literal_field_cross_root_program(kind: FieldBackedPointeeKind) -> String {
    literal_field_diagnostic_program(kind, "return literal_left_primary - literal_right_primary;")
}

fn literal_field_bounds_prefix(kind: FieldBackedPointeeKind) -> &'static str {
    match kind {
        FieldBackedPointeeKind::Int | FieldBackedPointeeKind::Char => "array",
        FieldBackedPointeeKind::Point | FieldBackedPointeeKind::Number => "struct array",
    }
}

fn literal_field_bounds_program(kind: FieldBackedPointeeKind) -> String {
    literal_field_diagnostic_program(
        kind,
        &format!(
            "{ty}result = forward_literal_{suffix}(literal_left_primary + 5); return result == 0;",
            ty = kind.mutable_pointer_type(),
            suffix = kind.suffix(),
        ),
    )
}

fn literal_field_type_mismatch_program(kind: FieldBackedPointeeKind) -> String {
    literal_field_diagnostic_program(
        kind,
        &format!(
            "{other_type}result = forward_literal_{suffix}(literal_left_primary + 1); return result == 0;",
            other_type = kind.other().mutable_pointer_type(),
            suffix = kind.suffix(),
        ),
    )
}

fn pointer_parameter_const_write_program(kind: ReturnedPointeeKind) -> String {
    let pointer_type = kind.const_pointer_type();
    let storage = ForwardedModelPointer {
        kind,
        root: ReturnedRoot::Left,
        index: 0,
        storage_const: true,
        points_to_const: true,
    }
    .storage_name();
    let write = match kind {
        ReturnedPointeeKind::Int => "*value = 1;",
        ReturnedPointeeKind::Point | ReturnedPointeeKind::Number => "value->value = 1;",
    };

    format!(
        "{FORWARDING_PROGRAM_PRELUDE}\n\
         void mutate_const({pointer_type}value) {{ {write} }}\n\
         int main(void) {{ mutate_const({storage} + 1); return 0; }}\n"
    )
}

fn mixed_qualification_const_storage_writer_program(kind: ReturnedPointeeKind) -> String {
    let writer_type = kind.mutable_pointer_type();
    let reader_type = kind.const_pointer_type();
    let storage = ForwardedModelPointer {
        kind,
        root: ReturnedRoot::Left,
        index: 0,
        storage_const: true,
        points_to_const: true,
    }
    .storage_name();

    format!(
        "{FORWARDING_PROGRAM_PRELUDE}\n\
         int observe_pair({writer_type}writer, {reader_type}reader) {{\n\
             return (writer == 0) + (reader == 0);\n\
         }}\n\
         int main(void) {{ return observe_pair({storage}, {storage}); }}\n"
    )
}

fn pointer_parameter_mutation_bounds_program(kind: ReturnedPointeeKind) -> String {
    let pointer_type = kind.mutable_pointer_type();
    let storage = ForwardedModelPointer {
        kind,
        root: ReturnedRoot::Left,
        index: 0,
        storage_const: false,
        points_to_const: false,
    }
    .storage_name();
    let write = match kind {
        ReturnedPointeeKind::Int => "value[2] = 1;",
        ReturnedPointeeKind::Point | ReturnedPointeeKind::Number => "value[2].value = 1;",
    };

    format!(
        "{FORWARDING_PROGRAM_PRELUDE}\n\
         void mutate_out_of_bounds({pointer_type}value) {{ {write} }}\n\
         int main(void) {{ mutate_out_of_bounds({storage} + 5); return 0; }}\n"
    )
}

fn pointer_parameter_mutation_type_mismatch_program(kind: ReturnedPointeeKind) -> String {
    let pointer_type = kind.mutable_pointer_type();
    let other_type = kind.other().mutable_pointer_type();
    let storage = ForwardedModelPointer {
        kind,
        root: ReturnedRoot::Left,
        index: 0,
        storage_const: false,
        points_to_const: false,
    }
    .storage_name();

    format!(
        "{FORWARDING_PROGRAM_PRELUDE}\n\
         int accepts_other({other_type}value) {{ return value == 0; }}\n\
         int main(void) {{ {pointer_type}result = {storage} + 1; return accepts_other(result); }}\n"
    )
}

fn forwarding_return_mismatch_program(kind: ReturnedPointeeKind, discards_const: bool) -> String {
    let other = kind.other();
    let (declaration, name) = match kind {
        ReturnedPointeeKind::Int => ("int local_int = 7;", "local_int"),
        ReturnedPointeeKind::Point => ("struct Point local_point = {7};", "local_point"),
        ReturnedPointeeKind::Number => ("union Number local_number = {7};", "local_number"),
    };
    if discards_const {
        format!(
            "struct Point {{ int value; }};\n\
             union Number {{ int value; char tag; }};\n\
             const {ty} *forward_const({ty} const *value) {{ return value; }}\n\
             {ty} *bad({ty} const *value) {{ return forward_const(value); }}\n\
             int main(void) {{ {declaration} {ty} *result = bad(&{name}); return result == 0; }}\n",
            ty = kind.mutable_pointer_type().trim_end_matches(" *"),
        )
    } else {
        format!(
            "struct Point {{ int value; }};\n\
             union Number {{ int value; char tag; }};\n\
             {ty} *forward_value({ty} *value) {{ return value; }}\n\
             {other_ty} *bad({ty} *value) {{ return forward_value(value); }}\n\
             int main(void) {{ {declaration} {other_ty} *result = bad(&{name}); return result == 0; }}\n",
            ty = kind.mutable_pointer_type().trim_end_matches(" *"),
            other_ty = other.mutable_pointer_type().trim_end_matches(" *"),
        )
    }
}

fn dangling_forwarded_pointer_program(kind: ReturnedPointeeKind) -> String {
    let (declaration, ty, name, read) = match kind {
        ReturnedPointeeKind::Int => ("int local_int = 7;", "int", "local_int", "return *result;"),
        ReturnedPointeeKind::Point => (
            "struct Point local_point = {7};",
            "struct Point",
            "local_point",
            "return result->value;",
        ),
        ReturnedPointeeKind::Number => (
            "union Number local_number = {7};",
            "union Number",
            "local_number",
            "return result->value;",
        ),
    };
    format!(
        "struct Point {{ int value; }};\n\
         union Number {{ int value; char tag; }};\n\
         {ty} *forward_value({ty} *value) {{ return value; }}\n\
         {ty} *forward_twice({ty} *value) {{ return forward_value(value); }}\n\
         {ty} *dangling(void) {{ {declaration} return forward_twice(&{name}); }}\n\
         int main(void) {{ {ty} *result = dangling(); {read} }}\n"
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HiddenAggregateLiteralStorage {
    MutableCompound,
    ConstTypedef,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HiddenAggregateLiteralRoot {
    kind: AggregateKind,
    storage: HiddenAggregateLiteralStorage,
    side: HiddenLiteralSide,
}

impl HiddenAggregateLiteralRoot {
    fn name(self) -> &'static str {
        match (self.kind, self.storage, self.side) {
            (
                AggregateKind::Point,
                HiddenAggregateLiteralStorage::MutableCompound,
                HiddenLiteralSide::Left,
            ) => "mutable_point_left",
            (
                AggregateKind::Point,
                HiddenAggregateLiteralStorage::MutableCompound,
                HiddenLiteralSide::Right,
            ) => "mutable_point_right",
            (
                AggregateKind::Number,
                HiddenAggregateLiteralStorage::MutableCompound,
                HiddenLiteralSide::Left,
            ) => "mutable_number_left",
            (
                AggregateKind::Number,
                HiddenAggregateLiteralStorage::MutableCompound,
                HiddenLiteralSide::Right,
            ) => "mutable_number_right",
            (
                AggregateKind::Point,
                HiddenAggregateLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Left,
            ) => "const_point_left",
            (
                AggregateKind::Point,
                HiddenAggregateLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Right,
            ) => "const_point_right",
            (
                AggregateKind::Number,
                HiddenAggregateLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Left,
            ) => "const_number_left",
            (
                AggregateKind::Number,
                HiddenAggregateLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Right,
            ) => "const_number_right",
        }
    }

    fn pointer_type(self) -> &'static str {
        if self.storage == HiddenAggregateLiteralStorage::ConstTypedef {
            self.kind.const_pointer_type()
        } else {
            self.kind.mutable_pointer_type()
        }
    }

    fn other_side(self) -> Self {
        Self {
            side: match self.side {
                HiddenLiteralSide::Left => HiddenLiteralSide::Right,
                HiddenLiteralSide::Right => HiddenLiteralSide::Left,
            },
            ..self
        }
    }

    fn other_storage(self) -> Self {
        Self {
            storage: match self.storage {
                HiddenAggregateLiteralStorage::MutableCompound => {
                    HiddenAggregateLiteralStorage::ConstTypedef
                }
                HiddenAggregateLiteralStorage::ConstTypedef => {
                    HiddenAggregateLiteralStorage::MutableCompound
                }
            },
            ..self
        }
    }

    fn value(self, index: i64) -> i64 {
        let base = match (self.kind, self.storage, self.side) {
            (
                AggregateKind::Point,
                HiddenAggregateLiteralStorage::MutableCompound,
                HiddenLiteralSide::Left,
            ) => 11,
            (
                AggregateKind::Point,
                HiddenAggregateLiteralStorage::MutableCompound,
                HiddenLiteralSide::Right,
            ) => 21,
            (
                AggregateKind::Number,
                HiddenAggregateLiteralStorage::MutableCompound,
                HiddenLiteralSide::Left,
            ) => 51,
            (
                AggregateKind::Number,
                HiddenAggregateLiteralStorage::MutableCompound,
                HiddenLiteralSide::Right,
            ) => 61,
            (
                AggregateKind::Point,
                HiddenAggregateLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Left,
            ) => 31,
            (
                AggregateKind::Point,
                HiddenAggregateLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Right,
            ) => 41,
            (
                AggregateKind::Number,
                HiddenAggregateLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Left,
            ) => 71,
            (
                AggregateKind::Number,
                HiddenAggregateLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Right,
            ) => 81,
        };
        base + index
    }
}

fn hidden_aggregate_literal_roots() -> Vec<HiddenAggregateLiteralRoot> {
    let mut roots = Vec::new();
    for kind in AggregateKind::ALL {
        for storage in [
            HiddenAggregateLiteralStorage::MutableCompound,
            HiddenAggregateLiteralStorage::ConstTypedef,
        ] {
            for side in [HiddenLiteralSide::Left, HiddenLiteralSide::Right] {
                roots.push(HiddenAggregateLiteralRoot {
                    kind,
                    storage,
                    side,
                });
            }
        }
    }
    roots
}

#[derive(Debug)]
struct HiddenAggregateLiteralPointerExpr {
    rendered: String,
    index: Result<i64, i64>,
}

fn generate_hidden_aggregate_literal_pointer_expr(
    state: &mut u64,
    root: HiddenAggregateLiteralRoot,
    depth: usize,
) -> HiddenAggregateLiteralPointerExpr {
    let initial_index = (next_u64(state) % HIDDEN_LITERAL_LEN as u64) as i64;
    let mut expression = HiddenAggregateLiteralPointerExpr {
        rendered: format!("({} + {initial_index})", root.name()),
        index: Ok(initial_index),
    };

    for _ in 0..depth {
        let current = expression.rendered;
        let current_index = expression.index;
        let (rendered, index) = match next_u64(state) % 5 {
            0 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                (
                    format!("({current} + {offset})"),
                    hidden_literal_offset(current_index, offset),
                )
            }
            1 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                (
                    format!("({offset} + {current})"),
                    hidden_literal_offset(current_index, offset),
                )
            }
            2 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                (
                    format!("({current} - {offset})"),
                    hidden_literal_offset(current_index, -offset),
                )
            }
            3 => {
                let condition = next_u64(state) & 1 == 0;
                let alternate_index = (next_u64(state) % HIDDEN_LITERAL_LEN as u64) as i64;
                let alternate = format!("({} + {alternate_index})", root.name());
                if condition {
                    (format!("(1 ? {current} : {alternate})"), current_index)
                } else {
                    (format!("(0 ? {alternate} : {current})"), current_index)
                }
            }
            _ => (
                format!("((({0} + 2) - ({0} + 1)), {current})", root.name()),
                current_index,
            ),
        };
        expression = HiddenAggregateLiteralPointerExpr { rendered, index };
    }
    expression
}

fn hidden_aggregate_literal_pointer_program(
    result_type: &str,
    expression: &str,
    operation: &str,
) -> String {
    format!(
        "struct Point {{ int value; }};\n\
         union Number {{ int value; char tag; }};\n\
         typedef const struct Point ConstPoints[4];\n\
         typedef const union Number ConstNumbers[4];\n\
         int main(void) {{\n\
         struct Point *mutable_point_left = (struct Point[4]){{{{11}}, {{12}}, {{13}}, {{14}}}};\n\
         struct Point *mutable_point_right = (struct Point[4]){{{{21}}, {{22}}, {{23}}, {{24}}}};\n\
         union Number *mutable_number_left = (union Number[4]){{{{51}}, {{52}}, {{53}}, {{54}}}};\n\
         union Number *mutable_number_right = (union Number[4]){{{{61}}, {{62}}, {{63}}, {{64}}}};\n\
         const struct Point *const_point_left = (ConstPoints){{{{31}}, {{32}}, {{33}}, {{34}}}};\n\
         const struct Point *const_point_right = (ConstPoints){{{{41}}, {{42}}, {{43}}, {{44}}}};\n\
         const union Number *const_number_left = (ConstNumbers){{{{71}}, {{72}}, {{73}}, {{74}}}};\n\
         const union Number *const_number_right = (ConstNumbers){{{{81}}, {{82}}, {{83}}, {{84}}}};\n\
         {result_type} result = {expression};\n\
         {operation}\n\
         }}\n"
    )
}

const HIDDEN_LITERAL_LEN: i64 = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HiddenLiteralStorage {
    MutableCompound,
    ConstTypedef,
    String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HiddenLiteralSide {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HiddenLiteralRoot {
    kind: ScalarFieldKind,
    storage: HiddenLiteralStorage,
    side: HiddenLiteralSide,
}

impl HiddenLiteralRoot {
    fn name(self) -> &'static str {
        match (self.kind, self.storage, self.side) {
            (
                ScalarFieldKind::Int,
                HiddenLiteralStorage::MutableCompound,
                HiddenLiteralSide::Left,
            ) => "mutable_int_left",
            (
                ScalarFieldKind::Int,
                HiddenLiteralStorage::MutableCompound,
                HiddenLiteralSide::Right,
            ) => "mutable_int_right",
            (
                ScalarFieldKind::Char,
                HiddenLiteralStorage::MutableCompound,
                HiddenLiteralSide::Left,
            ) => "mutable_char_left",
            (
                ScalarFieldKind::Char,
                HiddenLiteralStorage::MutableCompound,
                HiddenLiteralSide::Right,
            ) => "mutable_char_right",
            (ScalarFieldKind::Int, HiddenLiteralStorage::ConstTypedef, HiddenLiteralSide::Left) => {
                "const_int_left"
            }
            (
                ScalarFieldKind::Int,
                HiddenLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Right,
            ) => "const_int_right",
            (
                ScalarFieldKind::Char,
                HiddenLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Left,
            ) => "const_char_left",
            (
                ScalarFieldKind::Char,
                HiddenLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Right,
            ) => "const_char_right",
            (ScalarFieldKind::Char, HiddenLiteralStorage::String, HiddenLiteralSide::Left) => {
                "string_left"
            }
            (ScalarFieldKind::Char, HiddenLiteralStorage::String, HiddenLiteralSide::Right) => {
                "string_right"
            }
            (ScalarFieldKind::Int, HiddenLiteralStorage::String, _) => {
                unreachable!("string roots always have char elements")
            }
        }
    }

    fn pointer_type(self) -> &'static str {
        if self.storage == HiddenLiteralStorage::ConstTypedef {
            self.kind.const_pointer_type()
        } else {
            self.kind.mutable_pointer_type()
        }
    }

    fn other(self) -> Self {
        Self {
            side: match self.side {
                HiddenLiteralSide::Left => HiddenLiteralSide::Right,
                HiddenLiteralSide::Right => HiddenLiteralSide::Left,
            },
            ..self
        }
    }

    fn value(self, index: i64) -> i64 {
        let base = match (self.kind, self.storage, self.side) {
            (
                ScalarFieldKind::Int,
                HiddenLiteralStorage::MutableCompound,
                HiddenLiteralSide::Left,
            ) => 11,
            (
                ScalarFieldKind::Int,
                HiddenLiteralStorage::MutableCompound,
                HiddenLiteralSide::Right,
            ) => 21,
            (
                ScalarFieldKind::Char,
                HiddenLiteralStorage::MutableCompound,
                HiddenLiteralSide::Left,
            ) => 51,
            (
                ScalarFieldKind::Char,
                HiddenLiteralStorage::MutableCompound,
                HiddenLiteralSide::Right,
            ) => 61,
            (ScalarFieldKind::Int, HiddenLiteralStorage::ConstTypedef, HiddenLiteralSide::Left) => {
                31
            }
            (
                ScalarFieldKind::Int,
                HiddenLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Right,
            ) => 41,
            (
                ScalarFieldKind::Char,
                HiddenLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Left,
            ) => 71,
            (
                ScalarFieldKind::Char,
                HiddenLiteralStorage::ConstTypedef,
                HiddenLiteralSide::Right,
            ) => 81,
            (ScalarFieldKind::Char, HiddenLiteralStorage::String, HiddenLiteralSide::Left) => {
                [97, 98, 99, 0][index as usize]
            }
            (ScalarFieldKind::Char, HiddenLiteralStorage::String, HiddenLiteralSide::Right) => {
                [120, 121, 122, 0][index as usize]
            }
            (ScalarFieldKind::Int, HiddenLiteralStorage::String, _) => {
                unreachable!("string roots always have char elements")
            }
        };
        if self.storage == HiddenLiteralStorage::String {
            base
        } else {
            base + index
        }
    }
}

fn hidden_literal_roots() -> Vec<HiddenLiteralRoot> {
    let mut roots = Vec::new();
    for kind in ScalarFieldKind::ALL {
        for storage in [
            HiddenLiteralStorage::MutableCompound,
            HiddenLiteralStorage::ConstTypedef,
        ] {
            for side in [HiddenLiteralSide::Left, HiddenLiteralSide::Right] {
                roots.push(HiddenLiteralRoot {
                    kind,
                    storage,
                    side,
                });
            }
        }
    }
    for side in [HiddenLiteralSide::Left, HiddenLiteralSide::Right] {
        roots.push(HiddenLiteralRoot {
            kind: ScalarFieldKind::Char,
            storage: HiddenLiteralStorage::String,
            side,
        });
    }
    roots
}

#[derive(Debug)]
struct HiddenLiteralPointerExpr {
    rendered: String,
    index: Result<i64, i64>,
}

fn generate_hidden_literal_pointer_expr(
    state: &mut u64,
    root: HiddenLiteralRoot,
    depth: usize,
) -> HiddenLiteralPointerExpr {
    let initial_index = (next_u64(state) % HIDDEN_LITERAL_LEN as u64) as i64;
    let mut expression = HiddenLiteralPointerExpr {
        rendered: format!("({} + {initial_index})", root.name()),
        index: Ok(initial_index),
    };

    for _ in 0..depth {
        let current = expression.rendered;
        let current_index = expression.index;
        let (rendered, index) = match next_u64(state) % 5 {
            0 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                (
                    format!("({current} + {offset})"),
                    hidden_literal_offset(current_index, offset),
                )
            }
            1 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                (
                    format!("({offset} + {current})"),
                    hidden_literal_offset(current_index, offset),
                )
            }
            2 => {
                let offset = (next_u64(state) % 7) as i64 - 3;
                (
                    format!("({current} - {offset})"),
                    hidden_literal_offset(current_index, -offset),
                )
            }
            3 => {
                let condition = next_u64(state) & 1 == 0;
                let alternate_index = (next_u64(state) % HIDDEN_LITERAL_LEN as u64) as i64;
                let alternate = format!("({} + {alternate_index})", root.name());
                if condition {
                    (format!("(1 ? {current} : {alternate})"), current_index)
                } else {
                    (format!("(0 ? {alternate} : {current})"), current_index)
                }
            }
            _ => (
                format!("((({0} + 2) - ({0} + 1)), {current})", root.name()),
                current_index,
            ),
        };
        expression = HiddenLiteralPointerExpr { rendered, index };
    }
    expression
}

fn hidden_literal_offset(current: Result<i64, i64>, offset: i64) -> Result<i64, i64> {
    match current {
        Err(index) => Err(index),
        Ok(index) => {
            let next = index + offset;
            if (0..HIDDEN_LITERAL_LEN).contains(&next) {
                Ok(next)
            } else {
                Err(next)
            }
        }
    }
}

fn hidden_literal_pointer_program(result_type: &str, expression: &str, operation: &str) -> String {
    format!(
        "typedef const int ConstInts[4];\n\
         typedef const char ConstChars[4];\n\
         int main(void) {{\n\
         int *mutable_int_left = (int[4]){{11, 12, 13, 14}};\n\
         int *mutable_int_right = (int[4]){{21, 22, 23, 24}};\n\
         char *mutable_char_left = (char[4]){{51, 52, 53, 54}};\n\
         char *mutable_char_right = (char[4]){{61, 62, 63, 64}};\n\
         const int *const_int_left = (ConstInts){{31, 32, 33, 34}};\n\
         const int *const_int_right = (ConstInts){{41, 42, 43, 44}};\n\
         const char *const_char_left = (ConstChars){{71, 72, 73, 74}};\n\
         const char *const_char_right = (ConstChars){{81, 82, 83, 84}};\n\
         char *string_left = \"abc\";\n\
         char *string_right = \"xyz\";\n\
         {result_type} result = {expression};\n\
         {operation}\n\
         }}\n"
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScalarFieldKind {
    Int,
    Char,
}

impl ScalarFieldKind {
    const ALL: [Self; 2] = [Self::Int, Self::Char];

    fn type_name(self) -> &'static str {
        match self {
            Self::Int => "int",
            Self::Char => "char",
        }
    }

    fn prefix(self) -> &'static str {
        match self {
            Self::Int => "Int",
            Self::Char => "Char",
        }
    }

    fn mutable_pointer_type(self) -> &'static str {
        match self {
            Self::Int => "int *",
            Self::Char => "char *",
        }
    }

    fn const_pointer_type(self) -> &'static str {
        match self {
            Self::Int => "const int *",
            Self::Char => "const char *",
        }
    }

    fn pointee_label(self) -> &'static str {
        self.type_name()
    }

    fn other(self) -> Self {
        match self {
            Self::Int => Self::Char,
            Self::Char => Self::Int,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ScalarFieldModelPointer {
    kind: ScalarFieldKind,
    storage: EmbeddedStorage,
    index: i64,
}

impl ScalarFieldModelPointer {
    fn value(self) -> i64 {
        self.storage.root.base_value()
            + self.index
            + if self.storage.points_to_const { 10 } else { 0 }
            + if self.storage.nested { 20 } else { 0 }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ScalarFieldPointerBase {
    kind: ScalarFieldKind,
    root: EmbeddedRoot,
    index: i64,
    points_to_const: bool,
    route: EmbeddedPointerRoute,
    literal_id: u64,
}

impl ScalarFieldPointerBase {
    fn evaluate(self) -> Result<ScalarFieldModelPointer, ScalarFieldModelError> {
        scalar_field_pointer_at(
            self.kind,
            EmbeddedStorage {
                root: self.root,
                points_to_const: self.points_to_const,
                nested: self.route.is_nested(),
                literal_id: self.route.is_literal().then_some(self.literal_id),
            },
            self.index,
        )
    }

    fn render(self, container: EmbeddedContainerKind) -> String {
        let prefix = if self.points_to_const { "const_" } else { "" };
        let root = self.root.name();
        let base = match self.route {
            EmbeddedPointerRoute::DirectDecay | EmbeddedPointerRoute::DirectAddress => {
                format!("{prefix}{root}.items")
            }
            EmbeddedPointerRoute::ArrowDecay | EmbeddedPointerRoute::ArrowAddress => {
                format!("{prefix}{root}_view->items")
            }
            EmbeddedPointerRoute::NestedDecay | EmbeddedPointerRoute::NestedAddress => {
                format!("{prefix}{root}_nested.holder.items")
            }
            EmbeddedPointerRoute::LiteralDecay | EmbeddedPointerRoute::LiteralAddress => {
                let holder_type =
                    scalar_field_holder_type(self.kind, container, self.points_to_const);
                let initializer = scalar_field_initializer(self.root, self.points_to_const, false);
                format!("(({holder_type}){initializer}).items")
            }
        };
        if matches!(
            self.route,
            EmbeddedPointerRoute::DirectAddress
                | EmbeddedPointerRoute::ArrowAddress
                | EmbeddedPointerRoute::NestedAddress
                | EmbeddedPointerRoute::LiteralAddress
        ) {
            format!("&{base}[{}]", self.index)
        } else {
            format!("({base} + {})", self.index)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScalarFieldModelError {
    Bounds(i64),
    CrossOwnerDifference,
}

#[derive(Clone, Debug)]
enum ScalarFieldPointerExpr {
    Base(ScalarFieldPointerBase),
    Add(Box<Self>, Box<ScalarFieldScalarExpr>),
    ReverseAdd(Box<ScalarFieldScalarExpr>, Box<Self>),
    Subtract(Box<Self>, Box<ScalarFieldScalarExpr>),
    Conditional(bool, Box<Self>, Box<Self>),
    Comma(Box<ScalarFieldScalarExpr>, Box<Self>),
}

impl ScalarFieldPointerExpr {
    fn evaluate(&self) -> Result<ScalarFieldModelPointer, ScalarFieldModelError> {
        match self {
            Self::Base(base) => base.evaluate(),
            Self::Add(pointer, offset) => {
                let pointer = pointer.evaluate()?;
                scalar_field_pointer_at(
                    pointer.kind,
                    pointer.storage,
                    pointer.index + offset.evaluate()?,
                )
            }
            Self::ReverseAdd(offset, pointer) => {
                let offset = offset.evaluate()?;
                let pointer = pointer.evaluate()?;
                scalar_field_pointer_at(pointer.kind, pointer.storage, pointer.index + offset)
            }
            Self::Subtract(pointer, offset) => {
                let pointer = pointer.evaluate()?;
                scalar_field_pointer_at(
                    pointer.kind,
                    pointer.storage,
                    pointer.index - offset.evaluate()?,
                )
            }
            Self::Conditional(condition, when_true, when_false) => {
                if *condition {
                    when_true.evaluate()
                } else {
                    when_false.evaluate()
                }
            }
            Self::Comma(ignored, pointer) => {
                ignored.evaluate()?;
                pointer.evaluate()
            }
        }
    }

    fn points_to_const(&self) -> bool {
        match self {
            Self::Base(base) => base.points_to_const,
            Self::Add(pointer, _)
            | Self::ReverseAdd(_, pointer)
            | Self::Subtract(pointer, _)
            | Self::Comma(_, pointer) => pointer.points_to_const(),
            Self::Conditional(_, when_true, when_false) => {
                when_true.points_to_const() || when_false.points_to_const()
            }
        }
    }

    fn render(&self, container: EmbeddedContainerKind) -> String {
        match self {
            Self::Base(base) => base.render(container),
            Self::Add(pointer, offset) => format!(
                "({} + {})",
                pointer.render(container),
                offset.render(container)
            ),
            Self::ReverseAdd(offset, pointer) => format!(
                "({} + {})",
                offset.render(container),
                pointer.render(container)
            ),
            Self::Subtract(pointer, offset) => format!(
                "({} - {})",
                pointer.render(container),
                offset.render(container)
            ),
            Self::Conditional(condition, when_true, when_false) => format!(
                "({} ? {} : {})",
                i64::from(*condition),
                when_true.render(container),
                when_false.render(container)
            ),
            Self::Comma(ignored, pointer) => format!(
                "({}, {})",
                ignored.render(container),
                pointer.render(container)
            ),
        }
    }
}

#[derive(Clone, Debug)]
enum ScalarFieldScalarExpr {
    Literal(i64),
    PointerDifference(Box<ScalarFieldPointerExpr>, Box<ScalarFieldPointerExpr>),
}

impl ScalarFieldScalarExpr {
    fn evaluate(&self) -> Result<i64, ScalarFieldModelError> {
        match self {
            Self::Literal(value) => Ok(*value),
            Self::PointerDifference(left, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                if left.kind != right.kind || left.storage != right.storage {
                    return Err(ScalarFieldModelError::CrossOwnerDifference);
                }
                Ok(left.index - right.index)
            }
        }
    }

    fn render(&self, container: EmbeddedContainerKind) -> String {
        match self {
            Self::Literal(value) => value.to_string(),
            Self::PointerDifference(left, right) => {
                format!("({} - {})", left.render(container), right.render(container))
            }
        }
    }
}

fn scalar_field_pointer_at(
    kind: ScalarFieldKind,
    storage: EmbeddedStorage,
    index: i64,
) -> Result<ScalarFieldModelPointer, ScalarFieldModelError> {
    if !(0..EMBEDDED_ARRAY_LEN).contains(&index) {
        return Err(ScalarFieldModelError::Bounds(index));
    }
    Ok(ScalarFieldModelPointer {
        kind,
        storage,
        index,
    })
}

fn generate_scalar_field_pointer_expr(
    state: &mut u64,
    kind: ScalarFieldKind,
    required_route: EmbeddedPointerRoute,
    depth: usize,
) -> ScalarFieldPointerExpr {
    let mut expression = random_scalar_field_pointer_base(state, kind, required_route);
    for _ in 0..depth {
        expression = match next_u64(state) % 5 {
            0 => ScalarFieldPointerExpr::Add(
                Box::new(expression),
                Box::new(generate_scalar_field_scalar_expr(state, kind)),
            ),
            1 => ScalarFieldPointerExpr::ReverseAdd(
                Box::new(generate_scalar_field_scalar_expr(state, kind)),
                Box::new(expression),
            ),
            2 => ScalarFieldPointerExpr::Subtract(
                Box::new(expression),
                Box::new(generate_scalar_field_scalar_expr(state, kind)),
            ),
            3 => {
                let condition = next_u64(state) & 1 == 0;
                let other_route = EmbeddedPointerRoute::ALL
                    [(next_u64(state) as usize) % EmbeddedPointerRoute::ALL.len()];
                let other = random_scalar_field_pointer_base(state, kind, other_route);
                if next_u64(state) & 1 == 0 {
                    ScalarFieldPointerExpr::Conditional(
                        condition,
                        Box::new(expression),
                        Box::new(other),
                    )
                } else {
                    ScalarFieldPointerExpr::Conditional(
                        condition,
                        Box::new(other),
                        Box::new(expression),
                    )
                }
            }
            _ => ScalarFieldPointerExpr::Comma(
                Box::new(generate_scalar_field_scalar_expr(state, kind)),
                Box::new(expression),
            ),
        };
    }
    expression
}

fn random_scalar_field_pointer_base(
    state: &mut u64,
    kind: ScalarFieldKind,
    route: EmbeddedPointerRoute,
) -> ScalarFieldPointerExpr {
    ScalarFieldPointerExpr::Base(ScalarFieldPointerBase {
        kind,
        root: if next_u64(state) & 1 == 0 {
            EmbeddedRoot::Left
        } else {
            EmbeddedRoot::Right
        },
        index: (next_u64(state) % EMBEDDED_ARRAY_LEN as u64) as i64,
        points_to_const: next_u64(state) & 1 == 0,
        route,
        literal_id: next_u64(state),
    })
}

fn generate_scalar_field_scalar_expr(
    state: &mut u64,
    kind: ScalarFieldKind,
) -> ScalarFieldScalarExpr {
    if next_u64(state) % 3 != 0 {
        return ScalarFieldScalarExpr::Literal((next_u64(state) % 7) as i64 - 3);
    }
    let left_route = EmbeddedPointerRoute::STABLE
        [(next_u64(state) as usize) % EmbeddedPointerRoute::STABLE.len()];
    let right_route = EmbeddedPointerRoute::STABLE
        [(next_u64(state) as usize) % EmbeddedPointerRoute::STABLE.len()];
    ScalarFieldScalarExpr::PointerDifference(
        Box::new(random_scalar_field_pointer_base(state, kind, left_route)),
        Box::new(random_scalar_field_pointer_base(state, kind, right_route)),
    )
}

fn scalar_field_holder_type(
    kind: ScalarFieldKind,
    container: EmbeddedContainerKind,
    points_to_const: bool,
) -> String {
    let qualifier = if points_to_const { "const " } else { "" };
    if container.is_anonymous() {
        format!(
            "{} {{ {qualifier}{} items[{}]; }}",
            container.keyword(),
            kind.type_name(),
            EMBEDDED_ARRAY_LEN
        )
    } else {
        format!(
            "{} {}{}Holder",
            container.keyword(),
            if points_to_const { "Const" } else { "" },
            kind.prefix()
        )
    }
}

fn scalar_field_initializer(root: EmbeddedRoot, points_to_const: bool, nested: bool) -> String {
    let base =
        root.base_value() + if points_to_const { 10 } else { 0 } + if nested { 20 } else { 0 };
    let values = format!("{{{base}, {}, {}, {}}}", base + 1, base + 2, base + 3);
    if nested {
        format!("{{{{{values}}}}}")
    } else {
        format!("{{{values}}}")
    }
}

fn scalar_field_pointer_program(
    kind: ScalarFieldKind,
    container: EmbeddedContainerKind,
    result_type: &str,
    expression: &str,
    write: Option<&str>,
) -> String {
    let element = kind.type_name();
    let keyword = container.keyword();
    let prefix = kind.prefix();
    let mutable_init_left = scalar_field_initializer(EmbeddedRoot::Left, false, false);
    let mutable_init_right = scalar_field_initializer(EmbeddedRoot::Right, false, false);
    let const_init_left = scalar_field_initializer(EmbeddedRoot::Left, true, false);
    let const_init_right = scalar_field_initializer(EmbeddedRoot::Right, true, false);
    let nested_init_left = scalar_field_initializer(EmbeddedRoot::Left, false, true);
    let nested_init_right = scalar_field_initializer(EmbeddedRoot::Right, false, true);
    let const_nested_init_left = scalar_field_initializer(EmbeddedRoot::Left, true, true);
    let const_nested_init_right = scalar_field_initializer(EmbeddedRoot::Right, true, true);

    let (definitions, declarations) = if container.is_anonymous() {
        (
            format!(
                "struct {prefix}Outer {{ {keyword} {{ {element} items[4]; }} holder; }};\n\
                 struct Const{prefix}Outer {{ {keyword} {{ const {element} items[4]; }} holder; }};"
            ),
            format!(
                "{keyword} {{ {element} items[4]; }} left = {mutable_init_left}, *left_view = &left, right = {mutable_init_right}, *right_view = &right;\n\
                 {keyword} {{ const {element} items[4]; }} const_left = {const_init_left}, *const_left_view = &const_left, const_right = {const_init_right}, *const_right_view = &const_right;\n\
                 struct {prefix}Outer left_nested = {nested_init_left}, right_nested = {nested_init_right};\n\
                 struct Const{prefix}Outer const_left_nested = {const_nested_init_left}, const_right_nested = {const_nested_init_right};"
            ),
        )
    } else {
        (
            format!(
                "{keyword} {prefix}Holder {{ {element} items[4]; }};\n\
                 {keyword} Const{prefix}Holder {{ const {element} items[4]; }};\n\
                 struct {prefix}Outer {{ {keyword} {prefix}Holder holder; }};\n\
                 struct Const{prefix}Outer {{ {keyword} Const{prefix}Holder holder; }};"
            ),
            format!(
                "{keyword} {prefix}Holder left = {mutable_init_left}, *left_view = &left, right = {mutable_init_right}, *right_view = &right;\n\
                 {keyword} Const{prefix}Holder const_left = {const_init_left}, *const_left_view = &const_left, const_right = {const_init_right}, *const_right_view = &const_right;\n\
                 struct {prefix}Outer left_nested = {nested_init_left}, right_nested = {nested_init_right};\n\
                 struct Const{prefix}Outer const_left_nested = {const_nested_init_left}, const_right_nested = {const_nested_init_right};"
            ),
        )
    };
    let write = write.unwrap_or("");

    format!(
        "{definitions}\n\
         int main(void) {{\n\
         {declarations}\n\
         {result_type} result = {expression};\n\
         {write}\n\
         return *result;\n\
         }}\n"
    )
}

const EMBEDDED_ARRAY_LEN: i64 = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EmbeddedContainerKind {
    NamedStruct,
    NamedUnion,
    AnonymousStruct,
    AnonymousUnion,
}

impl EmbeddedContainerKind {
    const ALL: [Self; 4] = [
        Self::NamedStruct,
        Self::NamedUnion,
        Self::AnonymousStruct,
        Self::AnonymousUnion,
    ];

    fn keyword(self) -> &'static str {
        match self {
            Self::NamedStruct | Self::AnonymousStruct => "struct",
            Self::NamedUnion | Self::AnonymousUnion => "union",
        }
    }

    fn is_anonymous(self) -> bool {
        matches!(self, Self::AnonymousStruct | Self::AnonymousUnion)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EmbeddedRoot {
    Left,
    Right,
}

impl EmbeddedRoot {
    fn name(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    fn base_value(self) -> i64 {
        match self {
            Self::Left => 11,
            Self::Right => 71,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EmbeddedPointerRoute {
    DirectDecay,
    DirectAddress,
    ArrowDecay,
    ArrowAddress,
    NestedDecay,
    NestedAddress,
    LiteralDecay,
    LiteralAddress,
}

impl EmbeddedPointerRoute {
    const ALL: [Self; 8] = [
        Self::DirectDecay,
        Self::DirectAddress,
        Self::ArrowDecay,
        Self::ArrowAddress,
        Self::NestedDecay,
        Self::NestedAddress,
        Self::LiteralDecay,
        Self::LiteralAddress,
    ];
    const STABLE: [Self; 6] = [
        Self::DirectDecay,
        Self::DirectAddress,
        Self::ArrowDecay,
        Self::ArrowAddress,
        Self::NestedDecay,
        Self::NestedAddress,
    ];

    fn is_nested(self) -> bool {
        matches!(self, Self::NestedDecay | Self::NestedAddress)
    }

    fn is_literal(self) -> bool {
        matches!(self, Self::LiteralDecay | Self::LiteralAddress)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EmbeddedStorage {
    root: EmbeddedRoot,
    points_to_const: bool,
    nested: bool,
    literal_id: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EmbeddedModelPointer {
    kind: AggregateKind,
    storage: EmbeddedStorage,
    index: i64,
}

impl EmbeddedModelPointer {
    fn value(self) -> i64 {
        self.storage.root.base_value()
            + self.index
            + if self.storage.points_to_const { 10 } else { 0 }
            + if self.storage.nested { 20 } else { 0 }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EmbeddedPointerBase {
    kind: AggregateKind,
    root: EmbeddedRoot,
    index: i64,
    points_to_const: bool,
    route: EmbeddedPointerRoute,
    literal_id: u64,
}

impl EmbeddedPointerBase {
    fn evaluate(self) -> Result<EmbeddedModelPointer, EmbeddedModelError> {
        embedded_pointer_at(
            self.kind,
            EmbeddedStorage {
                root: self.root,
                points_to_const: self.points_to_const,
                nested: self.route.is_nested(),
                literal_id: self.route.is_literal().then_some(self.literal_id),
            },
            self.index,
        )
    }

    fn render(self, container: EmbeddedContainerKind) -> String {
        let prefix = if self.points_to_const { "const_" } else { "" };
        let root = self.root.name();
        let base = match self.route {
            EmbeddedPointerRoute::DirectDecay | EmbeddedPointerRoute::DirectAddress => {
                format!("{prefix}{root}.items")
            }
            EmbeddedPointerRoute::ArrowDecay | EmbeddedPointerRoute::ArrowAddress => {
                format!("{prefix}{root}_view->items")
            }
            EmbeddedPointerRoute::NestedDecay | EmbeddedPointerRoute::NestedAddress => {
                format!("{prefix}{root}_nested.holder.items")
            }
            EmbeddedPointerRoute::LiteralDecay | EmbeddedPointerRoute::LiteralAddress => {
                let holder_type = embedded_holder_type(self.kind, container, self.points_to_const);
                let initializer = embedded_initializer(self.root, self.points_to_const, false);
                format!("(({holder_type}){initializer}).items")
            }
        };
        if matches!(
            self.route,
            EmbeddedPointerRoute::DirectAddress
                | EmbeddedPointerRoute::ArrowAddress
                | EmbeddedPointerRoute::NestedAddress
                | EmbeddedPointerRoute::LiteralAddress
        ) {
            format!("&{base}[{}]", self.index)
        } else {
            format!("({base} + {})", self.index)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EmbeddedModelError {
    Bounds { index: i64, field: bool },
    CrossOwnerDifference,
}

#[derive(Clone, Debug)]
enum EmbeddedPointerExpr {
    Base(EmbeddedPointerBase),
    Add(Box<Self>, Box<EmbeddedScalarExpr>),
    ReverseAdd(Box<EmbeddedScalarExpr>, Box<Self>),
    Subtract(Box<Self>, Box<EmbeddedScalarExpr>),
    Conditional(bool, Box<Self>, Box<Self>),
    Comma(Box<EmbeddedScalarExpr>, Box<Self>),
}

impl EmbeddedPointerExpr {
    fn evaluate(&self) -> Result<EmbeddedModelPointer, EmbeddedModelError> {
        match self {
            Self::Base(base) => base.evaluate(),
            Self::Add(pointer, offset) => {
                let pointer = pointer.evaluate()?;
                embedded_pointer_at(
                    pointer.kind,
                    pointer.storage,
                    pointer.index + offset.evaluate()?,
                )
            }
            Self::ReverseAdd(offset, pointer) => {
                let offset = offset.evaluate()?;
                let pointer = pointer.evaluate()?;
                embedded_pointer_at(pointer.kind, pointer.storage, pointer.index + offset)
            }
            Self::Subtract(pointer, offset) => {
                let pointer = pointer.evaluate()?;
                embedded_pointer_at(
                    pointer.kind,
                    pointer.storage,
                    pointer.index - offset.evaluate()?,
                )
            }
            Self::Conditional(condition, when_true, when_false) => {
                if *condition {
                    when_true.evaluate()
                } else {
                    when_false.evaluate()
                }
            }
            Self::Comma(ignored, pointer) => {
                ignored.evaluate()?;
                pointer.evaluate()
            }
        }
    }

    fn points_to_const(&self) -> bool {
        match self {
            Self::Base(base) => base.points_to_const,
            Self::Add(pointer, _)
            | Self::ReverseAdd(_, pointer)
            | Self::Subtract(pointer, _)
            | Self::Comma(_, pointer) => pointer.points_to_const(),
            Self::Conditional(_, when_true, when_false) => {
                when_true.points_to_const() || when_false.points_to_const()
            }
        }
    }

    fn render(&self, container: EmbeddedContainerKind) -> String {
        match self {
            Self::Base(base) => base.render(container),
            Self::Add(pointer, offset) => format!(
                "({} + {})",
                pointer.render(container),
                offset.render(container)
            ),
            Self::ReverseAdd(offset, pointer) => format!(
                "({} + {})",
                offset.render(container),
                pointer.render(container)
            ),
            Self::Subtract(pointer, offset) => format!(
                "({} - {})",
                pointer.render(container),
                offset.render(container)
            ),
            Self::Conditional(condition, when_true, when_false) => format!(
                "({} ? {} : {})",
                i64::from(*condition),
                when_true.render(container),
                when_false.render(container)
            ),
            Self::Comma(ignored, pointer) => format!(
                "({}, {})",
                ignored.render(container),
                pointer.render(container)
            ),
        }
    }
}

#[derive(Clone, Debug)]
enum EmbeddedScalarExpr {
    Literal(i64),
    PointerDifference(Box<EmbeddedPointerExpr>, Box<EmbeddedPointerExpr>),
}

impl EmbeddedScalarExpr {
    fn evaluate(&self) -> Result<i64, EmbeddedModelError> {
        match self {
            Self::Literal(value) => Ok(*value),
            Self::PointerDifference(left, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                if left.kind != right.kind || left.storage != right.storage {
                    return Err(EmbeddedModelError::CrossOwnerDifference);
                }
                Ok(left.index - right.index)
            }
        }
    }

    fn render(&self, container: EmbeddedContainerKind) -> String {
        match self {
            Self::Literal(value) => value.to_string(),
            Self::PointerDifference(left, right) => {
                format!("({} - {})", left.render(container), right.render(container))
            }
        }
    }
}

fn embedded_pointer_at(
    kind: AggregateKind,
    storage: EmbeddedStorage,
    index: i64,
) -> Result<EmbeddedModelPointer, EmbeddedModelError> {
    if !(0..EMBEDDED_ARRAY_LEN).contains(&index) {
        return Err(EmbeddedModelError::Bounds {
            index,
            field: storage.literal_id.is_none(),
        });
    }
    Ok(EmbeddedModelPointer {
        kind,
        storage,
        index,
    })
}

fn generate_embedded_pointer_expr(
    state: &mut u64,
    kind: AggregateKind,
    required_route: EmbeddedPointerRoute,
    depth: usize,
) -> EmbeddedPointerExpr {
    let mut expression = random_embedded_pointer_base(state, kind, required_route);
    for _ in 0..depth {
        expression = match next_u64(state) % 5 {
            0 => EmbeddedPointerExpr::Add(
                Box::new(expression),
                Box::new(generate_embedded_scalar_expr(state, kind)),
            ),
            1 => EmbeddedPointerExpr::ReverseAdd(
                Box::new(generate_embedded_scalar_expr(state, kind)),
                Box::new(expression),
            ),
            2 => EmbeddedPointerExpr::Subtract(
                Box::new(expression),
                Box::new(generate_embedded_scalar_expr(state, kind)),
            ),
            3 => {
                let condition = next_u64(state) & 1 == 0;
                let other_route = EmbeddedPointerRoute::ALL
                    [(next_u64(state) as usize) % EmbeddedPointerRoute::ALL.len()];
                let other = random_embedded_pointer_base(state, kind, other_route);
                if next_u64(state) & 1 == 0 {
                    EmbeddedPointerExpr::Conditional(
                        condition,
                        Box::new(expression),
                        Box::new(other),
                    )
                } else {
                    EmbeddedPointerExpr::Conditional(
                        condition,
                        Box::new(other),
                        Box::new(expression),
                    )
                }
            }
            _ => EmbeddedPointerExpr::Comma(
                Box::new(generate_embedded_scalar_expr(state, kind)),
                Box::new(expression),
            ),
        };
    }
    expression
}

fn random_embedded_pointer_base(
    state: &mut u64,
    kind: AggregateKind,
    route: EmbeddedPointerRoute,
) -> EmbeddedPointerExpr {
    EmbeddedPointerExpr::Base(EmbeddedPointerBase {
        kind,
        root: if next_u64(state) & 1 == 0 {
            EmbeddedRoot::Left
        } else {
            EmbeddedRoot::Right
        },
        index: (next_u64(state) % EMBEDDED_ARRAY_LEN as u64) as i64,
        points_to_const: next_u64(state) & 1 == 0,
        route,
        literal_id: next_u64(state),
    })
}

fn generate_embedded_scalar_expr(state: &mut u64, kind: AggregateKind) -> EmbeddedScalarExpr {
    if next_u64(state) % 3 != 0 {
        return EmbeddedScalarExpr::Literal((next_u64(state) % 7) as i64 - 3);
    }
    let left_route = EmbeddedPointerRoute::STABLE
        [(next_u64(state) as usize) % EmbeddedPointerRoute::STABLE.len()];
    let right_route = EmbeddedPointerRoute::STABLE
        [(next_u64(state) as usize) % EmbeddedPointerRoute::STABLE.len()];
    EmbeddedScalarExpr::PointerDifference(
        Box::new(random_embedded_pointer_base(state, kind, left_route)),
        Box::new(random_embedded_pointer_base(state, kind, right_route)),
    )
}

fn embedded_holder_type(
    kind: AggregateKind,
    container: EmbeddedContainerKind,
    points_to_const: bool,
) -> String {
    let element = kind.mutable_pointer_type().trim_end_matches(" *");
    let qualifier = if points_to_const { "const " } else { "" };
    if container.is_anonymous() {
        format!(
            "{} {{ {qualifier}{element} items[{}]; }}",
            container.keyword(),
            EMBEDDED_ARRAY_LEN
        )
    } else {
        format!(
            "{} {}{}Holder",
            container.keyword(),
            if points_to_const { "Const" } else { "" },
            kind.prefix()
        )
    }
}

fn embedded_initializer(root: EmbeddedRoot, points_to_const: bool, nested: bool) -> String {
    let base =
        root.base_value() + if points_to_const { 10 } else { 0 } + if nested { 20 } else { 0 };
    let values = format!(
        "{{{{{base}}}, {{{}}}, {{{}}}, {{{}}}}}",
        base + 1,
        base + 2,
        base + 3
    );
    if nested {
        format!("{{{{{values}}}}}")
    } else {
        format!("{{{values}}}")
    }
}

fn embedded_pointer_program(
    kind: AggregateKind,
    container: EmbeddedContainerKind,
    result_type: &str,
    expression: &str,
) -> String {
    let element = kind.mutable_pointer_type().trim_end_matches(" *");
    let keyword = container.keyword();
    let prefix = kind.prefix();
    let mutable_init_left = embedded_initializer(EmbeddedRoot::Left, false, false);
    let mutable_init_right = embedded_initializer(EmbeddedRoot::Right, false, false);
    let const_init_left = embedded_initializer(EmbeddedRoot::Left, true, false);
    let const_init_right = embedded_initializer(EmbeddedRoot::Right, true, false);
    let nested_init_left = embedded_initializer(EmbeddedRoot::Left, false, true);
    let nested_init_right = embedded_initializer(EmbeddedRoot::Right, false, true);
    let const_nested_init_left = embedded_initializer(EmbeddedRoot::Left, true, true);
    let const_nested_init_right = embedded_initializer(EmbeddedRoot::Right, true, true);

    let (definitions, declarations) = if container.is_anonymous() {
        (
            format!(
                "struct {prefix}Outer {{ {keyword} {{ {element} items[4]; }} holder; }};\n\
                 struct Const{prefix}Outer {{ {keyword} {{ const {element} items[4]; }} holder; }};"
            ),
            format!(
                "{keyword} {{ {element} items[4]; }} left = {mutable_init_left}, *left_view = &left, right = {mutable_init_right}, *right_view = &right;\n\
                 {keyword} {{ const {element} items[4]; }} const_left = {const_init_left}, *const_left_view = &const_left, const_right = {const_init_right}, *const_right_view = &const_right;\n\
                 struct {prefix}Outer left_nested = {nested_init_left}, right_nested = {nested_init_right};\n\
                 struct Const{prefix}Outer const_left_nested = {const_nested_init_left}, const_right_nested = {const_nested_init_right};"
            ),
        )
    } else {
        (
            format!(
                "{keyword} {prefix}Holder {{ {element} items[4]; }};\n\
                 {keyword} Const{prefix}Holder {{ const {element} items[4]; }};\n\
                 struct {prefix}Outer {{ {keyword} {prefix}Holder holder; }};\n\
                 struct Const{prefix}Outer {{ {keyword} Const{prefix}Holder holder; }};"
            ),
            format!(
                "{keyword} {prefix}Holder left = {mutable_init_left}, *left_view = &left, right = {mutable_init_right}, *right_view = &right;\n\
                 {keyword} Const{prefix}Holder const_left = {const_init_left}, *const_left_view = &const_left, const_right = {const_init_right}, *const_right_view = &const_right;\n\
                 struct {prefix}Outer left_nested = {nested_init_left}, right_nested = {nested_init_right};\n\
                 struct Const{prefix}Outer const_left_nested = {const_nested_init_left}, const_right_nested = {const_nested_init_right};"
            ),
        )
    };

    format!(
        "struct Point {{ int value; }};\n\
         union Number {{ int value; char tag; }};\n\
         {definitions}\n\
         int main(void) {{\n\
         {declarations}\n\
         {result_type} result = {expression};\n\
         return result->value;\n\
         }}\n"
    )
}

const AGGREGATE_ARRAY_LEN: i64 = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AggregateKind {
    Point,
    Number,
}

impl AggregateKind {
    const ALL: [Self; 2] = [Self::Point, Self::Number];

    fn prefix(self) -> &'static str {
        match self {
            Self::Point => "point",
            Self::Number => "number",
        }
    }

    fn cursor_type(self) -> &'static str {
        match self {
            Self::Point => "PointCursor",
            Self::Number => "NumberCursor",
        }
    }

    fn mutable_pointer_type(self) -> &'static str {
        match self {
            Self::Point => "struct Point *",
            Self::Number => "union Number *",
        }
    }

    fn const_pointer_type(self) -> &'static str {
        match self {
            Self::Point => "const struct Point *",
            Self::Number => "const union Number *",
        }
    }

    fn pointee_label(self) -> &'static str {
        match self {
            Self::Point => "struct 'Point'",
            Self::Number => "union 'Number'",
        }
    }

    fn other(self) -> Self {
        match self {
            Self::Point => Self::Number,
            Self::Number => Self::Point,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AggregateRoot {
    Left,
    Right,
}

impl AggregateRoot {
    fn name(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AggregatePointerRoute {
    Array,
    DirectField,
    ArrowField,
    LiteralField,
    DirectFieldAssignment,
    ArrowFieldAssignment,
    LiteralFieldAssignment,
    DirectFieldCompoundAssignment,
    ArrowFieldCompoundAssignment,
    LiteralFieldCompoundAssignment,
}

impl AggregatePointerRoute {
    const STABLE: [Self; 7] = [
        Self::Array,
        Self::DirectField,
        Self::ArrowField,
        Self::LiteralField,
        Self::DirectFieldAssignment,
        Self::ArrowFieldAssignment,
        Self::LiteralFieldAssignment,
    ];
    const ALL: [Self; 10] = [
        Self::Array,
        Self::DirectField,
        Self::ArrowField,
        Self::LiteralField,
        Self::DirectFieldAssignment,
        Self::ArrowFieldAssignment,
        Self::LiteralFieldAssignment,
        Self::DirectFieldCompoundAssignment,
        Self::ArrowFieldCompoundAssignment,
        Self::LiteralFieldCompoundAssignment,
    ];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AggregateModelPointer {
    kind: AggregateKind,
    root: AggregateRoot,
    index: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AggregatePointerBase {
    kind: AggregateKind,
    root: AggregateRoot,
    index: i64,
    points_to_const: bool,
    route: AggregatePointerRoute,
}

impl AggregatePointerBase {
    fn evaluate(self) -> Result<AggregateModelPointer, AggregateModelError> {
        aggregate_pointer_at(self.kind, self.root, self.index)
    }

    fn field_name(self) -> String {
        format!(
            "{}{}",
            if self.points_to_const { "const_" } else { "" },
            self.root.name()
        )
    }

    fn array_name(self) -> String {
        format!("{}_{}", self.kind.prefix(), self.root.name())
    }

    fn render(self) -> String {
        let prefix = self.kind.prefix();
        let field = self.field_name();
        let array = self.array_name();
        let pointer = match self.route {
            AggregatePointerRoute::Array => format!(
                "{}{}",
                if self.points_to_const { "const_" } else { "" },
                array
            ),
            AggregatePointerRoute::DirectField => format!("{prefix}_cursor.{field}"),
            AggregatePointerRoute::ArrowField => format!("{prefix}_cursor_view->{field}"),
            AggregatePointerRoute::LiteralField => format!(
                "((struct {}){{{prefix}_left, {prefix}_right, {prefix}_left, {prefix}_right}}).{field}",
                self.kind.cursor_type()
            ),
            AggregatePointerRoute::DirectFieldAssignment => {
                format!("({prefix}_cursor.{field} = {array})")
            }
            AggregatePointerRoute::ArrowFieldAssignment => {
                format!("({prefix}_cursor_view->{field} = {array})")
            }
            AggregatePointerRoute::LiteralFieldAssignment => format!(
                "(((struct {}){{{prefix}_left, {prefix}_right, {prefix}_left, {prefix}_right}}).{field} = {array})",
                self.kind.cursor_type()
            ),
            AggregatePointerRoute::DirectFieldCompoundAssignment => {
                format!("({prefix}_cursor.{field} += {})", self.index)
            }
            AggregatePointerRoute::ArrowFieldCompoundAssignment => {
                format!("({prefix}_cursor_view->{field} += {})", self.index)
            }
            AggregatePointerRoute::LiteralFieldCompoundAssignment => format!(
                "(((struct {}){{{prefix}_left, {prefix}_right, {prefix}_left, {prefix}_right}}).{field} += {})",
                self.kind.cursor_type(),
                self.index
            ),
        };
        if matches!(
            self.route,
            AggregatePointerRoute::DirectFieldCompoundAssignment
                | AggregatePointerRoute::ArrowFieldCompoundAssignment
                | AggregatePointerRoute::LiteralFieldCompoundAssignment
        ) {
            pointer
        } else {
            format!("({pointer} + {})", self.index)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AggregateModelError {
    Bounds(i64),
    CrossArrayDifference,
}

#[derive(Clone, Debug)]
enum AggregatePointerExpr {
    Base(AggregatePointerBase),
    Add(Box<Self>, Box<AggregateScalarExpr>),
    ReverseAdd(Box<AggregateScalarExpr>, Box<Self>),
    Subtract(Box<Self>, Box<AggregateScalarExpr>),
    Conditional(bool, Box<Self>, Box<Self>),
    Comma(Box<AggregateScalarExpr>, Box<Self>),
}

impl AggregatePointerExpr {
    fn evaluate(&self) -> Result<AggregateModelPointer, AggregateModelError> {
        match self {
            Self::Base(base) => base.evaluate(),
            Self::Add(pointer, offset) => {
                let pointer = pointer.evaluate()?;
                aggregate_pointer_at(
                    pointer.kind,
                    pointer.root,
                    pointer.index + offset.evaluate()?,
                )
            }
            Self::ReverseAdd(offset, pointer) => {
                let offset = offset.evaluate()?;
                let pointer = pointer.evaluate()?;
                aggregate_pointer_at(pointer.kind, pointer.root, pointer.index + offset)
            }
            Self::Subtract(pointer, offset) => {
                let pointer = pointer.evaluate()?;
                aggregate_pointer_at(
                    pointer.kind,
                    pointer.root,
                    pointer.index - offset.evaluate()?,
                )
            }
            Self::Conditional(condition, when_true, when_false) => {
                if *condition {
                    when_true.evaluate()
                } else {
                    when_false.evaluate()
                }
            }
            Self::Comma(ignored, pointer) => {
                ignored.evaluate()?;
                pointer.evaluate()
            }
        }
    }

    fn points_to_const(&self) -> bool {
        match self {
            Self::Base(base) => base.points_to_const,
            Self::Add(pointer, _)
            | Self::ReverseAdd(_, pointer)
            | Self::Subtract(pointer, _)
            | Self::Comma(_, pointer) => pointer.points_to_const(),
            Self::Conditional(_, when_true, when_false) => {
                when_true.points_to_const() || when_false.points_to_const()
            }
        }
    }

    fn render(&self) -> String {
        match self {
            Self::Base(base) => base.render(),
            Self::Add(pointer, offset) => {
                format!("({} + {})", pointer.render(), offset.render())
            }
            Self::ReverseAdd(offset, pointer) => {
                format!("({} + {})", offset.render(), pointer.render())
            }
            Self::Subtract(pointer, offset) => {
                format!("({} - {})", pointer.render(), offset.render())
            }
            Self::Conditional(condition, when_true, when_false) => format!(
                "({} ? {} : {})",
                i64::from(*condition),
                when_true.render(),
                when_false.render()
            ),
            Self::Comma(ignored, pointer) => {
                format!("({}, {})", ignored.render(), pointer.render())
            }
        }
    }
}

#[derive(Clone, Debug)]
enum AggregateScalarExpr {
    Literal(i64),
    PointerDifference(Box<AggregatePointerExpr>, Box<AggregatePointerExpr>),
}

impl AggregateScalarExpr {
    fn evaluate(&self) -> Result<i64, AggregateModelError> {
        match self {
            Self::Literal(value) => Ok(*value),
            Self::PointerDifference(left, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                if left.kind != right.kind || left.root != right.root {
                    return Err(AggregateModelError::CrossArrayDifference);
                }
                Ok(left.index - right.index)
            }
        }
    }

    fn render(&self) -> String {
        match self {
            Self::Literal(value) => value.to_string(),
            Self::PointerDifference(left, right) => {
                format!("({} - {})", left.render(), right.render())
            }
        }
    }
}

fn aggregate_pointer_at(
    kind: AggregateKind,
    root: AggregateRoot,
    index: i64,
) -> Result<AggregateModelPointer, AggregateModelError> {
    if !(0..AGGREGATE_ARRAY_LEN).contains(&index) {
        return Err(AggregateModelError::Bounds(index));
    }
    Ok(AggregateModelPointer { kind, root, index })
}

fn generate_aggregate_pointer_expr(
    state: &mut u64,
    kind: AggregateKind,
    required_route: AggregatePointerRoute,
    depth: usize,
) -> AggregatePointerExpr {
    let mut expression = random_aggregate_pointer_base(state, kind, required_route);
    for _ in 0..depth {
        expression = match next_u64(state) % 5 {
            0 => AggregatePointerExpr::Add(
                Box::new(expression),
                Box::new(generate_aggregate_scalar_expr(state, kind)),
            ),
            1 => AggregatePointerExpr::ReverseAdd(
                Box::new(generate_aggregate_scalar_expr(state, kind)),
                Box::new(expression),
            ),
            2 => AggregatePointerExpr::Subtract(
                Box::new(expression),
                Box::new(generate_aggregate_scalar_expr(state, kind)),
            ),
            3 => {
                let condition = next_u64(state) & 1 == 0;
                let route_index = (next_u64(state) as usize) % AggregatePointerRoute::STABLE.len();
                let other = random_aggregate_pointer_base(
                    state,
                    kind,
                    AggregatePointerRoute::STABLE[route_index],
                );
                if next_u64(state) & 1 == 0 {
                    AggregatePointerExpr::Conditional(
                        condition,
                        Box::new(expression),
                        Box::new(other),
                    )
                } else {
                    AggregatePointerExpr::Conditional(
                        condition,
                        Box::new(other),
                        Box::new(expression),
                    )
                }
            }
            _ => AggregatePointerExpr::Comma(
                Box::new(generate_aggregate_scalar_expr(state, kind)),
                Box::new(expression),
            ),
        };
    }
    expression
}

fn random_aggregate_pointer_base(
    state: &mut u64,
    kind: AggregateKind,
    route: AggregatePointerRoute,
) -> AggregatePointerExpr {
    AggregatePointerExpr::Base(AggregatePointerBase {
        kind,
        root: if next_u64(state) & 1 == 0 {
            AggregateRoot::Left
        } else {
            AggregateRoot::Right
        },
        index: (next_u64(state) % AGGREGATE_ARRAY_LEN as u64) as i64,
        points_to_const: next_u64(state) & 1 == 0,
        route,
    })
}

fn generate_aggregate_scalar_expr(state: &mut u64, kind: AggregateKind) -> AggregateScalarExpr {
    if next_u64(state) % 3 != 0 {
        return AggregateScalarExpr::Literal((next_u64(state) % 9) as i64 - 4);
    }
    let left_route = AggregatePointerRoute::STABLE
        [(next_u64(state) as usize) % AggregatePointerRoute::STABLE.len()];
    let right_route = AggregatePointerRoute::STABLE
        [(next_u64(state) as usize) % AggregatePointerRoute::STABLE.len()];
    AggregateScalarExpr::PointerDifference(
        Box::new(random_aggregate_pointer_base(state, kind, left_route)),
        Box::new(random_aggregate_pointer_base(state, kind, right_route)),
    )
}

fn aggregate_pointer_program(
    result_type: &str,
    expression: &str,
    selected: Option<AggregateModelPointer>,
) -> String {
    let setup = selected.map_or_else(String::new, |pointer| {
        let value = match pointer.root {
            AggregateRoot::Left => 41,
            AggregateRoot::Right => 73,
        };
        format!(
            "{}_{}[{}].value = {value};",
            pointer.kind.prefix(),
            pointer.root.name(),
            pointer.index
        )
    });
    format!(
        "struct Point {{ int value; }};\n\
         union Number {{ int value; char tag; }};\n\
         struct PointCursor {{ struct Point *left; struct Point *right; const struct Point *const_left; const struct Point *const_right; }};\n\
         struct NumberCursor {{ union Number *left; union Number *right; const union Number *const_left; const union Number *const_right; }};\n\
         int main(void) {{\n\
         struct Point point_left[8]; struct Point point_right[8];\n\
         union Number number_left[8]; union Number number_right[8];\n\
         const struct Point *const_point_left = point_left;\n\
         const struct Point *const_point_right = point_right;\n\
         const union Number *const_number_left = number_left;\n\
         const union Number *const_number_right = number_right;\n\
         struct PointCursor point_cursor = {{point_left, point_right, point_left, point_right}};\n\
         struct NumberCursor number_cursor = {{number_left, number_right, number_left, number_right}};\n\
         struct PointCursor *point_cursor_view = &point_cursor;\n\
         struct NumberCursor *number_cursor_view = &number_cursor;\n\
         {setup}\n\
         {result_type} result = {expression};\n\
         return result->value;\n\
         }}\n"
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArrayRoot {
    Left,
    Right,
}

impl ArrayRoot {
    fn name(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ModelPointer {
    root: ArrayRoot,
    index: i64,
    points_to_const: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ModelError {
    CrossArrayDifference,
}

#[derive(Clone, Debug)]
enum PointerExpr {
    Base(ModelPointer),
    Add(Box<Self>, Box<ScalarExpr>),
    ReverseAdd(Box<ScalarExpr>, Box<Self>),
    Subtract(Box<Self>, Box<ScalarExpr>),
    Conditional(bool, Box<Self>, Box<Self>),
    Comma(Box<ScalarExpr>, Box<Self>),
}

impl PointerExpr {
    fn evaluate(&self) -> Result<ModelPointer, ModelError> {
        match self {
            Self::Base(pointer) => Ok(*pointer),
            Self::Add(pointer, offset) | Self::ReverseAdd(offset, pointer) => {
                let mut pointer = pointer.evaluate()?;
                pointer.index += offset.evaluate()?;
                Ok(pointer)
            }
            Self::Subtract(pointer, offset) => {
                let mut pointer = pointer.evaluate()?;
                pointer.index -= offset.evaluate()?;
                Ok(pointer)
            }
            Self::Conditional(condition, when_true, when_false) => {
                let mut pointer = if *condition {
                    when_true.evaluate()?
                } else {
                    when_false.evaluate()?
                };
                pointer.points_to_const = self.points_to_const();
                Ok(pointer)
            }
            Self::Comma(ignored, pointer) => {
                ignored.evaluate()?;
                pointer.evaluate()
            }
        }
    }

    fn points_to_const(&self) -> bool {
        match self {
            Self::Base(pointer) => pointer.points_to_const,
            Self::Add(pointer, _)
            | Self::ReverseAdd(_, pointer)
            | Self::Subtract(pointer, _)
            | Self::Comma(_, pointer) => pointer.points_to_const(),
            Self::Conditional(_, when_true, when_false) => {
                when_true.points_to_const() || when_false.points_to_const()
            }
        }
    }

    fn render(&self) -> String {
        match self {
            Self::Base(pointer) => {
                let base = match (pointer.root, pointer.points_to_const) {
                    (ArrayRoot::Left, false) => "left",
                    (ArrayRoot::Right, false) => "right",
                    (ArrayRoot::Left, true) => "const_left",
                    (ArrayRoot::Right, true) => "const_right",
                };
                format!("({base} + {})", pointer.index)
            }
            Self::Add(pointer, offset) => {
                format!("({} + {})", pointer.render(), offset.render())
            }
            Self::ReverseAdd(offset, pointer) => {
                format!("({} + {})", offset.render(), pointer.render())
            }
            Self::Subtract(pointer, offset) => {
                format!("({} - {})", pointer.render(), offset.render())
            }
            Self::Conditional(condition, when_true, when_false) => format!(
                "({} ? {} : {})",
                i64::from(*condition),
                when_true.render(),
                when_false.render()
            ),
            Self::Comma(ignored, pointer) => {
                format!("({}, {})", ignored.render(), pointer.render())
            }
        }
    }
}

#[derive(Clone, Debug)]
enum ScalarExpr {
    Literal(i64),
    PointerDifference(Box<PointerExpr>, Box<PointerExpr>),
    Conditional(bool, Box<Self>, Box<Self>),
    Comma(Box<Self>, Box<Self>),
}

impl ScalarExpr {
    fn evaluate(&self) -> Result<i64, ModelError> {
        match self {
            Self::Literal(value) => Ok(*value),
            Self::PointerDifference(left, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                if left.root != right.root {
                    return Err(ModelError::CrossArrayDifference);
                }
                Ok(left.index - right.index)
            }
            Self::Conditional(condition, when_true, when_false) => {
                if *condition {
                    when_true.evaluate()
                } else {
                    when_false.evaluate()
                }
            }
            Self::Comma(left, right) => {
                left.evaluate()?;
                right.evaluate()
            }
        }
    }

    fn render(&self) -> String {
        match self {
            Self::Literal(value) => value.to_string(),
            Self::PointerDifference(left, right) => {
                format!("({} - {})", left.render(), right.render())
            }
            Self::Conditional(condition, when_true, when_false) => format!(
                "({} ? {} : {})",
                i64::from(*condition),
                when_true.render(),
                when_false.render()
            ),
            Self::Comma(left, right) => format!("({}, {})", left.render(), right.render()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AdjustedParameterFieldKind {
    Scalar,
    Aggregate,
}

impl AdjustedParameterFieldKind {
    const ALL: [Self; 2] = [Self::Scalar, Self::Aggregate];

    fn pointer_type(self) -> &'static str {
        match self {
            Self::Scalar => "int *",
            Self::Aggregate => "struct Point *",
        }
    }

    fn const_pointer_type(self) -> &'static str {
        match self {
            Self::Scalar => "const int *",
            Self::Aggregate => "const struct Point *",
        }
    }

    fn field_name(self) -> &'static str {
        match self {
            Self::Scalar => "values",
            Self::Aggregate => "points",
        }
    }

    fn suffix(self) -> &'static str {
        match self {
            Self::Scalar => "int",
            Self::Aggregate => "point",
        }
    }

    fn read(self, pointer: &str) -> String {
        match self {
            Self::Scalar => format!("*{pointer}"),
            Self::Aggregate => format!("{pointer}->value"),
        }
    }

    fn write(self, pointer: &str, value: i64) -> String {
        match self {
            Self::Scalar => format!("*{pointer} = {value}"),
            Self::Aggregate => format!("{pointer}->value = {value}"),
        }
    }

    fn compound_add(self, pointer: &str, value: i64) -> String {
        match self {
            Self::Scalar => format!("*{pointer} += {value}"),
            Self::Aggregate => format!("{pointer}->value += {value}"),
        }
    }

    fn inner_bounds_error(self) -> &'static str {
        match self {
            Self::Scalar => "array 'values' index 3 out of bounds for length 3",
            Self::Aggregate => "struct array field 'points' index 3 out of bounds for length 3",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AdjustedParameterStorage {
    RootLeft,
    RootRight,
    LeftPrimary,
    LeftSecondary,
    RightPrimary,
    RightSecondary,
    NamedLeftPrimary,
    NamedLeftSecondary,
    NamedRightPrimary,
    NamedRightSecondary,
    AnonymousLeftPrimary,
    AnonymousLeftSecondary,
    AnonymousRightPrimary,
    AnonymousRightSecondary,
    UnionLeftPrimary,
    UnionLeftSecondary,
    UnionRightPrimary,
    UnionRightSecondary,
}

impl AdjustedParameterStorage {
    const ALL: [Self; 18] = [
        Self::RootLeft,
        Self::RootRight,
        Self::LeftPrimary,
        Self::LeftSecondary,
        Self::RightPrimary,
        Self::RightSecondary,
        Self::NamedLeftPrimary,
        Self::NamedLeftSecondary,
        Self::NamedRightPrimary,
        Self::NamedRightSecondary,
        Self::AnonymousLeftPrimary,
        Self::AnonymousLeftSecondary,
        Self::AnonymousRightPrimary,
        Self::AnonymousRightSecondary,
        Self::UnionLeftPrimary,
        Self::UnionLeftSecondary,
        Self::UnionRightPrimary,
        Self::UnionRightSecondary,
    ];

    fn is_root(self) -> bool {
        matches!(self, Self::RootLeft | Self::RootRight)
    }

    fn path_index(self) -> usize {
        match self {
            Self::RootLeft | Self::RootRight => 0,
            Self::LeftPrimary | Self::LeftSecondary | Self::RightPrimary | Self::RightSecondary => {
                1
            }
            Self::NamedLeftPrimary
            | Self::NamedLeftSecondary
            | Self::NamedRightPrimary
            | Self::NamedRightSecondary => 2,
            Self::AnonymousLeftPrimary
            | Self::AnonymousLeftSecondary
            | Self::AnonymousRightPrimary
            | Self::AnonymousRightSecondary => 3,
            Self::UnionLeftPrimary
            | Self::UnionLeftSecondary
            | Self::UnionRightPrimary
            | Self::UnionRightSecondary => 4,
        }
    }

    fn literal_path_index(self) -> usize {
        match self {
            Self::NamedLeftPrimary
            | Self::NamedLeftSecondary
            | Self::NamedRightPrimary
            | Self::NamedRightSecondary => 0,
            Self::AnonymousLeftPrimary
            | Self::AnonymousLeftSecondary
            | Self::AnonymousRightPrimary
            | Self::AnonymousRightSecondary => 1,
            Self::UnionLeftPrimary
            | Self::UnionLeftSecondary
            | Self::UnionRightPrimary
            | Self::UnionRightSecondary => 2,
            _ => unreachable!("literal outer-array models use only named/anonymous/union roots"),
        }
    }

    fn expression(self) -> &'static str {
        match self {
            Self::RootLeft => "root_left",
            Self::RootRight => "root_right",
            Self::LeftPrimary => "wrapper_left.primary",
            Self::LeftSecondary => "wrapper_left.secondary",
            Self::RightPrimary => "wrapper_right.primary",
            Self::RightSecondary => "wrapper_right.secondary",
            Self::NamedLeftPrimary => "named_left.nested.primary",
            Self::NamedLeftSecondary => "named_left.nested.secondary",
            Self::NamedRightPrimary => "named_right.nested.primary",
            Self::NamedRightSecondary => "named_right.nested.secondary",
            Self::AnonymousLeftPrimary => "anonymous_left.nested.primary",
            Self::AnonymousLeftSecondary => "anonymous_left.nested.secondary",
            Self::AnonymousRightPrimary => "anonymous_right.nested.primary",
            Self::AnonymousRightSecondary => "anonymous_right.nested.secondary",
            Self::UnionLeftPrimary => "choice_left.nested.primary",
            Self::UnionLeftSecondary => "choice_left.nested.secondary",
            Self::UnionRightPrimary => "choice_right.nested.primary",
            Self::UnionRightSecondary => "choice_right.nested.secondary",
        }
    }

    fn literal_expression(self) -> &'static str {
        match self {
            Self::NamedLeftPrimary => "named_left->nested.primary",
            Self::NamedLeftSecondary => "named_left->nested.secondary",
            Self::NamedRightPrimary => "named_right->nested.primary",
            Self::NamedRightSecondary => "named_right->nested.secondary",
            Self::AnonymousLeftPrimary => "anonymous_left->nested.primary",
            Self::AnonymousLeftSecondary => "anonymous_left->nested.secondary",
            Self::AnonymousRightPrimary => "anonymous_right->nested.primary",
            Self::AnonymousRightSecondary => "anonymous_right->nested.secondary",
            Self::UnionLeftPrimary => "choice_left->nested.primary",
            Self::UnionLeftSecondary => "choice_left->nested.secondary",
            Self::UnionRightPrimary => "choice_right->nested.primary",
            Self::UnionRightSecondary => "choice_right->nested.secondary",
            _ => unreachable!("literal outer-array models use only named/anonymous/union roots"),
        }
    }

    fn model_offset(self) -> i64 {
        Self::ALL
            .iter()
            .position(|storage| *storage == self)
            .expect("adjusted parameter storage must be modeled") as i64
            * 20
    }

    fn other_field(self) -> Self {
        match self {
            Self::RootLeft | Self::RootRight => Self::LeftSecondary,
            Self::LeftPrimary => Self::LeftSecondary,
            Self::LeftSecondary => Self::LeftPrimary,
            Self::RightPrimary => Self::RightSecondary,
            Self::RightSecondary => Self::RightPrimary,
            Self::NamedLeftPrimary => Self::NamedLeftSecondary,
            Self::NamedLeftSecondary => Self::NamedLeftPrimary,
            Self::NamedRightPrimary => Self::NamedRightSecondary,
            Self::NamedRightSecondary => Self::NamedRightPrimary,
            Self::AnonymousLeftPrimary => Self::AnonymousLeftSecondary,
            Self::AnonymousLeftSecondary => Self::AnonymousLeftPrimary,
            Self::AnonymousRightPrimary => Self::AnonymousRightSecondary,
            Self::AnonymousRightSecondary => Self::AnonymousRightPrimary,
            Self::UnionLeftPrimary => Self::UnionLeftSecondary,
            Self::UnionLeftSecondary => Self::UnionLeftPrimary,
            Self::UnionRightPrimary => Self::UnionRightSecondary,
            Self::UnionRightSecondary => Self::UnionRightPrimary,
        }
    }

    fn other_owner(self) -> Self {
        match self {
            Self::RootLeft => Self::RootRight,
            Self::RootRight => Self::RootLeft,
            Self::LeftPrimary => Self::RightPrimary,
            Self::LeftSecondary => Self::RightSecondary,
            Self::RightPrimary => Self::LeftPrimary,
            Self::RightSecondary => Self::LeftSecondary,
            Self::NamedLeftPrimary => Self::NamedRightPrimary,
            Self::NamedLeftSecondary => Self::NamedRightSecondary,
            Self::NamedRightPrimary => Self::NamedLeftPrimary,
            Self::NamedRightSecondary => Self::NamedLeftSecondary,
            Self::AnonymousLeftPrimary => Self::AnonymousRightPrimary,
            Self::AnonymousLeftSecondary => Self::AnonymousRightSecondary,
            Self::AnonymousRightPrimary => Self::AnonymousLeftPrimary,
            Self::AnonymousRightSecondary => Self::AnonymousLeftSecondary,
            Self::UnionLeftPrimary => Self::UnionRightPrimary,
            Self::UnionLeftSecondary => Self::UnionRightSecondary,
            Self::UnionRightPrimary => Self::UnionLeftPrimary,
            Self::UnionRightSecondary => Self::UnionLeftSecondary,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AdjustedParameterRoute {
    Direct,
    Reverse,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AdjustedParameterPointer {
    kind: AdjustedParameterFieldKind,
    storage: AdjustedParameterStorage,
    outer: i64,
    inner: i64,
    route: AdjustedParameterRoute,
}

impl AdjustedParameterPointer {
    fn render_address(self, parameter: &str, outer: &str, inner: &str) -> String {
        let field = self.kind.field_name();
        match self.route {
            AdjustedParameterRoute::Direct => {
                format!("&{parameter}[{outer}++].nested.{field}[{inner}++]")
            }
            AdjustedParameterRoute::Reverse => {
                format!("&({inner}++)[{parameter}[{outer}++].nested.{field}]")
            }
        }
    }

    fn render_static_address(self, parameter: &str) -> String {
        let field = self.kind.field_name();
        match self.route {
            AdjustedParameterRoute::Direct => {
                format!(
                    "&{parameter}[{}].nested.{field}[{}]",
                    self.outer, self.inner
                )
            }
            AdjustedParameterRoute::Reverse => {
                format!(
                    "&{}[{parameter}[{}].nested.{field}]",
                    self.inner, self.outer
                )
            }
        }
    }

    fn model_value(self) -> i64 {
        self.storage.model_offset() + self.outer * 4 + self.inner + 1
    }

    fn same_array(self, other: Self) -> bool {
        self.kind == other.kind && self.storage == other.storage && self.outer == other.outer
    }

    fn same_element(self, other: Self) -> bool {
        self.same_array(other) && self.inner == other.inner
    }
}

impl AdjustedParameterRelation {
    const ALL: [Self; 6] = [
        Self::SameElement,
        Self::SameArrayDistinctElement,
        Self::DifferentOuterElement,
        Self::DifferentFieldPath,
        Self::DifferentOwner,
        Self::RootVersusField,
    ];
}

impl AdjustedParameterOperation {
    const ALL: [Self; 5] = [
        Self::Read,
        Self::WriteThroughAddress,
        Self::Equality,
        Self::Difference,
        Self::Ordering,
    ];
}

impl LiteralAdjustedParameterRelation {
    const ALL: [Self; 4] = [
        Self::SameElement,
        Self::SameArrayDistinctElement,
        Self::DifferentField,
        Self::DifferentLiteralRoot,
    ];
}

impl DirectLiteralAdjustedParameterRelation {
    const ALL: [Self; 3] = [
        Self::SameElement,
        Self::SameArrayDistinctElement,
        Self::SeparateLiteralRoot,
    ];
}

fn direct_literal_adjusted_parameter_first_pointer(
    kind: AdjustedParameterFieldKind,
    case_index: usize,
) -> AdjustedParameterPointer {
    AdjustedParameterPointer {
        kind,
        storage: AdjustedParameterStorage::RootLeft,
        outer: ((case_index / 3) % 2) as i64,
        inner: ((case_index * 2 + 1) % 3) as i64,
        route: if case_index & 1 == 0 {
            AdjustedParameterRoute::Direct
        } else {
            AdjustedParameterRoute::Reverse
        },
    }
}

fn direct_literal_adjusted_parameter_related_pointer(
    first: AdjustedParameterPointer,
    relation: DirectLiteralAdjustedParameterRelation,
) -> AdjustedParameterPointer {
    let mut second = AdjustedParameterPointer {
        route: match first.route {
            AdjustedParameterRoute::Direct => AdjustedParameterRoute::Reverse,
            AdjustedParameterRoute::Reverse => AdjustedParameterRoute::Direct,
        },
        ..first
    };
    match relation {
        DirectLiteralAdjustedParameterRelation::SameElement => {}
        DirectLiteralAdjustedParameterRelation::SameArrayDistinctElement => {
            second.inner = (first.inner + 1) % 3;
        }
        DirectLiteralAdjustedParameterRelation::SeparateLiteralRoot => {
            second.storage = AdjustedParameterStorage::RootRight;
        }
    }
    second
}

fn literal_adjusted_parameter_first_pointer(
    kind: AdjustedParameterFieldKind,
    case_index: usize,
) -> AdjustedParameterPointer {
    const STORAGES: [AdjustedParameterStorage; 12] = [
        AdjustedParameterStorage::NamedLeftPrimary,
        AdjustedParameterStorage::NamedLeftSecondary,
        AdjustedParameterStorage::NamedRightPrimary,
        AdjustedParameterStorage::NamedRightSecondary,
        AdjustedParameterStorage::AnonymousLeftPrimary,
        AdjustedParameterStorage::AnonymousLeftSecondary,
        AdjustedParameterStorage::AnonymousRightPrimary,
        AdjustedParameterStorage::AnonymousRightSecondary,
        AdjustedParameterStorage::UnionLeftPrimary,
        AdjustedParameterStorage::UnionLeftSecondary,
        AdjustedParameterStorage::UnionRightPrimary,
        AdjustedParameterStorage::UnionRightSecondary,
    ];
    AdjustedParameterPointer {
        kind,
        storage: STORAGES[case_index % STORAGES.len()],
        outer: ((case_index / 3) % 2) as i64,
        inner: ((case_index * 2 + 1) % 3) as i64,
        route: if case_index & 1 == 0 {
            AdjustedParameterRoute::Direct
        } else {
            AdjustedParameterRoute::Reverse
        },
    }
}

fn literal_adjusted_parameter_related_pointer(
    first: AdjustedParameterPointer,
    relation: LiteralAdjustedParameterRelation,
) -> AdjustedParameterPointer {
    let mut second = AdjustedParameterPointer {
        route: match first.route {
            AdjustedParameterRoute::Direct => AdjustedParameterRoute::Reverse,
            AdjustedParameterRoute::Reverse => AdjustedParameterRoute::Direct,
        },
        ..first
    };
    match relation {
        LiteralAdjustedParameterRelation::SameElement => {}
        LiteralAdjustedParameterRelation::SameArrayDistinctElement => {
            second.inner = (first.inner + 1) % 3;
        }
        LiteralAdjustedParameterRelation::DifferentField => {
            second.storage = first.storage.other_field();
        }
        LiteralAdjustedParameterRelation::DifferentLiteralRoot => {
            second.storage = first.storage.other_owner();
        }
    }
    second
}

fn adjusted_parameter_first_pointer(
    kind: AdjustedParameterFieldKind,
    relation_index: usize,
    operation_index: usize,
) -> AdjustedParameterPointer {
    AdjustedParameterPointer {
        kind,
        storage: match (relation_index + operation_index) % 3 {
            0 => AdjustedParameterStorage::RootLeft,
            1 => AdjustedParameterStorage::LeftPrimary,
            _ => AdjustedParameterStorage::RightSecondary,
        },
        outer: ((relation_index + operation_index) % 2) as i64,
        inner: ((relation_index * 2 + operation_index) % 3) as i64,
        route: if (relation_index + operation_index) % 2 == 0 {
            AdjustedParameterRoute::Direct
        } else {
            AdjustedParameterRoute::Reverse
        },
    }
}

fn adjusted_parameter_nested_first_pointer(
    kind: AdjustedParameterFieldKind,
    relation_index: usize,
    case_index: usize,
) -> AdjustedParameterPointer {
    let mut pointer = adjusted_parameter_first_pointer(kind, relation_index, case_index);
    pointer.storage = AdjustedParameterStorage::ALL
        [(relation_index + case_index) % AdjustedParameterStorage::ALL.len()];
    pointer
}

fn adjusted_parameter_related_pointer(
    first: AdjustedParameterPointer,
    relation: AdjustedParameterRelation,
) -> AdjustedParameterPointer {
    let mut second = AdjustedParameterPointer {
        route: match first.route {
            AdjustedParameterRoute::Direct => AdjustedParameterRoute::Reverse,
            AdjustedParameterRoute::Reverse => AdjustedParameterRoute::Direct,
        },
        ..first
    };
    match relation {
        AdjustedParameterRelation::SameElement => {}
        AdjustedParameterRelation::SameArrayDistinctElement => second.inner = (first.inner + 1) % 3,
        AdjustedParameterRelation::DifferentOuterElement => second.outer = (first.outer + 1) % 2,
        AdjustedParameterRelation::DifferentFieldPath => {
            second.storage = first.storage.other_field();
        }
        AdjustedParameterRelation::DifferentOwner => {
            second.storage = first.storage.other_owner();
        }
        AdjustedParameterRelation::RootVersusField => {
            second.storage = if first.storage.is_root() {
                AdjustedParameterStorage::UnionRightPrimary
            } else {
                AdjustedParameterStorage::RootRight
            }
        }
    }
    second
}

fn adjusted_parameter_prelude() -> &'static str {
    "struct Point { int value; };\n\
     struct Inner { int values[3]; struct Point points[3]; };\n\
     struct Item { struct Inner nested; };\n\
     struct Wrapper { struct Item primary[2]; struct Item secondary[2]; };\n\
     struct NamedHolder { struct Wrapper nested; };\n\
     struct AnonymousHolder { struct { struct Item primary[2]; struct Item secondary[2]; } nested; };\n\
     union Choice { struct Wrapper nested; int marker; };"
}

fn adjusted_parameter_declarations() -> &'static str {
    "struct Item root_left[2];\n\
     struct Item root_right[2];\n\
     struct Wrapper wrapper_left;\n\
     struct Wrapper wrapper_right;\n\
     struct NamedHolder named_left;\n\
     struct NamedHolder named_right;\n\
     struct AnonymousHolder anonymous_left;\n\
     struct AnonymousHolder anonymous_right;\n\
     union Choice choice_left;\n\
     union Choice choice_right;"
}

fn adjusted_parameter_model_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    operation: AdjustedParameterOperation,
) -> (String, ExpectedInterpretation) {
    let first_initial = first.model_value();
    let second_initial = second.model_value();
    let effective_first_initial = if first.same_element(second) {
        second_initial
    } else {
        first_initial
    };
    let first_address = first.render_address("first", "outer_a", "inner_a");
    let second_address = second.render_address("second", "outer_b", "inner_b");
    let initialize_first = first.kind.write("a", first_initial);
    let initialize_second = second.kind.write("b", second_initial);
    let marker_score = 4;
    let (operation_source, expected) = match operation {
        AdjustedParameterOperation::Read => (
            format!(
                "return {} + {} + marker_score;",
                first.kind.read("a"),
                second.kind.read("b")
            ),
            ExpectedInterpretation::Value(effective_first_initial + second_initial + marker_score),
        ),
        AdjustedParameterOperation::WriteThroughAddress => {
            let replacement = 151;
            let observed_second = if first.same_element(second) {
                replacement
            } else {
                second_initial
            };
            (
                format!(
                    "{}; return {} + {} + marker_score;",
                    first.kind.write("a", replacement),
                    first.kind.read("a"),
                    second.kind.read("b")
                ),
                ExpectedInterpretation::Value(replacement + observed_second + marker_score),
            )
        }
        AdjustedParameterOperation::Equality => (
            "return (a == b) * 10 + marker_score;".to_string(),
            ExpectedInterpretation::Value(
                i64::from(first.same_element(second)) * 10 + marker_score,
            ),
        ),
        AdjustedParameterOperation::Difference if first.same_array(second) => (
            "return (a - b) + 20 + marker_score;".to_string(),
            ExpectedInterpretation::Value(first.inner - second.inner + 20 + marker_score),
        ),
        AdjustedParameterOperation::Difference => (
            "return (a - b) + marker_score;".to_string(),
            ExpectedInterpretation::Error("cannot subtract pointers to different arrays"),
        ),
        AdjustedParameterOperation::Ordering if first.same_array(second) => (
            "return (a < b) * 10 + marker_score;".to_string(),
            ExpectedInterpretation::Value(
                i64::from(first.inner < second.inner) * 10 + marker_score,
            ),
        ),
        AdjustedParameterOperation::Ordering => (
            "return (a < b) * 10 + marker_score;".to_string(),
            ExpectedInterpretation::Error("cannot compare pointers to different arrays"),
        ),
    };
    let source = format!(
        "{prelude}\n\
         int probe(struct Item first[], struct Item second[]) {{\n\
             int outer_a = {first_outer}; int inner_a = {first_inner};\n\
             int outer_b = {second_outer}; int inner_b = {second_inner};\n\
             {pointer_type}a = {first_address};\n\
             {pointer_type}b = {second_address};\n\
             {initialize_first};\n\
             {initialize_second};\n\
             int marker_score = (outer_a == {first_outer_plus_one}) +\n\
                 (inner_a == {first_inner_plus_one}) +\n\
                 (outer_b == {second_outer_plus_one}) +\n\
                 (inner_b == {second_inner_plus_one});\n\
             {operation_source}\n\
         }}\n\
         int main(void) {{\n\
             {declarations}\n\
             return probe({first_storage}, {second_storage});\n\
         }}\n",
        prelude = adjusted_parameter_prelude(),
        declarations = adjusted_parameter_declarations(),
        pointer_type = first.kind.pointer_type(),
        first_outer = first.outer,
        first_inner = first.inner,
        second_outer = second.outer,
        second_inner = second.inner,
        first_outer_plus_one = first.outer + 1,
        first_inner_plus_one = first.inner + 1,
        second_outer_plus_one = second.outer + 1,
        second_inner_plus_one = second.inner + 1,
        first_storage = first.storage.expression(),
        second_storage = second.storage.expression(),
    );
    (source, expected)
}

fn adjusted_parameter_inner_bounds_program(
    kind: AdjustedParameterFieldKind,
    storage: AdjustedParameterStorage,
) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ {declarations} return probe({storage}); }}\n",
        prelude = adjusted_parameter_prelude(),
        declarations = adjusted_parameter_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
        storage = storage.expression(),
    )
}

fn adjusted_parameter_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
    storage: AdjustedParameterStorage,
) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[2].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ {declarations} return probe({storage}); }}\n",
        prelude = adjusted_parameter_prelude(),
        declarations = adjusted_parameter_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
        storage = storage.expression(),
    )
}

fn adjusted_parameter_const_ancestor_program(
    kind: AdjustedParameterFieldKind,
    storage: AdjustedParameterStorage,
) -> String {
    let selected_storage = if storage.is_root() {
        "locked"
    } else {
        "wrapper.primary"
    };
    format!(
        "struct Point {{ int value; }};\n\
         struct Inner {{ int values[3]; struct Point points[3]; }};\n\
         struct Item {{ const struct Inner nested; }};\n\
         struct Wrapper {{ struct Item primary[2]; }};\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; {write}; return 0; }}\n\
         int main(void) {{ struct Item locked[2]; struct Wrapper wrapper; return probe({selected_storage}); }}\n",
        pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 9),
    )
}

fn adjusted_parameter_nested_const_ancestor_program(
    kind: AdjustedParameterFieldKind,
    storage: AdjustedParameterStorage,
) -> String {
    format!(
        "struct Point {{ int value; }};\n\
         struct Inner {{ int values[3]; struct Point points[3]; }};\n\
         struct Item {{ const struct Inner nested; }};\n\
         struct Wrapper {{ struct Item primary[2]; struct Item secondary[2]; }};\n\
         struct NamedHolder {{ struct Wrapper nested; }};\n\
         struct AnonymousHolder {{ struct {{ struct Item primary[2]; struct Item secondary[2]; }} nested; }};\n\
         union Choice {{ struct Wrapper nested; int marker; }};\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; {write}; return 0; }}\n\
         int main(void) {{\n\
             struct NamedHolder named_left; struct NamedHolder named_right;\n\
             struct AnonymousHolder anonymous_left; struct AnonymousHolder anonymous_right;\n\
             union Choice choice_left; union Choice choice_right;\n\
             return probe({storage});\n\
         }}\n",
        pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 9),
        storage = storage.expression(),
    )
}

fn adjusted_parameter_aggregate_type_mismatch_program(storage: AdjustedParameterStorage) -> String {
    format!(
        "{prelude}\n\
         struct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ {declarations} return probe({storage}); }}\n",
        prelude = adjusted_parameter_prelude(),
        declarations = adjusted_parameter_declarations(),
        storage = storage.expression(),
    )
}

fn adjusted_parameter_forward_expr(
    pointer: AdjustedParameterPointer,
    address: &str,
    two_hop: bool,
    is_const: bool,
) -> String {
    format!(
        "forward_{}alias_{}{}({address})",
        if is_const { "const_" } else { "" },
        pointer.kind.suffix(),
        if two_hop { "_twice" } else { "" },
    )
}

fn adjusted_parameter_alias_mutation_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    two_hop: [bool; 3],
) -> String {
    let kind = first.kind;
    let first_address = first.render_static_address("first_items");
    let second_address = second.render_static_address("second_items");
    let reader_address = reader.render_static_address("reader_items");
    let first_forward = adjusted_parameter_forward_expr(first, &first_address, two_hop[0], false);
    let second_forward =
        adjusted_parameter_forward_expr(second, &second_address, two_hop[1], false);
    let reader_forward = adjusted_parameter_forward_expr(reader, &reader_address, two_hop[2], true);
    let pointer_type = kind.pointer_type();
    let const_pointer_type = kind.const_pointer_type();
    let suffix = kind.suffix();

    format!(
        "{prelude}\n\
         {pointer_type}forward_alias_{suffix}({pointer_type}value) {{ return value; }}\n\
         {pointer_type}forward_alias_{suffix}_twice({pointer_type}value) {{ return forward_alias_{suffix}(value); }}\n\
         {const_pointer_type}forward_const_alias_{suffix}({const_pointer_type}value) {{ return value; }}\n\
         {const_pointer_type}forward_const_alias_{suffix}_twice({const_pointer_type}value) {{ return forward_const_alias_{suffix}(value); }}\n\
         int mutate_alias_{suffix}({pointer_type}first, {pointer_type}second, {const_pointer_type}reader, {pointer_type}fallback) {{\n\
             {write_first};\n\
             int before = {read_reader};\n\
             {compound_second};\n\
             int after = {read_reader};\n\
             first = fallback; second = fallback; reader = fallback;\n\
             return before * 3 + after * 5 + (first == fallback) + (second == fallback) + (reader == fallback);\n\
         }}\n\
         int probe(struct Item first_items[], struct Item second_items[], struct Item reader_items[]) {{\n\
             {pointer_type}a = {first_forward};\n\
             {pointer_type}b = {second_forward};\n\
             {const_pointer_type}r = {reader_forward};\n\
             {initialize_first}; {initialize_second};\n\
             int observed = mutate_alias_{suffix}(a, b, r, a);\n\
             int caller_identity = (a == {first_address}) + (b == {second_address}) + (r == {reader_address});\n\
             return observed + {read_a} * 7 + {read_b} * 11 + {read_r} * 13 + caller_identity * 17;\n\
         }}\n\
         int main(void) {{\n\
             {declarations}\n\
             return probe({first_storage}, {second_storage}, {reader_storage});\n\
         }}\n",
        prelude = adjusted_parameter_prelude(),
        declarations = adjusted_parameter_declarations(),
        write_first = kind.write("first", replacement),
        read_reader = kind.read("reader"),
        compound_second = kind.compound_add("second", delta),
        initialize_first = kind.write("a", first.model_value()),
        initialize_second = kind.write("b", second.model_value()),
        read_a = kind.read("a"),
        read_b = kind.read("b"),
        read_r = kind.read("r"),
        first_storage = first.storage.expression(),
        second_storage = second.storage.expression(),
        reader_storage = reader.storage.expression(),
    )
}

fn literal_adjusted_parameter_declarations() -> &'static str {
    "int marker = 0;\n\
     struct NamedHolder *named_left = &(struct NamedHolder){ .nested = { .primary = {{ .nested = { .values = {++marker} } }} } };\n\
     struct NamedHolder *named_right = &(struct NamedHolder){ .nested = { .primary = {{ .nested = { .values = {++marker} } }} } };\n\
     struct AnonymousHolder *anonymous_left = &(struct AnonymousHolder){ .nested = { .primary = {{ .nested = { .values = {++marker} } }} } };\n\
     struct AnonymousHolder *anonymous_right = &(struct AnonymousHolder){ .nested = { .primary = {{ .nested = { .values = {++marker} } }} } };\n\
     union Choice *choice_left = &(union Choice){ .nested = { .primary = {{ .nested = { .values = {++marker} } }} } };\n\
     union Choice *choice_right = &(union Choice){ .nested = { .primary = {{ .nested = { .values = {++marker} } }} } };"
}

fn literal_adjusted_parameter_inner_bounds_program(
    kind: AdjustedParameterFieldKind,
    storage: AdjustedParameterStorage,
) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ {declarations} return probe({storage}); }}\n",
        prelude = adjusted_parameter_prelude(),
        declarations = literal_adjusted_parameter_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
        storage = storage.literal_expression(),
    )
}

fn literal_adjusted_parameter_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
    storage: AdjustedParameterStorage,
) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[2].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ {declarations} return probe({storage}); }}\n",
        prelude = adjusted_parameter_prelude(),
        declarations = literal_adjusted_parameter_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
        storage = storage.literal_expression(),
    )
}

fn literal_adjusted_parameter_const_discard_program(kind: AdjustedParameterFieldKind) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{\n\
             const struct NamedHolder *locked = &(const struct NamedHolder){{ .nested = {{}} }};\n\
             return probe(locked->nested.primary);\n\
         }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn literal_adjusted_parameter_cross_root_program(kind: AdjustedParameterFieldKind) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item first[], struct Item second[]) {{\n\
             {pointer_type}a = &first[0].nested.{field}[0];\n\
             {pointer_type}b = &second[0].nested.{field}[0];\n\
             return a - b;\n\
         }}\n\
         int main(void) {{\n\
             {declarations}\n\
             return probe(named_left->nested.primary, named_right->nested.primary);\n\
         }}\n",
        prelude = adjusted_parameter_prelude(),
        declarations = literal_adjusted_parameter_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn literal_adjusted_parameter_type_mismatch_program(storage: AdjustedParameterStorage) -> String {
    format!(
        "{prelude}\n\
         struct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ {declarations} return probe({storage}); }}\n",
        prelude = adjusted_parameter_prelude(),
        declarations = literal_adjusted_parameter_declarations(),
        storage = storage.literal_expression(),
    )
}

fn literal_adjusted_parameter_alias_mutation_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    two_hop: [bool; 3],
) -> String {
    let kind = first.kind;
    let first_address = first.render_static_address("first_items");
    let second_address = second.render_static_address("second_items");
    let reader_address = reader.render_static_address("reader_items");
    let first_forward = adjusted_parameter_forward_expr(first, &first_address, two_hop[0], false);
    let second_forward =
        adjusted_parameter_forward_expr(second, &second_address, two_hop[1], false);
    let reader_forward = adjusted_parameter_forward_expr(reader, &reader_address, two_hop[2], true);
    let pointer_type = kind.pointer_type();
    let const_pointer_type = kind.const_pointer_type();
    let suffix = kind.suffix();

    format!(
        "{prelude}\n\
         {pointer_type}forward_alias_{suffix}({pointer_type}value) {{ return value; }}\n\
         {pointer_type}forward_alias_{suffix}_twice({pointer_type}value) {{ return forward_alias_{suffix}(value); }}\n\
         {const_pointer_type}forward_const_alias_{suffix}({const_pointer_type}value) {{ return value; }}\n\
         {const_pointer_type}forward_const_alias_{suffix}_twice({const_pointer_type}value) {{ return forward_const_alias_{suffix}(value); }}\n\
         int mutate_alias_{suffix}({pointer_type}first, {pointer_type}second, {const_pointer_type}reader, {pointer_type}fallback) {{\n\
             {write_first};\n\
             int before = {read_reader};\n\
             {compound_second};\n\
             int after = {read_reader};\n\
             first = fallback; second = fallback; reader = fallback;\n\
             return before * 3 + after * 5 + (first == fallback) + (second == fallback) + (reader == fallback);\n\
         }}\n\
         int probe(struct Item first_items[], struct Item second_items[], struct Item reader_items[]) {{\n\
             {pointer_type}a = {first_forward};\n\
             {pointer_type}b = {second_forward};\n\
             {const_pointer_type}r = {reader_forward};\n\
             {initialize_first}; {initialize_second};\n\
             int observed = mutate_alias_{suffix}(a, b, r, a);\n\
             int caller_identity = (a == {first_address}) + (b == {second_address}) + (r == {reader_address});\n\
             return observed + {read_a} * 7 + {read_b} * 11 + {read_r} * 13 + caller_identity * 17;\n\
         }}\n\
         int main(void) {{\n\
             {declarations}\n\
             int result = probe({first_storage}, {second_storage}, {reader_storage});\n\
             return result + (marker == 6) * 19;\n\
         }}\n",
        prelude = adjusted_parameter_prelude(),
        declarations = literal_adjusted_parameter_declarations(),
        write_first = kind.write("first", replacement),
        read_reader = kind.read("reader"),
        compound_second = kind.compound_add("second", delta),
        initialize_first = kind.write("a", first.model_value()),
        initialize_second = kind.write("b", second.model_value()),
        read_a = kind.read("a"),
        read_b = kind.read("b"),
        read_r = kind.read("r"),
        first_storage = first.storage.literal_expression(),
        second_storage = second.storage.literal_expression(),
        reader_storage = reader.storage.literal_expression(),
    )
}

fn adjusted_parameter_alias_mutation_expected(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
) -> i64 {
    let same = first.same_element(second);
    let mut first_value = replacement;
    let mut second_value = if same {
        replacement
    } else {
        second.model_value()
    };
    let before = if reader.same_element(first) {
        first_value
    } else {
        second_value
    };
    if same {
        first_value += delta;
        second_value = first_value;
    } else {
        second_value += delta;
    }
    let after = if reader.same_element(first) {
        first_value
    } else {
        second_value
    };
    let reader_value = after;
    let observed = before * 3 + after * 5 + 3;
    observed + first_value * 7 + second_value * 11 + reader_value * 13 + 3 * 17
}

fn direct_literal_adjusted_parameter_alias_mutation_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    two_hop: [bool; 3],
) -> String {
    let kind = first.kind;
    let separate_roots = second.storage == AdjustedParameterStorage::RootRight;
    let first_parameter = if separate_roots {
        "first_items"
    } else {
        "items"
    };
    let second_parameter = if separate_roots {
        "second_items"
    } else {
        "items"
    };
    let reader_parameter = if reader.storage == AdjustedParameterStorage::RootRight {
        second_parameter
    } else {
        first_parameter
    };
    let first_address = first.render_static_address(first_parameter);
    let second_address = second.render_static_address(second_parameter);
    let reader_address = reader.render_static_address(reader_parameter);
    let first_forward = adjusted_parameter_forward_expr(first, &first_address, two_hop[0], false);
    let second_forward =
        adjusted_parameter_forward_expr(second, &second_address, two_hop[1], false);
    let reader_forward = adjusted_parameter_forward_expr(reader, &reader_address, two_hop[2], true);
    let pointer_type = kind.pointer_type();
    let const_pointer_type = kind.const_pointer_type();
    let suffix = kind.suffix();
    let parameters = if separate_roots {
        "struct Item first_items[], struct Item second_items[]"
    } else {
        "struct Item items[]"
    };
    let literal = "(struct Item[2]){{ .nested = { .values = {++marker} } }, {}}";
    let left_literal = "(struct Item[2]){{ .nested = { .values = {++left_marker} } }, {}}";
    let right_literal = "(struct Item[2]){{ .nested = { .values = {++right_marker} } }, {}}";
    let (arguments, marker_check) = if separate_roots {
        (
            format!("{left_literal}, {right_literal}"),
            "left_marker == 1 && right_marker == 1",
        )
    } else {
        (literal.to_string(), "marker == 1")
    };

    format!(
        "{prelude}\n\
         int marker = 0; int left_marker = 0; int right_marker = 0;\n\
         {pointer_type}forward_alias_{suffix}({pointer_type}value) {{ return value; }}\n\
         {pointer_type}forward_alias_{suffix}_twice({pointer_type}value) {{ return forward_alias_{suffix}(value); }}\n\
         {const_pointer_type}forward_const_alias_{suffix}({const_pointer_type}value) {{ return value; }}\n\
         {const_pointer_type}forward_const_alias_{suffix}_twice({const_pointer_type}value) {{ return forward_const_alias_{suffix}(value); }}\n\
         int mutate_alias_{suffix}({pointer_type}first, {pointer_type}second, {const_pointer_type}reader, {pointer_type}fallback) {{\n\
             {write_first};\n\
             int before = {read_reader};\n\
             {compound_second};\n\
             int after = {read_reader};\n\
             first = fallback; second = fallback; reader = fallback;\n\
             return before * 3 + after * 5 + (first == fallback) + (second == fallback) + (reader == fallback);\n\
         }}\n\
         int probe({parameters}) {{\n\
             {pointer_type}a = {first_forward};\n\
             {pointer_type}b = {second_forward};\n\
             {const_pointer_type}r = {reader_forward};\n\
             {initialize_first}; {initialize_second};\n\
             int observed = mutate_alias_{suffix}(a, b, r, a);\n\
             int caller_identity = (a == {first_address}) + (b == {second_address}) + (r == {reader_address});\n\
             return observed + {read_a} * 7 + {read_b} * 11 + {read_r} * 13 + caller_identity * 17;\n\
         }}\n\
         int main(void) {{\n\
             int result = probe({arguments});\n\
             return result + ({marker_check}) * 19;\n\
         }}\n",
        prelude = adjusted_parameter_prelude(),
        write_first = kind.write("first", replacement),
        read_reader = kind.read("reader"),
        compound_second = kind.compound_add("second", delta),
        initialize_first = kind.write("a", first.model_value()),
        initialize_second = kind.write("b", second.model_value()),
        read_a = kind.read("a"),
        read_b = kind.read("b"),
        read_r = kind.read("r"),
    )
}

fn direct_literal_adjusted_parameter_inner_bounds_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ return probe((struct Item[2]){{{{}}, {{}}}}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn direct_literal_adjusted_parameter_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[2].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe((struct Item[2]){{{{}}, {{}}}}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn direct_literal_adjusted_parameter_const_discard_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    format!(
        "{prelude}\n\
         typedef const struct Item ConstItems[2];\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe((ConstItems){{{{}}, {{}}}}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn direct_literal_adjusted_parameter_const_write_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let write = kind.write("slot", 7);
    format!(
        "{prelude}\n\
         typedef const struct Item ConstItems[2];\n\
         int probe(const struct Item items[]) {{ {const_pointer_type}slot = &items[0].nested.{field}[0]; {write}; return {read}; }}\n\
         int main(void) {{ return probe((ConstItems){{{{}}, {{}}}}); }}\n",
        prelude = adjusted_parameter_prelude(),
        const_pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn direct_literal_adjusted_parameter_cross_root_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item first[], struct Item second[]) {{\n\
             {pointer_type}a = &first[0].nested.{field}[0];\n\
             {pointer_type}b = &second[0].nested.{field}[0];\n\
             return a - b;\n\
         }}\n\
         int main(void) {{ return probe((struct Item[2]){{{{}}, {{}}}}, (struct Item[2]){{{{}}, {{}}}}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn direct_literal_adjusted_parameter_type_mismatch_program() -> String {
    format!(
        "{prelude}\n\
         struct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ return probe((struct Item[2]){{{{}}, {{}}}}); }}\n",
        prelude = adjusted_parameter_prelude(),
    )
}

impl WrappedDirectLiteralRoute {
    const ALL: [Self; 3] = [Self::ConditionalTrue, Self::ConditionalFalse, Self::Comma];

    fn index(self) -> usize {
        match self {
            Self::ConditionalTrue => 0,
            Self::ConditionalFalse => 1,
            Self::Comma => 2,
        }
    }

    fn render(self, type_name: &str, prefix: &str) -> String {
        let selected = format!(
            "({type_name}){{{{ .nested = {{ .values = {{++{prefix}_selected}} }} }}, {{}}}}"
        );
        let unselected = format!(
            "({type_name}){{{{ .nested = {{ .values = {{++{prefix}_unselected}} }} }}, {{}}}}"
        );
        match self {
            Self::ConditionalTrue => format!("(1 ? {selected} : {unselected})"),
            Self::ConditionalFalse => format!("(0 ? {unselected} : {selected})"),
            Self::Comma => format!("(++{prefix}_comma, {selected})"),
        }
    }

    fn marker_check(self, prefix: &str) -> String {
        let comma_expected = i64::from(self == Self::Comma);
        format!(
            "{prefix}_selected == 1 && {prefix}_unselected == 0 && {prefix}_comma == {comma_expected}"
        )
    }

    fn render_with_offset(
        self,
        type_name: &str,
        prefix: &str,
        offset: WrappedDirectLiteralOffsetRoute,
    ) -> String {
        offset.render(&wrapped_direct_literal_offset_base(self, type_name, prefix))
    }
}

fn wrapped_direct_literal_offset_base(
    wrapper: WrappedDirectLiteralRoute,
    type_name: &str,
    prefix: &str,
) -> String {
    let selected = format!(
        "({type_name}){{{{}}, {{ .nested = {{ .values = {{++{prefix}_selected}} }} }}, {{}}}}"
    );
    let unselected = format!(
        "({type_name}){{{{}}, {{ .nested = {{ .values = {{++{prefix}_unselected}} }} }}, {{}}}}"
    );
    match wrapper {
        WrappedDirectLiteralRoute::ConditionalTrue => format!("(1 ? {selected} : {unselected})"),
        WrappedDirectLiteralRoute::ConditionalFalse => format!("(0 ? {unselected} : {selected})"),
        WrappedDirectLiteralRoute::Comma => format!("(++{prefix}_comma, {selected})"),
    }
}

fn outer_forwarded_wrapped_direct_literal_argument(
    wrapper: WrappedDirectLiteralRoute,
    type_name: &str,
    prefix: &str,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_forward: CapturedOuterForwardRoute,
    points_to_const: bool,
) -> String {
    let wrapped = wrapped_direct_literal_offset_base(wrapper, type_name, prefix);
    outer_forward.render(&wrapped, offset, points_to_const)
}

impl WrappedDirectLiteralOffsetRoute {
    const ALL: [Self; 3] = [
        Self::PointerPlusOne,
        Self::OnePlusPointer,
        Self::IndexedAddress,
    ];

    fn index(self) -> usize {
        match self {
            Self::PointerPlusOne => 0,
            Self::OnePlusPointer => 1,
            Self::IndexedAddress => 2,
        }
    }

    fn render(self, wrapped: &str) -> String {
        match self {
            Self::PointerPlusOne => format!("({wrapped} + 1)"),
            Self::OnePlusPointer => format!("(1 + {wrapped})"),
            Self::IndexedAddress => format!("&({wrapped})[1]"),
        }
    }
}

impl CapturedOuterForwardRoute {
    const ALL: [Self; 4] = [
        Self::OneHopBeforeOffset,
        Self::TwoHopBeforeOffset,
        Self::OneHopAfterOffset,
        Self::TwoHopAfterOffset,
    ];

    fn index(self) -> usize {
        match self {
            Self::OneHopBeforeOffset => 0,
            Self::TwoHopBeforeOffset => 1,
            Self::OneHopAfterOffset => 2,
            Self::TwoHopAfterOffset => 3,
        }
    }

    fn render(
        self,
        wrapped: &str,
        offset: WrappedDirectLiteralOffsetRoute,
        points_to_const: bool,
    ) -> String {
        let helper = match (points_to_const, self) {
            (false, Self::OneHopBeforeOffset | Self::OneHopAfterOffset) => "forward_outer",
            (false, Self::TwoHopBeforeOffset | Self::TwoHopAfterOffset) => "forward_outer_twice",
            (true, Self::OneHopBeforeOffset | Self::OneHopAfterOffset) => "forward_const_outer",
            (true, Self::TwoHopBeforeOffset | Self::TwoHopAfterOffset) => {
                "forward_const_outer_twice"
            }
        };
        match self {
            Self::OneHopBeforeOffset | Self::TwoHopBeforeOffset => {
                offset.render(&format!("{helper}({wrapped})"))
            }
            Self::OneHopAfterOffset | Self::TwoHopAfterOffset => {
                format!("{helper}({})", offset.render(wrapped))
            }
        }
    }
}

impl PostForwardWrapperPlacement {
    const ALL: [Self; 2] = [Self::BeforeOffset, Self::AfterOffset];

    fn index(self) -> usize {
        match self {
            Self::BeforeOffset => 0,
            Self::AfterOffset => 1,
        }
    }
}

fn post_forward_pointer_wrapper(
    wrapper: WrappedDirectLiteralRoute,
    selected: &str,
    unselected: &str,
    prefix: &str,
) -> String {
    match wrapper {
        WrappedDirectLiteralRoute::ConditionalTrue => format!(
            "(1 ? (++{prefix}_selected, {selected}) : (++{prefix}_unselected, {unselected}))"
        ),
        WrappedDirectLiteralRoute::ConditionalFalse => format!(
            "(0 ? (++{prefix}_unselected, {unselected}) : (++{prefix}_selected, {selected}))"
        ),
        WrappedDirectLiteralRoute::Comma => {
            format!("(++{prefix}_comma, ++{prefix}_selected, {selected})")
        }
    }
}

fn post_forward_wrapped_direct_literal_argument(
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    type_name: &str,
    prefix: &str,
    offset: WrappedDirectLiteralOffsetRoute,
    placement: PostForwardWrapperPlacement,
    outer_two_hop: bool,
    points_to_const: bool,
) -> String {
    let inner_prefix = format!("{prefix}_inner");
    let post_prefix = format!("{prefix}_post");
    let wrapped = wrapped_direct_literal_offset_base(inner_wrapper, type_name, &inner_prefix);
    let helper = match (points_to_const, outer_two_hop) {
        (false, false) => "forward_outer",
        (false, true) => "forward_outer_twice",
        (true, false) => "forward_const_outer",
        (true, true) => "forward_const_outer_twice",
    };
    let forwarded = format!("{helper}({wrapped})");
    let unselected_literal = format!("({type_name})") + "{{}, {}, {}}";
    let unselected = format!("{helper}({unselected_literal})");

    match placement {
        PostForwardWrapperPlacement::BeforeOffset => offset.render(&post_forward_pointer_wrapper(
            post_wrapper,
            &forwarded,
            &unselected,
            &post_prefix,
        )),
        PostForwardWrapperPlacement::AfterOffset => post_forward_pointer_wrapper(
            post_wrapper,
            &offset.render(&forwarded),
            &offset.render(&unselected),
            &post_prefix,
        ),
    }
}

fn post_forward_marker_check(
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    prefix: &str,
) -> String {
    format!(
        "({}) && ({})",
        inner_wrapper.marker_check(&format!("{prefix}_inner")),
        post_wrapper.marker_check(&format!("{prefix}_post")),
    )
}

fn post_forward_marker_declarations() -> &'static str {
    "int root_inner_selected = 0; int root_inner_unselected = 0; int root_inner_comma = 0;\n\
     int root_post_selected = 0; int root_post_unselected = 0; int root_post_comma = 0;\n\
     int left_inner_selected = 0; int left_inner_unselected = 0; int left_inner_comma = 0;\n\
     int left_post_selected = 0; int left_post_unselected = 0; int left_post_comma = 0;\n\
     int right_inner_selected = 0; int right_inner_unselected = 0; int right_inner_comma = 0;\n\
     int right_post_selected = 0; int right_post_unselected = 0; int right_post_comma = 0;"
}

#[allow(clippy::too_many_arguments)]
fn post_forward_wrapped_direct_literal_adjusted_parameter_alias_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    inner_two_hop: [bool; 3],
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    placement: PostForwardWrapperPlacement,
    outer_two_hop: bool,
) -> String {
    let kind = first.kind;
    let separate_roots = second.storage == AdjustedParameterStorage::RootRight;
    let first_parameter = if separate_roots {
        "first_items"
    } else {
        "items"
    };
    let second_parameter = if separate_roots {
        "second_items"
    } else {
        "items"
    };
    let reader_parameter = if reader.storage == AdjustedParameterStorage::RootRight {
        second_parameter
    } else {
        first_parameter
    };
    let first_address = first.render_static_address(first_parameter);
    let second_address = second.render_static_address(second_parameter);
    let reader_address = reader.render_static_address(reader_parameter);
    let first_forward =
        adjusted_parameter_forward_expr(first, &first_address, inner_two_hop[0], false);
    let second_forward =
        adjusted_parameter_forward_expr(second, &second_address, inner_two_hop[1], false);
    let reader_forward =
        adjusted_parameter_forward_expr(reader, &reader_address, inner_two_hop[2], true);
    let pointer_type = kind.pointer_type();
    let const_pointer_type = kind.const_pointer_type();
    let suffix = kind.suffix();
    let parameters = if separate_roots {
        "struct Item first_items[], struct Item second_items[]"
    } else {
        "struct Item items[]"
    };
    let render_argument = |prefix: &str| {
        post_forward_wrapped_direct_literal_argument(
            inner_wrapper,
            post_wrapper,
            "struct Item[3]",
            prefix,
            offset,
            placement,
            outer_two_hop,
            false,
        )
    };
    let (arguments, marker_check) = if separate_roots {
        (
            format!("{}, {}", render_argument("left"), render_argument("right")),
            format!(
                "({}) && ({})",
                post_forward_marker_check(inner_wrapper, post_wrapper, "left"),
                post_forward_marker_check(inner_wrapper, post_wrapper, "right")
            ),
        )
    } else {
        (
            render_argument("root"),
            post_forward_marker_check(inner_wrapper, post_wrapper, "root"),
        )
    };

    format!(
        "{prelude}\n{outer_helpers}\n{marker_declarations}\n\
         {pointer_type}forward_alias_{suffix}({pointer_type}value) {{ return value; }}\n\
         {pointer_type}forward_alias_{suffix}_twice({pointer_type}value) {{ return forward_alias_{suffix}(value); }}\n\
         {const_pointer_type}forward_const_alias_{suffix}({const_pointer_type}value) {{ return value; }}\n\
         {const_pointer_type}forward_const_alias_{suffix}_twice({const_pointer_type}value) {{ return forward_const_alias_{suffix}(value); }}\n\
         int post_forward_mutate_{suffix}({pointer_type}first, {pointer_type}second, {const_pointer_type}reader, {pointer_type}fallback) {{\n\
             {write_first}; int before = {read_reader}; {compound_second}; int after = {read_reader};\n\
             first = fallback; second = fallback; reader = fallback;\n\
             return before * 3 + after * 5 + (first == fallback) + (second == fallback) + (reader == fallback);\n\
         }}\n\
         int probe({parameters}) {{\n\
             {pointer_type}a = {first_forward}; {pointer_type}b = {second_forward}; {const_pointer_type}r = {reader_forward};\n\
             {initialize_first}; {initialize_second};\n\
             int observed = post_forward_mutate_{suffix}(a, b, r, a);\n\
             int caller_identity = (a == {first_address}) + (b == {second_address}) + (r == {reader_address});\n\
             return observed + {read_a} * 7 + {read_b} * 11 + {read_r} * 13 + caller_identity * 17;\n\
         }}\n\
         int main(void) {{ int result = probe({arguments}); return result + ({marker_check}) * 19; }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        marker_declarations = post_forward_marker_declarations(),
        write_first = kind.write("first", replacement),
        read_reader = kind.read("reader"),
        compound_second = kind.compound_add("second", delta),
        initialize_first = kind.write("a", first.model_value()),
        initialize_second = kind.write("b", second.model_value()),
        read_a = kind.read("a"),
        read_b = kind.read("b"),
        read_r = kind.read("r"),
    )
}

fn post_forward_wrapped_direct_literal_const_parameter_program(
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    placement: PostForwardWrapperPlacement,
    outer_two_hop: bool,
) -> String {
    let argument = post_forward_wrapped_direct_literal_argument(
        inner_wrapper,
        post_wrapper,
        "ConstItems",
        "root",
        offset,
        placement,
        outer_two_hop,
        true,
    );
    let marker_check = post_forward_marker_check(inner_wrapper, post_wrapper, "root");
    format!(
        "{prelude}\n{outer_helpers}\ntypedef const struct Item ConstItems[3];\n\
         {marker_declarations}\n\
         int read_const(const struct Item items[]) {{ return items[0].nested.values[0] + 6; }}\n\
         int main(void) {{ return read_const({argument}) + ({marker_check}) * 10; }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        marker_declarations = post_forward_marker_declarations(),
    )
}

fn post_forward_wrapped_direct_literal_inner_bounds_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_forward_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        PostForwardWrapperPlacement::AfterOffset,
        true,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{markers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_forward_wrapped_direct_literal_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
    outer: i64,
) -> String {
    let argument = post_forward_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::ConditionalFalse,
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        PostForwardWrapperPlacement::BeforeOffset,
        false,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{markers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[{outer}].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_forward_wrapped_direct_literal_const_discard_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_forward_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralRoute::ConditionalTrue,
        "ConstItems",
        "root",
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        PostForwardWrapperPlacement::AfterOffset,
        false,
        true,
    );
    format!(
        "{prelude}\n{outer_helpers}\ntypedef const struct Item ConstItems[3];\n{markers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_forward_wrapped_direct_literal_const_write_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_forward_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::Comma,
        "ConstItems",
        "root",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        PostForwardWrapperPlacement::BeforeOffset,
        true,
        true,
    );
    format!(
        "{prelude}\n{outer_helpers}\ntypedef const struct Item ConstItems[3];\n{markers}\n\
         int probe(const struct Item items[]) {{ {const_pointer_type}slot = &items[0].nested.{field}[0]; {write}; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
        const_pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 7),
        read = kind.read("slot"),
    )
}

fn post_forward_wrapped_direct_literal_cross_root_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let first_argument = post_forward_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::ConditionalFalse,
        "struct Item[3]",
        "left",
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        PostForwardWrapperPlacement::BeforeOffset,
        false,
        false,
    );
    let second_argument = post_forward_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::Comma,
        "struct Item[3]",
        "right",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        PostForwardWrapperPlacement::AfterOffset,
        true,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{markers}\n\
         int probe(struct Item first[], struct Item second[]) {{ {pointer_type}a = &first[0].nested.{field}[0]; {pointer_type}b = &second[0].nested.{field}[0]; return a - b; }}\n\
         int main(void) {{ return probe({first_argument}, {second_argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn post_forward_wrapped_direct_literal_type_mismatch_program() -> String {
    let argument = post_forward_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralRoute::ConditionalTrue,
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        PostForwardWrapperPlacement::AfterOffset,
        true,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{markers}\nstruct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
    )
}

fn post_forward_wrapped_direct_literal_lifetime_program() -> String {
    let argument = post_forward_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        PostForwardWrapperPlacement::AfterOffset,
        true,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{markers}\n\
         struct Item *dangling(void) {{ return {argument}; }}\n\
         int main(void) {{ struct Item *result = dangling(); return result[-1].nested.values[0]; }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
    )
}

#[allow(clippy::too_many_arguments)]
fn post_wrapper_reforwarded_direct_literal_argument(
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    type_name: &str,
    prefix: &str,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_two_hop: bool,
    reforward_two_hop: bool,
    reforward_placement: PostForwardWrapperPlacement,
    points_to_const: bool,
) -> String {
    let inner_prefix = format!("{prefix}_inner");
    let post_prefix = format!("{prefix}_post");
    let inner = wrapped_direct_literal_offset_base(inner_wrapper, type_name, &inner_prefix);
    let outer_helper = match (points_to_const, outer_two_hop) {
        (false, false) => "forward_outer",
        (false, true) => "forward_outer_twice",
        (true, false) => "forward_const_outer",
        (true, true) => "forward_const_outer_twice",
    };
    let reforward_helper = match (points_to_const, reforward_two_hop) {
        (false, false) => "forward_outer",
        (false, true) => "forward_outer_twice",
        (true, false) => "forward_const_outer",
        (true, true) => "forward_const_outer_twice",
    };
    let forwarded = format!("{outer_helper}({inner})");
    let unselected_literal = format!("({type_name})") + "{{}, {}, {}}";
    let unselected = format!("{outer_helper}({unselected_literal})");
    let wrapped = post_forward_pointer_wrapper(post_wrapper, &forwarded, &unselected, &post_prefix);

    match reforward_placement {
        PostForwardWrapperPlacement::BeforeOffset => {
            offset.render(&format!("{reforward_helper}({wrapped})"))
        }
        PostForwardWrapperPlacement::AfterOffset => {
            format!("{reforward_helper}({})", offset.render(&wrapped))
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn mutable_to_const_reforwarded_direct_literal_argument(
    transition_before_post_wrapper: bool,
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    prefix: &str,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_two_hop: bool,
    reforward_two_hop: bool,
    reforward_placement: PostForwardWrapperPlacement,
) -> String {
    let inner_prefix = format!("{prefix}_inner");
    let post_prefix = format!("{prefix}_post");
    let inner = wrapped_direct_literal_offset_base(inner_wrapper, "struct Item[3]", &inner_prefix);
    let outer_helper = match (transition_before_post_wrapper, outer_two_hop) {
        (false, false) => "forward_outer",
        (false, true) => "forward_outer_twice",
        (true, false) => "forward_const_outer",
        (true, true) => "forward_const_outer_twice",
    };
    let reforward_helper = if reforward_two_hop {
        "forward_const_outer_twice"
    } else {
        "forward_const_outer"
    };
    let forwarded = format!("{outer_helper}({inner})");
    let unselected = format!("{outer_helper}((struct Item[3]){{{{}}, {{}}, {{}}}})");
    let wrapped = post_forward_pointer_wrapper(post_wrapper, &forwarded, &unselected, &post_prefix);

    match reforward_placement {
        PostForwardWrapperPlacement::BeforeOffset => {
            offset.render(&format!("{reforward_helper}({wrapped})"))
        }
        PostForwardWrapperPlacement::AfterOffset => {
            format!("{reforward_helper}({})", offset.render(&wrapped))
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn mutable_to_const_reforwarded_direct_literal_adjusted_parameter_program(
    transition_before_post_wrapper: bool,
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_two_hop: bool,
    reforward_two_hop: bool,
    reforward_placement: PostForwardWrapperPlacement,
) -> String {
    let argument = mutable_to_const_reforwarded_direct_literal_argument(
        transition_before_post_wrapper,
        inner_wrapper,
        post_wrapper,
        "root",
        offset,
        outer_two_hop,
        reforward_two_hop,
        reforward_placement,
    );
    let marker_check = post_forward_marker_check(inner_wrapper, post_wrapper, "root");
    format!(
        "{prelude}\n{helpers}\n{markers}\n\
         int read_promoted(const struct Item items[]) {{\n\
             const struct Item *original = items;\n\
             const int *scalar = &items[0].nested.values[0];\n\
             const struct Point *point = &items[0].nested.points[1];\n\
             int score = (items[-1].nested.values[0] == 0) + (*scalar == 1) + (point->value == 0)\n\
                 + (scalar == &items[0].nested.values[0]) + (point == &items[0].nested.points[1]) + (original == items);\n\
             const struct Item *fallback = items - 1;\n\
             items = fallback; scalar = &fallback[0].nested.values[0]; point = &fallback[0].nested.points[0];\n\
             return score + (items == fallback) + (scalar == &fallback[0].nested.values[0]) + (point == &fallback[0].nested.points[0]);\n\
         }}\n\
         int main(void) {{ return read_promoted({argument}) + ({marker_check}) * 19; }}\n",
        prelude = adjusted_parameter_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
    )
}

fn mutable_to_const_reforwarded_direct_literal_const_discard_program(
    transition_before_post_wrapper: bool,
) -> String {
    let argument = mutable_to_const_reforwarded_direct_literal_argument(
        transition_before_post_wrapper,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        "root",
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        false,
        true,
        PostForwardWrapperPlacement::BeforeOffset,
    );
    format!(
        "{prelude}\n{helpers}\n{markers}\n\
         int main(void) {{ struct Item *slot = {argument}; return slot[0].nested.values[0]; }}\n",
        prelude = adjusted_parameter_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
    )
}

fn mutable_to_const_reforwarded_direct_literal_const_write_program(
    transition_before_post_wrapper: bool,
) -> String {
    let argument = mutable_to_const_reforwarded_direct_literal_argument(
        transition_before_post_wrapper,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::ConditionalFalse,
        "root",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        true,
        false,
        PostForwardWrapperPlacement::AfterOffset,
    );
    format!(
        "{prelude}\n{helpers}\n{markers}\n\
         int probe(const struct Item items[]) {{ const int *slot = &items[0].nested.values[0]; *slot = 7; return *slot; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
    )
}

#[allow(clippy::too_many_arguments)]
fn post_wrapper_reforwarded_direct_literal_adjusted_parameter_alias_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    inner_two_hop: [bool; 3],
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_two_hop: bool,
    reforward_two_hop: bool,
    reforward_placement: PostForwardWrapperPlacement,
) -> String {
    let kind = first.kind;
    let separate_roots = second.storage == AdjustedParameterStorage::RootRight;
    let first_parameter = if separate_roots {
        "first_items"
    } else {
        "items"
    };
    let second_parameter = if separate_roots {
        "second_items"
    } else {
        "items"
    };
    let reader_parameter = if reader.storage == AdjustedParameterStorage::RootRight {
        second_parameter
    } else {
        first_parameter
    };
    let first_address = first.render_static_address(first_parameter);
    let second_address = second.render_static_address(second_parameter);
    let reader_address = reader.render_static_address(reader_parameter);
    let first_forward =
        adjusted_parameter_forward_expr(first, &first_address, inner_two_hop[0], false);
    let second_forward =
        adjusted_parameter_forward_expr(second, &second_address, inner_two_hop[1], false);
    let reader_forward =
        adjusted_parameter_forward_expr(reader, &reader_address, inner_two_hop[2], true);
    let pointer_type = kind.pointer_type();
    let const_pointer_type = kind.const_pointer_type();
    let suffix = kind.suffix();
    let parameters = if separate_roots {
        "struct Item first_items[], struct Item second_items[]"
    } else {
        "struct Item items[]"
    };
    let render_argument = |prefix: &str| {
        post_wrapper_reforwarded_direct_literal_argument(
            inner_wrapper,
            post_wrapper,
            "struct Item[3]",
            prefix,
            offset,
            outer_two_hop,
            reforward_two_hop,
            reforward_placement,
            false,
        )
    };
    let (arguments, marker_check) = if separate_roots {
        (
            format!("{}, {}", render_argument("left"), render_argument("right")),
            format!(
                "({}) && ({})",
                post_forward_marker_check(inner_wrapper, post_wrapper, "left"),
                post_forward_marker_check(inner_wrapper, post_wrapper, "right")
            ),
        )
    } else {
        (
            render_argument("root"),
            post_forward_marker_check(inner_wrapper, post_wrapper, "root"),
        )
    };

    format!(
        "{prelude}\n{outer_helpers}\n{marker_declarations}\n\
         {pointer_type}forward_alias_{suffix}({pointer_type}value) {{ return value; }}\n\
         {pointer_type}forward_alias_{suffix}_twice({pointer_type}value) {{ return forward_alias_{suffix}(value); }}\n\
         {const_pointer_type}forward_const_alias_{suffix}({const_pointer_type}value) {{ return value; }}\n\
         {const_pointer_type}forward_const_alias_{suffix}_twice({const_pointer_type}value) {{ return forward_const_alias_{suffix}(value); }}\n\
         int reforward_mutate_{suffix}({pointer_type}first, {pointer_type}second, {const_pointer_type}reader, {pointer_type}fallback) {{\n\
             {write_first}; int before = {read_reader}; {compound_second}; int after = {read_reader};\n\
             first = fallback; second = fallback; reader = fallback;\n\
             return before * 3 + after * 5 + (first == fallback) + (second == fallback) + (reader == fallback);\n\
         }}\n\
         int probe({parameters}) {{\n\
             {pointer_type}a = {first_forward}; {pointer_type}b = {second_forward}; {const_pointer_type}r = {reader_forward};\n\
             {initialize_first}; {initialize_second};\n\
             int observed = reforward_mutate_{suffix}(a, b, r, a);\n\
             int caller_identity = (a == {first_address}) + (b == {second_address}) + (r == {reader_address});\n\
             return observed + {read_a} * 7 + {read_b} * 11 + {read_r} * 13 + caller_identity * 17;\n\
         }}\n\
         int main(void) {{ int result = probe({arguments}); return result + ({marker_check}) * 19; }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        marker_declarations = post_forward_marker_declarations(),
        write_first = kind.write("first", replacement),
        read_reader = kind.read("reader"),
        compound_second = kind.compound_add("second", delta),
        initialize_first = kind.write("a", first.model_value()),
        initialize_second = kind.write("b", second.model_value()),
        read_a = kind.read("a"),
        read_b = kind.read("b"),
        read_r = kind.read("r"),
    )
}

fn post_wrapper_reforwarded_direct_literal_const_parameter_program(
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_two_hop: bool,
    reforward_two_hop: bool,
    reforward_placement: PostForwardWrapperPlacement,
) -> String {
    let argument = post_wrapper_reforwarded_direct_literal_argument(
        inner_wrapper,
        post_wrapper,
        "ConstItems",
        "root",
        offset,
        outer_two_hop,
        reforward_two_hop,
        reforward_placement,
        true,
    );
    let marker_check = post_forward_marker_check(inner_wrapper, post_wrapper, "root");
    format!(
        "{prelude}\n{outer_helpers}\ntypedef const struct Item ConstItems[3];\n\
         {marker_declarations}\n\
         int read_const(const struct Item items[]) {{ return items[0].nested.values[0] + 6; }}\n\
         int main(void) {{ return read_const({argument}) + ({marker_check}) * 10; }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        marker_declarations = post_forward_marker_declarations(),
    )
}

fn post_wrapper_reforwarded_direct_literal_inner_bounds_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_wrapper_reforwarded_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        true,
        false,
        PostForwardWrapperPlacement::BeforeOffset,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{markers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_wrapper_reforwarded_direct_literal_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
    outer: i64,
) -> String {
    let argument = post_wrapper_reforwarded_direct_literal_argument(
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::ConditionalFalse,
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        false,
        true,
        PostForwardWrapperPlacement::AfterOffset,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{markers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[{outer}].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_wrapper_reforwarded_direct_literal_const_discard_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_wrapper_reforwarded_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralRoute::ConditionalTrue,
        "ConstItems",
        "root",
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        false,
        true,
        PostForwardWrapperPlacement::AfterOffset,
        true,
    );
    format!(
        "{prelude}\n{outer_helpers}\ntypedef const struct Item ConstItems[3];\n{markers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_wrapper_reforwarded_direct_literal_const_write_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_wrapper_reforwarded_direct_literal_argument(
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::Comma,
        "ConstItems",
        "root",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        true,
        false,
        PostForwardWrapperPlacement::BeforeOffset,
        true,
    );
    format!(
        "{prelude}\n{outer_helpers}\ntypedef const struct Item ConstItems[3];\n{markers}\n\
         int probe(const struct Item items[]) {{ {const_pointer_type}slot = &items[0].nested.{field}[0]; {write}; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
        const_pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 7),
        read = kind.read("slot"),
    )
}

fn post_wrapper_reforwarded_direct_literal_cross_root_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let first_argument = post_wrapper_reforwarded_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        "struct Item[3]",
        "left",
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        false,
        true,
        PostForwardWrapperPlacement::AfterOffset,
        false,
    );
    let second_argument = post_wrapper_reforwarded_direct_literal_argument(
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::ConditionalFalse,
        "struct Item[3]",
        "right",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        true,
        false,
        PostForwardWrapperPlacement::BeforeOffset,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{markers}\n\
         int probe(struct Item first[], struct Item second[]) {{ {pointer_type}a = &first[0].nested.{field}[0]; {pointer_type}b = &second[0].nested.{field}[0]; return a - b; }}\n\
         int main(void) {{ return probe({first_argument}, {second_argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn post_wrapper_reforwarded_direct_literal_type_mismatch_program() -> String {
    let argument = post_wrapper_reforwarded_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralRoute::ConditionalTrue,
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        true,
        true,
        PostForwardWrapperPlacement::AfterOffset,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{markers}\nstruct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
    )
}

fn post_wrapper_reforwarded_direct_literal_lifetime_program() -> String {
    let argument = post_wrapper_reforwarded_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        true,
        true,
        PostForwardWrapperPlacement::AfterOffset,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{markers}\n\
         struct Item *dangling(void) {{ return {argument}; }}\n\
         int main(void) {{ struct Item *result = dangling(); return result[-1].nested.values[0]; }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_marker_declarations(),
    )
}

fn wrapped_direct_literal_adjusted_parameter_alias_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    two_hop: [bool; 3],
    route: WrappedDirectLiteralRoute,
) -> String {
    let kind = first.kind;
    let separate_roots = second.storage == AdjustedParameterStorage::RootRight;
    let first_parameter = if separate_roots {
        "first_items"
    } else {
        "items"
    };
    let second_parameter = if separate_roots {
        "second_items"
    } else {
        "items"
    };
    let reader_parameter = if reader.storage == AdjustedParameterStorage::RootRight {
        second_parameter
    } else {
        first_parameter
    };
    let first_address = first.render_static_address(first_parameter);
    let second_address = second.render_static_address(second_parameter);
    let reader_address = reader.render_static_address(reader_parameter);
    let first_forward = adjusted_parameter_forward_expr(first, &first_address, two_hop[0], false);
    let second_forward =
        adjusted_parameter_forward_expr(second, &second_address, two_hop[1], false);
    let reader_forward = adjusted_parameter_forward_expr(reader, &reader_address, two_hop[2], true);
    let pointer_type = kind.pointer_type();
    let const_pointer_type = kind.const_pointer_type();
    let suffix = kind.suffix();
    let parameters = if separate_roots {
        "struct Item first_items[], struct Item second_items[]"
    } else {
        "struct Item items[]"
    };
    let (arguments, marker_check) = if separate_roots {
        (
            format!(
                "{}, {}",
                route.render("struct Item[2]", "left"),
                route.render("struct Item[2]", "right")
            ),
            format!(
                "({}) && ({})",
                route.marker_check("left"),
                route.marker_check("right")
            ),
        )
    } else {
        (
            route.render("struct Item[2]", "root"),
            route.marker_check("root"),
        )
    };

    format!(
        "{prelude}\n\
         int root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int left_selected = 0; int left_unselected = 0; int left_comma = 0;\n\
         int right_selected = 0; int right_unselected = 0; int right_comma = 0;\n\
         {pointer_type}forward_alias_{suffix}({pointer_type}value) {{ return value; }}\n\
         {pointer_type}forward_alias_{suffix}_twice({pointer_type}value) {{ return forward_alias_{suffix}(value); }}\n\
         {const_pointer_type}forward_const_alias_{suffix}({const_pointer_type}value) {{ return value; }}\n\
         {const_pointer_type}forward_const_alias_{suffix}_twice({const_pointer_type}value) {{ return forward_const_alias_{suffix}(value); }}\n\
         int wrapped_mutate_{suffix}({pointer_type}first, {pointer_type}second, {const_pointer_type}reader, {pointer_type}fallback) {{\n\
             {write_first}; int before = {read_reader}; {compound_second}; int after = {read_reader};\n\
             first = fallback; second = fallback; reader = fallback;\n\
             return before * 3 + after * 5 + (first == fallback) + (second == fallback) + (reader == fallback);\n\
         }}\n\
         int probe({parameters}) {{\n\
             {pointer_type}a = {first_forward}; {pointer_type}b = {second_forward}; {const_pointer_type}r = {reader_forward};\n\
             {initialize_first}; {initialize_second};\n\
             int observed = wrapped_mutate_{suffix}(a, b, r, a);\n\
             int caller_identity = (a == {first_address}) + (b == {second_address}) + (r == {reader_address});\n\
             return observed + {read_a} * 7 + {read_b} * 11 + {read_r} * 13 + caller_identity * 17;\n\
         }}\n\
         int main(void) {{ int result = probe({arguments}); return result + ({marker_check}) * 19; }}\n",
        prelude = adjusted_parameter_prelude(),
        write_first = kind.write("first", replacement),
        read_reader = kind.read("reader"),
        compound_second = kind.compound_add("second", delta),
        initialize_first = kind.write("a", first.model_value()),
        initialize_second = kind.write("b", second.model_value()),
        read_a = kind.read("a"),
        read_b = kind.read("b"),
        read_r = kind.read("r"),
    )
}

fn wrapped_offset_direct_literal_adjusted_parameter_alias_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    two_hop: [bool; 3],
    wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
) -> String {
    wrapped_offset_direct_literal_adjusted_parameter_alias_program_with_outer_forwarding(
        first,
        second,
        reader,
        replacement,
        delta,
        two_hop,
        wrapper,
        offset,
        None,
    )
}

fn outer_forwarded_wrapped_offset_direct_literal_adjusted_parameter_alias_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    two_hop: [bool; 3],
    wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_forward: CapturedOuterForwardRoute,
) -> String {
    wrapped_offset_direct_literal_adjusted_parameter_alias_program_with_outer_forwarding(
        first,
        second,
        reader,
        replacement,
        delta,
        two_hop,
        wrapper,
        offset,
        Some(outer_forward),
    )
}

fn wrapped_offset_direct_literal_adjusted_parameter_alias_program_with_outer_forwarding(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    two_hop: [bool; 3],
    wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_forward: Option<CapturedOuterForwardRoute>,
) -> String {
    let kind = first.kind;
    let separate_roots = second.storage == AdjustedParameterStorage::RootRight;
    let first_parameter = if separate_roots {
        "first_items"
    } else {
        "items"
    };
    let second_parameter = if separate_roots {
        "second_items"
    } else {
        "items"
    };
    let reader_parameter = if reader.storage == AdjustedParameterStorage::RootRight {
        second_parameter
    } else {
        first_parameter
    };
    let first_address = first.render_static_address(first_parameter);
    let second_address = second.render_static_address(second_parameter);
    let reader_address = reader.render_static_address(reader_parameter);
    let first_forward = adjusted_parameter_forward_expr(first, &first_address, two_hop[0], false);
    let second_forward =
        adjusted_parameter_forward_expr(second, &second_address, two_hop[1], false);
    let reader_forward = adjusted_parameter_forward_expr(reader, &reader_address, two_hop[2], true);
    let pointer_type = kind.pointer_type();
    let const_pointer_type = kind.const_pointer_type();
    let suffix = kind.suffix();
    let parameters = if separate_roots {
        "struct Item first_items[], struct Item second_items[]"
    } else {
        "struct Item items[]"
    };
    let render_argument = |prefix: &str| match outer_forward {
        Some(route) => outer_forwarded_wrapped_direct_literal_argument(
            wrapper,
            "struct Item[3]",
            prefix,
            offset,
            route,
            false,
        ),
        None => wrapper.render_with_offset("struct Item[3]", prefix, offset),
    };
    let (arguments, marker_check) = if separate_roots {
        (
            format!("{}, {}", render_argument("left"), render_argument("right")),
            format!(
                "({}) && ({})",
                wrapper.marker_check("left"),
                wrapper.marker_check("right")
            ),
        )
    } else {
        (render_argument("root"), wrapper.marker_check("root"))
    };
    let outer_helpers = if outer_forward.is_some() {
        captured_outer_forwarding_helpers()
    } else {
        ""
    };

    format!(
        "{prelude}\n\
         {outer_helpers}\n\
         int root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int left_selected = 0; int left_unselected = 0; int left_comma = 0;\n\
         int right_selected = 0; int right_unselected = 0; int right_comma = 0;\n\
         {pointer_type}forward_alias_{suffix}({pointer_type}value) {{ return value; }}\n\
         {pointer_type}forward_alias_{suffix}_twice({pointer_type}value) {{ return forward_alias_{suffix}(value); }}\n\
         {const_pointer_type}forward_const_alias_{suffix}({const_pointer_type}value) {{ return value; }}\n\
         {const_pointer_type}forward_const_alias_{suffix}_twice({const_pointer_type}value) {{ return forward_const_alias_{suffix}(value); }}\n\
         int wrapped_offset_mutate_{suffix}({pointer_type}first, {pointer_type}second, {const_pointer_type}reader, {pointer_type}fallback) {{\n\
             {write_first}; int before = {read_reader}; {compound_second}; int after = {read_reader};\n\
             first = fallback; second = fallback; reader = fallback;\n\
             return before * 3 + after * 5 + (first == fallback) + (second == fallback) + (reader == fallback);\n\
         }}\n\
         int probe({parameters}) {{\n\
             {pointer_type}a = {first_forward}; {pointer_type}b = {second_forward}; {const_pointer_type}r = {reader_forward};\n\
             {initialize_first}; {initialize_second};\n\
             int observed = wrapped_offset_mutate_{suffix}(a, b, r, a);\n\
             int caller_identity = (a == {first_address}) + (b == {second_address}) + (r == {reader_address});\n\
             return observed + {read_a} * 7 + {read_b} * 11 + {read_r} * 13 + caller_identity * 17;\n\
         }}\n\
         int main(void) {{ int result = probe({arguments}); return result + ({marker_check}) * 19; }}\n",
        prelude = adjusted_parameter_prelude(),
        write_first = kind.write("first", replacement),
        read_reader = kind.read("reader"),
        compound_second = kind.compound_add("second", delta),
        initialize_first = kind.write("a", first.model_value()),
        initialize_second = kind.write("b", second.model_value()),
        read_a = kind.read("a"),
        read_b = kind.read("b"),
        read_r = kind.read("r"),
    )
}

fn wrapped_offset_direct_literal_const_parameter_program(
    wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
) -> String {
    let argument = wrapper.render_with_offset("ConstItems", "root", offset);
    let marker_check = wrapper.marker_check("root");
    format!(
        "{prelude}\n\
         typedef const struct Item ConstItems[3];\n\
         int root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int read_const(const struct Item items[]) {{ return items[0].nested.values[0] + 6; }}\n\
         int main(void) {{ return read_const({argument}) + ({marker_check}) * 10; }}\n",
        prelude = adjusted_parameter_prelude(),
    )
}

fn wrapped_offset_direct_literal_inner_bounds_program(kind: AdjustedParameterFieldKind) -> String {
    let argument = WrappedDirectLiteralRoute::ConditionalTrue.render_with_offset(
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
    );
    format!(
        "{prelude}\nint root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn wrapped_offset_direct_literal_upper_bounds_program(kind: AdjustedParameterFieldKind) -> String {
    wrapped_offset_direct_literal_outer_bounds_program(kind, 2)
}

fn wrapped_offset_direct_literal_lower_bounds_program(kind: AdjustedParameterFieldKind) -> String {
    wrapped_offset_direct_literal_outer_bounds_program(kind, -2)
}

fn wrapped_offset_direct_literal_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
    outer: i64,
) -> String {
    let argument = WrappedDirectLiteralRoute::Comma.render_with_offset(
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
    );
    format!(
        "{prelude}\nint root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[{outer}].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn wrapped_offset_direct_literal_const_discard_program(kind: AdjustedParameterFieldKind) -> String {
    let argument = WrappedDirectLiteralRoute::ConditionalFalse.render_with_offset(
        "ConstItems",
        "root",
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
    );
    format!(
        "{prelude}\ntypedef const struct Item ConstItems[3];\n\
         int root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn wrapped_offset_direct_literal_const_write_program(kind: AdjustedParameterFieldKind) -> String {
    let argument = WrappedDirectLiteralRoute::Comma.render_with_offset(
        "ConstItems",
        "root",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
    );
    format!(
        "{prelude}\ntypedef const struct Item ConstItems[3];\n\
         int root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(const struct Item items[]) {{ {const_pointer_type}slot = &items[0].nested.{field}[0]; {write}; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        const_pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 7),
        read = kind.read("slot"),
    )
}

fn wrapped_offset_direct_literal_cross_root_program(kind: AdjustedParameterFieldKind) -> String {
    let first_argument = WrappedDirectLiteralRoute::ConditionalTrue.render_with_offset(
        "struct Item[3]",
        "left",
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
    );
    let second_argument = WrappedDirectLiteralRoute::Comma.render_with_offset(
        "struct Item[3]",
        "right",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
    );
    format!(
        "{prelude}\nint left_selected = 0; int left_unselected = 0; int left_comma = 0;\n\
         int right_selected = 0; int right_unselected = 0; int right_comma = 0;\n\
         int probe(struct Item first[], struct Item second[]) {{\n\
             {pointer_type}a = &first[0].nested.{field}[0]; {pointer_type}b = &second[0].nested.{field}[0]; return a - b;\n\
         }}\nint main(void) {{ return probe({first_argument}, {second_argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn wrapped_offset_direct_literal_type_mismatch_program() -> String {
    let argument = WrappedDirectLiteralRoute::ConditionalFalse.render_with_offset(
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
    );
    format!(
        "{prelude}\nint root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         struct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
    )
}

fn outer_forwarded_wrapped_direct_literal_const_parameter_program(
    wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_forward: CapturedOuterForwardRoute,
) -> String {
    let argument = outer_forwarded_wrapped_direct_literal_argument(
        wrapper,
        "ConstItems",
        "root",
        offset,
        outer_forward,
        true,
    );
    let marker_check = wrapper.marker_check("root");
    format!(
        "{prelude}\n{outer_helpers}\n\
         typedef const struct Item ConstItems[3];\n\
         int root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int read_const(const struct Item items[]) {{ return items[0].nested.values[0] + 6; }}\n\
         int main(void) {{ return read_const({argument}) + ({marker_check}) * 10; }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
    )
}

fn outer_forwarded_wrapped_direct_literal_inner_bounds_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = outer_forwarded_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalTrue,
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        CapturedOuterForwardRoute::TwoHopAfterOffset,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\nint root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn outer_forwarded_wrapped_direct_literal_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
    outer: i64,
) -> String {
    let argument = outer_forwarded_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::Comma,
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        CapturedOuterForwardRoute::TwoHopBeforeOffset,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\nint root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[{outer}].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn outer_forwarded_wrapped_direct_literal_const_discard_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = outer_forwarded_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalFalse,
        "ConstItems",
        "root",
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        CapturedOuterForwardRoute::OneHopBeforeOffset,
        true,
    );
    format!(
        "{prelude}\n{outer_helpers}\ntypedef const struct Item ConstItems[3];\n\
         int root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn outer_forwarded_wrapped_direct_literal_const_write_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = outer_forwarded_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::Comma,
        "ConstItems",
        "root",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        CapturedOuterForwardRoute::TwoHopAfterOffset,
        true,
    );
    format!(
        "{prelude}\n{outer_helpers}\ntypedef const struct Item ConstItems[3];\n\
         int root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(const struct Item items[]) {{ {const_pointer_type}slot = &items[0].nested.{field}[0]; {write}; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        const_pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 7),
        read = kind.read("slot"),
    )
}

fn outer_forwarded_wrapped_direct_literal_cross_root_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let first_argument = outer_forwarded_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalTrue,
        "struct Item[3]",
        "left",
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        CapturedOuterForwardRoute::OneHopBeforeOffset,
        false,
    );
    let second_argument = outer_forwarded_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::Comma,
        "struct Item[3]",
        "right",
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        CapturedOuterForwardRoute::TwoHopAfterOffset,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\nint left_selected = 0; int left_unselected = 0; int left_comma = 0;\n\
         int right_selected = 0; int right_unselected = 0; int right_comma = 0;\n\
         int probe(struct Item first[], struct Item second[]) {{ {pointer_type}a = &first[0].nested.{field}[0]; {pointer_type}b = &second[0].nested.{field}[0]; return a - b; }}\n\
         int main(void) {{ return probe({first_argument}, {second_argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn outer_forwarded_wrapped_direct_literal_type_mismatch_program() -> String {
    let argument = outer_forwarded_wrapped_direct_literal_argument(
        WrappedDirectLiteralRoute::ConditionalFalse,
        "struct Item[3]",
        "root",
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        CapturedOuterForwardRoute::TwoHopBeforeOffset,
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\nint root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         struct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
    )
}

fn outer_forwarded_wrapped_direct_literal_lifetime_program() -> String {
    format!(
        "{prelude}\n{outer_helpers}\n\
         struct Item *dangling(void) {{ return forward_outer_twice((struct Item[3]){{{{}}, {{}}, {{}}}} + 1); }}\n\
         int main(void) {{ struct Item *result = dangling(); return result[-1].nested.values[0]; }}\n",
        prelude = adjusted_parameter_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
    )
}

fn captured_literal_field_offset_prelude() -> &'static str {
    "struct Point { int value; };\n\
     struct Inner { int values[3]; struct Point points[3]; };\n\
     struct Item { struct Inner nested; };\n\
     struct Wrapper { struct Item primary[3]; struct Item secondary[3]; };\n\
     struct NamedHolder { struct Wrapper nested; };\n\
     struct AnonymousHolder { struct { struct Item primary[3]; struct Item secondary[3]; } nested; };\n\
     union Choice { struct Wrapper nested; int marker; };"
}

fn captured_literal_field_offset_declarations() -> &'static str {
    "int marker = 0;\n\
     struct NamedHolder *named_left = &(struct NamedHolder){ .nested = { .primary = {{}, { .nested = { .values = {++marker} } }, {} } } };\n\
     struct NamedHolder *named_right = &(struct NamedHolder){ .nested = { .primary = {{}, { .nested = { .values = {++marker} } }, {} } } };\n\
     struct AnonymousHolder *anonymous_left = &(struct AnonymousHolder){ .nested = { .primary = {{}, { .nested = { .values = {++marker} } }, {} } } };\
     struct AnonymousHolder *anonymous_right = &(struct AnonymousHolder){ .nested = { .primary = {{}, { .nested = { .values = {++marker} } }, {} } } };\
     union Choice *choice_left = &(union Choice){ .nested = { .primary = {{}, { .nested = { .values = {++marker} } }, {} } } };\
     union Choice *choice_right = &(union Choice){ .nested = { .primary = {{}, { .nested = { .values = {++marker} } }, {} } } };"
}

fn captured_literal_field_offset_argument(
    storage: AdjustedParameterStorage,
    offset: WrappedDirectLiteralOffsetRoute,
) -> String {
    offset.render(storage.literal_expression())
}

fn captured_literal_field_offset_alias_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    two_hop: [bool; 3],
    offset: WrappedDirectLiteralOffsetRoute,
) -> String {
    let kind = first.kind;
    let first_address = first.render_static_address("first_items");
    let second_address = second.render_static_address("second_items");
    let reader_address = reader.render_static_address("reader_items");
    let first_forward = adjusted_parameter_forward_expr(first, &first_address, two_hop[0], false);
    let second_forward =
        adjusted_parameter_forward_expr(second, &second_address, two_hop[1], false);
    let reader_forward = adjusted_parameter_forward_expr(reader, &reader_address, two_hop[2], true);
    let pointer_type = kind.pointer_type();
    let const_pointer_type = kind.const_pointer_type();
    let suffix = kind.suffix();

    format!(
        "{prelude}\n\
         {pointer_type}forward_alias_{suffix}({pointer_type}value) {{ return value; }}\n\
         {pointer_type}forward_alias_{suffix}_twice({pointer_type}value) {{ return forward_alias_{suffix}(value); }}\n\
         {const_pointer_type}forward_const_alias_{suffix}({const_pointer_type}value) {{ return value; }}\n\
         {const_pointer_type}forward_const_alias_{suffix}_twice({const_pointer_type}value) {{ return forward_const_alias_{suffix}(value); }}\n\
         int mutate_alias_{suffix}({pointer_type}first, {pointer_type}second, {const_pointer_type}reader, {pointer_type}fallback) {{\n\
             {write_first}; int before = {read_reader}; {compound_second}; int after = {read_reader};\n\
             first = fallback; second = fallback; reader = fallback;\n\
             return before * 3 + after * 5 + (first == fallback) + (second == fallback) + (reader == fallback);\n\
         }}\n\
         int probe(struct Item first_items[], struct Item second_items[], struct Item reader_items[]) {{\n\
             {pointer_type}a = {first_forward}; {pointer_type}b = {second_forward}; {const_pointer_type}r = {reader_forward};\n\
             {initialize_first}; {initialize_second};\n\
             int observed = mutate_alias_{suffix}(a, b, r, a);\n\
             int caller_identity = (a == {first_address}) + (b == {second_address}) + (r == {reader_address});\n\
             return observed + {read_a} * 7 + {read_b} * 11 + {read_r} * 13 + caller_identity * 17;\n\
         }}\n\
         int main(void) {{\n\
             {declarations}\n\
             int result = probe({first_storage}, {second_storage}, {reader_storage});\n\
             return result + (marker == 6) * 19;\n\
         }}\n",
        prelude = captured_literal_field_offset_prelude(),
        declarations = captured_literal_field_offset_declarations(),
        write_first = kind.write("first", replacement),
        read_reader = kind.read("reader"),
        compound_second = kind.compound_add("second", delta),
        initialize_first = kind.write("a", first.model_value()),
        initialize_second = kind.write("b", second.model_value()),
        read_a = kind.read("a"),
        read_b = kind.read("b"),
        read_r = kind.read("r"),
        first_storage = captured_literal_field_offset_argument(first.storage, offset),
        second_storage = captured_literal_field_offset_argument(second.storage, offset),
        reader_storage = captured_literal_field_offset_argument(reader.storage, offset),
    )
}

fn wrapped_captured_literal_field_offset_argument(
    storage: AdjustedParameterStorage,
    wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    prefix: &str,
) -> String {
    wrapped_captured_literal_field_offset_expression(
        storage.literal_expression(),
        storage.other_owner().literal_expression(),
        wrapper,
        offset,
        prefix,
    )
}

fn wrapped_captured_literal_field_offset_argument_with_outer_forwarding(
    storage: AdjustedParameterStorage,
    wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    prefix: &str,
    outer_forward: Option<CapturedOuterForwardRoute>,
    points_to_const: bool,
) -> String {
    let selected = storage.literal_expression();
    let unselected = storage.other_owner().literal_expression();
    match outer_forward {
        Some(route) => {
            let wrapped = match wrapper {
                WrappedDirectLiteralRoute::ConditionalTrue => format!(
                    "(1 ? (++{prefix}_selected, {selected}) : (++{prefix}_unselected, {unselected}))"
                ),
                WrappedDirectLiteralRoute::ConditionalFalse => format!(
                    "(0 ? (++{prefix}_unselected, {unselected}) : (++{prefix}_selected, {selected}))"
                ),
                WrappedDirectLiteralRoute::Comma => {
                    format!("(++{prefix}_comma, ++{prefix}_selected, {selected})")
                }
            };
            route.render(&wrapped, offset, points_to_const)
        }
        None => wrapped_captured_literal_field_offset_expression(
            selected, unselected, wrapper, offset, prefix,
        ),
    }
}

fn wrapped_captured_literal_field_offset_expression(
    selected: &str,
    unselected: &str,
    wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    prefix: &str,
) -> String {
    let wrapped = match wrapper {
        WrappedDirectLiteralRoute::ConditionalTrue => format!(
            "(1 ? (++{prefix}_selected, {selected}) : (++{prefix}_unselected, {unselected}))"
        ),
        WrappedDirectLiteralRoute::ConditionalFalse => format!(
            "(0 ? (++{prefix}_unselected, {unselected}) : (++{prefix}_selected, {selected}))"
        ),
        WrappedDirectLiteralRoute::Comma => {
            format!("(++{prefix}_comma, ++{prefix}_selected, {selected})")
        }
    };
    offset.render(&wrapped)
}

fn wrapped_captured_literal_field_offset_alias_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    two_hop: [bool; 3],
    wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_forward: Option<CapturedOuterForwardRoute>,
) -> String {
    let kind = first.kind;
    let first_address = first.render_static_address("first_items");
    let second_address = second.render_static_address("second_items");
    let reader_address = reader.render_static_address("reader_items");
    let first_forward = adjusted_parameter_forward_expr(first, &first_address, two_hop[0], false);
    let second_forward =
        adjusted_parameter_forward_expr(second, &second_address, two_hop[1], false);
    let reader_forward = adjusted_parameter_forward_expr(reader, &reader_address, two_hop[2], true);
    let pointer_type = kind.pointer_type();
    let const_pointer_type = kind.const_pointer_type();
    let suffix = kind.suffix();
    let marker_check = ["first", "second", "reader"]
        .into_iter()
        .map(|prefix| wrapper.marker_check(prefix))
        .collect::<Vec<_>>()
        .join(" && ");
    let outer_helpers = if outer_forward.is_some() {
        "struct Item *forward_outer(struct Item *value) { return value; }\n\
         struct Item *forward_outer_twice(struct Item *value) { return forward_outer(value); }\n\
         const struct Item *forward_const_outer(const struct Item *value) { return value; }\n\
         const struct Item *forward_const_outer_twice(const struct Item *value) { return forward_const_outer(value); }"
    } else {
        ""
    };
    let reader_parameter = if outer_forward.is_some() {
        "const struct Item reader_items[]"
    } else {
        "struct Item reader_items[]"
    };

    format!(
        "{prelude}\n\
         {outer_helpers}\n\
         {pointer_type}forward_alias_{suffix}({pointer_type}value) {{ return value; }}\n\
         {pointer_type}forward_alias_{suffix}_twice({pointer_type}value) {{ return forward_alias_{suffix}(value); }}\n\
         {const_pointer_type}forward_const_alias_{suffix}({const_pointer_type}value) {{ return value; }}\n\
         {const_pointer_type}forward_const_alias_{suffix}_twice({const_pointer_type}value) {{ return forward_const_alias_{suffix}(value); }}\n\
         int mutate_alias_{suffix}({pointer_type}first, {pointer_type}second, {const_pointer_type}reader, {pointer_type}fallback) {{\n\
             {write_first}; int before = {read_reader}; {compound_second}; int after = {read_reader};\n\
             first = fallback; second = fallback; reader = fallback;\n\
             return before * 3 + after * 5 + (first == fallback) + (second == fallback) + (reader == fallback);\n\
         }}\n\
         int probe(struct Item first_items[], struct Item second_items[], {reader_parameter}) {{\n\
             {pointer_type}a = {first_forward}; {pointer_type}b = {second_forward}; {const_pointer_type}r = {reader_forward};\n\
             {initialize_first}; {initialize_second};\n\
             int observed = mutate_alias_{suffix}(a, b, r, a);\n\
             int caller_identity = (a == {first_address}) + (b == {second_address}) + (r == {reader_address});\n\
             return observed + {read_a} * 7 + {read_b} * 11 + {read_r} * 13 + caller_identity * 17;\n\
         }}\n\
         int main(void) {{\n\
             {declarations}\n\
             int first_selected = 0; int first_unselected = 0; int first_comma = 0;\n\
             int second_selected = 0; int second_unselected = 0; int second_comma = 0;\n\
             int reader_selected = 0; int reader_unselected = 0; int reader_comma = 0;\n\
             int result = probe({first_storage}, {second_storage}, {reader_storage});\n\
             return result + (marker == 6) * 19 + ({marker_check}) * 23;\n\
         }}\n",
        prelude = captured_literal_field_offset_prelude(),
        declarations = captured_literal_field_offset_declarations(),
        write_first = kind.write("first", replacement),
        read_reader = kind.read("reader"),
        compound_second = kind.compound_add("second", delta),
        initialize_first = kind.write("a", first.model_value()),
        initialize_second = kind.write("b", second.model_value()),
        read_a = kind.read("a"),
        read_b = kind.read("b"),
        read_r = kind.read("r"),
        first_storage = wrapped_captured_literal_field_offset_argument_with_outer_forwarding(
            first.storage,
            wrapper,
            offset,
            "first",
            outer_forward,
            false,
        ),
        second_storage = wrapped_captured_literal_field_offset_argument_with_outer_forwarding(
            second.storage,
            wrapper,
            offset,
            "second",
            outer_forward,
            false,
        ),
        reader_storage = wrapped_captured_literal_field_offset_argument_with_outer_forwarding(
            reader.storage,
            wrapper,
            offset,
            "reader",
            outer_forward,
            true,
        ),
    )
}

#[allow(clippy::too_many_arguments)]
fn post_forward_wrapped_captured_literal_field_offset_expression(
    selected: &str,
    unselected: &str,
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    prefix: &str,
    placement: PostForwardWrapperPlacement,
    outer_two_hop: bool,
    points_to_const: bool,
) -> String {
    let inner_prefix = format!("{prefix}_inner");
    let post_prefix = format!("{prefix}_post");
    let inner = post_forward_pointer_wrapper(inner_wrapper, selected, unselected, &inner_prefix);
    let helper = match (points_to_const, outer_two_hop) {
        (false, false) => "forward_outer",
        (false, true) => "forward_outer_twice",
        (true, false) => "forward_const_outer",
        (true, true) => "forward_const_outer_twice",
    };
    let forwarded = format!("{helper}({inner})");
    let post_unselected = format!("{helper}({unselected})");

    match placement {
        PostForwardWrapperPlacement::BeforeOffset => offset.render(&post_forward_pointer_wrapper(
            post_wrapper,
            &forwarded,
            &post_unselected,
            &post_prefix,
        )),
        PostForwardWrapperPlacement::AfterOffset => post_forward_pointer_wrapper(
            post_wrapper,
            &offset.render(&forwarded),
            &offset.render(&post_unselected),
            &post_prefix,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn post_forward_wrapped_captured_literal_field_offset_argument(
    storage: AdjustedParameterStorage,
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    prefix: &str,
    placement: PostForwardWrapperPlacement,
    outer_two_hop: bool,
    points_to_const: bool,
) -> String {
    post_forward_wrapped_captured_literal_field_offset_expression(
        storage.literal_expression(),
        storage.other_owner().literal_expression(),
        inner_wrapper,
        post_wrapper,
        offset,
        prefix,
        placement,
        outer_two_hop,
        points_to_const,
    )
}

fn post_forward_captured_marker_declarations(prefix: &str) -> String {
    format!(
        "int {prefix}_inner_selected = 0; int {prefix}_inner_unselected = 0; int {prefix}_inner_comma = 0;\n\
         int {prefix}_post_selected = 0; int {prefix}_post_unselected = 0; int {prefix}_post_comma = 0;"
    )
}

#[allow(clippy::too_many_arguments)]
fn post_forward_wrapped_captured_literal_field_offset_alias_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    inner_two_hop: [bool; 3],
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    placement: PostForwardWrapperPlacement,
    outer_two_hop: bool,
) -> String {
    let kind = first.kind;
    let first_address = first.render_static_address("first_items");
    let second_address = second.render_static_address("second_items");
    let reader_address = reader.render_static_address("reader_items");
    let first_forward =
        adjusted_parameter_forward_expr(first, &first_address, inner_two_hop[0], false);
    let second_forward =
        adjusted_parameter_forward_expr(second, &second_address, inner_two_hop[1], false);
    let reader_forward =
        adjusted_parameter_forward_expr(reader, &reader_address, inner_two_hop[2], true);
    let pointer_type = kind.pointer_type();
    let const_pointer_type = kind.const_pointer_type();
    let suffix = kind.suffix();
    let marker_check = ["first", "second", "reader"]
        .into_iter()
        .map(|prefix| post_forward_marker_check(inner_wrapper, post_wrapper, prefix))
        .collect::<Vec<_>>()
        .join(" && ");
    let marker_declarations = ["first", "second", "reader"]
        .into_iter()
        .map(post_forward_captured_marker_declarations)
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "{prelude}\n{outer_helpers}\n\
         {pointer_type}forward_alias_{suffix}({pointer_type}value) {{ return value; }}\n\
         {pointer_type}forward_alias_{suffix}_twice({pointer_type}value) {{ return forward_alias_{suffix}(value); }}\n\
         {const_pointer_type}forward_const_alias_{suffix}({const_pointer_type}value) {{ return value; }}\n\
         {const_pointer_type}forward_const_alias_{suffix}_twice({const_pointer_type}value) {{ return forward_const_alias_{suffix}(value); }}\n\
         int mutate_alias_{suffix}({pointer_type}first, {pointer_type}second, {const_pointer_type}reader, {pointer_type}fallback) {{\n\
             {write_first}; int before = {read_reader}; {compound_second}; int after = {read_reader};\n\
             first = fallback; second = fallback; reader = fallback;\n\
             return before * 3 + after * 5 + (first == fallback) + (second == fallback) + (reader == fallback);\n\
         }}\n\
         int probe(struct Item first_items[], struct Item second_items[], const struct Item reader_items[]) {{\n\
             {pointer_type}a = {first_forward}; {pointer_type}b = {second_forward}; {const_pointer_type}r = {reader_forward};\n\
             {initialize_first}; {initialize_second};\n\
             int observed = mutate_alias_{suffix}(a, b, r, a);\n\
             int caller_identity = (a == {first_address}) + (b == {second_address}) + (r == {reader_address});\n\
             return observed + {read_a} * 7 + {read_b} * 11 + {read_r} * 13 + caller_identity * 17;\n\
         }}\n\
         int main(void) {{\n\
             {declarations}\n{marker_declarations}\n\
             int result = probe({first_storage}, {second_storage}, {reader_storage});\n\
             return result + (marker == 6) * 19 + ({marker_check}) * 23;\n\
         }}\n",
        prelude = captured_literal_field_offset_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        write_first = kind.write("first", replacement),
        read_reader = kind.read("reader"),
        compound_second = kind.compound_add("second", delta),
        initialize_first = kind.write("a", first.model_value()),
        initialize_second = kind.write("b", second.model_value()),
        read_a = kind.read("a"),
        read_b = kind.read("b"),
        read_r = kind.read("r"),
        first_storage = post_forward_wrapped_captured_literal_field_offset_argument(
            first.storage,
            inner_wrapper,
            post_wrapper,
            offset,
            "first",
            placement,
            outer_two_hop,
            false,
        ),
        second_storage = post_forward_wrapped_captured_literal_field_offset_argument(
            second.storage,
            inner_wrapper,
            post_wrapper,
            offset,
            "second",
            placement,
            outer_two_hop,
            false,
        ),
        reader_storage = post_forward_wrapped_captured_literal_field_offset_argument(
            reader.storage,
            inner_wrapper,
            post_wrapper,
            offset,
            "reader",
            placement,
            outer_two_hop,
            true,
        ),
    )
}

fn post_forward_wrapped_captured_field_offset_inner_bounds_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_forward_wrapped_captured_literal_field_offset_argument(
        AdjustedParameterStorage::NamedLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        "root",
        PostForwardWrapperPlacement::AfterOffset,
        true,
        false,
    );
    format!(
        "{prelude}\n{helpers}\n{declarations}\n{markers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        markers = post_forward_captured_marker_declarations("root"),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_forward_wrapped_captured_field_offset_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
    outer: i64,
) -> String {
    let argument = post_forward_wrapped_captured_literal_field_offset_argument(
        AdjustedParameterStorage::AnonymousLeftPrimary,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "root",
        PostForwardWrapperPlacement::BeforeOffset,
        false,
        false,
    );
    format!(
        "{prelude}\n{helpers}\n{declarations}\n{markers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[{outer}].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        markers = post_forward_captured_marker_declarations("root"),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_forward_wrapped_captured_field_offset_const_discard_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_forward_wrapped_captured_literal_field_offset_expression(
        "locked->nested.primary",
        "locked->nested.secondary",
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        "root",
        PostForwardWrapperPlacement::AfterOffset,
        false,
        true,
    );
    format!(
        "{prelude}\n{helpers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ const struct NamedHolder *locked = &(const struct NamedHolder){{}}; {markers} return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_captured_marker_declarations("root"),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_forward_wrapped_captured_field_offset_const_write_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_forward_wrapped_captured_literal_field_offset_expression(
        "locked->nested.primary",
        "locked->nested.secondary",
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "root",
        PostForwardWrapperPlacement::BeforeOffset,
        true,
        true,
    );
    format!(
        "{prelude}\n{helpers}\n\
         int probe(const struct Item items[]) {{ {const_pointer_type}slot = &items[0].nested.{field}[0]; {write}; return {read}; }}\n\
         int main(void) {{ const struct NamedHolder *locked = &(const struct NamedHolder){{}}; {markers} return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_captured_marker_declarations("root"),
        const_pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 7),
        read = kind.read("slot"),
    )
}

fn post_forward_wrapped_captured_field_offset_cross_root_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let first_argument = post_forward_wrapped_captured_literal_field_offset_argument(
        AdjustedParameterStorage::NamedLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        "first",
        PostForwardWrapperPlacement::AfterOffset,
        false,
        false,
    );
    let second_argument = post_forward_wrapped_captured_literal_field_offset_argument(
        AdjustedParameterStorage::NamedRightPrimary,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "second",
        PostForwardWrapperPlacement::BeforeOffset,
        true,
        false,
    );
    format!(
        "{prelude}\n{helpers}\n{declarations}\n{first_markers}\n{second_markers}\n\
         int probe(struct Item first[], struct Item second[]) {{ {pointer_type}a = &first[0].nested.{field}[0]; {pointer_type}b = &second[0].nested.{field}[0]; return a - b; }}\n\
         int main(void) {{ return probe({first_argument}, {second_argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        first_markers = post_forward_captured_marker_declarations("first"),
        second_markers = post_forward_captured_marker_declarations("second"),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn post_forward_wrapped_captured_field_offset_type_mismatch_program() -> String {
    let argument = post_forward_wrapped_captured_literal_field_offset_argument(
        AdjustedParameterStorage::UnionLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        "root",
        PostForwardWrapperPlacement::AfterOffset,
        true,
        false,
    );
    format!(
        "{prelude}\n{helpers}\n{declarations}\n{markers}\nstruct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        markers = post_forward_captured_marker_declarations("root"),
    )
}

fn post_forward_wrapped_captured_field_offset_lifetime_program() -> String {
    let argument = post_forward_wrapped_captured_literal_field_offset_expression(
        "local->nested.primary",
        "local->nested.secondary",
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        "root",
        PostForwardWrapperPlacement::AfterOffset,
        true,
        false,
    );
    format!(
        "{prelude}\n{helpers}\n\
         struct Item *dangling(void) {{ struct NamedHolder *local = &(struct NamedHolder){{}}; {markers} return {argument}; }}\n\
         int main(void) {{ struct Item *result = dangling(); return result[-1].nested.values[0]; }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_captured_marker_declarations("root"),
    )
}

#[allow(clippy::too_many_arguments)]
fn post_wrapper_reforwarded_captured_literal_field_offset_expression(
    selected: &str,
    unselected: &str,
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    prefix: &str,
    outer_two_hop: bool,
    reforward_two_hop: bool,
    reforward_placement: PostForwardWrapperPlacement,
    points_to_const: bool,
) -> String {
    let inner_prefix = format!("{prefix}_inner");
    let post_prefix = format!("{prefix}_post");
    let inner = post_forward_pointer_wrapper(inner_wrapper, selected, unselected, &inner_prefix);
    let outer_helper = match (points_to_const, outer_two_hop) {
        (false, false) => "forward_outer",
        (false, true) => "forward_outer_twice",
        (true, false) => "forward_const_outer",
        (true, true) => "forward_const_outer_twice",
    };
    let reforward_helper = match (points_to_const, reforward_two_hop) {
        (false, false) => "forward_outer",
        (false, true) => "forward_outer_twice",
        (true, false) => "forward_const_outer",
        (true, true) => "forward_const_outer_twice",
    };
    let forwarded = format!("{outer_helper}({inner})");
    let post_unselected = format!("{outer_helper}({unselected})");
    let wrapped =
        post_forward_pointer_wrapper(post_wrapper, &forwarded, &post_unselected, &post_prefix);

    match reforward_placement {
        PostForwardWrapperPlacement::BeforeOffset => {
            offset.render(&format!("{reforward_helper}({wrapped})"))
        }
        PostForwardWrapperPlacement::AfterOffset => {
            format!("{reforward_helper}({})", offset.render(&wrapped))
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn post_wrapper_reforwarded_captured_literal_field_offset_argument(
    storage: AdjustedParameterStorage,
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    prefix: &str,
    outer_two_hop: bool,
    reforward_two_hop: bool,
    reforward_placement: PostForwardWrapperPlacement,
    points_to_const: bool,
) -> String {
    post_wrapper_reforwarded_captured_literal_field_offset_expression(
        storage.literal_expression(),
        storage.other_owner().literal_expression(),
        inner_wrapper,
        post_wrapper,
        offset,
        prefix,
        outer_two_hop,
        reforward_two_hop,
        reforward_placement,
        points_to_const,
    )
}

#[allow(clippy::too_many_arguments)]
fn mutable_to_const_reforwarded_captured_literal_field_argument(
    storage: AdjustedParameterStorage,
    transition_before_post_wrapper: bool,
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    prefix: &str,
    outer_two_hop: bool,
    reforward_two_hop: bool,
    reforward_placement: PostForwardWrapperPlacement,
) -> String {
    let selected = storage.literal_expression();
    let unselected = storage.other_owner().literal_expression();
    let inner_prefix = format!("{prefix}_inner");
    let post_prefix = format!("{prefix}_post");
    let inner = post_forward_pointer_wrapper(inner_wrapper, selected, unselected, &inner_prefix);
    let outer_helper = match (transition_before_post_wrapper, outer_two_hop) {
        (false, false) => "forward_outer",
        (false, true) => "forward_outer_twice",
        (true, false) => "forward_const_outer",
        (true, true) => "forward_const_outer_twice",
    };
    let reforward_helper = if reforward_two_hop {
        "forward_const_outer_twice"
    } else {
        "forward_const_outer"
    };
    let forwarded = format!("{outer_helper}({inner})");
    let post_unselected = format!("{outer_helper}({unselected})");
    let wrapped =
        post_forward_pointer_wrapper(post_wrapper, &forwarded, &post_unselected, &post_prefix);

    match reforward_placement {
        PostForwardWrapperPlacement::BeforeOffset => {
            offset.render(&format!("{reforward_helper}({wrapped})"))
        }
        PostForwardWrapperPlacement::AfterOffset => {
            format!("{reforward_helper}({})", offset.render(&wrapped))
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn mutable_to_const_reforwarded_captured_literal_field_adjusted_parameter_program(
    kind: AdjustedParameterFieldKind,
    storage: AdjustedParameterStorage,
    transition_before_post_wrapper: bool,
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_two_hop: bool,
    reforward_two_hop: bool,
    reforward_placement: PostForwardWrapperPlacement,
) -> String {
    let argument = mutable_to_const_reforwarded_captured_literal_field_argument(
        storage,
        transition_before_post_wrapper,
        inner_wrapper,
        post_wrapper,
        offset,
        "root",
        outer_two_hop,
        reforward_two_hop,
        reforward_placement,
    );
    let marker_check = post_forward_marker_check(inner_wrapper, post_wrapper, "root");
    let field = kind.field_name();
    let pointer_type = kind.const_pointer_type();
    let read = kind.read("inner");
    format!(
        "{prelude}\n{helpers}\n\
         int read_promoted(const struct Item items[]) {{\n\
             const struct Item *original = items;\n\
             {pointer_type}inner = &items[0].nested.{field}[1];\n\
             int score = ({read} == {read}) + (inner == &items[0].nested.{field}[1])\n\
                 + (original == items) + (items[-1].nested.values[0] == items[-1].nested.values[0]);\n\
             const struct Item *fallback = items - 1;\n\
             items = fallback; inner = &fallback[0].nested.{field}[0];\n\
             return score + (items == fallback) + (inner == &fallback[0].nested.{field}[0]);\n\
         }}\n\
         int main(void) {{ {declarations} {markers} return read_promoted({argument}) + (marker == 6) * 19 + ({marker_check}) * 23; }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        markers = post_forward_captured_marker_declarations("root"),
    )
}

fn mutable_to_const_reforwarded_captured_literal_field_const_discard_program(
    storage: AdjustedParameterStorage,
    transition_before_post_wrapper: bool,
) -> String {
    let argument = mutable_to_const_reforwarded_captured_literal_field_argument(
        storage,
        transition_before_post_wrapper,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        "root",
        false,
        true,
        PostForwardWrapperPlacement::BeforeOffset,
    );
    format!(
        "{prelude}\n{helpers}\n\
         int main(void) {{ {declarations} {markers} struct Item *slot = {argument}; return slot[0].nested.values[0]; }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        markers = post_forward_captured_marker_declarations("root"),
    )
}

fn mutable_to_const_reforwarded_captured_literal_field_const_write_program(
    storage: AdjustedParameterStorage,
    transition_before_post_wrapper: bool,
) -> String {
    let argument = mutable_to_const_reforwarded_captured_literal_field_argument(
        storage,
        transition_before_post_wrapper,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "root",
        true,
        false,
        PostForwardWrapperPlacement::AfterOffset,
    );
    format!(
        "{prelude}\n{helpers}\n\
         int probe(const struct Item items[]) {{ const int *slot = &items[0].nested.values[0]; *slot = 7; return *slot; }}\n\
         int main(void) {{ {declarations} {markers} return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        markers = post_forward_captured_marker_declarations("root"),
    )
}

#[allow(clippy::too_many_arguments)]
fn post_wrapper_reforwarded_captured_literal_field_offset_alias_program(
    first: AdjustedParameterPointer,
    second: AdjustedParameterPointer,
    reader: AdjustedParameterPointer,
    replacement: i64,
    delta: i64,
    inner_two_hop: [bool; 3],
    inner_wrapper: WrappedDirectLiteralRoute,
    post_wrapper: WrappedDirectLiteralRoute,
    offset: WrappedDirectLiteralOffsetRoute,
    outer_two_hop: bool,
    reforward_two_hop: bool,
    reforward_placement: PostForwardWrapperPlacement,
) -> String {
    let kind = first.kind;
    let first_address = first.render_static_address("first_items");
    let second_address = second.render_static_address("second_items");
    let reader_address = reader.render_static_address("reader_items");
    let first_forward =
        adjusted_parameter_forward_expr(first, &first_address, inner_two_hop[0], false);
    let second_forward =
        adjusted_parameter_forward_expr(second, &second_address, inner_two_hop[1], false);
    let reader_forward =
        adjusted_parameter_forward_expr(reader, &reader_address, inner_two_hop[2], true);
    let pointer_type = kind.pointer_type();
    let const_pointer_type = kind.const_pointer_type();
    let suffix = kind.suffix();
    let marker_check = ["first", "second", "reader"]
        .into_iter()
        .map(|prefix| post_forward_marker_check(inner_wrapper, post_wrapper, prefix))
        .collect::<Vec<_>>()
        .join(" && ");
    let marker_declarations = ["first", "second", "reader"]
        .into_iter()
        .map(post_forward_captured_marker_declarations)
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "{prelude}\n{outer_helpers}\n\
         {pointer_type}forward_alias_{suffix}({pointer_type}value) {{ return value; }}\n\
         {pointer_type}forward_alias_{suffix}_twice({pointer_type}value) {{ return forward_alias_{suffix}(value); }}\n\
         {const_pointer_type}forward_const_alias_{suffix}({const_pointer_type}value) {{ return value; }}\n\
         {const_pointer_type}forward_const_alias_{suffix}_twice({const_pointer_type}value) {{ return forward_const_alias_{suffix}(value); }}\n\
         int mutate_alias_{suffix}({pointer_type}first, {pointer_type}second, {const_pointer_type}reader, {pointer_type}fallback) {{\n\
             {write_first}; int before = {read_reader}; {compound_second}; int after = {read_reader};\n\
             first = fallback; second = fallback; reader = fallback;\n\
             return before * 3 + after * 5 + (first == fallback) + (second == fallback) + (reader == fallback);\n\
         }}\n\
         int probe(struct Item first_items[], struct Item second_items[], const struct Item reader_items[]) {{\n\
             {pointer_type}a = {first_forward}; {pointer_type}b = {second_forward}; {const_pointer_type}r = {reader_forward};\n\
             {initialize_first}; {initialize_second};\n\
             int observed = mutate_alias_{suffix}(a, b, r, a);\n\
             int caller_identity = (a == {first_address}) + (b == {second_address}) + (r == {reader_address});\n\
             return observed + {read_a} * 7 + {read_b} * 11 + {read_r} * 13 + caller_identity * 17;\n\
         }}\n\
         int main(void) {{\n\
             {declarations}\n{marker_declarations}\n\
             int result = probe({first_storage}, {second_storage}, {reader_storage});\n\
             return result + (marker == 6) * 19 + ({marker_check}) * 23;\n\
         }}\n",
        prelude = captured_literal_field_offset_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        write_first = kind.write("first", replacement),
        read_reader = kind.read("reader"),
        compound_second = kind.compound_add("second", delta),
        initialize_first = kind.write("a", first.model_value()),
        initialize_second = kind.write("b", second.model_value()),
        read_a = kind.read("a"),
        read_b = kind.read("b"),
        read_r = kind.read("r"),
        first_storage = post_wrapper_reforwarded_captured_literal_field_offset_argument(
            first.storage,
            inner_wrapper,
            post_wrapper,
            offset,
            "first",
            outer_two_hop,
            reforward_two_hop,
            reforward_placement,
            false,
        ),
        second_storage = post_wrapper_reforwarded_captured_literal_field_offset_argument(
            second.storage,
            inner_wrapper,
            post_wrapper,
            offset,
            "second",
            outer_two_hop,
            reforward_two_hop,
            reforward_placement,
            false,
        ),
        reader_storage = post_wrapper_reforwarded_captured_literal_field_offset_argument(
            reader.storage,
            inner_wrapper,
            post_wrapper,
            offset,
            "reader",
            outer_two_hop,
            reforward_two_hop,
            reforward_placement,
            true,
        ),
    )
}

fn post_wrapper_reforwarded_captured_field_offset_inner_bounds_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_wrapper_reforwarded_captured_literal_field_offset_argument(
        AdjustedParameterStorage::NamedLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        "root",
        true,
        false,
        PostForwardWrapperPlacement::BeforeOffset,
        false,
    );
    format!(
        "{prelude}\n{helpers}\n{declarations}\n{markers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        markers = post_forward_captured_marker_declarations("root"),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_wrapper_reforwarded_captured_field_offset_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
    outer: i64,
) -> String {
    let argument = post_wrapper_reforwarded_captured_literal_field_offset_argument(
        AdjustedParameterStorage::AnonymousLeftPrimary,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "root",
        false,
        true,
        PostForwardWrapperPlacement::AfterOffset,
        false,
    );
    format!(
        "{prelude}\n{helpers}\n{declarations}\n{markers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[{outer}].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        markers = post_forward_captured_marker_declarations("root"),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_wrapper_reforwarded_captured_field_offset_const_discard_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_wrapper_reforwarded_captured_literal_field_offset_expression(
        "locked->nested.primary",
        "locked->nested.secondary",
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        "root",
        false,
        true,
        PostForwardWrapperPlacement::AfterOffset,
        true,
    );
    format!(
        "{prelude}\n{helpers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ const struct NamedHolder *locked = &(const struct NamedHolder){{}}; {markers} return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_captured_marker_declarations("root"),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn post_wrapper_reforwarded_captured_field_offset_const_write_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = post_wrapper_reforwarded_captured_literal_field_offset_expression(
        "locked->nested.primary",
        "locked->nested.secondary",
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "root",
        true,
        false,
        PostForwardWrapperPlacement::BeforeOffset,
        true,
    );
    format!(
        "{prelude}\n{helpers}\n\
         int probe(const struct Item items[]) {{ {const_pointer_type}slot = &items[0].nested.{field}[0]; {write}; return {read}; }}\n\
         int main(void) {{ const struct NamedHolder *locked = &(const struct NamedHolder){{}}; {markers} return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_captured_marker_declarations("root"),
        const_pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 7),
        read = kind.read("slot"),
    )
}

fn post_wrapper_reforwarded_captured_field_offset_cross_root_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let first_argument = post_wrapper_reforwarded_captured_literal_field_offset_argument(
        AdjustedParameterStorage::NamedLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        "first",
        false,
        true,
        PostForwardWrapperPlacement::AfterOffset,
        false,
    );
    let second_argument = post_wrapper_reforwarded_captured_literal_field_offset_argument(
        AdjustedParameterStorage::NamedRightPrimary,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "second",
        true,
        false,
        PostForwardWrapperPlacement::BeforeOffset,
        false,
    );
    format!(
        "{prelude}\n{helpers}\n{declarations}\n{first_markers}\n{second_markers}\n\
         int probe(struct Item first[], struct Item second[]) {{ {pointer_type}a = &first[0].nested.{field}[0]; {pointer_type}b = &second[0].nested.{field}[0]; return a - b; }}\n\
         int main(void) {{ return probe({first_argument}, {second_argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        first_markers = post_forward_captured_marker_declarations("first"),
        second_markers = post_forward_captured_marker_declarations("second"),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn post_wrapper_reforwarded_captured_field_offset_type_mismatch_program() -> String {
    let argument = post_wrapper_reforwarded_captured_literal_field_offset_argument(
        AdjustedParameterStorage::UnionLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        "root",
        true,
        true,
        PostForwardWrapperPlacement::AfterOffset,
        false,
    );
    format!(
        "{prelude}\n{helpers}\n{declarations}\n{markers}\nstruct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        markers = post_forward_captured_marker_declarations("root"),
    )
}

fn post_wrapper_reforwarded_captured_field_offset_lifetime_program() -> String {
    let argument = post_wrapper_reforwarded_captured_literal_field_offset_expression(
        "local->nested.primary",
        "local->nested.secondary",
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        "root",
        true,
        true,
        PostForwardWrapperPlacement::AfterOffset,
        false,
    );
    format!(
        "{prelude}\n{helpers}\n\
         struct Item *dangling(void) {{ struct NamedHolder *local = &(struct NamedHolder){{}}; {markers} return {argument}; }}\n\
         int main(void) {{ struct Item *result = dangling(); return result[-1].nested.values[0]; }}\n",
        prelude = captured_literal_field_offset_prelude(),
        helpers = captured_outer_forwarding_helpers(),
        markers = post_forward_captured_marker_declarations("root"),
    )
}

fn captured_outer_forwarding_helpers() -> &'static str {
    "struct Item *forward_outer(struct Item *value) { return value; }\n\
     struct Item *forward_outer_twice(struct Item *value) { return forward_outer(value); }\n\
     const struct Item *forward_const_outer(const struct Item *value) { return value; }\n\
     const struct Item *forward_const_outer_twice(const struct Item *value) { return forward_const_outer(value); }"
}

fn outer_forwarded_wrapped_captured_field_offset_inner_bounds_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = wrapped_captured_literal_field_offset_argument_with_outer_forwarding(
        AdjustedParameterStorage::NamedLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        "root",
        Some(CapturedOuterForwardRoute::TwoHopAfterOffset),
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{declarations}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ int root_selected = 0; int root_unselected = 0; int root_comma = 0; return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn outer_forwarded_wrapped_captured_field_offset_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
    outer: i64,
) -> String {
    let argument = wrapped_captured_literal_field_offset_argument_with_outer_forwarding(
        AdjustedParameterStorage::AnonymousLeftPrimary,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "root",
        Some(CapturedOuterForwardRoute::TwoHopBeforeOffset),
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{declarations}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[{outer}].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ int root_selected = 0; int root_unselected = 0; int root_comma = 0; return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn outer_forwarded_wrapped_captured_field_offset_const_discard_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let wrapped = "(0 ? (++root_unselected, locked->nested.secondary) : (++root_selected, locked->nested.primary))";
    let argument = CapturedOuterForwardRoute::OneHopBeforeOffset.render(
        wrapped,
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        true,
    );
    format!(
        "{prelude}\n{outer_helpers}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ const struct NamedHolder *locked = &(const struct NamedHolder){{}}; int root_selected = 0; int root_unselected = 0; return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn outer_forwarded_wrapped_captured_field_offset_const_write_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let wrapped = "(++root_comma, ++root_selected, locked->nested.primary)";
    let argument = CapturedOuterForwardRoute::TwoHopAfterOffset.render(
        wrapped,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        true,
    );
    format!(
        "{prelude}\n{outer_helpers}\n\
         int probe(const struct Item items[]) {{ {const_pointer_type}slot = &items[0].nested.{field}[0]; {write}; return {read}; }}\n\
         int main(void) {{ const struct NamedHolder *locked = &(const struct NamedHolder){{}}; int root_selected = 0; int root_comma = 0; return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        const_pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 7),
        read = kind.read("slot"),
    )
}

fn outer_forwarded_wrapped_captured_field_offset_cross_root_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let first_argument = wrapped_captured_literal_field_offset_argument_with_outer_forwarding(
        AdjustedParameterStorage::NamedLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        "first",
        Some(CapturedOuterForwardRoute::OneHopBeforeOffset),
        false,
    );
    let second_argument = wrapped_captured_literal_field_offset_argument_with_outer_forwarding(
        AdjustedParameterStorage::NamedRightPrimary,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "second",
        Some(CapturedOuterForwardRoute::TwoHopAfterOffset),
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{declarations}\n\
         int probe(struct Item first[], struct Item second[]) {{ {pointer_type}a = &first[0].nested.{field}[0]; {pointer_type}b = &second[0].nested.{field}[0]; return a - b; }}\n\
         int main(void) {{ int first_selected = 0; int first_unselected = 0; int first_comma = 0; int second_selected = 0; int second_unselected = 0; int second_comma = 0; return probe({first_argument}, {second_argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn outer_forwarded_wrapped_captured_field_offset_type_mismatch_program() -> String {
    let argument = wrapped_captured_literal_field_offset_argument_with_outer_forwarding(
        AdjustedParameterStorage::UnionLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        "root",
        Some(CapturedOuterForwardRoute::TwoHopBeforeOffset),
        false,
    );
    format!(
        "{prelude}\n{outer_helpers}\n{declarations}\n\
         struct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ int root_selected = 0; int root_unselected = 0; int root_comma = 0; return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
        declarations = captured_literal_field_offset_declarations(),
    )
}

fn outer_forwarded_wrapped_captured_field_offset_lifetime_program() -> String {
    format!(
        "{prelude}\n{outer_helpers}\n\
         struct Item *dangling(void) {{\n\
             struct NamedHolder *local = &(struct NamedHolder){{}};\n\
             return forward_outer_twice(local->nested.primary + 1);\n\
         }}\n\
         int main(void) {{ struct Item *result = dangling(); return result[-1].nested.values[0]; }}\n",
        prelude = captured_literal_field_offset_prelude(),
        outer_helpers = captured_outer_forwarding_helpers(),
    )
}

fn wrapped_captured_literal_field_offset_inner_bounds_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = wrapped_captured_literal_field_offset_argument(
        AdjustedParameterStorage::NamedLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        "root",
    );
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ {declarations} int root_selected = 0; int root_unselected = 0; int root_comma = 0; return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        declarations = captured_literal_field_offset_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn wrapped_captured_literal_field_offset_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
    outer: i64,
) -> String {
    let argument = wrapped_captured_literal_field_offset_argument(
        AdjustedParameterStorage::AnonymousLeftPrimary,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "root",
    );
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[{outer}].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ {declarations} int root_selected = 0; int root_unselected = 0; int root_comma = 0; return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        declarations = captured_literal_field_offset_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn wrapped_captured_literal_field_offset_const_discard_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = wrapped_captured_literal_field_offset_expression(
        "locked->nested.primary",
        "locked->nested.secondary",
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        "root",
    );
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ const struct NamedHolder *locked = &(const struct NamedHolder){{}}; int root_selected = 0; int root_unselected = 0; int root_comma = 0; return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn wrapped_captured_literal_field_offset_const_write_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let argument = wrapped_captured_literal_field_offset_expression(
        "locked->nested.primary",
        "locked->nested.secondary",
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "root",
    );
    format!(
        "{prelude}\n\
         int probe(const struct Item items[]) {{ {const_pointer_type}slot = &items[0].nested.{field}[0]; {write}; return {read}; }}\n\
         int main(void) {{ const struct NamedHolder *locked = &(const struct NamedHolder){{}}; int root_selected = 0; int root_unselected = 0; int root_comma = 0; return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        const_pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 7),
        read = kind.read("slot"),
    )
}

fn wrapped_captured_literal_field_offset_cross_root_program(
    kind: AdjustedParameterFieldKind,
) -> String {
    let first_argument = wrapped_captured_literal_field_offset_argument(
        AdjustedParameterStorage::NamedLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalTrue,
        WrappedDirectLiteralOffsetRoute::PointerPlusOne,
        "first",
    );
    let second_argument = wrapped_captured_literal_field_offset_argument(
        AdjustedParameterStorage::NamedRightPrimary,
        WrappedDirectLiteralRoute::Comma,
        WrappedDirectLiteralOffsetRoute::IndexedAddress,
        "second",
    );
    format!(
        "{prelude}\n\
         int probe(struct Item first[], struct Item second[]) {{ {pointer_type}a = &first[0].nested.{field}[0]; {pointer_type}b = &second[0].nested.{field}[0]; return a - b; }}\n\
         int main(void) {{ {declarations} int first_selected = 0; int first_unselected = 0; int first_comma = 0; int second_selected = 0; int second_unselected = 0; int second_comma = 0; return probe({first_argument}, {second_argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        declarations = captured_literal_field_offset_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn wrapped_captured_literal_field_offset_type_mismatch_program() -> String {
    let argument = wrapped_captured_literal_field_offset_argument(
        AdjustedParameterStorage::UnionLeftPrimary,
        WrappedDirectLiteralRoute::ConditionalFalse,
        WrappedDirectLiteralOffsetRoute::OnePlusPointer,
        "root",
    );
    format!(
        "{prelude}\n\
         struct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ {declarations} int root_selected = 0; int root_unselected = 0; int root_comma = 0; return probe({argument}); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        declarations = captured_literal_field_offset_declarations(),
    )
}

fn captured_literal_field_offset_inner_bounds_program(kind: AdjustedParameterFieldKind) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ {declarations} return probe(named_left->nested.primary + 1); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        declarations = captured_literal_field_offset_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn captured_literal_field_offset_outer_bounds_program(
    kind: AdjustedParameterFieldKind,
    outer: i64,
) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[{outer}].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ {declarations} return probe(&anonymous_left->nested.primary[1]); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        declarations = captured_literal_field_offset_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn captured_literal_field_offset_const_discard_program(kind: AdjustedParameterFieldKind) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ const struct NamedHolder *locked = &(const struct NamedHolder){{}}; return probe(1 + locked->nested.primary); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn captured_literal_field_offset_const_write_program(kind: AdjustedParameterFieldKind) -> String {
    format!(
        "{prelude}\n\
         int probe(const struct Item items[]) {{ {const_pointer_type}slot = &items[0].nested.{field}[0]; {write}; return {read}; }}\n\
         int main(void) {{ const struct NamedHolder *locked = &(const struct NamedHolder){{}}; return probe(&locked->nested.primary[1]); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        const_pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 7),
        read = kind.read("slot"),
    )
}

fn captured_literal_field_offset_cross_path_program(kind: AdjustedParameterFieldKind) -> String {
    format!(
        "{prelude}\n\
         int probe(struct Item first[], struct Item second[]) {{\n\
             {pointer_type}a = &first[0].nested.{field}[0]; {pointer_type}b = &second[0].nested.{field}[0]; return a - b;\n\
         }}\n\
         int main(void) {{ {declarations} return probe(named_left->nested.primary + 1, &anonymous_left->nested.primary[1]); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        declarations = captured_literal_field_offset_declarations(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn captured_literal_field_offset_type_mismatch_program() -> String {
    format!(
        "{prelude}\n\
         struct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ {declarations} return probe(1 + choice_left->nested.primary); }}\n",
        prelude = captured_literal_field_offset_prelude(),
        declarations = captured_literal_field_offset_declarations(),
    )
}

fn wrapped_direct_literal_const_parameter_program(route: WrappedDirectLiteralRoute) -> String {
    let argument = route.render("ConstItems", "root");
    let marker_check = route.marker_check("root");
    format!(
        "{prelude}\n\
         typedef const struct Item ConstItems[2];\n\
         int root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int read_const(const struct Item items[]) {{ return items[0].nested.values[0] + 6; }}\n\
         int main(void) {{ return read_const({argument}) + ({marker_check}) * 10; }}\n",
        prelude = adjusted_parameter_prelude(),
    )
}

fn wrapped_direct_literal_inner_bounds_program(kind: AdjustedParameterFieldKind) -> String {
    let argument = WrappedDirectLiteralRoute::ConditionalTrue.render("struct Item[2]", "root");
    format!(
        "{prelude}\nint root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[3]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn wrapped_direct_literal_outer_bounds_program(kind: AdjustedParameterFieldKind) -> String {
    let argument = WrappedDirectLiteralRoute::Comma.render("struct Item[2]", "root");
    format!(
        "{prelude}\nint root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[2].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn wrapped_direct_literal_const_discard_program(kind: AdjustedParameterFieldKind) -> String {
    let argument = WrappedDirectLiteralRoute::ConditionalFalse.render("ConstItems", "root");
    format!(
        "{prelude}\ntypedef const struct Item ConstItems[2];\n\
         int root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(struct Item items[]) {{ {pointer_type}slot = &items[0].nested.{field}[0]; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
        read = kind.read("slot"),
    )
}

fn wrapped_direct_literal_const_write_program(kind: AdjustedParameterFieldKind) -> String {
    let argument = WrappedDirectLiteralRoute::Comma.render("ConstItems", "root");
    format!(
        "{prelude}\ntypedef const struct Item ConstItems[2];\n\
         int root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         int probe(const struct Item items[]) {{ {const_pointer_type}slot = &items[0].nested.{field}[0]; {write}; return {read}; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        const_pointer_type = kind.const_pointer_type(),
        field = kind.field_name(),
        write = kind.write("slot", 7),
        read = kind.read("slot"),
    )
}

fn wrapped_direct_literal_cross_root_program(kind: AdjustedParameterFieldKind) -> String {
    let first_argument =
        WrappedDirectLiteralRoute::ConditionalTrue.render("struct Item[2]", "left");
    let second_argument = WrappedDirectLiteralRoute::Comma.render("struct Item[2]", "right");
    format!(
        "{prelude}\nint left_selected = 0; int left_unselected = 0; int left_comma = 0;\n\
         int right_selected = 0; int right_unselected = 0; int right_comma = 0;\n\
         int probe(struct Item first[], struct Item second[]) {{\n\
             {pointer_type}a = &first[0].nested.{field}[0]; {pointer_type}b = &second[0].nested.{field}[0]; return a - b;\n\
         }}\nint main(void) {{ return probe({first_argument}, {second_argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
        pointer_type = kind.pointer_type(),
        field = kind.field_name(),
    )
}

fn wrapped_direct_literal_type_mismatch_program() -> String {
    let argument = WrappedDirectLiteralRoute::ConditionalFalse.render("struct Item[2]", "root");
    format!(
        "{prelude}\nint root_selected = 0; int root_unselected = 0; int root_comma = 0;\n\
         struct Other {{ int value; }};\n\
         int probe(struct Item items[]) {{ struct Other *slot = &items[0].nested.points[0]; return slot->value; }}\n\
         int main(void) {{ return probe({argument}); }}\n",
        prelude = adjusted_parameter_prelude(),
    )
}

#[derive(Clone)]
enum ExpectedInterpretation {
    Value(i64),
    Error(&'static str),
    OwnedError(String),
}

fn assert_interpretation(source: &str, expected: ExpectedInterpretation, context: &str) {
    let caught = panic::catch_unwind(|| interpret(source));
    let result = caught.unwrap_or_else(|_| panic!("interpret panicked for {context}: {source}"));

    match expected {
        ExpectedInterpretation::Value(expected) => {
            let actual = result.unwrap_or_else(|error| {
                panic!("expected value {expected}, got error {error:?}; {context}: {source}")
            });
            assert_eq!(actual, expected, "{context}: {source}");
        }
        ExpectedInterpretation::Error(expected) => {
            let actual = result.err().unwrap_or_else(|| {
                panic!("expected error {expected:?}, got value; {context}: {source}")
            });
            assert_eq!(actual.to_string(), expected, "{context}: {source}");
        }
        ExpectedInterpretation::OwnedError(expected) => {
            let actual = result.err().unwrap_or_else(|| {
                panic!("expected error {expected:?}, got value; {context}: {source}")
            });
            assert_eq!(actual.to_string(), expected, "{context}: {source}");
        }
    }
}

fn pointer_program(result_type: &str, expression: &str, setup: &str) -> String {
    format!(
        "int main(void) {{\n\
         int left[256];\n\
         int right[256];\n\
         const int *const_left = left;\n\
         const int *const_right = right;\n\
         {setup}\n\
         {result_type} result = {expression};\n\
         return *result;\n\
         }}\n"
    )
}

fn generate_pointer_expr(state: &mut u64, depth: usize) -> PointerExpr {
    if depth == 0 || next_u64(state) % 5 == 0 {
        return PointerExpr::Base(ModelPointer {
            root: if next_u64(state) & 1 == 0 {
                ArrayRoot::Left
            } else {
                ArrayRoot::Right
            },
            index: 64 + (next_u64(state) % 128) as i64,
            points_to_const: next_u64(state) & 1 == 0,
        });
    }

    match next_u64(state) % 5 {
        0 => PointerExpr::Add(
            Box::new(generate_pointer_expr(state, depth - 1)),
            Box::new(generate_scalar_expr(state, depth - 1)),
        ),
        1 => PointerExpr::ReverseAdd(
            Box::new(generate_scalar_expr(state, depth - 1)),
            Box::new(generate_pointer_expr(state, depth - 1)),
        ),
        2 => PointerExpr::Subtract(
            Box::new(generate_pointer_expr(state, depth - 1)),
            Box::new(generate_scalar_expr(state, depth - 1)),
        ),
        3 => PointerExpr::Conditional(
            next_u64(state) & 1 == 0,
            Box::new(generate_pointer_expr(state, depth - 1)),
            Box::new(generate_pointer_expr(state, depth - 1)),
        ),
        _ => PointerExpr::Comma(
            Box::new(generate_scalar_expr(state, depth - 1)),
            Box::new(generate_pointer_expr(state, depth - 1)),
        ),
    }
}

fn generate_scalar_expr(state: &mut u64, depth: usize) -> ScalarExpr {
    if depth == 0 || next_u64(state) % 4 == 0 {
        return ScalarExpr::Literal((next_u64(state) % 7) as i64 - 3);
    }

    match next_u64(state) % 3 {
        0 => ScalarExpr::PointerDifference(
            Box::new(generate_pointer_expr(state, depth - 1)),
            Box::new(generate_pointer_expr(state, depth - 1)),
        ),
        1 => ScalarExpr::Conditional(
            next_u64(state) & 1 == 0,
            Box::new(generate_scalar_expr(state, depth - 1)),
            Box::new(generate_scalar_expr(state, depth - 1)),
        ),
        _ => ScalarExpr::Comma(
            Box::new(generate_scalar_expr(state, depth - 1)),
            Box::new(generate_scalar_expr(state, depth - 1)),
        ),
    }
}

fn next_u64(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}
