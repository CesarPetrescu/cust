use std::panic;

use cust::interpret;

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum PointerOutputKind {
    Character,
    Integer,
    Boolean,
    Double,
}

impl PointerOutputKind {
    const COUNT: usize = 4;
    const ALL: [Self; Self::COUNT] = [Self::Character, Self::Integer, Self::Boolean, Self::Double];

    fn index(self) -> usize {
        self as usize
    }

    fn scalar_type(self) -> &'static str {
        match self {
            Self::Character => "char",
            Self::Integer => "int",
            Self::Boolean => "_Bool",
            Self::Double => "double",
        }
    }

    fn values(self) -> &'static str {
        match self {
            Self::Character | Self::Integer => "{10, 11, 12, 13, 14, 15, 16, 17}",
            Self::Boolean => "{0, 1, 0, 1, 0, 1, 0, 1}",
            Self::Double => "{0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5}",
        }
    }

    fn wrong_scalar_type(self) -> &'static str {
        match self {
            Self::Character => "int",
            Self::Integer | Self::Boolean | Self::Double => "char",
        }
    }

    fn kind_label(self) -> &'static str {
        match self {
            Self::Character => "character",
            Self::Integer => "integer",
            Self::Boolean => "boolean",
            Self::Double => "double",
        }
    }

    fn indefinite_article(self) -> &'static str {
        match self {
            Self::Integer => "an",
            Self::Character | Self::Boolean | Self::Double => "a",
        }
    }

    fn initializer_error(self) -> String {
        match self {
            Self::Character => "character pointer object 'result' initializer requires null, another character pointer output object, or the address of a mutable char pointer variable".to_owned(),
            _ => format!(
                "{} pointer object 'result' initializer requires null, another compatible pointer output object, or the address of a mutable {} pointer variable",
                self.scalar_type(),
                self.scalar_type()
            ),
        }
    }

    fn assignment_error(self) -> String {
        match self {
            Self::Character => "character pointer object 'result' assignment requires null, another character pointer output object, or the address of a mutable char pointer variable".to_owned(),
            _ => format!(
                "{} pointer object 'result' assignment requires null, another compatible pointer output object, or the address of a mutable {} pointer variable",
                self.scalar_type(),
                self.scalar_type()
            ),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum PointerOutputRoute {
    AddressOfSlot,
    LocalObject,
    GlobalObject,
    FileStaticObject,
    BlockStaticObject,
    ConditionalObject,
    CommaObject,
    GenericObject,
    AssignmentResult,
}

impl PointerOutputRoute {
    const COUNT: usize = 9;
    const ALL: [Self; Self::COUNT] = [
        Self::AddressOfSlot,
        Self::LocalObject,
        Self::GlobalObject,
        Self::FileStaticObject,
        Self::BlockStaticObject,
        Self::ConditionalObject,
        Self::CommaObject,
        Self::GenericObject,
        Self::AssignmentResult,
    ];

    fn index(self) -> usize {
        self as usize
    }

    fn expression(self) -> &'static str {
        match self {
            Self::AddressOfSlot => "&local_slot",
            Self::LocalObject => "local_output",
            Self::GlobalObject => "global_output",
            Self::FileStaticObject => "file_output",
            Self::BlockStaticObject => "block_output",
            Self::ConditionalObject => "choose() ? local_output : global_output",
            Self::CommaObject => "(touch(), file_output)",
            Self::GenericObject => {
                "_Generic(generic_control(), int: block_output, default: local_output)"
            }
            Self::AssignmentResult => "(scratch = global_output)",
        }
    }

    fn selected_slot(self) -> &'static str {
        match self {
            Self::AddressOfSlot | Self::LocalObject | Self::ConditionalObject => "local_slot",
            Self::GlobalObject | Self::AssignmentResult => "global_slot",
            Self::FileStaticObject | Self::CommaObject => "file_slot",
            Self::BlockStaticObject | Self::GenericObject => "block_slot",
        }
    }

    fn selected_output(self) -> &'static str {
        match self {
            Self::AddressOfSlot | Self::LocalObject | Self::ConditionalObject => "local_output",
            Self::GlobalObject | Self::AssignmentResult => "global_output",
            Self::FileStaticObject | Self::CommaObject => "file_output",
            Self::BlockStaticObject | Self::GenericObject => "block_output",
        }
    }

    fn expected_sequence(self) -> i64 {
        match self {
            Self::ConditionalObject => 1,
            Self::CommaObject => 2,
            _ => 0,
        }
    }

    fn evaluated_side_effect_check(self) -> &'static str {
        match self {
            Self::AssignmentResult => "scratch == global_output",
            _ => "1",
        }
    }

    fn distinct_output(self) -> &'static str {
        match self {
            Self::GlobalObject | Self::AssignmentResult => "local_output",
            _ => "global_output",
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum PointerOutputContext {
    Initializer,
    Assignment,
    Argument,
    Equality,
    Truthiness,
    Sizeof,
}

impl PointerOutputContext {
    const COUNT: usize = 6;
    const ALL: [Self; Self::COUNT] = [
        Self::Initializer,
        Self::Assignment,
        Self::Argument,
        Self::Equality,
        Self::Truthiness,
        Self::Sizeof,
    ];

    fn index(self) -> usize {
        self as usize
    }

    fn body(self, route: PointerOutputRoute, scalar_type: &str) -> String {
        let expression = route.expression();
        let selected_slot = route.selected_slot();
        let selected_output = route.selected_output();
        let distinct_output = route.distinct_output();
        let expected_sequence = route.expected_sequence();
        let evaluated_side_effect_check = route.evaluated_side_effect_check();
        let checks = format!(
            "if ({selected_slot} != values + 7) return 1;\n\
             if (sequence != {expected_sequence}) return 2;\n\
             if (!({evaluated_side_effect_check})) return 3;\n\
             return 0;"
        );

        match self {
            Self::Initializer => {
                format!("{scalar_type} **result = {expression};\n*result = values + 7;\n{checks}")
            }
            Self::Assignment => format!(
                "{scalar_type} **result = 0;\nresult = {expression};\n*result = values + 7;\n{checks}"
            ),
            Self::Argument => {
                format!("retarget({expression}, values + 7);\n{checks}")
            }
            Self::Equality => format!(
                "if (!(({expression}) == {selected_output})) return 1;\n\
                 if (({expression}) == {distinct_output}) return 2;\n\
                 if (sequence != {}) return 3;\n\
                 if (!({evaluated_side_effect_check})) return 4;\n\
                 return 0;",
                expected_sequence * 11
            ),
            Self::Truthiness => format!(
                "if (!({expression})) return 1;\n\
                 {{ {scalar_type} **nil = 0; if (nil) return 2; }}\n\
                 if (sequence != {expected_sequence}) return 3;\n\
                 if (!({evaluated_side_effect_check})) return 4;\n\
                 return 0;"
            ),
            Self::Sizeof => format!(
                "if (sizeof({expression}) != sizeof(local_output)) return 1;\n\
                 if (sequence != 0) return 2;\n\
                 if (scratch != local_output) return 3;\n\
                 return 0;"
            ),
        }
    }

    fn program(self, kind: PointerOutputKind, route: PointerOutputRoute) -> String {
        let scalar_type = kind.scalar_type();
        let values = kind.values();
        let body = self.body(route, scalar_type);
        format!(
            "{scalar_type} values[8] = {values};\n\
             {scalar_type} *global_slot = values;\n\
             {scalar_type} **global_output = &global_slot;\n\
             static {scalar_type} *file_slot = values + 1;\n\
             static {scalar_type} **file_output = &file_slot;\n\
             int sequence;\n\
             int choose(void) {{ sequence = sequence * 10 + 1; return 1; }}\n\
             int touch(void) {{ sequence = sequence * 10 + 2; return 0; }}\n\
             int generic_control(void) {{ sequence = sequence * 10 + 3; return 0; }}\n\
             void retarget({scalar_type} **output, {scalar_type} *value) {{ *output = value; }}\n\
             int main(void) {{\n\
                 {scalar_type} *local_slot = values + 2;\n\
                 {scalar_type} **local_output = &local_slot;\n\
                 static {scalar_type} *block_slot = values + 3;\n\
                 static {scalar_type} **block_output = &block_slot;\n\
                 {scalar_type} **scratch = local_output;\n\
                 sequence = 0;\n\
                 {body}\n\
             }}\n"
        )
    }
}

#[test]
fn generated_tracked_pointer_output_routes_stay_in_classifier_evaluator_parity() {
    let mut kind_counts = [0; PointerOutputKind::COUNT];
    let mut route_counts = [0; PointerOutputRoute::COUNT];
    let mut context_counts = [0; PointerOutputContext::COUNT];
    let mut cell_counts =
        [0; PointerOutputKind::COUNT * PointerOutputRoute::COUNT * PointerOutputContext::COUNT];

    for kind in PointerOutputKind::ALL {
        for route in PointerOutputRoute::ALL {
            for context in PointerOutputContext::ALL {
                let kind_index = kind.index();
                let route_index = route.index();
                let context_index = context.index();
                kind_counts[kind_index] += 1;
                route_counts[route_index] += 1;
                context_counts[context_index] += 1;
                cell_counts[(kind_index * PointerOutputRoute::COUNT + route_index)
                    * PointerOutputContext::COUNT
                    + context_index] += 1;

                let source = context.program(kind, route);
                let result = panic::catch_unwind(|| interpret(&source)).unwrap_or_else(|payload| {
                    panic!(
                        "tracked pointer-output route panicked for {kind:?}, {route:?}, {context:?}: {payload:?}"
                    )
                });
                assert_eq!(
                    result.unwrap_or_else(|error| panic!(
                        "tracked pointer-output route failed for {kind:?}, {route:?}, {context:?}: {error}"
                    )),
                    0,
                    "tracked pointer-output route returned the wrong value for {kind:?}, {route:?}, {context:?}"
                );
            }
        }
    }

    assert_eq!(kind_counts, [54; 4]);
    assert_eq!(route_counts, [24; 9]);
    assert_eq!(context_counts, [36; 6]);
    assert!(cell_counts.into_iter().all(|count| count == 1));
}

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum PointerOutputDiagnosticContext {
    Initializer,
    Assignment,
    Argument,
    Equality,
    Conditional,
}

impl PointerOutputDiagnosticContext {
    const COUNT: usize = 5;
    const ALL: [Self; Self::COUNT] = [
        Self::Initializer,
        Self::Assignment,
        Self::Argument,
        Self::Equality,
        Self::Conditional,
    ];

    fn index(self) -> usize {
        self as usize
    }

    fn body(self) -> &'static str {
        match self {
            Self::Initializer => "TARGET **result = (trap(), wrong_output); return result != 0;",
            Self::Assignment => {
                "TARGET **result = target_output; result = (trap(), wrong_output); return result != 0;"
            }
            Self::Argument => "consume((trap(), wrong_output)); return 0;",
            Self::Equality => "return (trap(), target_output) == wrong_output;",
            Self::Conditional => "return sizeof(1 ? target_output : wrong_output);",
        }
    }

    fn expected(self, kind: PointerOutputKind) -> String {
        match self {
            Self::Initializer => kind.initializer_error(),
            Self::Assignment => kind.assignment_error(),
            Self::Argument => format!(
                "function 'consume' parameter 'output' requires {} {} pointer slot address",
                kind.indefinite_article(),
                kind.scalar_type()
            ),
            Self::Equality => {
                "pointer output equality requires compatible pointee types".to_owned()
            }
            Self::Conditional => format!(
                "conditional {} pointer output branches require compatible output values or null",
                kind.kind_label()
            ),
        }
    }

    fn program(self, kind: PointerOutputKind) -> String {
        let target = kind.scalar_type();
        let wrong = kind.wrong_scalar_type();
        let body = self.body().replace("TARGET", target);
        format!(
            "{target} target_value;\n\
             {target} *target_slot = &target_value;\n\
             {target} **target_output = &target_slot;\n\
             {wrong} wrong_value;\n\
             {wrong} *wrong_slot = &wrong_value;\n\
             {wrong} **wrong_output = &wrong_slot;\n\
             int *expired;\n\
             int trap(void) {{ return *expired; }}\n\
             void consume({target} **output) {{ if (output) {{ **output = **output; }} }}\n\
             int main(void) {{ {{ int dead = 0; expired = &dead; }} {body} }}\n"
        )
    }
}

