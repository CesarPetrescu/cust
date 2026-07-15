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
