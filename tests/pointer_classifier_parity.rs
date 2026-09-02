use std::panic;

use cust::interpret;

#[derive(Clone, Copy, Debug)]
enum ExpectedInterpretation {
    Value(i64),
    Error(&'static str),
}

fn assert_interpretation(source: &str, expected: ExpectedInterpretation, context: &str) {
    let result = panic::catch_unwind(|| interpret(source));
    let result = result.unwrap_or_else(|payload| {
        panic!("pointer classifier/evaluator route panicked for {context}: {payload:?}")
    });
    match expected {
        ExpectedInterpretation::Value(expected) => assert_eq!(
            result.unwrap_or_else(|error| panic!("{context}: unexpected error: {error}")),
            expected,
            "{context}"
        ),
        ExpectedInterpretation::Error(expected) => assert_eq!(
            result
                .expect_err(&format!("{context}: expected error"))
                .to_string(),
            expected,
            "{context}"
        ),
    }
}

#[derive(Clone, Copy, Debug)]
enum PointerValueKind {
    Int,
    Char,
    Cell,
    Leaf,
    Row,
}

impl PointerValueKind {
    fn declaration(self, name: &str) -> String {
        match self {
            Self::Int => format!("int *{name}"),
            Self::Char => format!("char *{name}"),
            Self::Cell => format!("struct Cell *{name}"),
            Self::Leaf => format!("struct Leaf *{name}"),
            Self::Row => format!("int (*{name})[3]"),
        }
    }

    fn return_declaration(self, name: &str) -> String {
        match self {
            Self::Int => format!("int *{name}(void)"),
            Self::Char => format!("char *{name}(void)"),
            Self::Cell => format!("struct Cell *{name}(void)"),
            Self::Leaf => format!("struct Leaf *{name}(void)"),
            Self::Row => format!("int (*{name}(void))[3]"),
        }
    }

    fn observe(self, expression: &str) -> String {
        match self {
            Self::Int | Self::Char => format!("*({expression})"),
            Self::Cell | Self::Leaf => format!("({expression})->value"),
            Self::Row => format!("(*({expression}))[0]"),
        }
    }

    fn consumer(self) -> &'static str {
        match self {
            Self::Int => "consume_int",
            Self::Char => "consume_char",
            Self::Cell => "consume_cell",
            Self::Leaf => "consume_leaf",
            Self::Row => "consume_row",
        }
    }