#[test]
fn generated_tracked_pointer_output_diagnostics_stay_in_type_parity() {
    let trap_witness = "int *expired;\n\
                        int trap(void) { return *expired; }\n\
                        int main(void) { { int dead = 0; expired = &dead; } return trap(); }\n";
    assert_eq!(
        interpret(trap_witness).unwrap_err().to_string(),
        "pointer to out-of-scope variable 'dead'"
    );

    let mut kind_counts = [0; PointerOutputKind::COUNT];
    let mut context_counts = [0; PointerOutputDiagnosticContext::COUNT];
    let mut cell_counts = [0; PointerOutputKind::COUNT * PointerOutputDiagnosticContext::COUNT];

    for kind in PointerOutputKind::ALL {
        for context in PointerOutputDiagnosticContext::ALL {
            kind_counts[kind.index()] += 1;
            context_counts[context.index()] += 1;
            cell_counts[kind.index() * PointerOutputDiagnosticContext::COUNT + context.index()] +=
                1;

            let source = context.program(kind);
            let result = panic::catch_unwind(|| interpret(&source)).unwrap_or_else(|payload| {
                panic!(
                    "tracked pointer-output diagnostic route panicked for {kind:?}, {context:?}: {payload:?}"
                )
            });
            assert_eq!(
                result
                    .expect_err(&format!(
                        "tracked pointer-output diagnostic route unexpectedly passed for {kind:?}, {context:?}"
                    ))
                    .to_string(),
                context.expected(kind),
                "tracked pointer-output diagnostic mismatch for {kind:?}, {context:?}"
            );
        }
    }

    assert_eq!(kind_counts, [5; 4]);
    assert_eq!(context_counts, [4; 5]);
    assert!(cell_counts.into_iter().all(|count| count == 1));
}