    fn incompatible_declaration(self) -> &'static str {
        match self {
            Self::Int | Self::Cell | Self::Leaf | Self::Row => "char *result",
            Self::Char => "int *result",
        }
    }

    fn incompatible_error(self) -> &'static str {
        match self {
            Self::Int => "cannot convert pointer to int to pointer to char",
            Self::Char => "cannot convert pointer to char to pointer to int",
            Self::Cell => "cannot convert pointer to struct 'Cell' to pointer to char",
            Self::Leaf => "cannot convert pointer to struct 'Leaf' to pointer to char",
            Self::Row => "cannot convert pointer to two-dimensional int row to pointer to char",
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum PointerValueRoute {
    GenericSelection,
    DirectTwoDimensionalRow,
    StructTwoDimensionalRow,
    StructElementTwoDimensionalRow,
    StructPointerTwoDimensionalRow,
    AddressOfScalar,
    AddressOfArrayElement,
    AddressOfAggregateArrayElement,
    AddressOfStructField,
    AddressOfStructElementField,
    AddressOfStructArrayField,
    AddressOfStructElementArrayField,
    AddressOfStructPointerField,
    AddressOfStructPointerArrayField,
    AddressOfScalarLiteral,
    AddressOfAggregateLiteral,
    AddressOfAggregateField,
    StringLiteral,
    ArrayLiteral,
    AggregateArrayLiteral,
    PointerCast,
    PointerVariable,
    ScalarArrayVariable,
    AggregateArrayVariable,
    TwoDimensionalArrayVariable,
    PointerAssignment,
    PointerCompoundAssignment,
    StructField,
    StructScalarArrayField,
    StructRowArrayField,
    StructAggregateArrayField,
    StructElementField,
    StructElementScalarArrayField,
    StructElementRowArrayField,
    StructElementAggregateArrayField,
    StructElementFieldAssignment,
    StructElementFieldCompoundAssignment,
    StructPointerField,
    StructPointerScalarArrayField,
    StructPointerRowArrayField,
    StructPointerAggregateArrayField,
    AggregateLiteralField,
    AggregateLiteralScalarArrayField,
    AggregateLiteralRowArrayField,
    AggregateLiteralAggregateArrayField,
    AggregateLiteralFieldAssignment,
    AggregateLiteralFieldCompoundAssignment,
    StructFieldAssignment,
    StructFieldCompoundAssignment,
    StructPointerFieldAssignment,
    StructPointerFieldCompoundAssignment,
    EmbeddedAggregateArrayElementField,
    EmbeddedAggregateArrayElementScalarArrayField,
    EmbeddedAggregateArrayElementRowArrayField,
    EmbeddedAggregateArrayElementAggregateArrayField,
    EmbeddedAggregateArrayElementFieldAssignment,
    EmbeddedAggregateArrayElementFieldCompoundAssignment,
    PointerIncrement,
    RowPointerDereference,
    CharacterPointerOutputDereference,
    CharacterPointerOutputAssignment,
    FunctionCall,
    Conditional,
    ConditionalNullLeft,
    ConditionalNullRight,
    Comma,
    PointerAddition,
    ReverseSubscript,
    PointerSubtraction,
}

impl PointerValueRoute {
    const COUNT: usize = 69;
    const ALL: [Self; Self::COUNT] = [
        Self::GenericSelection,
        Self::DirectTwoDimensionalRow,
        Self::StructTwoDimensionalRow,
        Self::StructElementTwoDimensionalRow,
        Self::StructPointerTwoDimensionalRow,
        Self::AddressOfScalar,
        Self::AddressOfArrayElement,
        Self::AddressOfAggregateArrayElement,
        Self::AddressOfStructField,
        Self::AddressOfStructElementField,
        Self::AddressOfStructArrayField,
        Self::AddressOfStructElementArrayField,
        Self::AddressOfStructPointerField,
        Self::AddressOfStructPointerArrayField,
        Self::AddressOfScalarLiteral,
        Self::AddressOfAggregateLiteral,
        Self::AddressOfAggregateField,
        Self::StringLiteral,
        Self::ArrayLiteral,
        Self::AggregateArrayLiteral,
        Self::PointerCast,
        Self::PointerVariable,
        Self::ScalarArrayVariable,
        Self::AggregateArrayVariable,
        Self::TwoDimensionalArrayVariable,
        Self::PointerAssignment,
        Self::PointerCompoundAssignment,
        Self::StructField,
        Self::StructScalarArrayField,
        Self::StructRowArrayField,
        Self::StructAggregateArrayField,
        Self::StructElementField,
        Self::StructElementScalarArrayField,
        Self::StructElementRowArrayField,
        Self::StructElementAggregateArrayField,
        Self::StructElementFieldAssignment,
        Self::StructElementFieldCompoundAssignment,
        Self::StructPointerField,
        Self::StructPointerScalarArrayField,
        Self::StructPointerRowArrayField,
        Self::StructPointerAggregateArrayField,
        Self::AggregateLiteralField,
        Self::AggregateLiteralScalarArrayField,
        Self::AggregateLiteralRowArrayField,
        Self::AggregateLiteralAggregateArrayField,
        Self::AggregateLiteralFieldAssignment,
        Self::AggregateLiteralFieldCompoundAssignment,
        Self::StructFieldAssignment,
        Self::StructFieldCompoundAssignment,
        Self::StructPointerFieldAssignment,
        Self::StructPointerFieldCompoundAssignment,
        Self::EmbeddedAggregateArrayElementField,
        Self::EmbeddedAggregateArrayElementScalarArrayField,
        Self::EmbeddedAggregateArrayElementRowArrayField,
        Self::EmbeddedAggregateArrayElementAggregateArrayField,
        Self::EmbeddedAggregateArrayElementFieldAssignment,
        Self::EmbeddedAggregateArrayElementFieldCompoundAssignment,
        Self::PointerIncrement,
        Self::RowPointerDereference,
        Self::CharacterPointerOutputDereference,
        Self::CharacterPointerOutputAssignment,
        Self::FunctionCall,
        Self::Conditional,
        Self::ConditionalNullLeft,
        Self::ConditionalNullRight,
        Self::Comma,
        Self::PointerAddition,
        Self::ReverseSubscript,
        Self::PointerSubtraction,
    ];

    fn index(self) -> usize {
        self as usize
    }

    fn expression(self) -> &'static str {
        match self {
            Self::GenericSelection => "_Generic(pointer, int *: values + 1)",
            Self::DirectTwoDimensionalRow => "matrix[row_index()]",
            Self::StructTwoDimensionalRow => "matrix_holder.rows[row_index()]",
            Self::StructElementTwoDimensionalRow => {
                "matrix_holders[outer_index()].rows[row_index()]"
            }
            Self::StructPointerTwoDimensionalRow => "matrix_holder_pointer->rows[row_index()]",
            Self::AddressOfScalar => "&scalar",
            Self::AddressOfArrayElement => "&values[element_index()]",
            Self::AddressOfAggregateArrayElement => "&cells[outer_index()]",
            Self::AddressOfStructField => "&holder.scalar",
            Self::AddressOfStructElementField => "&holders[outer_index()].scalar",
            Self::AddressOfStructArrayField => "&holder.values[element_index()]",
            Self::AddressOfStructElementArrayField => {
                "&holders[outer_index()].values[element_index()]"
            }
            Self::AddressOfStructPointerField => "&holder_pointer->scalar",
            Self::AddressOfStructPointerArrayField => "&holder_pointer->values[element_index()]",
            Self::AddressOfScalarLiteral => "&(int){17}",
            Self::AddressOfAggregateLiteral => {
                "&((struct Cell){.value = 61, .values = {62, 63, 64}, .cursor = values})"
            }
            Self::AddressOfAggregateField => {
                "&((struct Cell){.value = 65, .values = {66, 67, 68}, .cursor = values}).value"
            }
            Self::StringLiteral => "\"abc\" + 1",
            Self::ArrayLiteral => "(int[]){70, 71, 72} + 1",
            Self::AggregateArrayLiteral => "(struct Cell[]){{.value = 73}, {.value = 74}} + 1",
            Self::PointerCast => "(int *)(values + 1)",
            Self::PointerVariable => "pointer",
            Self::ScalarArrayVariable => "values",
            Self::AggregateArrayVariable => "cells",
            Self::TwoDimensionalArrayVariable => "matrix",
            Self::PointerAssignment => "(pointer = values + 1)",
            Self::PointerCompoundAssignment => "(pointer += 1)",
            Self::StructField => "holder.cursor",
            Self::StructScalarArrayField => "holder.values",
            Self::StructRowArrayField => "holder.rows",
            Self::StructAggregateArrayField => "holder.cells",
            Self::StructElementField => "holders[outer_index()].cursor",
            Self::StructElementScalarArrayField => "holders[outer_index()].values",
            Self::StructElementRowArrayField => "holders[outer_index()].rows",
            Self::StructElementAggregateArrayField => "holders[outer_index()].cells",
            Self::StructElementFieldAssignment => "(holders[outer_index()].cursor = values + 1)",
            Self::StructElementFieldCompoundAssignment => "(holders[outer_index()].cursor += 1)",
            Self::StructPointerField => "holder_pointer->cursor",
            Self::StructPointerScalarArrayField => "holder_pointer->values",
            Self::StructPointerRowArrayField => "holder_pointer->rows",
            Self::StructPointerAggregateArrayField => "holder_pointer->cells",
            Self::AggregateLiteralField => "((struct Holder){.cursor = values + 1}).cursor",
            Self::AggregateLiteralScalarArrayField => {
                "((struct Holder){.values = {91, 92, 93}}).values"
            }
            Self::AggregateLiteralRowArrayField => {
                "((struct Holder){.rows = {{94, 95, 96}, {97, 98, 99}}}).rows"
            }
            Self::AggregateLiteralAggregateArrayField => {
                "((struct Holder){.cells = {{.value = 97}, {.value = 98}}}).cells"
            }
            Self::AggregateLiteralFieldAssignment => {
                "(((struct Holder){.cursor = values}).cursor = values + 1)"
            }
            Self::AggregateLiteralFieldCompoundAssignment => {
                "(((struct Holder){.cursor = values}).cursor += 1)"
            }
            Self::StructFieldAssignment => "(holder.cursor = values + 1)",
            Self::StructFieldCompoundAssignment => "(holder.cursor += 1)",
            Self::StructPointerFieldAssignment => "(holder_pointer->cursor = values + 1)",
            Self::StructPointerFieldCompoundAssignment => "(holder_pointer->cursor += 1)",
            Self::EmbeddedAggregateArrayElementField => "holder.cells[inner_index()].cursor",
            Self::EmbeddedAggregateArrayElementScalarArrayField => {
                "holder.cells[inner_index()].values"
            }
            Self::EmbeddedAggregateArrayElementRowArrayField => "holder.cells[inner_index()].rows",
            Self::EmbeddedAggregateArrayElementAggregateArrayField => {
                "holder.cells[inner_index()].children"
            }
            Self::EmbeddedAggregateArrayElementFieldAssignment => {
                "(holder.cells[inner_index()].cursor = values + 1)"
            }
            Self::EmbeddedAggregateArrayElementFieldCompoundAssignment => {
                "(holder.cells[inner_index()].cursor += 1)"
            }
            Self::PointerIncrement => "pointer++",
            Self::RowPointerDereference => "*row_pointer",
            Self::CharacterPointerOutputDereference => "*output",
            Self::CharacterPointerOutputAssignment => "(*output = text + 1)",
            Self::FunctionCall => "make_pointer()",
            Self::Conditional => "condition() ? values + 1 : values + 2",
            Self::ConditionalNullLeft => "!condition() ? 0 : values + 1",
            Self::ConditionalNullRight => "condition() ? values + 1 : 0",
            Self::Comma => "(touch(), values + 1)",
            Self::PointerAddition => "values + offset()",
            Self::ReverseSubscript => "row_index()[row_pointer]",
            Self::PointerSubtraction => "values + 2 - offset()",
        }
    }

    fn kind(self) -> PointerValueKind {
        match self {
            Self::AddressOfAggregateLiteral
            | Self::AggregateArrayLiteral
            | Self::AggregateArrayVariable
            | Self::AddressOfAggregateArrayElement
            | Self::StructAggregateArrayField
            | Self::StructElementAggregateArrayField
            | Self::StructPointerAggregateArrayField
            | Self::AggregateLiteralAggregateArrayField => PointerValueKind::Cell,
            Self::EmbeddedAggregateArrayElementAggregateArrayField => PointerValueKind::Leaf,
            Self::TwoDimensionalArrayVariable
            | Self::StructRowArrayField
            | Self::StructElementRowArrayField
            | Self::StructPointerRowArrayField
            | Self::AggregateLiteralRowArrayField
            | Self::EmbeddedAggregateArrayElementRowArrayField => PointerValueKind::Row,
            Self::StringLiteral
            | Self::CharacterPointerOutputDereference
            | Self::CharacterPointerOutputAssignment => PointerValueKind::Char,
            _ => PointerValueKind::Int,
        }
    }

    fn expected_value(self) -> i64 {
        match self {
            Self::DirectTwoDimensionalRow
            | Self::StructTwoDimensionalRow
            | Self::StructElementTwoDimensionalRow
            | Self::StructPointerTwoDimensionalRow
            | Self::RowPointerDereference
            | Self::TwoDimensionalArrayVariable
            | Self::ReverseSubscript => 51,
            Self::AddressOfScalar => 9,
            Self::AddressOfArrayElement
            | Self::GenericSelection
            | Self::PointerCast
            | Self::PointerAssignment
            | Self::PointerCompoundAssignment
            | Self::StructElementFieldAssignment
            | Self::StructElementFieldCompoundAssignment
            | Self::AggregateLiteralField
            | Self::AggregateLiteralFieldAssignment
            | Self::AggregateLiteralFieldCompoundAssignment
            | Self::StructFieldAssignment
            | Self::StructFieldCompoundAssignment
            | Self::StructPointerFieldAssignment
            | Self::StructPointerFieldCompoundAssignment
            | Self::EmbeddedAggregateArrayElementFieldAssignment
            | Self::EmbeddedAggregateArrayElementFieldCompoundAssignment
            | Self::FunctionCall
            | Self::Conditional
            | Self::ConditionalNullLeft
            | Self::ConditionalNullRight
            | Self::Comma
            | Self::PointerAddition
            | Self::PointerSubtraction => 20,
            Self::AddressOfStructField
            | Self::AddressOfStructElementField
            | Self::AddressOfStructPointerField => 31,
            Self::AddressOfStructArrayField
            | Self::AddressOfStructElementArrayField
            | Self::AddressOfStructPointerArrayField => 33,
            Self::AddressOfScalarLiteral => 17,
            Self::AddressOfAggregateLiteral => 61,
            Self::AddressOfAggregateField => 65,
            Self::StringLiteral | Self::CharacterPointerOutputAssignment => 98,
            Self::CharacterPointerOutputDereference => 97,
            Self::ArrayLiteral => 71,
            Self::AggregateArrayLiteral => 74,
            Self::PointerVariable
            | Self::ScalarArrayVariable
            | Self::StructField
            | Self::StructElementField
            | Self::StructPointerField
            | Self::EmbeddedAggregateArrayElementField
            | Self::PointerIncrement => 10,
            Self::AggregateArrayVariable => 41,
            Self::AddressOfAggregateArrayElement => 41,
            Self::StructScalarArrayField
            | Self::StructElementScalarArrayField
            | Self::StructPointerScalarArrayField => 32,
            Self::StructRowArrayField
            | Self::StructElementRowArrayField
            | Self::StructPointerRowArrayField => 81,
            Self::StructAggregateArrayField
            | Self::StructElementAggregateArrayField
            | Self::StructPointerAggregateArrayField => 35,
            Self::AggregateLiteralScalarArrayField => 91,
            Self::AggregateLiteralRowArrayField => 94,
            Self::AggregateLiteralAggregateArrayField => 97,
            Self::EmbeddedAggregateArrayElementScalarArrayField => 36,
            Self::EmbeddedAggregateArrayElementRowArrayField => 111,
            Self::EmbeddedAggregateArrayElementAggregateArrayField => 117,
        }
    }

    fn expected_sequence(self) -> i64 {
        match self {
            Self::DirectTwoDimensionalRow
            | Self::StructTwoDimensionalRow
            | Self::StructPointerTwoDimensionalRow
            | Self::ReverseSubscript => 2,
            Self::StructElementTwoDimensionalRow => 21,
            Self::AddressOfArrayElement
            | Self::AddressOfStructArrayField
            | Self::AddressOfStructPointerArrayField => 3,
            Self::AddressOfStructElementField
            | Self::AddressOfAggregateArrayElement
            | Self::StructElementField
            | Self::StructElementScalarArrayField
            | Self::StructElementRowArrayField
            | Self::StructElementAggregateArrayField
            | Self::StructElementFieldAssignment
            | Self::StructElementFieldCompoundAssignment => 1,
            Self::AddressOfStructElementArrayField => 13,
            Self::EmbeddedAggregateArrayElementField
            | Self::EmbeddedAggregateArrayElementScalarArrayField
            | Self::EmbeddedAggregateArrayElementRowArrayField
            | Self::EmbeddedAggregateArrayElementAggregateArrayField
            | Self::EmbeddedAggregateArrayElementFieldAssignment
            | Self::EmbeddedAggregateArrayElementFieldCompoundAssignment => 4,
            Self::FunctionCall => 5,
            Self::Conditional | Self::ConditionalNullLeft | Self::ConditionalNullRight => 6,
            Self::Comma => 7,
            Self::PointerAddition | Self::PointerSubtraction => 8,
            _ => 0,
        }
    }

    fn expected_size(self) -> &'static str {
        match self {
            Self::DirectTwoDimensionalRow
            | Self::StructTwoDimensionalRow
            | Self::StructElementTwoDimensionalRow
            | Self::StructPointerTwoDimensionalRow
            | Self::RowPointerDereference
            | Self::ReverseSubscript => "3 * sizeof(int)",
            Self::AggregateArrayVariable
            | Self::StructAggregateArrayField
            | Self::StructElementAggregateArrayField
            | Self::StructPointerAggregateArrayField
            | Self::AggregateLiteralAggregateArrayField => "2 * sizeof(struct Cell)",
            Self::EmbeddedAggregateArrayElementAggregateArrayField => "2 * sizeof(struct Leaf)",
            Self::ScalarArrayVariable => "4 * sizeof(int)",
            Self::TwoDimensionalArrayVariable
            | Self::StructRowArrayField
            | Self::StructElementRowArrayField
            | Self::StructPointerRowArrayField
            | Self::AggregateLiteralRowArrayField => "6 * sizeof(int)",
            Self::EmbeddedAggregateArrayElementRowArrayField => "6 * sizeof(int)",
            Self::StructScalarArrayField
            | Self::StructElementScalarArrayField
            | Self::StructPointerScalarArrayField
            | Self::AggregateLiteralScalarArrayField
            | Self::EmbeddedAggregateArrayElementScalarArrayField => "3 * sizeof(int)",
            _ => "sizeof(void *)",
        }
    }

    fn mutation_check(self) -> &'static str {
        match self {
            Self::PointerAssignment | Self::PointerCompoundAssignment | Self::PointerIncrement => {
                "pointer == values + 1"
            }
            Self::StructElementFieldAssignment | Self::StructElementFieldCompoundAssignment => {
                "holders[0].cursor == values + 1"
            }
            Self::StructFieldAssignment | Self::StructFieldCompoundAssignment => {
                "holder.cursor == values + 1"
            }
            Self::StructPointerFieldAssignment | Self::StructPointerFieldCompoundAssignment => {
                "holder.cursor == values + 1"
            }
            Self::EmbeddedAggregateArrayElementFieldAssignment
            | Self::EmbeddedAggregateArrayElementFieldCompoundAssignment => {
                "holder.cells[0].cursor == values + 1"
            }
            Self::CharacterPointerOutputAssignment => "slot == text + 1",
            _ => "1",
        }
    }

    fn non_evaluated_check(self) -> &'static str {
        match self {
            Self::PointerAssignment | Self::PointerCompoundAssignment | Self::PointerIncrement => {
                "pointer == values"
            }
            Self::StructElementFieldAssignment | Self::StructElementFieldCompoundAssignment => {
                "holders[0].cursor == values"
            }
            Self::StructFieldAssignment
            | Self::StructFieldCompoundAssignment
            | Self::StructPointerFieldAssignment
            | Self::StructPointerFieldCompoundAssignment => "holder.cursor == values",
            Self::EmbeddedAggregateArrayElementFieldAssignment
            | Self::EmbeddedAggregateArrayElementFieldCompoundAssignment => {
                "holder.cells[0].cursor == values"
            }
            Self::CharacterPointerOutputAssignment => "slot == text",
            _ => "1",
        }
    }

    fn return_lifetime_error(self) -> Option<&'static str> {
        match self {
            Self::AddressOfScalarLiteral => {
                Some("pointer to out-of-scope variable '__cust_compound_scalar#0'")
            }
            Self::AddressOfAggregateLiteral => {
                Some("pointer to out-of-scope variable '__cust_compound_aggregate#0'")
            }
            Self::AddressOfAggregateField => {
                Some("pointer to out-of-scope variable '__cust_compound_aggregate#0'")
            }
            Self::ArrayLiteral => {
                Some("pointer to out-of-scope variable '__cust_compound_array#0'")
            }
            Self::AggregateArrayLiteral => {
                Some("pointer to out-of-scope variable '__cust_compound_aggregate_array#0'")
            }
            Self::AggregateLiteralScalarArrayField | Self::AggregateLiteralRowArrayField => {
                Some("pointer to out-of-scope variable '__cust_compound_aggregate_field#0'")
            }
            Self::AggregateLiteralAggregateArrayField => {
                Some("pointer to out-of-scope variable '__cust_compound_aggregate_field_array#0'")
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum PointerValueContext {
    Initializer,
    Argument,
    Return,
    Sizeof,
    IncompatibleInitializer,
    IncompatibleArgument,
    IncompatibleReturn,
    IncompatibleAssignment,
}

impl PointerValueContext {
    const COUNT: usize = 8;
    const ALL: [Self; Self::COUNT] = [
        Self::Initializer,
        Self::Argument,
        Self::Return,
        Self::Sizeof,
        Self::IncompatibleInitializer,
        Self::IncompatibleArgument,
        Self::IncompatibleReturn,
        Self::IncompatibleAssignment,
    ];

    fn index(self) -> usize {
        self as usize
    }

    fn program(self, route: PointerValueRoute) -> String {
        let expression = route.expression();
        let kind = route.kind();
        let declaration = kind.declaration("result");
        let return_declaration = kind.return_declaration("route_value");
        let expected = route.expected_value();
        let sequence = route.expected_sequence();
        let mutation_check = route.mutation_check();
        let non_evaluated_check = route.non_evaluated_check();
        let body = match self {
            Self::Initializer => {
                let observed = kind.observe("result");
                format!(
                    "int main(void) {{ {declaration} = {expression};\n\
                     if ({observed} != {expected}) return 1;\n\
                     if (sequence != {sequence}) return 2;\n\
                     if (!({mutation_check})) return 3; return 0; }}"
                )
            }
            Self::Argument => format!(
                "int main(void) {{ int observed = {}({expression});\n\
                 if (observed != {expected}) return 1;\n\
                 if (sequence != {sequence}) return 2;\n\
                 if (!({mutation_check})) return 3; return 0; }}",
                kind.consumer()
            ),
            Self::Return => {
                let observed = kind.observe("result");
                format!(
                    "{return_declaration} {{ return {expression}; }}\n\
                     int main(void) {{ {declaration} = route_value();\n\
                     if ({observed} != {expected}) return 1;\n\
                     if (sequence != {sequence}) return 2;\n\
                     if (!({mutation_check})) return 3; return 0; }}"
                )
            }
            Self::Sizeof => format!(
                "int main(void) {{ if (sizeof({expression}) != {}) return 1;\n\
                 if (sequence != 0) return 2;\n\
                 if (!({non_evaluated_check})) return 3; return 0; }}",
                route.expected_size()
            ),
            Self::IncompatibleInitializer => format!(
                "int main(void) {{ {} = (trap(), {expression}); return result != 0; }}",
                kind.incompatible_declaration()
            ),
            Self::IncompatibleArgument => format!(
                "int main(void) {{ return {}((trap(), {expression})) != 0; }}",
                if matches!(kind, PointerValueKind::Char) {
                    "consume_int"
                } else {
                    "consume_char"
                }
            ),
            Self::IncompatibleReturn => format!(
                "{} {{ return (trap(), {expression}); }}\n\
                 int main(void) {{ return route_value() != 0; }}",
                if matches!(kind, PointerValueKind::Char) {
                    "int *route_value(void)"
                } else {
                    "char *route_value(void)"
                }
            ),
            Self::IncompatibleAssignment => format!(
                "int main(void) {{ {} = 0; result = (trap(), {expression});\n\
                 return result != 0; }}",
                kind.incompatible_declaration()
            ),
        };
        format!("{POINTER_VALUE_PREAMBLE}\n{body}\n")
    }

    fn expected(self, route: PointerValueRoute) -> ExpectedInterpretation {
        match self {
            Self::Return if route.return_lifetime_error().is_some() => {
                ExpectedInterpretation::Error(
                    route
                        .return_lifetime_error()
                        .expect("guarded pointer route lifetime error"),
                )
            }
            Self::Initializer | Self::Argument | Self::Return | Self::Sizeof => {
                ExpectedInterpretation::Value(0)
            }
            Self::IncompatibleInitializer
            | Self::IncompatibleArgument
            | Self::IncompatibleReturn
            | Self::IncompatibleAssignment => {
                ExpectedInterpretation::Error(route.kind().incompatible_error())
            }
        }
    }
}

const POINTER_VALUE_PREAMBLE: &str = r#"
struct Leaf { int value; };
struct Cell {
    int value;
    int values[3];
    int rows[2][3];
    struct Leaf children[2];
    int *cursor;
};
struct Holder {
    int scalar;
    int values[3];
    int rows[2][3];
    struct Cell cells[2];
    int *cursor;
};
struct MatrixHolder { int rows[2][3]; };
int scalar = 9;
int values[4] = {10, 20, 30, 40};
int *pointer = values;
int matrix[2][3] = {{51, 52, 53}, {54, 55, 56}};
int (*row_pointer)[3] = matrix;
struct Cell cells[2] = {
    {.value = 41, .values = {42, 43, 44}, .cursor = values},
    {.value = 45, .values = {46, 47, 48}, .cursor = values}
};
struct Holder holder = {
    .scalar = 31,
    .values = {32, 33, 34},
    .rows = {{81, 82, 83}, {84, 85, 86}},
    .cells = {
        {.value = 35, .values = {36, 37, 38}, .rows = {{111, 112, 113}, {114, 115, 116}}, .children = {{.value = 117}, {.value = 118}}, .cursor = values},
        {.value = 39, .values = {40, 41, 42}, .cursor = values}
    },
    .cursor = values
};
struct Holder holders[2] = {
    {.scalar = 31, .values = {32, 33, 34}, .rows = {{81, 82, 83}, {84, 85, 86}}, .cells = {{.value = 35, .cursor = values}}, .cursor = values},
    {.scalar = 41, .values = {42, 43, 44}, .rows = {{87, 88, 89}, {90, 91, 92}}, .cells = {{.value = 45, .cursor = values}}, .cursor = values}
};
struct Holder *holder_pointer = &holder;
struct MatrixHolder matrix_holder = {{{51, 52, 53}, {54, 55, 56}}};
struct MatrixHolder matrix_holders[2] = {
    {{{51, 52, 53}, {54, 55, 56}}},
    {{{57, 58, 59}, {60, 61, 62}}}
};
struct MatrixHolder *matrix_holder_pointer = &matrix_holder;
char text[] = "abc";
char *slot = text;
char **output = &slot;
int sequence;
int outer_index(void) { sequence = sequence * 10 + 1; return 0; }
int row_index(void) { sequence = sequence * 10 + 2; return 0; }
int element_index(void) { sequence = sequence * 10 + 3; return 1; }
int inner_index(void) { sequence = sequence * 10 + 4; return 0; }
int *make_pointer(void) { sequence = sequence * 10 + 5; return values + 1; }
int condition(void) { sequence = sequence * 10 + 6; return 1; }
int touch(void) { sequence = sequence * 10 + 7; return 0; }
int offset(void) { sequence = sequence * 10 + 8; return 1; }
int trap(void) { return 1 / 0; }
int consume_int(int *value) { return *value; }
int consume_char(char *value) { return *value; }
int consume_cell(struct Cell *value) { return value->value; }
int consume_leaf(struct Leaf *value) { return value->value; }
int consume_row(int (*value)[3]) { return (*value)[0]; }
"#;

#[test]
fn generated_pointer_classifier_and_evaluator_routes_stay_in_parity() {
    let mut route_counts = [0; PointerValueRoute::COUNT];
    let mut context_counts = [0; PointerValueContext::COUNT];
    let mut cell_counts = [0; PointerValueRoute::COUNT * PointerValueContext::COUNT];

    for route in PointerValueRoute::ALL {
        for context in PointerValueContext::ALL {
            let route_index = route.index();
            let context_index = context.index();
            route_counts[route_index] += 1;
            context_counts[context_index] += 1;
            cell_counts[route_index * PointerValueContext::COUNT + context_index] += 1;
            assert_interpretation(
                &context.program(route),
                context.expected(route),
                &format!("pointer classifier/evaluator route {route:?}, context {context:?}"),
            );
        }
    }

    assert_eq!(route_counts, [8; 69]);
    assert_eq!(context_counts, [69; 8]);
    assert!(cell_counts.into_iter().all(|count| count == 1));

    let mut const_route_counts = [0; 7];
    for (route_index, (name, helper, expression)) in [
        ("array decay", "", "const_values"),
        ("aggregate array element address", "", "&const_cells[0]"),
        (
            "generic selection",
            "",
            "_Generic(const_values, const int *: const_values)",
        ),
        (
            "conditional",
            "",
            "condition() ? const_values : const_values + 1",
        ),
        ("comma", "", "(touch(), const_values)"),
        ("addition", "", "const_values + 1"),
        (
            "function call",
            "const int *make_const_pointer(void) { return const_values; }",
            "make_const_pointer()",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        const_route_counts[route_index] += 1;
        let source = format!(
            "struct ConstCell {{ int value; }};\n\
             const int const_values[2] = {{1, 2}};\n\
             const struct ConstCell const_cells[1] = {{{{1}}}}; int sequence;\n\
             int condition(void) {{ sequence += 1; return 1; }}\n\
             int touch(void) {{ sequence += 1; return 0; }}\n\
             {helper}\n\
             int main(void) {{ int *result = {expression}; return result != 0; }}"
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot discard const qualifier from pointer target"),
            &format!("pointer classifier/evaluator const route {name}"),
        );
    }
    assert_eq!(const_route_counts, [1; 7]);
}

#[test]
fn two_dimensional_rows_convert_to_void_pointers_across_value_boundaries() {
    let source = r#"
int matrix[2][3] = {{1, 2, 3}, {4, 5, 6}};
void *saved;
void *forward(void *value) { return value; }
void *return_row(void) { return matrix; }
int main(void) {
    void *initialized = matrix;
    void *assigned = 0;
    assigned = matrix;
    saved = forward(matrix);
    if (initialized == 0 || assigned == 0 || saved == 0 || return_row() == 0) {
        return 1;
    }
    return 0;
}
"#;

    assert_interpretation(
        source,
        ExpectedInterpretation::Value(0),
        "two-dimensional row to void pointer conversions",
    );
}

#[test]
fn static_pointer_returning_calls_classify_without_evaluating_the_body() {
    let source = r#"
int sequence;
int values[2] = {7, 8};
static int *make_pointer(void) {
    sequence += 1;
    return values;
}
int main(void) {
    if (sizeof(make_pointer()) != sizeof(int *)) {
        return 1;
    }
    return sequence;
}
"#;

    assert_interpretation(
        source,
        ExpectedInterpretation::Value(0),
        "static pointer-returning function call classification",
    );
}

#[test]
fn pointer_assignment_expression_validates_rhs_before_evaluation() {
    let source = r#"
int values[1] = {1};
char chars[1] = {2};
int trap(void) { return 1 / 0; }
int consume(int *value) { return *value; }
int main(void) {
    int *pointer = values;
    return consume(pointer = (trap(), chars));
}
"#;

    assert_interpretation(
        source,
        ExpectedInterpretation::Error("cannot convert pointer to char to pointer to int"),
        "pointer assignment expression RHS validation",
    );
}

#[test]
fn static_pointer_initializer_validates_type_before_evaluation() {
    let source = r#"
int values[1] = {1};
char chars[1] = {2};
int trap(void) { return 1 / 0; }
int main(void) {
    static int *result = (trap(), chars);
    return result != 0;
}
"#;

    assert_interpretation(
        source,
        ExpectedInterpretation::Error("cannot convert pointer to char to pointer to int"),
        "static pointer initializer pre-evaluation type validation",
    );
}

#[test]
fn pointer_field_assignment_expressions_validate_rhs_before_evaluation() {
    for (name, assignment) in [
        ("direct field", "holder.cursor = (trap(), text)"),
        (
            "aggregate-array element field",
            "holders[outer_index()].cursor = (trap(), text)",
        ),
        (
            "struct-pointer field",
            "holder_pointer->cursor = (trap(), text)",
        ),
        (
            "aggregate literal field",
            "((struct Holder){.cursor = values}).cursor = (trap(), text)",
        ),
        (
            "embedded aggregate-array element field",
            "holder.cells[inner_index()].cursor = (trap(), text)",
        ),
    ] {
        let source = format!(
            "{POINTER_VALUE_PREAMBLE}\nint main(void) {{ return consume_int({assignment}); }}\n"
        );
        assert_interpretation(
            &source,
            ExpectedInterpretation::Error("cannot convert pointer to char to pointer to int"),
            &format!("{name} assignment RHS validation"),
        );
    }
}