#[test]
fn generated_tracked_pointer_outputs_preserve_const_and_lifetime_without_evaluation() {
    let mut kind_counts = [0; PointerOutputKind::COUNT];

    for kind in PointerOutputKind::ALL {
        kind_counts[kind.index()] += 1;
        let scalar_type = kind.scalar_type();
        let values = kind.values();
        let const_source = format!(
            "{scalar_type} values[8] = {values};\n\
             const {scalar_type} const_values[1] = {{0}};\n\
             int main(void) {{\n\
                 {scalar_type} *slot = values;\n\
                 {scalar_type} **output = &slot;\n\
                 return sizeof(*output = const_values);\n\
             }}\n"
        );
        assert_eq!(
            interpret(&const_source).unwrap_err().to_string(),
            "cannot discard const qualifier from pointer target",
            "const preservation for {kind:?}"
        );

        let non_evaluating_lifetime_source = format!(
            "{scalar_type} values[8] = {values};\n\
             int ping(void) {{ return 7; }}\n\
             int main(void) {{\n\
                 {scalar_type} *slot = values;\n\
                 {scalar_type} **output = &slot;\n\
                 {{ {scalar_type} local = 0; *output = &local; }}\n\
                 if (sizeof(**output) != sizeof({scalar_type})) return 1;\n\
                 if (ping() != 7) return 2;\n\
                 return 0;\n\
             }}\n"
        );
        assert_eq!(
            interpret(&non_evaluating_lifetime_source).unwrap(),
            0,
            "non-evaluating lifetime preservation for {kind:?}"
        );

        let evaluated_lifetime_source = format!(
            "{scalar_type} values[8] = {values};\n\
             int main(void) {{\n\
                 {scalar_type} *slot = values;\n\
                 {scalar_type} **output = &slot;\n\
                 {{ {scalar_type} local = 0; *output = &local; }}\n\
                 return **output != 0;\n\
             }}\n"
        );
        assert_eq!(
            interpret(&evaluated_lifetime_source)
                .unwrap_err()
                .to_string(),
            "pointer to out-of-scope variable 'local'",
            "evaluated lifetime preservation for {kind:?}"
        );
    }

    assert_eq!(kind_counts, [1; 4]);
}
