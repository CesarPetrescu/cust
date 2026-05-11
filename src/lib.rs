use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::rc::Rc;

pub type CustResult<T> = Result<T, CustError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustError {
    message: String,
}

impl CustError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for CustError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustError {}

const INT_SIZE: i64 = 8;
const CHAR_SIZE: i64 = 1;
const POINTER_SIZE: i64 = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Int,
    Char,
    Bool,
    Signed,
    Unsigned,
    Long,
    Short,
    Const,
    Volatile,
    Restrict,
    Atomic,
    Static,
    Extern,
    ThreadLocal,
    Inline,
    Noreturn,
    Auto,
    Register,
    Void,
    Enum,
    Struct,
    Union,
    Typedef,
    Sizeof,
    Alignof,
    Alignas,
    StaticAssert,
    Return,
    If,
    Else,
    While,
    Do,
    For,
    Switch,
    Case,
    Default,
    Break,
    Continue,
    Ident(String),
    Number(i64),
    StringLiteral(Vec<i64>),
    Plus,
    Minus,
    PlusPlus,
    MinusMinus,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    AmpAssign,
    PipeAssign,
    CaretAssign,
    ShiftLeftAssign,
    ShiftRightAssign,
    Star,
    Slash,
    Percent,
    Amp,
    AndAnd,
    Pipe,
    OrOr,
    Caret,
    Tilde,
    Bang,
    Assign,
    Eq,
    Ne,
    Lt,
    Le,
    ShiftLeft,
    Gt,
    Ge,
    ShiftRight,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semi,
    Dot,
    Arrow,
    Question,
    Colon,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LocatedToken {
    kind: Token,
    line: usize,
    column: usize,
}

impl LocatedToken {
    fn new(kind: Token, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number(i64),
    StringLiteral(Vec<i64>),
    SizeOfType(SizeOfType),
    SizeOfValue(Box<Expr>),
    AlignOfType(SizeOfType),
    Var(String),
    StructGet {
        name: String,
        fields: Vec<String>,
    },
    StructPtrGet {
        pointer: Box<Expr>,
        fields: Vec<String>,
    },
    StructPtrArrayGet {
        pointer: Box<Expr>,
        fields: Vec<String>,
        index: Box<Expr>,
    },
    StructArrayGet {
        name: String,
        fields: Vec<String>,
        index: Box<Expr>,
    },
    StructFieldArrayElementGet {
        name: String,
        array_fields: Vec<String>,
        index: Box<Expr>,
        fields: Vec<String>,
    },
    StructFieldArrayElementSet {
        name: String,
        array_fields: Vec<String>,
        index: Box<Expr>,
        fields: Vec<String>,
        value: Box<Expr>,
    },
    StructFieldArrayElementCompoundSet {
        name: String,
        array_fields: Vec<String>,
        index: Box<Expr>,
        fields: Vec<String>,
        op: CompoundOp,
        value: Box<Expr>,
    },
    StructElementGet {
        name: String,
        index: Box<Expr>,
        fields: Vec<String>,
    },
    StructElementArrayGet {
        name: String,
        index: Box<Expr>,
        fields: Vec<String>,
        array_index: Box<Expr>,
    },
    ArrayGet {
        name: String,
        index: Box<Expr>,
    },
    StringGet {
        values: Vec<i64>,
        index: Box<Expr>,
    },
    AddressOf(String),
    AddressOfArray {
        name: String,
        index: Box<Expr>,
    },
    AddressOfStructField {
        name: String,
        fields: Vec<String>,
    },
    AddressOfStructElementField {
        name: String,
        index: Box<Expr>,
        fields: Vec<String>,
    },
    AddressOfStructArrayField {
        name: String,
        fields: Vec<String>,
        index: Box<Expr>,
    },
    AddressOfStructElementArrayField {
        name: String,
        index: Box<Expr>,
        fields: Vec<String>,
        array_index: Box<Expr>,
    },
    AddressOfStructPtrArrayField {
        pointer: Box<Expr>,
        fields: Vec<String>,
        index: Box<Expr>,
    },
    AddressOfStructPtrField {
        pointer: Box<Expr>,
        fields: Vec<String>,
    },
    Deref(Box<Expr>),
    Assign {
        name: String,
        value: Box<Expr>,
    },
    ArraySet {
        name: String,
        index: Box<Expr>,
        value: Box<Expr>,
    },
    DerefSet {
        pointer: Box<Expr>,
        value: Box<Expr>,
    },
    StructSet {
        name: String,
        fields: Vec<String>,
        value: Box<Expr>,
    },
    StructPtrSet {
        pointer: Box<Expr>,
        fields: Vec<String>,
        value: Box<Expr>,
    },
    StructArraySet {
        name: String,
        fields: Vec<String>,
        index: Box<Expr>,
        value: Box<Expr>,
    },
    StructElementSet {
        name: String,
        index: Box<Expr>,
        fields: Vec<String>,
        value: Box<Expr>,
    },
    StructElementArraySet {
        name: String,
        index: Box<Expr>,
        fields: Vec<String>,
        array_index: Box<Expr>,
        value: Box<Expr>,
    },
    CompoundAssign {
        name: String,
        op: CompoundOp,
        value: Box<Expr>,
    },
    ArrayCompoundSet {
        name: String,
        index: Box<Expr>,
        op: CompoundOp,
        value: Box<Expr>,
    },
    DerefCompoundSet {
        pointer: Box<Expr>,
        op: CompoundOp,
        value: Box<Expr>,
    },
    StructCompoundSet {
        name: String,
        fields: Vec<String>,
        op: CompoundOp,
        value: Box<Expr>,
    },
    StructPtrCompoundSet {
        pointer: Box<Expr>,
        fields: Vec<String>,
        op: CompoundOp,
        value: Box<Expr>,
    },
    StructArrayCompoundSet {
        name: String,
        fields: Vec<String>,
        index: Box<Expr>,
        op: CompoundOp,
        value: Box<Expr>,
    },
    StructElementCompoundSet {
        name: String,
        index: Box<Expr>,
        fields: Vec<String>,
        op: CompoundOp,
        value: Box<Expr>,
    },
    StructElementArrayCompoundSet {
        name: String,
        index: Box<Expr>,
        fields: Vec<String>,
        array_index: Box<Expr>,
        op: CompoundOp,
        value: Box<Expr>,
    },
    Increment {
        target: Box<Expr>,
        op: IncrementOp,
        prefix: bool,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Cast {
        ty: CType,
        expr: Box<Expr>,
    },
    ScalarLiteral {
        ty: CType,
        init: Box<Expr>,
    },
    ScalarLiteralSet {
        ty: CType,
        init: Box<Expr>,
        value: Box<Expr>,
    },
    ScalarLiteralCompoundSet {
        ty: CType,
        init: Box<Expr>,
        op: CompoundOp,
        value: Box<Expr>,
    },
    AddressOfScalarLiteral {
        ty: CType,
        init: Box<Expr>,
    },
    AggregateLiteral {
        type_name: String,
        init: Vec<StructInitializer>,
    },
    AddressOfAggregateLiteral {
        type_name: String,
        init: Vec<StructInitializer>,
    },
    AddressOfAggregateField {
        aggregate: Box<Expr>,
        fields: Vec<String>,
    },
    ArrayLiteral {
        elem_type: CType,
        len: Option<usize>,
        init: Vec<ArrayInitializer>,
    },
    AggregateArrayLiteral {
        type_name: String,
        len: Option<usize>,
        init: Vec<StructArrayInitializer>,
    },
    AggregateFieldGet {
        aggregate: Box<Expr>,
        fields: Vec<String>,
    },
    AggregateFieldSet {
        aggregate: Box<Expr>,
        fields: Vec<String>,
        value: Box<Expr>,
    },
    AggregateFieldCompoundSet {
        aggregate: Box<Expr>,
        fields: Vec<String>,
        op: CompoundOp,
        value: Box<Expr>,
    },
    UnaryPlus(Box<Expr>),
    UnaryMinus(Box<Expr>),
    BitwiseNot(Box<Expr>),
    LogicalNot(Box<Expr>),
    Conditional {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    Comma(Box<Expr>, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Program {
    globals: Vec<Stmt>,
    functions: HashMap<String, Function>,
    struct_types: HashMap<String, StructTypeDef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypeAlias {
    Scalar(CType),
    Struct(String),
    Pointer {
        pointee: PointeeType,
        points_to_const: bool,
    },
    Array(PointeeType, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DeclType {
    Scalar(CType),
    Struct(String),
    Pointer {
        pointee: PointeeType,
        points_to_const: bool,
    },
    Array(PointeeType, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructTypeDef {
    fields: Vec<StructFieldDef>,
    kind: AggregateKind,
    display_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AggregateKind {
    Struct,
    Union,
}

impl AggregateKind {
    fn keyword(self) -> &'static str {
        match self {
            AggregateKind::Struct => "struct",
            AggregateKind::Union => "union",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructFieldDef {
    name: String,
    ty: StructFieldType,
    is_const: bool,
    points_to_const: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StructFieldType {
    Scalar(CType),
    Array(CType, usize),
    Struct(String),
    StructArray(String, usize),
    Pointer(PointeeType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StructInitializer {
    Expr(Expr),
    Array(Vec<ArrayInitializer>),
    Struct(Vec<StructInitializer>),
    StructArray(Vec<StructArrayInitializer>),
    Designated {
        field: String,
        value: Box<StructInitializer>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StructVarInitializer {
    Fields(Vec<StructInitializer>),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StructArrayInitializer {
    Element(Vec<StructInitializer>),
    Designated {
        index: usize,
        value: Vec<StructInitializer>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ArrayInitializer {
    Expr(Expr),
    Designated { index: usize, value: Expr },
    StringLiteral(Vec<i64>),
}

impl StructFieldType {
    fn size(&self, struct_types: &HashMap<String, StructTypeDef>) -> CustResult<i64> {
        match self {
            StructFieldType::Scalar(ty) => Ok(ty.size()),
            StructFieldType::Array(ty, len) => Ok(*len as i64 * ty.size()),
            StructFieldType::Struct(type_name) => struct_types
                .get(type_name)
                .map(|struct_type| struct_type.size(struct_types))
                .transpose()?
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
            StructFieldType::StructArray(type_name, len) => struct_types
                .get(type_name)
                .map(|struct_type| {
                    struct_type
                        .size(struct_types)
                        .map(|size| size * *len as i64)
                })
                .transpose()?
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
            StructFieldType::Pointer(_) => Ok(POINTER_SIZE),
        }
    }

    fn alignment(&self, struct_types: &HashMap<String, StructTypeDef>) -> CustResult<i64> {
        match self {
            StructFieldType::Scalar(ty) => Ok(ty.alignment()),
            StructFieldType::Array(ty, _) => Ok(ty.alignment()),
            StructFieldType::Struct(type_name) | StructFieldType::StructArray(type_name, _) => {
                struct_types
                    .get(type_name)
                    .map(|struct_type| struct_type.alignment(struct_types))
                    .transpose()?
                    .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'")))
            }
            StructFieldType::Pointer(_) => Ok(POINTER_SIZE),
        }
    }
}

impl StructTypeDef {
    fn size(&self, struct_types: &HashMap<String, StructTypeDef>) -> CustResult<i64> {
        match self.kind {
            AggregateKind::Struct => self
                .fields
                .iter()
                .map(|field| field.ty.size(struct_types))
                .try_fold(0, |sum, size| size.map(|size| sum + size)),
            AggregateKind::Union => self
                .fields
                .iter()
                .map(|field| field.ty.size(struct_types))
                .try_fold(0, |max, size| size.map(|size| max.max(size))),
        }
    }

    fn alignment(&self, struct_types: &HashMap<String, StructTypeDef>) -> CustResult<i64> {
        self.fields
            .iter()
            .map(|field| field.ty.alignment(struct_types))
            .try_fold(1, |max, alignment| {
                alignment.map(|alignment| max.max(alignment))
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Function {
    return_type: ReturnType,
    params: Vec<Param>,
    body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FunctionSignature {
    return_type: ReturnType,
    params: Vec<ParamSignature>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParamSignature {
    ty: ParamType,
    kind: ParamKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TopLevelFunction {
    Definition(Function),
    Prototype(FunctionSignature),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReturnType {
    Scalar(CType),
    Pointer {
        ty: PointeeType,
        points_to_const: bool,
    },
    Struct(String),
    Void,
}

impl ReturnType {
    fn value_return_label(&self) -> &'static str {
        match self {
            ReturnType::Scalar(CType::Int) => "int",
            ReturnType::Scalar(CType::Char) => "char",
            ReturnType::Scalar(CType::Bool) => "_Bool",
            ReturnType::Pointer { .. } => "pointer",
            ReturnType::Struct(_) => "struct",
            ReturnType::Void => "void",
        }
    }

    fn size(&self, struct_types: &HashMap<String, StructTypeDef>) -> Option<i64> {
        match self {
            ReturnType::Scalar(ty) => Some(ty.size()),
            ReturnType::Pointer { .. } => Some(POINTER_SIZE),
            ReturnType::Struct(type_name) => struct_types
                .get(type_name)
                .map(|struct_type| struct_type.size(struct_types))
                .transpose()
                .ok()
                .flatten(),
            ReturnType::Void => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CType {
    Int,
    Char,
    Bool,
}

impl CType {
    fn size(self) -> i64 {
        match self {
            CType::Int => INT_SIZE,
            CType::Char => CHAR_SIZE,
            CType::Bool => CHAR_SIZE,
        }
    }

    fn alignment(self) -> i64 {
        self.size()
    }
}

impl PointeeType {
    fn size(&self, struct_types: &HashMap<String, StructTypeDef>) -> CustResult<i64> {
        match self {
            PointeeType::Scalar(ty) => Ok(ty.size()),
            PointeeType::Struct(type_name) => struct_types
                .get(type_name)
                .map(|struct_type| struct_type.size(struct_types))
                .transpose()?
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
        }
    }

    fn alignment(&self, struct_types: &HashMap<String, StructTypeDef>) -> CustResult<i64> {
        match self {
            PointeeType::Scalar(ty) => Ok(ty.alignment()),
            PointeeType::Struct(type_name) => struct_types
                .get(type_name)
                .map(|struct_type| struct_type.alignment(struct_types))
                .transpose()?
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SizeOfType {
    Scalar(CType),
    Struct(String),
    Pointer,
    Array(PointeeType, usize),
}

impl SizeOfType {
    fn size(&self, struct_types: &HashMap<String, StructTypeDef>) -> CustResult<i64> {
        match self {
            SizeOfType::Scalar(ty) => Ok(ty.size()),
            SizeOfType::Struct(type_name) => struct_types
                .get(type_name)
                .map(|struct_type| struct_type.size(struct_types))
                .transpose()?
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
            SizeOfType::Pointer => Ok(POINTER_SIZE),
            SizeOfType::Array(element_type, len) => {
                let len =
                    i64::try_from(*len).map_err(|_| CustError::new("array length is too large"))?;
                Ok(element_type.size(struct_types)? * len)
            }
        }
    }

    fn alignment(&self, struct_types: &HashMap<String, StructTypeDef>) -> CustResult<i64> {
        match self {
            SizeOfType::Scalar(ty) => Ok(ty.alignment()),
            SizeOfType::Struct(type_name) => struct_types
                .get(type_name)
                .map(|struct_type| struct_type.alignment(struct_types))
                .transpose()?
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
            SizeOfType::Pointer => Ok(POINTER_SIZE),
            SizeOfType::Array(element_type, _) => element_type.alignment(struct_types),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Param {
    ty: ParamType,
    name: String,
    kind: ParamKind,
    is_const: bool,
    points_to_const: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParamType {
    Scalar(CType),
    Struct(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PointeeType {
    Scalar(CType),
    Struct(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParamKind {
    Scalar,
    Pointer,
    Struct,
}

impl FunctionSignature {
    fn new(return_type: ReturnType, params: &[Param]) -> Self {
        Self {
            return_type,
            params: params
                .iter()
                .map(|param| ParamSignature {
                    ty: param.ty.clone(),
                    kind: param.kind,
                })
                .collect(),
        }
    }

    fn from_function(function: &Function) -> Self {
        Self::new(function.return_type.clone(), &function.params)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    ShiftLeft,
    ShiftRight,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    BitAnd,
    BitXor,
    BitOr,
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompoundOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IncrementOp {
    Inc,
    Dec,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Stmt {
    Empty,
    StaticLocal {
        id: usize,
        decl: Box<Stmt>,
    },
    VarDecl {
        name: String,
        ty: CType,
        expr: Expr,
        is_const: bool,
    },
    PointerDecl {
        name: String,
        ty: PointeeType,
        expr: Expr,
        is_const: bool,
        points_to_const: bool,
    },
    ArrayDecl {
        name: String,
        elem_type: CType,
        len: usize,
        init: Vec<ArrayInitializer>,
        is_const: bool,
    },
    StructVarDecl {
        type_name: String,
        name: String,
        init: Option<StructVarInitializer>,
        is_const: bool,
    },
    StructArrayDecl {
        type_name: String,
        name: String,
        len: usize,
        init: Vec<StructArrayInitializer>,
        is_const: bool,
    },
    EnumDecl {
        constants: Vec<EnumConstant>,
    },
    StaticAssert {
        condition: Expr,
        message: String,
    },
    Assign(String, Expr),
    DerefAssign {
        pointer: Expr,
        value: Expr,
    },
    ArrayAssign {
        name: String,
        index: Expr,
        value: Expr,
    },
    StructAssign {
        name: String,
        fields: Vec<String>,
        value: Expr,
    },
    Expr(Expr),
    Return(Option<Expr>),
    Break,
    Continue,
    Block(Vec<Stmt>),
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Vec<Stmt>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    DoWhile {
        body: Vec<Stmt>,
        cond: Expr,
    },
    For {
        init: Option<Box<Stmt>>,
        cond: Option<Expr>,
        increment: Option<Box<Stmt>>,
        body: Vec<Stmt>,
    },
    Switch {
        expr: Expr,
        sections: Vec<SwitchSection>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SwitchSection {
    label: SwitchLabel,
    statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SwitchLabel {
    Case(i64),
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EnumConstant {
    name: String,
    value: i64,
}

/// Interpret a small, safe C subset and return `main()`'s integer exit value.
///
/// Supported v0.1 syntax:
/// - `int main() { ... }` or `int main(void) { ... }`
/// - top-level `int`/`char` scalar, array, and pointer globals initialized before `main()`
/// - `int name(int param, char param, struct Point param, ...) { ... }` plus empty `void` parameter lists for function definitions/prototypes, including bounded recursion
/// - `int name = expression;` and `char name = expression;`
/// - `name = expression;`
/// - `return expression;`
/// - block statements: `{ ... }`
/// - `if (expression) statement else statement` with braced blocks, single-statement bodies, and `else if`
/// - `while (expression) statement`
/// - `do statement while (expression);`
/// - `for (initializer; condition; increment) statement`
/// - `switch (expression) { case constant: ... default: ... }` with C-style fallthrough
/// - `break;` and `continue;` inside loops
/// - empty statements (`;`) and side-effect-free expression statements (`expr;`)
/// - integer, character, and string literals (string literals are read-only NUL-terminated byte arrays)
/// - integer arithmetic/comparisons/logical operators: `+ - * / % == != < <= > >= && || !`
/// - pointer truthiness and pointer equality/inequality for null, scalar pointers, and array-backed pointers
/// - line comments (`// ...`) and C-style block comments (`/* ... */`)
pub fn interpret(source: &str) -> CustResult<i64> {
    interpret_with_options(source, InterpretOptions::default())
}

/// Execution options for interpreting a Cust source program.
#[derive(Debug, Clone, Copy, Default)]
pub struct InterpretOptions {
    pub max_loop_iterations: Option<usize>,
}

/// Interpret a Cust source program with explicit execution options.
pub fn interpret_with_options(source: &str, options: InterpretOptions) -> CustResult<i64> {
    let tokens = lex(source)?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let mut interpreter = Interpreter::new(options);
    interpreter.run(&program)
}

/// Format the lexer token stream for a Cust source program.
///
/// This powers the `cust --tokens <file.c>` CLI inspection mode. It intentionally
/// stops after lexing, so runtime errors such as division by zero are not
/// evaluated while inspecting tokens.
pub fn format_tokens(source: &str) -> CustResult<String> {
    let tokens = lex(source)?;
    Ok(tokens
        .into_iter()
        .map(|token| format!("{}:{} {:?}", token.line, token.column, token.kind))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n")
}

/// Format the parsed AST with deterministic function ordering.
///
/// This powers the `cust --ast <file.c>` CLI inspection mode. It intentionally
/// stops after parsing, so runtime errors such as division by zero are not
/// evaluated while inspecting the syntax tree.
pub fn format_ast(source: &str) -> CustResult<String> {
    let tokens = lex(source)?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let mut names = program.functions.keys().collect::<Vec<_>>();
    names.sort();

    let mut output = String::new();
    if !program.globals.is_empty() {
        output.push_str(&format!("globals: {:?}\n", program.globals));
    }
    for name in names {
        let function = &program.functions[name];
        output.push_str(&format!("function {name}\n"));
        output.push_str(&format!("  params: {:?}\n", function.params));
        output.push_str(&format!("  body: {:?}\n", function.body));
    }
    Ok(output)
}

fn lex(source: &str) -> CustResult<Vec<LocatedToken>> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut line = 1usize;
    let mut column = 1usize;

    while i < chars.len() {
        let c = chars[i];
        match c {
            c if c.is_whitespace() => advance_position(c, &mut line, &mut column, &mut i),
            '/' if chars.get(i + 1) == Some(&'/') => {
                advance_position('/', &mut line, &mut column, &mut i);
                advance_position('/', &mut line, &mut column, &mut i);
                while i < chars.len() && chars[i] != '\n' {
                    advance_position(chars[i], &mut line, &mut column, &mut i);
                }
            }
            '/' if chars.get(i + 1) == Some(&'*') => {
                let start_line = line;
                let start_column = column;
                advance_position('/', &mut line, &mut column, &mut i);
                advance_position('*', &mut line, &mut column, &mut i);

                let mut closed = false;
                while i < chars.len() {
                    if chars[i] == '*' && chars.get(i + 1) == Some(&'/') {
                        advance_position('*', &mut line, &mut column, &mut i);
                        advance_position('/', &mut line, &mut column, &mut i);
                        closed = true;
                        break;
                    }
                    advance_position(chars[i], &mut line, &mut column, &mut i);
                }

                if !closed {
                    return Err(lexer_error_with_context(
                        "unterminated block comment",
                        source,
                        start_line,
                        start_column,
                    ));
                }
            }
            '0'..='9' => {
                let start = i;
                let start_line = line;
                let start_column = column;
                let (digits_start, radix) = if chars[i] == '0'
                    && matches!(chars.get(i + 1), Some('x') | Some('X'))
                {
                    advance_position('0', &mut line, &mut column, &mut i);
                    let prefix = chars[i];
                    advance_position(prefix, &mut line, &mut column, &mut i);
                    let digits_start = i;
                    while i < chars.len() && chars[i].is_ascii_hexdigit() {
                        advance_position(chars[i], &mut line, &mut column, &mut i);
                    }
                    if digits_start == i {
                        return Err(lexer_error_with_context(
                            "expected hexadecimal digits after integer literal prefix",
                            source,
                            start_line,
                            start_column,
                        ));
                    }
                    (digits_start, 16)
                } else if chars[i] == '0' {
                    advance_position('0', &mut line, &mut column, &mut i);
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        if !matches!(chars[i], '0'..='7') {
                            return Err(lexer_error_with_context(
                                format!("invalid digit '{}' in octal integer literal", chars[i]),
                                source,
                                start_line,
                                start_column,
                            ));
                        }
                        advance_position(chars[i], &mut line, &mut column, &mut i);
                    }
                    (start, 8)
                } else {
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        advance_position(chars[i], &mut line, &mut column, &mut i);
                    }
                    (start, 10)
                };
                let text: String = chars[digits_start..i].iter().collect();
                consume_integer_suffix(&chars, &mut line, &mut column, &mut i);
                let value = i64::from_str_radix(&text, radix).map_err(|_| {
                    lexer_error_with_context(
                        "integer literal out of range",
                        source,
                        start_line,
                        start_column,
                    )
                })?;
                tokens.push(LocatedToken::new(
                    Token::Number(value),
                    start_line,
                    start_column,
                ));
            }
            '"' => {
                let start_line = line;
                let start_column = column;
                advance_position('"', &mut line, &mut column, &mut i);
                let mut values = Vec::new();

                loop {
                    let Some(next) = chars.get(i).copied() else {
                        return Err(lexer_error_with_context(
                            "unterminated string literal",
                            source,
                            start_line,
                            start_column,
                        ));
                    };
                    match next {
                        '"' => {
                            advance_position('"', &mut line, &mut column, &mut i);
                            values.push('\0' as i64);
                            tokens.push(LocatedToken::new(
                                Token::StringLiteral(values),
                                start_line,
                                start_column,
                            ));
                            break;
                        }
                        '\\' => {
                            advance_position('\\', &mut line, &mut column, &mut i);
                            let escaped = parse_escape_sequence(
                                source,
                                &chars,
                                &mut line,
                                &mut column,
                                &mut i,
                                (start_line, start_column),
                                "string",
                            )?;
                            values.push(escaped);
                        }
                        '\n' => {
                            return Err(lexer_error_with_context(
                                "unterminated string literal",
                                source,
                                start_line,
                                start_column,
                            ));
                        }
                        value => {
                            advance_position(value, &mut line, &mut column, &mut i);
                            values.push(value as i64);
                        }
                    }
                }
            }
            '\'' => {
                let start_line = line;
                let start_column = column;
                advance_position('\'', &mut line, &mut column, &mut i);

                let value = match chars.get(i).copied() {
                    Some('\\') => {
                        advance_position('\\', &mut line, &mut column, &mut i);
                        parse_escape_sequence(
                            source,
                            &chars,
                            &mut line,
                            &mut column,
                            &mut i,
                            (start_line, start_column),
                            "character",
                        )?
                    }
                    Some('\n') | None => {
                        return Err(lexer_error_with_context(
                            "unterminated character literal",
                            source,
                            start_line,
                            start_column,
                        ));
                    }
                    Some(value) => {
                        advance_position(value, &mut line, &mut column, &mut i);
                        value as i64
                    }
                };

                if chars.get(i) != Some(&'\'') {
                    return Err(lexer_error_with_context(
                        "unterminated character literal",
                        source,
                        start_line,
                        start_column,
                    ));
                }
                advance_position('\'', &mut line, &mut column, &mut i);
                tokens.push(LocatedToken::new(
                    Token::Number(value),
                    start_line,
                    start_column,
                ));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = i;
                let start_line = line;
                let start_column = column;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    advance_position(chars[i], &mut line, &mut column, &mut i);
                }
                let text: String = chars[start..i].iter().collect();
                let kind = match text.as_str() {
                    "int" => Token::Int,
                    "char" => Token::Char,
                    "_Bool" => Token::Bool,
                    "signed" => Token::Signed,
                    "unsigned" => Token::Unsigned,
                    "long" => Token::Long,
                    "short" => Token::Short,
                    "const" => Token::Const,
                    "volatile" => Token::Volatile,
                    "restrict" => Token::Restrict,
                    "_Atomic" => Token::Atomic,
                    "static" => Token::Static,
                    "extern" => Token::Extern,
                    "_Thread_local" => Token::ThreadLocal,
                    "inline" => Token::Inline,
                    "_Noreturn" => Token::Noreturn,
                    "auto" => Token::Auto,
                    "register" => Token::Register,
                    "void" => Token::Void,
                    "enum" => Token::Enum,
                    "struct" => Token::Struct,
                    "union" => Token::Union,
                    "typedef" => Token::Typedef,
                    "sizeof" => Token::Sizeof,
                    "_Alignof" => Token::Alignof,
                    "_Alignas" => Token::Alignas,
                    "_Static_assert" => Token::StaticAssert,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "do" => Token::Do,
                    "for" => Token::For,
                    "switch" => Token::Switch,
                    "case" => Token::Case,
                    "default" => Token::Default,
                    "break" => Token::Break,
                    "continue" => Token::Continue,
                    _ => Token::Ident(text),
                };
                tokens.push(LocatedToken::new(kind, start_line, start_column));
            }
            '+' if chars.get(i + 1) == Some(&'+') => {
                push_token(&mut tokens, Token::PlusPlus, line, column);
                advance_position('+', &mut line, &mut column, &mut i);
                advance_position('+', &mut line, &mut column, &mut i);
            }
            '+' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::PlusAssign, line, column);
                advance_position('+', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '+' => {
                push_token(&mut tokens, Token::Plus, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '-' if chars.get(i + 1) == Some(&'-') => {
                push_token(&mut tokens, Token::MinusMinus, line, column);
                advance_position('-', &mut line, &mut column, &mut i);
                advance_position('-', &mut line, &mut column, &mut i);
            }
            '-' if chars.get(i + 1) == Some(&'>') => {
                push_token(&mut tokens, Token::Arrow, line, column);
                advance_position('-', &mut line, &mut column, &mut i);
                advance_position('>', &mut line, &mut column, &mut i);
            }
            '-' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::MinusAssign, line, column);
                advance_position('-', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '-' => {
                push_token(&mut tokens, Token::Minus, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '*' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::StarAssign, line, column);
                advance_position('*', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '*' => {
                push_token(&mut tokens, Token::Star, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '/' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::SlashAssign, line, column);
                advance_position('/', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '/' => {
                push_token(&mut tokens, Token::Slash, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '%' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::PercentAssign, line, column);
                advance_position('%', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '%' => {
                push_token(&mut tokens, Token::Percent, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '&' if chars.get(i + 1) == Some(&'&') => {
                push_token(&mut tokens, Token::AndAnd, line, column);
                advance_position('&', &mut line, &mut column, &mut i);
                advance_position('&', &mut line, &mut column, &mut i);
            }
            '&' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::AmpAssign, line, column);
                advance_position('&', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '&' => {
                push_token(&mut tokens, Token::Amp, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '|' if chars.get(i + 1) == Some(&'|') => {
                push_token(&mut tokens, Token::OrOr, line, column);
                advance_position('|', &mut line, &mut column, &mut i);
                advance_position('|', &mut line, &mut column, &mut i);
            }
            '|' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::PipeAssign, line, column);
                advance_position('|', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '|' => {
                push_token(&mut tokens, Token::Pipe, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '^' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::CaretAssign, line, column);
                advance_position('^', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '^' => {
                push_token(&mut tokens, Token::Caret, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '~' => {
                push_token(&mut tokens, Token::Tilde, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '(' => {
                push_token(&mut tokens, Token::LParen, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            ')' => {
                push_token(&mut tokens, Token::RParen, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '[' => {
                push_token(&mut tokens, Token::LBracket, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            ']' => {
                push_token(&mut tokens, Token::RBracket, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '{' => {
                push_token(&mut tokens, Token::LBrace, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '}' => {
                push_token(&mut tokens, Token::RBrace, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            ',' => {
                push_token(&mut tokens, Token::Comma, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            ';' => {
                push_token(&mut tokens, Token::Semi, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '.' => {
                push_token(&mut tokens, Token::Dot, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '?' => {
                push_token(&mut tokens, Token::Question, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            ':' => {
                push_token(&mut tokens, Token::Colon, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '=' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::Eq, line, column);
                advance_position('=', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '!' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::Ne, line, column);
                advance_position('!', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '!' => {
                push_token(&mut tokens, Token::Bang, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '<' if chars.get(i + 1) == Some(&'<') && chars.get(i + 2) == Some(&'=') => {
                push_token(&mut tokens, Token::ShiftLeftAssign, line, column);
                advance_position('<', &mut line, &mut column, &mut i);
                advance_position('<', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '<' if chars.get(i + 1) == Some(&'<') => {
                push_token(&mut tokens, Token::ShiftLeft, line, column);
                advance_position('<', &mut line, &mut column, &mut i);
                advance_position('<', &mut line, &mut column, &mut i);
            }
            '<' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::Le, line, column);
                advance_position('<', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '>' if chars.get(i + 1) == Some(&'>') && chars.get(i + 2) == Some(&'=') => {
                push_token(&mut tokens, Token::ShiftRightAssign, line, column);
                advance_position('>', &mut line, &mut column, &mut i);
                advance_position('>', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '>' if chars.get(i + 1) == Some(&'>') => {
                push_token(&mut tokens, Token::ShiftRight, line, column);
                advance_position('>', &mut line, &mut column, &mut i);
                advance_position('>', &mut line, &mut column, &mut i);
            }
            '>' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::Ge, line, column);
                advance_position('>', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '=' => {
                push_token(&mut tokens, Token::Assign, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '<' => {
                push_token(&mut tokens, Token::Lt, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '>' => {
                push_token(&mut tokens, Token::Gt, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            other => {
                return Err(lexer_error_with_context(
                    format!("unexpected character '{other}'"),
                    source,
                    line,
                    column,
                ));
            }
        }
    }

    tokens.push(LocatedToken::new(Token::Eof, line, column));
    Ok(tokens)
}

fn lexer_error_with_context(
    message: impl Into<String>,
    source: &str,
    line: usize,
    column: usize,
) -> CustError {
    let source_line = source.lines().nth(line.saturating_sub(1)).unwrap_or("");
    let caret_padding = " ".repeat(column.saturating_sub(1));
    CustError::new(format!(
        "{} at line {line}, column {column}\n{source_line}\n{caret_padding}^",
        message.into()
    ))
}

fn push_token(tokens: &mut Vec<LocatedToken>, kind: Token, line: usize, column: usize) {
    tokens.push(LocatedToken::new(kind, line, column));
}

fn parse_escape_sequence(
    source: &str,
    chars: &[char],
    line: &mut usize,
    column: &mut usize,
    i: &mut usize,
    literal_start: (usize, usize),
    literal_kind: &str,
) -> CustResult<i64> {
    let (literal_start_line, literal_start_column) = literal_start;
    let Some(escape_char) = chars.get(*i).copied() else {
        return Err(lexer_error_with_context(
            format!("unterminated {literal_kind} literal"),
            source,
            literal_start_line,
            literal_start_column,
        ));
    };

    match escape_char {
        'a' => {
            advance_position('a', line, column, i);
            Ok(0x07)
        }
        'b' => {
            advance_position('b', line, column, i);
            Ok(0x08)
        }
        'f' => {
            advance_position('f', line, column, i);
            Ok(0x0c)
        }
        'n' => {
            advance_position('n', line, column, i);
            Ok('\n' as i64)
        }
        'r' => {
            advance_position('r', line, column, i);
            Ok('\r' as i64)
        }
        't' => {
            advance_position('t', line, column, i);
            Ok('\t' as i64)
        }
        'v' => {
            advance_position('v', line, column, i);
            Ok(0x0b)
        }
        '\\' => {
            advance_position('\\', line, column, i);
            Ok('\\' as i64)
        }
        '\'' => {
            advance_position('\'', line, column, i);
            Ok('\'' as i64)
        }
        '"' => {
            advance_position('"', line, column, i);
            Ok('"' as i64)
        }
        '?' => {
            advance_position('?', line, column, i);
            Ok('?' as i64)
        }
        'x' => {
            advance_position('x', line, column, i);
            let digits_start = *i;
            let mut value = 0i64;
            while let Some(digit) = chars.get(*i).copied().filter(char::is_ascii_hexdigit) {
                let digit_value = digit.to_digit(16).expect("hex digit should have value") as i64;
                value = value
                    .checked_mul(16)
                    .and_then(|base| base.checked_add(digit_value))
                    .ok_or_else(|| {
                        lexer_error_with_context(
                            "escape sequence value out of range",
                            source,
                            literal_start_line,
                            literal_start_column,
                        )
                    })?;
                advance_position(digit, line, column, i);
            }
            if *i == digits_start {
                return Err(lexer_error_with_context(
                    "hex escape sequence requires at least one digit",
                    source,
                    literal_start_line,
                    literal_start_column,
                ));
            }
            Ok(value)
        }
        '0'..='7' => {
            let mut value = 0i64;
            for _ in 0..3 {
                let Some(digit) = chars.get(*i).copied() else {
                    break;
                };
                if !matches!(digit, '0'..='7') {
                    break;
                }
                let digit_value = digit.to_digit(8).expect("octal digit should have value") as i64;
                value = value * 8 + digit_value;
                advance_position(digit, line, column, i);
            }
            Ok(value)
        }
        other => Err(lexer_error_with_context(
            format!("unsupported {literal_kind} escape '\\{other}'"),
            source,
            literal_start_line,
            literal_start_column,
        )),
    }
}

fn consume_integer_suffix(chars: &[char], line: &mut usize, column: &mut usize, i: &mut usize) {
    if matches!(chars.get(*i), Some('u') | Some('U')) {
        let suffix = chars[*i];
        advance_position(suffix, line, column, i);
    }

    if matches!(chars.get(*i), Some('l') | Some('L')) {
        let suffix = chars[*i];
        advance_position(suffix, line, column, i);
        if matches!(chars.get(*i), Some('l') | Some('L')) {
            let suffix = chars[*i];
            advance_position(suffix, line, column, i);
        }
    }

    if matches!(chars.get(*i), Some('u') | Some('U')) {
        let suffix = chars[*i];
        advance_position(suffix, line, column, i);
    }
}

fn advance_position(c: char, line: &mut usize, column: &mut usize, i: &mut usize) {
    *i += 1;
    if c == '\n' {
        *line += 1;
        *column = 1;
    } else {
        *column += 1;
    }
}

struct Parser {
    tokens: Vec<LocatedToken>,
    pos: usize,
    struct_types: HashMap<String, StructTypeDef>,
    aggregate_type_scopes: Vec<HashMap<String, String>>,
    enum_type_scopes: Vec<HashSet<String>>,
    enum_constant_scopes: Vec<HashMap<String, i64>>,
    type_alias_scopes: Vec<HashMap<String, TypeAlias>>,
    const_type_alias_scopes: Vec<HashSet<String>>,
    next_static_local_id: usize,
    next_aggregate_type_id: usize,
    last_decl_had_initializer: bool,
}

impl Parser {
    fn new(tokens: Vec<LocatedToken>) -> Self {
        Self {
            tokens,
            pos: 0,
            struct_types: HashMap::new(),
            aggregate_type_scopes: vec![HashMap::new()],
            enum_type_scopes: vec![HashSet::new()],
            enum_constant_scopes: vec![HashMap::new()],
            type_alias_scopes: vec![HashMap::new()],
            const_type_alias_scopes: vec![HashSet::new()],
            next_static_local_id: 0,
            next_aggregate_type_id: 0,
            last_decl_had_initializer: false,
        }
    }

    fn parse_program(&mut self) -> CustResult<Program> {
        let mut globals = Vec::new();
        let mut functions = HashMap::new();
        let mut prototypes = HashMap::new();
        while !self.check(&Token::Eof) {
            if self.check(&Token::RBrace) {
                return Err(Self::error_at(
                    "unmatched '}' at top level".to_string(),
                    self.peek_located(),
                ));
            }
            if self.check(&Token::StaticAssert) {
                globals.push(self.parse_static_assert()?);
                continue;
            }
            let _has_alignment_specifier = self.consume_alignment_specifiers()?;
            let leading_function_specifier = self.consume_function_specifiers();
            self.consume_thread_local_specifiers();
            let is_extern = self.matches(&Token::Extern);
            let _is_static = !is_extern && self.matches(&Token::Static);
            self.consume_thread_local_specifiers();
            self.consume_alignment_specifiers()?;
            let has_function_specifier =
                leading_function_specifier || self.consume_function_specifiers();
            if self.starts_function_definition()
                || self.starts_struct_function_declaration()
                || self.starts_alias_function_declaration()
                || self.starts_malformed_function_definition()
                || self.check(&Token::Void)
            {
                let (name, top_level_function) = self.parse_function_declaration()?;
                match top_level_function {
                    TopLevelFunction::Definition(function) => {
                        let signature = FunctionSignature::from_function(&function);
                        match prototypes.get(&name) {
                            Some(prototype) if prototype != &signature => {
                                return Err(CustError::new(format!(
                                    "function definition for '{name}' conflicts with previous prototype"
                                )));
                            }
                            _ => {}
                        }
                        if functions.insert(name.clone(), function).is_some() {
                            return Err(CustError::new(format!(
                                "function '{name}' already defined"
                            )));
                        }
                    }
                    TopLevelFunction::Prototype(signature) => {
                        match functions.get(&name) {
                            Some(function)
                                if FunctionSignature::from_function(function) != signature =>
                            {
                                return Err(CustError::new(format!(
                                    "function prototype for '{name}' conflicts with previous definition"
                                )));
                            }
                            _ => {}
                        }
                        if let Some(previous) = prototypes.get(&name) {
                            if previous != &signature {
                                return Err(CustError::new(format!(
                                    "function prototype for '{name}' conflicts with previous declaration"
                                )));
                            }
                        } else {
                            prototypes.insert(name, signature);
                        }
                    }
                }
            } else if has_function_specifier {
                return Err(Self::error_at(
                    "function specifiers are only supported on function declarations".to_string(),
                    self.peek_located(),
                ));
            } else if matches!(
                self.peek(),
                Token::Int
                    | Token::Char
                    | Token::Bool
                    | Token::Signed
                    | Token::Unsigned
                    | Token::Long
                    | Token::Short
                    | Token::Const
                    | Token::Volatile
                    | Token::Restrict
                    | Token::Atomic
            ) || self.current_alias().is_some()
            {
                let global = self.parse_var_decl()?;
                if !is_extern || self.last_decl_had_initializer {
                    globals.push(global);
                }
            } else if self.check(&Token::Typedef) {
                if let Some(stmt) = self.parse_typedef_decl()? {
                    globals.push(stmt);
                }
            } else if self.check(&Token::Enum) {
                if self.starts_typedef_enum_definition() {
                    globals.push(self.parse_enum_decl()?);
                } else {
                    let global = self.parse_var_decl()?;
                    if !is_extern || self.last_decl_had_initializer {
                        globals.push(global);
                    }
                }
            } else if matches!(self.peek(), Token::Struct | Token::Union) {
                if self.is_aggregate_definition() {
                    self.parse_aggregate_definition()?;
                } else {
                    let global = self.parse_aggregate_var_decl()?;
                    if !is_extern || self.last_decl_had_initializer {
                        globals.push(global);
                    }
                }
            } else {
                let found = self.peek_located().clone();
                return Err(Self::error_at(
                    format!("unexpected token at top level: {:?}", found.kind),
                    &found,
                ));
            }
        }
        self.expect(Token::Eof)?;
        if !functions.contains_key("main") {
            return Err(CustError::new("missing main() function"));
        }
        Ok(Program {
            globals,
            functions,
            struct_types: self.struct_types.clone(),
        })
    }

    fn consume_function_specifiers(&mut self) -> bool {
        let mut consumed = false;
        while matches!(self.peek(), Token::Inline | Token::Noreturn) {
            self.advance();
            consumed = true;
        }
        consumed
    }

    fn consume_thread_local_specifiers(&mut self) -> bool {
        let mut consumed = false;
        while self.matches(&Token::ThreadLocal) {
            consumed = true;
        }
        consumed
    }

    fn consume_alignment_specifiers(&mut self) -> CustResult<bool> {
        let mut consumed = false;
        while self.matches(&Token::Alignas) {
            consumed = true;
            self.expect_opening_paren_after("_Alignas")?;
            if self.is_type_name_start() {
                self.parse_sizeof_like_type_name("_Alignas")?;
            } else {
                self.parse_assignment_expr()?;
            }
            self.expect_closing_paren_after("_Alignas specifier")?;
        }
        Ok(consumed)
    }

    fn is_aggregate_definition(&self) -> bool {
        matches!(
            (
                self.peek(),
                self.tokens.get(self.pos + 1).map(|token| &token.kind),
                self.tokens.get(self.pos + 2).map(|token| &token.kind)
            ),
            (
                Token::Struct | Token::Union,
                Some(Token::Ident(_)),
                Some(Token::LBrace)
            )
        )
    }

    fn starts_struct_function_declaration(&self) -> bool {
        if !matches!(self.peek(), Token::Struct | Token::Union) {
            return false;
        }
        if !matches!(
            self.tokens.get(self.pos + 1).map(|token| &token.kind),
            Some(Token::Ident(_))
        ) {
            return false;
        }
        let mut index = self.pos + 2;
        while matches!(
            self.tokens.get(index).map(|token| &token.kind),
            Some(Token::Star)
        ) || self.type_qualifier_at(index)
        {
            index += 1;
        }
        matches!(
            (
                self.tokens.get(index).map(|token| &token.kind),
                self.tokens.get(index + 1).map(|token| &token.kind)
            ),
            (Some(Token::Ident(_)), Some(Token::LParen))
        )
    }

    fn starts_alias_function_declaration(&self) -> bool {
        self.current_alias().is_some()
            && matches!(
                (
                    self.tokens.get(self.pos + 1).map(|token| &token.kind),
                    self.tokens.get(self.pos + 2).map(|token| &token.kind)
                ),
                (Some(Token::Ident(_)), Some(Token::LParen))
            )
    }

    fn current_alias(&self) -> Option<&TypeAlias> {
        match self.peek() {
            Token::Ident(name) => self.lookup_type_alias(name),
            _ => None,
        }
    }

    fn alias_at_index(&self, index: usize) -> Option<&TypeAlias> {
        match self.tokens.get(index).map(|token| &token.kind) {
            Some(Token::Ident(name)) => self.lookup_type_alias(name),
            _ => None,
        }
    }

    fn lookup_type_alias(&self, name: &str) -> Option<&TypeAlias> {
        self.type_alias_scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
    }

    fn type_alias_is_const(&self, name: &str) -> bool {
        self.const_type_alias_scopes
            .iter()
            .rev()
            .any(|scope| scope.contains(name))
    }

    fn enum_type_is_declared(&self, name: &str) -> bool {
        self.enum_type_scopes
            .iter()
            .rev()
            .any(|scope| scope.contains(name))
    }

    fn lookup_enum_constant(&self, name: &str) -> Option<i64> {
        self.enum_constant_scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }

    fn resolve_aggregate_type(&self, name: &str) -> Option<String> {
        self.aggregate_type_scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).cloned())
    }

    fn parse_decl_type(&mut self, context: &str) -> CustResult<DeclType> {
        let (_, decl_type) = self.parse_decl_type_with_embedded_qualifiers(context)?;
        Ok(decl_type)
    }

    fn parse_decl_type_with_embedded_qualifiers(
        &mut self,
        context: &str,
    ) -> CustResult<(bool, DeclType)> {
        let found = self.advance();
        let mut saw_const = false;
        match found.kind.clone() {
            Token::Int => {
                saw_const |= self.consume_type_qualifiers();
                Ok((saw_const, DeclType::Scalar(CType::Int)))
            }
            Token::Char => {
                saw_const |= self.consume_type_qualifiers();
                Ok((saw_const, DeclType::Scalar(CType::Char)))
            }
            Token::Bool => {
                saw_const |= self.consume_type_qualifiers();
                Ok((saw_const, DeclType::Scalar(CType::Bool)))
            }
            Token::Atomic => {
                self.expect_opening_paren_after("_Atomic")?;
                let (nested_const, decl_type) =
                    self.parse_decl_type_with_embedded_qualifiers("_Atomic type name")?;
                self.expect_closing_paren_after("_Atomic type")?;
                saw_const |= nested_const || self.consume_type_qualifiers();
                Ok((saw_const, decl_type))
            }
            Token::Signed | Token::Unsigned => {
                saw_const |= self.consume_type_qualifiers();
                if self.matches(&Token::Char) {
                    saw_const |= self.consume_type_qualifiers();
                    Ok((saw_const, DeclType::Scalar(CType::Char)))
                } else {
                    self.matches(&Token::Int);
                    saw_const |= self.consume_type_qualifiers();
                    if self.matches(&Token::Long) {
                        saw_const |= self.consume_type_qualifiers();
                        self.matches(&Token::Long);
                        saw_const |= self.consume_type_qualifiers();
                        self.matches(&Token::Int);
                        saw_const |= self.consume_type_qualifiers();
                    } else if self.matches(&Token::Short) {
                        saw_const |= self.consume_type_qualifiers();
                        self.matches(&Token::Int);
                        saw_const |= self.consume_type_qualifiers();
                    }
                    Ok((saw_const, DeclType::Scalar(CType::Int)))
                }
            }
            Token::Long => {
                saw_const |= self.consume_type_qualifiers();
                self.matches(&Token::Long);
                saw_const |= self.consume_type_qualifiers();
                self.matches(&Token::Int);
                saw_const |= self.consume_type_qualifiers();
                Ok((saw_const, DeclType::Scalar(CType::Int)))
            }
            Token::Short => {
                saw_const |= self.consume_type_qualifiers();
                self.matches(&Token::Int);
                saw_const |= self.consume_type_qualifiers();
                Ok((saw_const, DeclType::Scalar(CType::Int)))
            }
            Token::Struct | Token::Union => {
                let keyword = if matches!(found.kind, Token::Union) {
                    "union"
                } else {
                    "struct"
                };
                let type_name = self.expect_ident_after(context)?;
                saw_const |= self.consume_type_qualifiers();
                let Some(internal_type_name) = self.resolve_aggregate_type(&type_name) else {
                    return Err(CustError::new(format!(
                        "undefined {keyword} type '{type_name}'"
                    )));
                };
                Ok((saw_const, DeclType::Struct(internal_type_name)))
            }
            Token::Enum => {
                let type_name_token = self.advance();
                let type_name = match type_name_token.kind.clone() {
                    Token::Ident(type_name) => type_name,
                    token => {
                        return Err(Self::error_at(
                            format!("expected enum type name, found {token:?}"),
                            &type_name_token,
                        ));
                    }
                };
                saw_const |= self.consume_type_qualifiers();
                if !self.enum_type_is_declared(&type_name) {
                    return Err(Self::error_at(
                        format!("undefined enum type '{type_name}'"),
                        &type_name_token,
                    ));
                }
                Ok((saw_const, DeclType::Scalar(CType::Int)))
            }
            Token::Ident(name) => match self.lookup_type_alias(&name).cloned() {
                Some(TypeAlias::Scalar(ty)) => {
                    saw_const |= self.consume_type_qualifiers();
                    Ok((saw_const, DeclType::Scalar(ty)))
                }
                Some(TypeAlias::Struct(type_name)) => {
                    saw_const |= self.consume_type_qualifiers();
                    Ok((saw_const, DeclType::Struct(type_name)))
                }
                Some(TypeAlias::Pointer {
                    pointee,
                    points_to_const,
                }) => {
                    saw_const |= self.consume_type_qualifiers();
                    Ok((
                        saw_const,
                        DeclType::Pointer {
                            pointee,
                            points_to_const,
                        },
                    ))
                }
                Some(TypeAlias::Array(pointee, len)) => {
                    saw_const |= self.consume_type_qualifiers();
                    Ok((saw_const, DeclType::Array(pointee, len)))
                }
                None => Err(Self::error_at(
                    format!("expected {context}, found Ident(\"{name}\")"),
                    &found,
                )),
            },
            token => Err(Self::error_at(
                format!("expected type, found {token:?}"),
                &found,
            )),
        }
    }

    fn consume_type_qualifiers(&mut self) -> bool {
        let mut saw_const = false;
        while matches!(
            self.peek(),
            Token::Const | Token::Volatile | Token::Restrict
        ) || self.bare_atomic_qualifier_at(self.pos)
        {
            if self.matches(&Token::Const) {
                saw_const = true;
            } else if self.matches(&Token::Restrict) {
                // `restrict` is accepted as parser-level C syntax over Cust's
                // existing interpreter-owned pointer model. It does not change
                // runtime aliasing behavior.
            } else if self.matches(&Token::Atomic) {
                // Bare `_Atomic` is accepted as parser-level C syntax over Cust's
                // deterministic storage model. The `_Atomic(type-name)` form is
                // parsed as a type specifier by parse_decl_type.
            } else {
                self.expect(Token::Volatile)
                    .expect("peek confirmed volatile token");
            }
        }
        saw_const
    }

    fn bare_atomic_qualifier_at(&self, index: usize) -> bool {
        matches!(
            self.tokens.get(index).map(|token| &token.kind),
            Some(Token::Atomic)
        ) && !matches!(
            self.tokens.get(index + 1).map(|token| &token.kind),
            Some(Token::LParen)
        )
    }

    fn parse_const_qualified_decl_type(&mut self, context: &str) -> CustResult<(bool, DeclType)> {
        let leading_const = self.consume_type_qualifiers();
        let alias_const = match self.peek() {
            Token::Ident(name) => self.type_alias_is_const(name),
            _ => false,
        };
        let (embedded_const, decl_type) = self.parse_decl_type_with_embedded_qualifiers(context)?;
        Ok((leading_const || embedded_const || alias_const, decl_type))
    }

    fn decl_type_to_return_type(decl_type: DeclType) -> ReturnType {
        match decl_type {
            DeclType::Scalar(ty) => ReturnType::Scalar(ty),
            DeclType::Struct(type_name) => ReturnType::Struct(type_name),
            DeclType::Pointer {
                pointee,
                points_to_const,
            } => ReturnType::Pointer {
                ty: pointee,
                points_to_const,
            },
            DeclType::Array(_, _) => unreachable!("array aliases are not valid return types"),
        }
    }

    fn decl_type_to_param_type(decl_type: &DeclType) -> ParamType {
        match decl_type {
            DeclType::Scalar(ty) => ParamType::Scalar(*ty),
            DeclType::Struct(type_name) => ParamType::Struct(type_name.clone()),
            DeclType::Pointer { pointee, .. } => match pointee {
                PointeeType::Scalar(ty) => ParamType::Scalar(*ty),
                PointeeType::Struct(type_name) => ParamType::Struct(type_name.clone()),
            },
            DeclType::Array(pointee, _) => match pointee {
                PointeeType::Scalar(ty) => ParamType::Scalar(*ty),
                PointeeType::Struct(type_name) => ParamType::Struct(type_name.clone()),
            },
        }
    }

    fn decl_type_to_pointee_type(decl_type: &DeclType) -> PointeeType {
        match decl_type {
            DeclType::Scalar(ty) => PointeeType::Scalar(*ty),
            DeclType::Struct(type_name) => PointeeType::Struct(type_name.clone()),
            DeclType::Pointer { pointee, .. } => pointee.clone(),
            DeclType::Array(pointee, _) => pointee.clone(),
        }
    }

    fn decl_type_points_to_const(decl_type: &DeclType) -> bool {
        match decl_type {
            DeclType::Pointer {
                points_to_const, ..
            } => *points_to_const,
            _ => false,
        }
    }

    fn parse_typedef_decl(&mut self) -> CustResult<Option<Stmt>> {
        self.expect(Token::Typedef)?;
        let mut enum_constants = None;
        let (alias, alias_context, anonymous_aggregate, alias_is_const) = if self
            .starts_typedef_aggregate_definition()
        {
            let (type_name, _, is_anonymous) = self.parse_aggregate_definition_body(false, true)?;
            (
                TypeAlias::Struct(type_name),
                "typedef alias name after aggregate definition",
                is_anonymous,
                false,
            )
        } else if self.starts_typedef_enum_definition() {
            let constants = self.parse_enum_decl_body(false)?;
            enum_constants = Some(constants);
            (
                TypeAlias::Scalar(CType::Int),
                "typedef alias name after enum definition",
                false,
                false,
            )
        } else {
            let (leading_const, base_type) =
                self.parse_const_qualified_decl_type("typedef struct type name")?;
            let (alias, alias_is_const) = if self.matches(&Token::Star) {
                if self.check(&Token::Star) {
                    return Err(Self::error_at(
                        "pointer-to-pointer typedef aliases are not supported".to_string(),
                        self.peek_located(),
                    ));
                }
                let post_star_const = self.consume_type_qualifiers();
                let alias = match base_type {
                    DeclType::Scalar(ty) => TypeAlias::Pointer {
                        pointee: PointeeType::Scalar(ty),
                        points_to_const: leading_const,
                    },
                    DeclType::Struct(type_name) => TypeAlias::Pointer {
                        pointee: PointeeType::Struct(type_name),
                        points_to_const: leading_const,
                    },
                    DeclType::Pointer { .. } => {
                        return Err(Self::error_at(
                            "pointer-to-pointer typedef aliases are not supported".to_string(),
                            self.previous(),
                        ));
                    }
                    DeclType::Array(_, _) => {
                        return Err(Self::error_at(
                            "pointer-to-array typedef aliases are not supported".to_string(),
                            self.previous(),
                        ));
                    }
                };
                (alias, post_star_const)
            } else {
                let alias = match base_type {
                    DeclType::Scalar(ty) => TypeAlias::Scalar(ty),
                    DeclType::Struct(type_name) => TypeAlias::Struct(type_name),
                    DeclType::Pointer {
                        pointee,
                        points_to_const,
                    } => TypeAlias::Pointer {
                        pointee,
                        points_to_const,
                    },
                    DeclType::Array(pointee, len) => TypeAlias::Array(pointee, len),
                };
                (alias, leading_const)
            };
            (
                alias,
                "typedef alias name after type",
                false,
                alias_is_const,
            )
        };
        let alias_name = self.expect_ident_after(alias_context)?;
        let alias = if self.matches(&Token::LBracket) {
            let len = self.expect_array_len()?;
            self.expect_closing_bracket_after("typedef array length")?;
            if self.check(&Token::LBracket) {
                return Err(Self::error_at(
                    "multidimensional array typedef aliases are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            match alias {
                TypeAlias::Scalar(ty) => TypeAlias::Array(PointeeType::Scalar(ty), len),
                TypeAlias::Struct(type_name) => {
                    TypeAlias::Array(PointeeType::Struct(type_name), len)
                }
                TypeAlias::Pointer { .. } => {
                    return Err(Self::error_at(
                        "pointer array typedef aliases are not supported".to_string(),
                        self.previous(),
                    ));
                }
                TypeAlias::Array(_, _) => {
                    return Err(Self::error_at(
                        "multidimensional array typedef aliases are not supported".to_string(),
                        self.previous(),
                    ));
                }
            }
        } else {
            alias
        };
        self.expect_semicolon_after("typedef declaration")?;
        let anonymous_type_name = match (anonymous_aggregate, &alias) {
            (true, TypeAlias::Struct(type_name)) => Some(type_name.clone()),
            _ => None,
        };
        if let Some(struct_type) = anonymous_type_name
            .as_ref()
            .and_then(|type_name| self.struct_types.get_mut(type_name))
        {
            struct_type.display_name = alias_name.clone();
        }
        let current_scope = self
            .type_alias_scopes
            .last_mut()
            .expect("parser always has a typedef scope");
        if current_scope.insert(alias_name.clone(), alias).is_some() {
            return Err(CustError::new(format!(
                "typedef alias '{alias_name}' already declared"
            )));
        }
        if alias_is_const {
            let const_scope = self
                .const_type_alias_scopes
                .last_mut()
                .expect("parser always has a const typedef scope");
            const_scope.insert(alias_name);
        }
        Ok(enum_constants.map(|constants| Stmt::EnumDecl { constants }))
    }

    fn starts_function_definition(&self) -> bool {
        let mut index = self.skip_type_qualifiers_at(self.pos);
        match self.tokens.get(index).map(|token| &token.kind) {
            Some(Token::Int | Token::Char | Token::Bool | Token::Void) => {
                index += 1;
                index = self.skip_type_qualifiers_at(index);
            }
            Some(Token::Long) => {
                index += 1;
                index = self.skip_type_qualifiers_at(index);
                if matches!(
                    self.tokens.get(index).map(|token| &token.kind),
                    Some(Token::Long)
                ) {
                    index += 1;
                    index = self.skip_type_qualifiers_at(index);
                }
                if matches!(
                    self.tokens.get(index).map(|token| &token.kind),
                    Some(Token::Int)
                ) {
                    index += 1;
                    index = self.skip_type_qualifiers_at(index);
                }
            }
            Some(Token::Short) => {
                index += 1;
                index = self.skip_type_qualifiers_at(index);
                if matches!(
                    self.tokens.get(index).map(|token| &token.kind),
                    Some(Token::Int)
                ) {
                    index += 1;
                    index = self.skip_type_qualifiers_at(index);
                }
            }
            Some(Token::Signed | Token::Unsigned) => {
                index += 1;
                index = self.skip_type_qualifiers_at(index);
                if matches!(
                    self.tokens.get(index).map(|token| &token.kind),
                    Some(Token::Int | Token::Char)
                ) {
                    index += 1;
                    index = self.skip_type_qualifiers_at(index);
                } else if matches!(
                    self.tokens.get(index).map(|token| &token.kind),
                    Some(Token::Long)
                ) {
                    index += 1;
                    index = self.skip_type_qualifiers_at(index);
                    if matches!(
                        self.tokens.get(index).map(|token| &token.kind),
                        Some(Token::Long)
                    ) {
                        index += 1;
                        index = self.skip_type_qualifiers_at(index);
                    }
                    if matches!(
                        self.tokens.get(index).map(|token| &token.kind),
                        Some(Token::Int)
                    ) {
                        index += 1;
                        index = self.skip_type_qualifiers_at(index);
                    }
                } else if matches!(
                    self.tokens.get(index).map(|token| &token.kind),
                    Some(Token::Short)
                ) {
                    index += 1;
                    index = self.skip_type_qualifiers_at(index);
                    if matches!(
                        self.tokens.get(index).map(|token| &token.kind),
                        Some(Token::Int)
                    ) {
                        index += 1;
                        index = self.skip_type_qualifiers_at(index);
                    }
                }
            }
            Some(Token::Enum) => {
                if !matches!(
                    self.tokens.get(index + 1).map(|token| &token.kind),
                    Some(Token::Ident(_))
                ) {
                    return false;
                }
                index += 2;
                index = self.skip_type_qualifiers_at(index);
            }
            Some(Token::Struct | Token::Union) => {
                if !matches!(
                    self.tokens.get(index + 1).map(|token| &token.kind),
                    Some(Token::Ident(_))
                ) {
                    return false;
                }
                index += 2;
                index = self.skip_type_qualifiers_at(index);
            }
            Some(Token::Atomic) => {
                let Some(next_index) = self.skip_atomic_type_specifier_at(index) else {
                    return false;
                };
                index = self.skip_type_qualifiers_at(next_index);
            }
            Some(Token::Ident(_)) if self.alias_at_index(index).is_some() => {
                index += 1;
                index = self.skip_type_qualifiers_at(index);
            }
            _ => return false,
        }
        while matches!(
            self.tokens.get(index).map(|token| &token.kind),
            Some(Token::Star)
        ) || self.type_qualifier_at(index)
        {
            index += 1;
        }

        matches!(
            (
                self.tokens.get(index).map(|token| &token.kind),
                self.tokens.get(index + 1).map(|token| &token.kind)
            ),
            (Some(Token::Ident(_)), Some(Token::LParen))
        )
    }

    fn skip_type_qualifiers_at(&self, mut index: usize) -> usize {
        while self.type_qualifier_at(index) {
            index += 1;
        }
        index
    }

    fn type_qualifier_at(&self, index: usize) -> bool {
        matches!(
            self.tokens.get(index).map(|token| &token.kind),
            Some(Token::Const | Token::Volatile | Token::Restrict)
        ) || self.bare_atomic_qualifier_at(index)
    }

    fn skip_atomic_type_specifier_at(&self, index: usize) -> Option<usize> {
        if !matches!(
            self.tokens.get(index).map(|token| &token.kind),
            Some(Token::Atomic)
        ) || !matches!(
            self.tokens.get(index + 1).map(|token| &token.kind),
            Some(Token::LParen)
        ) {
            return None;
        }

        let mut depth = 1;
        let mut cursor = index + 2;
        while let Some(token) = self.tokens.get(cursor) {
            match &token.kind {
                Token::LParen => depth += 1,
                Token::RParen => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(cursor + 1);
                    }
                }
                Token::Eof => return None,
                _ => {}
            }
            cursor += 1;
        }
        None
    }

    fn starts_malformed_function_definition(&self) -> bool {
        if !matches!(
            self.peek(),
            Token::Int
                | Token::Char
                | Token::Bool
                | Token::Signed
                | Token::Unsigned
                | Token::Long
                | Token::Short
        ) {
            return false;
        }

        matches!(
            (
                self.tokens.get(self.pos + 1).map(|token| &token.kind),
                self.tokens.get(self.pos + 2).map(|token| &token.kind)
            ),
            (Some(Token::LParen), _) | (Some(Token::Ident(_)), Some(Token::LBrace))
        )
    }

    fn parse_function_declaration(&mut self) -> CustResult<(String, TopLevelFunction)> {
        let return_type = self.parse_function_return_type()?;
        let name = self.expect_ident_after("function name after return type")?;
        self.expect_opening_paren_after("function name")?;
        let allow_unnamed_params = self.parameter_list_is_prototype();
        let params = self.parse_params(allow_unnamed_params)?;
        self.expect_closing_paren_after("function parameters")?;
        if self.matches(&Token::Semi) {
            return Ok((
                name,
                TopLevelFunction::Prototype(FunctionSignature::new(return_type, &params)),
            ));
        }
        let body = self.parse_block_after("function header")?;
        Ok((
            name,
            TopLevelFunction::Definition(Function {
                return_type,
                params,
                body,
            }),
        ))
    }

    fn parameter_list_is_prototype(&self) -> bool {
        let mut cursor = self.pos;
        while let Some(token) = self.tokens.get(cursor) {
            match token.kind {
                Token::RParen => {
                    return matches!(
                        self.tokens.get(cursor + 1).map(|next| &next.kind),
                        Some(Token::Semi)
                    );
                }
                Token::Eof => return false,
                _ => cursor += 1,
            }
        }
        false
    }

    fn parse_function_return_type(&mut self) -> CustResult<ReturnType> {
        if self.check(&Token::Void) {
            self.advance();
            return Ok(ReturnType::Void);
        }
        let (leading_const, decl_type) =
            self.parse_const_qualified_decl_type("struct return type name")?;
        if self.matches(&Token::Star) {
            if matches!(decl_type, DeclType::Pointer { .. }) || self.check(&Token::Star) {
                return Err(Self::error_at(
                    "pointer-to-pointer return types are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            self.consume_type_qualifiers();
            return Ok(ReturnType::Pointer {
                ty: Self::decl_type_to_pointee_type(&decl_type),
                points_to_const: leading_const,
            });
        }
        if let DeclType::Pointer {
            pointee,
            points_to_const,
        } = decl_type
        {
            return Ok(ReturnType::Pointer {
                ty: pointee,
                points_to_const: leading_const || points_to_const,
            });
        }
        if matches!(decl_type, DeclType::Array(_, _)) {
            return Err(Self::error_at(
                "array return types are not supported".to_string(),
                self.previous(),
            ));
        }
        Ok(Self::decl_type_to_return_type(decl_type))
    }

    fn parse_params(&mut self, allow_unnamed: bool) -> CustResult<Vec<Param>> {
        let mut params = Vec::new();
        if self.check(&Token::RParen) {
            return Ok(params);
        }
        if self.check(&Token::Void) {
            let void_token = self.advance();
            if self.check(&Token::RParen) {
                return Ok(params);
            }
            return Err(Self::error_at(
                "void parameter lists must be empty".to_string(),
                &void_token,
            ));
        }

        loop {
            let (leading_const, decl_type) =
                self.parse_const_qualified_decl_type("parameter type")?;
            let has_explicit_star = self.matches(&Token::Star);
            let post_star_const = has_explicit_star && self.consume_type_qualifiers();
            if matches!(decl_type, DeclType::Pointer { .. }) && has_explicit_star {
                return Err(Self::error_at(
                    "pointer-to-pointer parameters are not supported".to_string(),
                    self.previous(),
                ));
            }
            if matches!(decl_type, DeclType::Array(_, _)) && has_explicit_star {
                return Err(Self::error_at(
                    "pointer-to-array parameters are not supported".to_string(),
                    self.previous(),
                ));
            }
            if has_explicit_star && self.check(&Token::Star) {
                return Err(Self::error_at(
                    "pointer-to-pointer parameters are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            if !has_explicit_star
                && self.check(&Token::LParen)
                && matches!(self.peek_next(), Token::Star)
            {
                return Err(Self::error_at(
                    "parenthesized pointer parameters are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            let is_pointer = has_explicit_star
                || matches!(decl_type, DeclType::Pointer { .. } | DeclType::Array(_, _));
            let name = if allow_unnamed
                && matches!(self.peek(), Token::Comma | Token::RParen | Token::LBracket)
            {
                format!("__cust_prototype_param_{}", params.len())
            } else if has_explicit_star {
                match &decl_type {
                    DeclType::Scalar(_) => self.expect_ident_after("parameter name after '*'")?,
                    DeclType::Struct(_) => {
                        self.expect_ident_after("struct pointer parameter name after '*'")?
                    }
                    DeclType::Pointer { .. } => {
                        unreachable!("pointer aliases with explicit stars return above")
                    }
                    DeclType::Array(_, _) => {
                        unreachable!("array aliases with explicit stars return above")
                    }
                }
            } else {
                match &decl_type {
                    DeclType::Scalar(_) => self.expect_ident_after("parameter name after type")?,
                    DeclType::Struct(_) => {
                        self.expect_ident_after("struct parameter name after type")?
                    }
                    DeclType::Pointer { .. } => {
                        self.expect_ident_after("pointer parameter name after type")?
                    }
                    DeclType::Array(_, _) => {
                        self.expect_ident_after("array parameter name after type")?
                    }
                }
            };
            let kind = if is_pointer {
                if self.check(&Token::LBracket) {
                    return Err(Self::error_at(
                        "pointer array parameters are not supported".to_string(),
                        self.peek_located(),
                    ));
                }
                ParamKind::Pointer
            } else if matches!(decl_type, DeclType::Struct(_)) {
                if self.matches(&Token::LBracket) {
                    if !self.check(&Token::RBracket) {
                        self.expect_array_len()?;
                    }
                    self.expect_closing_bracket_after("array parameter length")?;
                    if self.check(&Token::LBracket) {
                        return Err(Self::error_at(
                            "multidimensional array parameters are not supported".to_string(),
                            self.peek_located(),
                        ));
                    }
                    ParamKind::Pointer
                } else {
                    ParamKind::Struct
                }
            } else if self.matches(&Token::LBracket) {
                if self.check(&Token::RBracket) {
                    self.expect_closing_bracket_after("array parameter length")?;
                } else {
                    self.expect_array_len()?;
                    self.expect_closing_bracket_after("array parameter length")?;
                }
                if self.check(&Token::LBracket) {
                    return Err(Self::error_at(
                        "multidimensional array parameters are not supported".to_string(),
                        self.peek_located(),
                    ));
                }
                ParamKind::Pointer
            } else {
                ParamKind::Scalar
            };
            let (is_const, points_to_const) = if matches!(kind, ParamKind::Pointer) {
                if has_explicit_star {
                    (post_star_const, leading_const)
                } else if is_pointer {
                    (leading_const, Self::decl_type_points_to_const(&decl_type))
                } else {
                    (false, leading_const)
                }
            } else {
                (leading_const, false)
            };
            params.push(Param {
                ty: Self::decl_type_to_param_type(&decl_type),
                name,
                kind,
                is_const,
                points_to_const,
            });

            if self.matches(&Token::Comma) {
                if matches!(
                    self.peek(),
                    Token::RParen | Token::Semi | Token::LBrace | Token::RBrace | Token::Eof
                ) {
                    return Err(Self::error_at(
                        format!(
                            "expected function parameter after ',', found {:?}",
                            self.peek()
                        ),
                        self.peek_located(),
                    ));
                }
            } else if matches!(
                self.peek(),
                Token::RParen | Token::Semi | Token::LBrace | Token::RBrace | Token::Eof
            ) {
                break;
            } else {
                return Err(Self::error_at(
                    format!(
                        "expected ',' or ')' after function parameter, found {:?}",
                        self.peek()
                    ),
                    self.peek_located(),
                ));
            }
        }
        Ok(params)
    }

    fn parse_block_after(&mut self, context: &str) -> CustResult<Vec<Stmt>> {
        self.expect_opening_brace_after(context)?;
        self.type_alias_scopes.push(HashMap::new());
        self.const_type_alias_scopes.push(HashSet::new());
        self.enum_type_scopes.push(HashSet::new());
        self.enum_constant_scopes.push(HashMap::new());
        self.aggregate_type_scopes.push(HashMap::new());
        let mut statements = Vec::new();
        let result = (|| {
            while !self.check(&Token::RBrace) {
                if self.check(&Token::Eof) {
                    let eof = self.peek_located().clone();
                    return Err(Self::error_at(
                        format!("unterminated block after {context}"),
                        &eof,
                    ));
                }
                statements.push(self.parse_stmt()?);
            }
            self.expect(Token::RBrace)?;
            Ok(statements)
        })();
        self.aggregate_type_scopes.pop();
        self.enum_constant_scopes.pop();
        self.enum_type_scopes.pop();
        self.const_type_alias_scopes.pop();
        self.type_alias_scopes.pop();
        result
    }

    fn parse_control_body_after(&mut self, context: &str) -> CustResult<Vec<Stmt>> {
        if self.check(&Token::LBrace) {
            Ok(vec![Stmt::Block(self.parse_block_after(context)?)])
        } else {
            Ok(vec![self.parse_stmt()?])
        }
    }

    fn parse_stmt(&mut self) -> CustResult<Stmt> {
        match self.peek() {
            Token::Semi => self.parse_empty(),
            Token::Static => self.parse_static_local_decl(),
            Token::ThreadLocal => self.parse_thread_local_local_decl(),
            Token::Auto | Token::Register => self.parse_auto_register_local_decl(),
            Token::Alignas => self.parse_aligned_decl(),
            Token::Int
            | Token::Char
            | Token::Bool
            | Token::Signed
            | Token::Unsigned
            | Token::Long
            | Token::Short
            | Token::Const
            | Token::Volatile
            | Token::Restrict
            | Token::Atomic => self.parse_var_decl(),
            Token::Ident(_) if self.current_alias().is_some() => self.parse_var_decl(),
            Token::Typedef => match self.parse_typedef_decl()? {
                Some(stmt) => Ok(stmt),
                None => Ok(Stmt::Empty),
            },
            Token::Enum => {
                if self.starts_typedef_enum_definition() {
                    self.parse_enum_decl()
                } else {
                    self.parse_var_decl()
                }
            }
            Token::Struct | Token::Union => self.parse_aggregate_var_decl(),
            Token::Return => self.parse_return(),
            Token::LBrace => Ok(Stmt::Block(self.parse_block_after("block statement")?)),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Do => self.parse_do_while(),
            Token::For => self.parse_for(),
            Token::Switch => self.parse_switch(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
            Token::StaticAssert => self.parse_static_assert(),
            Token::Case => Err(Self::error_at(
                "case label outside switch".to_string(),
                self.peek_located(),
            )),
            Token::Default => Err(Self::error_at(
                "default label outside switch".to_string(),
                self.peek_located(),
            )),
            Token::Star
                if self.starts_deref_assignment_stmt()
                    || self.starts_missing_deref_assignment_operator_stmt() =>
            {
                self.parse_deref_assign()
            }
            Token::Ident(_)
                if self.starts_assignment_stmt()
                    || self.starts_missing_assignment_operator_stmt() =>
            {
                self.parse_assign()
            }
            Token::Ident(_)
            | Token::Number(_)
            | Token::StringLiteral(_)
            | Token::Plus
            | Token::Minus
            | Token::PlusPlus
            | Token::MinusMinus
            | Token::Bang
            | Token::Sizeof
            | Token::Alignof
            | Token::Star
            | Token::Amp
            | Token::LParen => self.parse_expr_stmt_with_semi(true),
            Token::RParen => Err(Self::error_at(
                "unmatched ')' in statement".to_string(),
                self.peek_located(),
            )),
            Token::RBracket => Err(Self::error_at(
                "unmatched ']' in statement".to_string(),
                self.peek_located(),
            )),
            token => Err(Self::error_at(
                format!("unexpected token in statement: {token:?}"),
                self.peek_located(),
            )),
        }
    }

    fn parse_empty(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Semi)?;
        Ok(Stmt::Empty)
    }

    fn parse_static_assert(&mut self) -> CustResult<Stmt> {
        self.expect(Token::StaticAssert)?;
        self.expect_opening_paren_after("_Static_assert")?;
        let condition = self.parse_assignment_expr()?;
        if !self.matches(&Token::Comma) {
            return Err(Self::error_at(
                format!(
                    "expected ',' after _Static_assert condition, found {:?}",
                    self.peek()
                ),
                self.peek_located(),
            ));
        }
        let message = match self.advance().kind.clone() {
            Token::StringLiteral(values) => Self::static_assert_message(values),
            token => {
                return Err(Self::error_at(
                    format!(
                        "expected string literal after _Static_assert condition, found {token:?}"
                    ),
                    self.previous(),
                ));
            }
        };
        self.expect_closing_paren_after("_Static_assert message")?;
        self.expect_semicolon_after("_Static_assert")?;
        Ok(Stmt::StaticAssert { condition, message })
    }

    fn static_assert_message(values: Vec<i64>) -> String {
        values
            .into_iter()
            .take_while(|value| *value != 0)
            .map(|value| char::from_u32(value as u32).unwrap_or('\u{FFFD}'))
            .collect()
    }

    fn parse_static_local_decl(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Static)?;
        let id = self.next_static_local_id;
        self.next_static_local_id += 1;
        self.consume_thread_local_specifiers();
        self.consume_alignment_specifiers()?;
        let decl = match self.peek() {
            Token::Int
            | Token::Char
            | Token::Bool
            | Token::Signed
            | Token::Unsigned
            | Token::Long
            | Token::Short
            | Token::Const
            | Token::Volatile
            | Token::Restrict
            | Token::Atomic => self.parse_var_decl()?,
            Token::Ident(_) if self.current_alias().is_some() => self.parse_var_decl()?,
            Token::Struct | Token::Union => self.parse_aggregate_var_decl()?,
            token => {
                return Err(Self::error_at(
                    format!("expected declaration after static, found {token:?}"),
                    self.peek_located(),
                ));
            }
        };
        Ok(Stmt::StaticLocal {
            id,
            decl: Box::new(decl),
        })
    }

    fn parse_thread_local_local_decl(&mut self) -> CustResult<Stmt> {
        self.consume_thread_local_specifiers();
        if self.check(&Token::Static) {
            return self.parse_static_local_decl();
        }
        self.consume_alignment_specifiers()?;
        match self.peek() {
            Token::Int
            | Token::Char
            | Token::Bool
            | Token::Signed
            | Token::Unsigned
            | Token::Long
            | Token::Short
            | Token::Const
            | Token::Volatile
            | Token::Restrict
            | Token::Atomic => self.parse_var_decl(),
            Token::Ident(_) if self.current_alias().is_some() => self.parse_var_decl(),
            Token::Struct | Token::Union => self.parse_aggregate_var_decl(),
            token => Err(Self::error_at(
                format!("expected declaration after _Thread_local, found {token:?}"),
                self.peek_located(),
            )),
        }
    }

    fn parse_auto_register_local_decl(&mut self) -> CustResult<Stmt> {
        let specifier = match self.peek() {
            Token::Auto => "auto",
            Token::Register => "register",
            _ => unreachable!("caller checked storage-class token"),
        };
        self.advance();
        self.consume_alignment_specifiers()?;
        match self.peek() {
            Token::Int
            | Token::Char
            | Token::Bool
            | Token::Signed
            | Token::Unsigned
            | Token::Long
            | Token::Short
            | Token::Const
            | Token::Volatile
            | Token::Restrict
            | Token::Atomic => self.parse_var_decl(),
            Token::Ident(_) if self.current_alias().is_some() => self.parse_var_decl(),
            Token::Struct | Token::Union => self.parse_aggregate_var_decl(),
            token => Err(Self::error_at(
                format!("expected declaration after {specifier}, found {token:?}"),
                self.peek_located(),
            )),
        }
    }

    fn parse_var_decl(&mut self) -> CustResult<Stmt> {
        self.parse_var_decl_with_semi(true)
    }

    fn parse_aligned_decl(&mut self) -> CustResult<Stmt> {
        self.consume_alignment_specifiers()?;
        match self.peek() {
            Token::Int
            | Token::Char
            | Token::Bool
            | Token::Signed
            | Token::Unsigned
            | Token::Long
            | Token::Short
            | Token::Const
            | Token::Volatile
            | Token::Restrict
            | Token::Atomic => self.parse_var_decl(),
            Token::Ident(_) if self.current_alias().is_some() => self.parse_var_decl(),
            Token::Struct | Token::Union => self.parse_aggregate_var_decl(),
            token => Err(Self::error_at(
                format!("expected declaration after _Alignas specifier, found {token:?}"),
                self.peek_located(),
            )),
        }
    }

    fn parse_var_decl_with_semi(&mut self, require_semi: bool) -> CustResult<Stmt> {
        self.last_decl_had_initializer = false;
        self.consume_alignment_specifiers()?;
        let (leading_const, decl_type) =
            self.parse_const_qualified_decl_type("struct type name")?;
        let has_explicit_star = self.matches(&Token::Star);
        let post_star_const = has_explicit_star && self.consume_type_qualifiers();
        if matches!(decl_type, DeclType::Pointer { .. }) && has_explicit_star {
            return Err(Self::error_at(
                "pointer-to-pointer declarations are not supported".to_string(),
                self.previous(),
            ));
        }
        if has_explicit_star && self.check(&Token::Star) {
            return Err(Self::error_at(
                "pointer-to-pointer declarations are not supported".to_string(),
                self.peek_located(),
            ));
        }
        if !has_explicit_star
            && self.check(&Token::LParen)
            && matches!(self.peek_next(), Token::Star)
        {
            return Err(Self::error_at(
                "parenthesized pointer declarations are not supported".to_string(),
                self.peek_located(),
            ));
        }
        let is_pointer = has_explicit_star || matches!(decl_type, DeclType::Pointer { .. });
        let name = if has_explicit_star {
            match &decl_type {
                DeclType::Scalar(_) => self.expect_ident_after("pointer name after '*'")?,
                DeclType::Struct(_) => self.expect_ident_after("struct pointer name after '*'")?,
                DeclType::Array(_, _) => {
                    return Err(Self::error_at(
                        "pointer-to-array declarations are not supported".to_string(),
                        self.previous(),
                    ));
                }
                DeclType::Pointer { .. } => {
                    unreachable!("pointer aliases with explicit stars return above")
                }
            }
        } else {
            match &decl_type {
                DeclType::Scalar(_) => self.expect_ident_after("variable name after type")?,
                DeclType::Struct(_) => self.expect_ident_after("struct variable name")?,
                DeclType::Pointer { .. } => self.expect_ident_after("pointer name after type")?,
                DeclType::Array(_, _) => self.expect_ident_after("array name after type")?,
            }
        };
        if let DeclType::Array(pointee, len) = decl_type.clone() {
            if self.check(&Token::LBracket) {
                return Err(Self::error_at(
                    "multidimensional array declarations are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            match pointee {
                PointeeType::Scalar(ty) => {
                    let init = if self.matches(&Token::Assign) {
                        self.last_decl_had_initializer = true;
                        self.parse_array_initializer_or_string(&name, len, ty)?
                    } else {
                        Vec::new()
                    };
                    if require_semi {
                        self.expect_semicolon_after("array declaration")?;
                    }
                    return Ok(Stmt::ArrayDecl {
                        name,
                        elem_type: ty,
                        len,
                        init,
                        is_const: leading_const,
                    });
                }
                PointeeType::Struct(type_name) => {
                    let init = if self.matches(&Token::Assign) {
                        self.last_decl_had_initializer = true;
                        self.parse_struct_array_initializer(&name, &type_name, len)?
                    } else {
                        Vec::new()
                    };
                    if require_semi {
                        self.expect_semicolon_after("struct array declaration")?;
                    }
                    return Ok(Stmt::StructArrayDecl {
                        type_name,
                        name,
                        len,
                        init,
                        is_const: leading_const,
                    });
                }
            }
        }
        if is_pointer {
            let (is_const, points_to_const) = if has_explicit_star {
                (post_star_const, leading_const)
            } else {
                (leading_const, Self::decl_type_points_to_const(&decl_type))
            };
            if self.check(&Token::LBracket) {
                return Err(Self::error_at(
                    "pointer array declarations are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            let expr = if self.matches(&Token::Assign) {
                self.last_decl_had_initializer = true;
                self.parse_expr()?
            } else if self.check(&Token::Semi) {
                self.advance();
                return Ok(Stmt::PointerDecl {
                    name,
                    ty: Self::decl_type_to_pointee_type(&decl_type),
                    expr: Expr::Number(0),
                    is_const,
                    points_to_const,
                });
            } else {
                let context = match &decl_type {
                    DeclType::Scalar(_)
                    | DeclType::Pointer {
                        pointee: PointeeType::Scalar(_),
                        ..
                    } => "pointer declaration",
                    DeclType::Struct(_)
                    | DeclType::Pointer {
                        pointee: PointeeType::Struct(_),
                        ..
                    } => "struct pointer declaration",
                    DeclType::Array(_, _) => {
                        unreachable!("array aliases return before pointer declarations")
                    }
                };
                self.expect_assign_after(context)?;
                unreachable!("expect_assign_after only returns Ok after consuming '='")
            };
            if require_semi {
                let context = match &decl_type {
                    DeclType::Scalar(_)
                    | DeclType::Pointer {
                        pointee: PointeeType::Scalar(_),
                        ..
                    } => "pointer declaration",
                    DeclType::Struct(_)
                    | DeclType::Pointer {
                        pointee: PointeeType::Struct(_),
                        ..
                    } => "struct pointer declaration",
                    DeclType::Array(_, _) => {
                        unreachable!("array aliases return before pointer declarations")
                    }
                };
                self.expect_semicolon_after(context)?;
            }
            return Ok(Stmt::PointerDecl {
                name,
                ty: Self::decl_type_to_pointee_type(&decl_type),
                expr,
                is_const,
                points_to_const,
            });
        }
        if matches!(decl_type, DeclType::Struct(_)) {
            if self.matches(&Token::LBracket) {
                let len = self.expect_array_len()?;
                self.expect_closing_bracket_after("struct array length")?;
                if self.check(&Token::LBracket) {
                    return Err(Self::error_at(
                        "multidimensional array declarations are not supported".to_string(),
                        self.peek_located(),
                    ));
                }
                let DeclType::Struct(type_name) = &decl_type else {
                    unreachable!("struct declarations return above")
                };
                let init = if self.matches(&Token::Assign) {
                    self.last_decl_had_initializer = true;
                    self.parse_struct_array_initializer(&name, type_name, len)?
                } else {
                    Vec::new()
                };
                if require_semi {
                    self.expect_semicolon_after("struct array declaration")?;
                }
                if let DeclType::Struct(type_name) = decl_type {
                    return Ok(Stmt::StructArrayDecl {
                        type_name,
                        name,
                        len,
                        init,
                        is_const: leading_const,
                    });
                }
            }
            let init = if self.matches(&Token::Assign) {
                self.last_decl_had_initializer = true;
                let DeclType::Struct(type_name) = &decl_type else {
                    unreachable!("struct declarations return above")
                };
                if self.check(&Token::LBrace) {
                    Some(StructVarInitializer::Fields(
                        self.parse_struct_initializer(type_name)?,
                    ))
                } else {
                    Some(StructVarInitializer::Expr(self.parse_expr()?))
                }
            } else {
                None
            };
            if require_semi {
                self.expect_semicolon_after("struct variable declaration")?;
            }
            if let DeclType::Struct(type_name) = decl_type {
                return Ok(Stmt::StructVarDecl {
                    type_name,
                    name,
                    init,
                    is_const: leading_const,
                });
            }
        }
        let DeclType::Scalar(ty) = decl_type else {
            unreachable!("struct declarations return above")
        };
        if self.matches(&Token::LBracket) {
            let len = self.expect_array_len()?;
            self.expect_closing_bracket_after("array length")?;
            if self.check(&Token::LBracket) {
                return Err(Self::error_at(
                    "multidimensional array declarations are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            let init = if self.matches(&Token::Assign) {
                self.last_decl_had_initializer = true;
                self.parse_array_initializer_or_string(&name, len, ty)?
            } else {
                Vec::new()
            };
            if require_semi {
                self.expect_semicolon_after("array declaration")?;
            }
            return Ok(Stmt::ArrayDecl {
                name,
                elem_type: ty,
                len,
                init,
                is_const: leading_const,
            });
        }
        let expr = if self.matches(&Token::Assign) {
            self.last_decl_had_initializer = true;
            self.parse_scalar_initializer_expr(&format!("variable '{name}'"))?
        } else if self.check(&Token::Semi) {
            self.advance();
            return Ok(Stmt::VarDecl {
                name,
                ty,
                expr: Expr::Number(0),
                is_const: leading_const,
            });
        } else {
            self.expect_assign_after("variable declaration")?;
            unreachable!("expect_assign_after only returns Ok after consuming '='")
        };
        if require_semi {
            self.expect_semicolon_after("variable declaration")?;
        }
        Ok(Stmt::VarDecl {
            name,
            ty,
            expr,
            is_const: leading_const,
        })
    }

    fn parse_array_initializer_or_string(
        &mut self,
        name: &str,
        len: usize,
        elem_type: CType,
    ) -> CustResult<Vec<ArrayInitializer>> {
        if let Some(init) = self.parse_string_literal_array_initializer(name, len, elem_type)? {
            return Ok(init);
        }

        self.parse_array_initializer(name, len)
    }

    fn parse_string_literal_array_initializer(
        &mut self,
        name: &str,
        len: usize,
        elem_type: CType,
    ) -> CustResult<Option<Vec<ArrayInitializer>>> {
        let Token::StringLiteral(values) = self.peek().clone() else {
            return Ok(None);
        };
        self.advance();
        let values = self.concatenate_adjacent_string_literals(values);
        if elem_type != CType::Char {
            return Err(CustError::new(format!(
                "string literal initializer requires char array '{name}'"
            )));
        }
        let too_long =
            values.len() > len && !(values.len() == len + 1 && values.last() == Some(&0));
        if too_long {
            return Err(CustError::new(format!(
                "initializer string for char array '{name}' is too long"
            )));
        }
        Ok(Some(vec![ArrayInitializer::StringLiteral(values)]))
    }

    fn parse_array_initializer(
        &mut self,
        name: &str,
        len: usize,
    ) -> CustResult<Vec<ArrayInitializer>> {
        self.expect_opening_brace_after("array initializer")?;
        let mut values = Vec::new();
        let mut next_positional_index = 0usize;
        if self.matches(&Token::RBrace) {
            return Ok(values);
        }
        loop {
            if self.check(&Token::LBracket) {
                let index = self.parse_array_designator_index(name, len)?;
                self.expect_assign_after("array designator")?;
                let value =
                    self.parse_scalar_initializer_expr(&format!("array '{name}' element"))?;
                next_positional_index = index + 1;
                values.push(ArrayInitializer::Designated { index, value });
            } else {
                if next_positional_index == len {
                    return Err(CustError::new(format!(
                        "too many initializers for array '{name}'"
                    )));
                }
                values.push(ArrayInitializer::Expr(self.parse_scalar_initializer_expr(
                    &format!("array '{name}' element"),
                )?));
                next_positional_index += 1;
            }
            if self.matches(&Token::RBrace) {
                break;
            }
            if self.matches(&Token::Comma) {
                if self.matches(&Token::RBrace) {
                    break;
                }
                continue;
            }
            self.expect_closing_brace_after("array initializer")?;
        }
        Ok(values)
    }

    fn parse_array_compound_initializer(
        &mut self,
        len: Option<usize>,
        elem_type: CType,
    ) -> CustResult<Vec<ArrayInitializer>> {
        self.expect_opening_brace_after("array compound literal initializer")?;
        let mut values = Vec::new();
        let mut next_positional_index = 0usize;
        if self.matches(&Token::RBrace) {
            return Ok(values);
        }
        loop {
            if self.check(&Token::LBracket) {
                let index = match len {
                    Some(len) => self
                        .parse_array_designator_index_with_context("array compound literal", len)?,
                    None => self.parse_unbounded_array_designator_index()?,
                };
                self.expect_assign_after("array designator")?;
                let value = self.parse_scalar_initializer_expr("array compound literal element")?;
                next_positional_index = index + 1;
                values.push(ArrayInitializer::Designated { index, value });
            } else if let Token::StringLiteral(string_values) = self.peek().clone() {
                self.advance();
                let string_values = self.concatenate_adjacent_string_literals(string_values);
                if elem_type != CType::Char {
                    return Err(CustError::new(
                        "string literal initializer requires char array compound literal",
                    ));
                }
                let too_long = len.is_some_and(|len| {
                    string_values.len() > len
                        && !(string_values.len() == len + 1 && string_values.last() == Some(&0))
                });
                if too_long {
                    return Err(CustError::new(
                        "initializer string for char array compound literal is too long",
                    ));
                }
                next_positional_index = string_values.len();
                values.push(ArrayInitializer::StringLiteral(string_values));
            } else {
                if matches!(len, Some(len) if next_positional_index == len) {
                    return Err(CustError::new(
                        "too many initializers for array compound literal",
                    ));
                }
                values.push(ArrayInitializer::Expr(
                    self.parse_scalar_initializer_expr("array compound literal element")?,
                ));
                next_positional_index += 1;
            }
            if self.matches(&Token::RBrace) {
                break;
            }
            if self.matches(&Token::Comma) {
                if self.matches(&Token::RBrace) {
                    break;
                }
                continue;
            }
            self.expect_closing_brace_after("array compound literal initializer")?;
        }
        Ok(values)
    }

    fn parse_unbounded_array_designator_index(&mut self) -> CustResult<usize> {
        self.expect(Token::LBracket)?;
        let found = self.advance();
        let index = match &found.kind {
            Token::Number(value) if *value >= 0 => usize::try_from(*value).map_err(|_| {
                Self::error_at("array designator index is too large".to_string(), &found)
            })?,
            token => {
                return Err(Self::error_at(
                    format!("expected array designator index, found {token:?}"),
                    &found,
                ));
            }
        };
        self.expect_closing_bracket_after("array designator")?;
        Ok(index)
    }

    fn parse_array_designator_index(&mut self, name: &str, len: usize) -> CustResult<usize> {
        self.parse_array_designator_index_with_context(&format!("array '{name}'"), len)
    }

    fn parse_array_designator_index_with_context(
        &mut self,
        context: &str,
        len: usize,
    ) -> CustResult<usize> {
        self.expect(Token::LBracket)?;
        let found = self.advance();
        let index = match &found.kind {
            Token::Number(value) if *value >= 0 => usize::try_from(*value).map_err(|_| {
                Self::error_at("array designator index is too large".to_string(), &found)
            })?,
            token => {
                return Err(Self::error_at(
                    format!("expected array designator index, found {token:?}"),
                    &found,
                ));
            }
        };
        self.expect_closing_bracket_after("array designator")?;
        if index >= len {
            return Err(CustError::new(format!(
                "array designator index {index} out of bounds for {context}"
            )));
        }
        Ok(index)
    }

    fn parse_struct_initializer(&mut self, type_name: &str) -> CustResult<Vec<StructInitializer>> {
        let struct_type = self
            .struct_types
            .get(type_name)
            .cloned()
            .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'")))?;
        let fields = struct_type.fields.clone();
        let aggregate_keyword = struct_type.kind.keyword();
        self.expect_opening_brace_after(&format!("{aggregate_keyword} initializer"))?;
        let mut values = Vec::new();
        let mut next_positional_index = 0usize;
        if self.matches(&Token::RBrace) {
            return Ok(values);
        }
        loop {
            if self.matches(&Token::Dot) {
                let (field_index, field_name, value) =
                    self.parse_struct_designator_after_dot(type_name, &fields)?;
                next_positional_index = field_index + 1;
                values.push(StructInitializer::Designated {
                    field: field_name,
                    value: Box::new(value),
                });
            } else {
                if next_positional_index == fields.len()
                    || (struct_type.kind == AggregateKind::Union && next_positional_index > 0)
                {
                    return Err(CustError::new(format!(
                        "too many initializers for {aggregate_keyword} '{type_name}'"
                    )));
                }
                let field = &fields[next_positional_index];
                let value = self.parse_struct_initializer_value(field)?;
                values.push(StructInitializer::Designated {
                    field: field.name.clone(),
                    value: Box::new(value),
                });
                next_positional_index += 1;
            }
            if self.matches(&Token::RBrace) {
                break;
            }
            if self.matches(&Token::Comma) {
                if self.matches(&Token::RBrace) {
                    break;
                }
                continue;
            }
            self.expect_closing_brace_after("struct initializer")?;
        }
        Ok(values)
    }

    fn parse_struct_designator_after_dot(
        &mut self,
        type_name: &str,
        fields: &[StructFieldDef],
    ) -> CustResult<(usize, String, StructInitializer)> {
        let field_name = self.expect_ident_after("struct field name after '.'")?;
        let field_index = fields
            .iter()
            .position(|field| field.name == field_name)
            .ok_or_else(|| {
                CustError::new(format!("struct '{type_name}' has no field '{field_name}'"))
            })?;
        let field = &fields[field_index];
        let value = if self.matches(&Token::Dot) {
            match &field.ty {
                StructFieldType::Struct(nested_type) => {
                    let nested_fields = self
                        .struct_types
                        .get(nested_type)
                        .map(|struct_type| struct_type.fields.clone())
                        .ok_or_else(|| {
                            CustError::new(format!("undefined struct type '{nested_type}'"))
                        })?;
                    let (_, nested_field, nested_value) =
                        self.parse_struct_designator_after_dot(nested_type, &nested_fields)?;
                    StructInitializer::Struct(vec![StructInitializer::Designated {
                        field: nested_field,
                        value: Box::new(nested_value),
                    }])
                }
                _ => {
                    return Err(CustError::new(format!(
                        "field '{}' is not a struct for path designator",
                        field.name
                    )));
                }
            }
        } else if self.check(&Token::LBracket) {
            match &field.ty {
                StructFieldType::Array(_, len) => {
                    let index = self.parse_array_designator_index_with_context(
                        &format!("array field '{}'", field.name),
                        *len,
                    )?;
                    self.expect_assign_after("array designator")?;
                    StructInitializer::Array(vec![ArrayInitializer::Designated {
                        index,
                        value: self.parse_scalar_initializer_expr(&format!(
                            "array field '{}' element",
                            field.name
                        ))?,
                    }])
                }
                _ => {
                    return Err(CustError::new(format!(
                        "field '{}' is not an array for path designator",
                        field.name
                    )));
                }
            }
        } else {
            self.expect_assign_after("struct field designator")?;
            self.parse_struct_initializer_value(field)?
        };
        Ok((field_index, field_name, value))
    }

    fn parse_struct_initializer_value(
        &mut self,
        field: &StructFieldDef,
    ) -> CustResult<StructInitializer> {
        match &field.ty {
            StructFieldType::Array(elem_type, len) if self.check(&Token::LBrace) => Ok(
                StructInitializer::Array(self.parse_array_initializer(&field.name, *len)?),
            ),
            StructFieldType::Array(elem_type, len)
                if matches!(self.peek(), Token::StringLiteral(_)) =>
            {
                let init = self
                    .parse_string_literal_array_initializer(&field.name, *len, *elem_type)?
                    .expect("string literal token was checked before parsing");
                Ok(StructInitializer::Array(init))
            }
            StructFieldType::Struct(nested_type) if self.check(&Token::LBrace) => Ok(
                StructInitializer::Struct(self.parse_struct_initializer(nested_type)?),
            ),
            StructFieldType::StructArray(type_name, len) if self.check(&Token::LBrace) => {
                Ok(StructInitializer::StructArray(
                    self.parse_struct_array_initializer(&field.name, type_name, *len)?,
                ))
            }
            StructFieldType::Scalar(_) if self.check(&Token::LBrace) => {
                Ok(StructInitializer::Expr(
                    self.parse_scalar_initializer_expr(&format!("struct field '{}'", field.name))?,
                ))
            }
            _ => Ok(StructInitializer::Expr(self.parse_assignment_expr()?)),
        }
    }

    fn parse_scalar_initializer_expr(&mut self, context: &str) -> CustResult<Expr> {
        if !self.check(&Token::LBrace) {
            return self.parse_assignment_expr();
        }
        self.expect_opening_brace_after(context)?;
        let expr = self.parse_assignment_expr()?;
        if self.matches(&Token::Comma) {
            if self.matches(&Token::RBrace) {
                return Ok(expr);
            }
            return Err(CustError::new(format!(
                "too many initializers for {context}"
            )));
        }
        self.expect_closing_brace_after(context)?;
        Ok(expr)
    }

    fn parse_struct_array_initializer(
        &mut self,
        name: &str,
        type_name: &str,
        len: usize,
    ) -> CustResult<Vec<StructArrayInitializer>> {
        self.expect_opening_brace_after("struct array initializer")?;
        let mut values = Vec::new();
        let mut next_positional_index = 0usize;
        if self.matches(&Token::RBrace) {
            return Ok(values);
        }
        loop {
            if self.check(&Token::LBracket) {
                let index = self.parse_array_designator_index_with_context(
                    &format!("struct array '{name}'"),
                    len,
                )?;
                self.expect_assign_after("array designator")?;
                values.push(StructArrayInitializer::Designated {
                    index,
                    value: self.parse_struct_initializer(type_name)?,
                });
                next_positional_index = index + 1;
            } else {
                if next_positional_index == len {
                    return Err(CustError::new(format!(
                        "too many initializers for struct array '{name}'"
                    )));
                }
                values.push(StructArrayInitializer::Element(
                    self.parse_struct_initializer(type_name)?,
                ));
                next_positional_index += 1;
            }
            if self.matches(&Token::RBrace) {
                break;
            }
            if self.matches(&Token::Comma) {
                if self.matches(&Token::RBrace) {
                    break;
                }
                continue;
            }
            self.expect_closing_brace_after("struct array initializer")?;
        }
        Ok(values)
    }

    fn parse_aggregate_array_compound_initializer(
        &mut self,
        type_name: &str,
        len: Option<usize>,
    ) -> CustResult<Vec<StructArrayInitializer>> {
        self.expect_opening_brace_after("aggregate array compound literal initializer")?;
        let mut values = Vec::new();
        let mut next_positional_index = 0usize;
        if self.matches(&Token::RBrace) {
            return Ok(values);
        }
        loop {
            if self.check(&Token::LBracket) {
                let index = match len {
                    Some(len) => self.parse_array_designator_index_with_context(
                        "aggregate array compound literal",
                        len,
                    )?,
                    None => self.parse_unbounded_array_designator_index()?,
                };
                self.expect_assign_after("array designator")?;
                values.push(StructArrayInitializer::Designated {
                    index,
                    value: self.parse_struct_initializer(type_name)?,
                });
                next_positional_index = index + 1;
            } else {
                if matches!(len, Some(len) if next_positional_index == len) {
                    return Err(CustError::new(
                        "too many initializers for aggregate array compound literal",
                    ));
                }
                values.push(StructArrayInitializer::Element(
                    self.parse_struct_initializer(type_name)?,
                ));
                next_positional_index += 1;
            }
            if self.matches(&Token::RBrace) {
                break;
            }
            if self.matches(&Token::Comma) {
                if self.matches(&Token::RBrace) {
                    break;
                }
                continue;
            }
            self.expect_closing_brace_after("aggregate array compound literal initializer")?;
        }
        Ok(values)
    }

    fn parse_aggregate_definition(&mut self) -> CustResult<()> {
        self.parse_aggregate_definition_body(true, false)?;
        Ok(())
    }

    fn starts_typedef_aggregate_definition(&self) -> bool {
        matches!(
            (
                self.peek(),
                self.tokens.get(self.pos + 1).map(|token| &token.kind),
                self.tokens.get(self.pos + 2).map(|token| &token.kind)
            ),
            (Token::Struct | Token::Union, Some(Token::LBrace), _)
                | (
                    Token::Struct | Token::Union,
                    Some(Token::Ident(_)),
                    Some(Token::LBrace)
                )
        )
    }

    fn starts_typedef_enum_definition(&self) -> bool {
        matches!(
            (
                self.peek(),
                self.tokens.get(self.pos + 1).map(|token| &token.kind),
                self.tokens.get(self.pos + 2).map(|token| &token.kind)
            ),
            (Token::Enum, Some(Token::LBrace), _)
                | (Token::Enum, Some(Token::Ident(_)), Some(Token::LBrace))
        )
    }

    fn parse_aggregate_definition_body(
        &mut self,
        require_semicolon: bool,
        allow_anonymous: bool,
    ) -> CustResult<(String, AggregateKind, bool)> {
        let kind = match self.advance().kind {
            Token::Struct => AggregateKind::Struct,
            Token::Union => AggregateKind::Union,
            token => unreachable!("expected aggregate keyword, found {token:?}"),
        };
        let keyword = kind.keyword();
        let (type_name, is_anonymous) = if allow_anonymous && self.check(&Token::LBrace) {
            let anonymous_id = self.next_aggregate_type_id;
            self.next_aggregate_type_id += 1;
            (format!("__anonymous_{keyword}#{anonymous_id}"), true)
        } else {
            (self.expect_ident_after(&format!("{keyword} name"))?, false)
        };
        let opening_brace_context = if is_anonymous {
            "anonymous aggregate typedef".to_string()
        } else {
            format!("{keyword} name")
        };
        self.expect_opening_brace_after(&opening_brace_context)?;
        let internal_type_name = if is_anonymous {
            type_name.clone()
        } else {
            if self
                .aggregate_type_scopes
                .last()
                .expect("parser always has an aggregate type scope")
                .contains_key(&type_name)
            {
                return Err(CustError::new(format!(
                    "{keyword} '{type_name}' already declared"
                )));
            }
            if self.struct_types.contains_key(&type_name) {
                let name = format!("{}#{}", type_name, self.next_aggregate_type_id);
                self.next_aggregate_type_id += 1;
                name
            } else {
                type_name.clone()
            }
        };
        if !is_anonymous {
            self.aggregate_type_scopes
                .last_mut()
                .expect("parser always has an aggregate type scope")
                .insert(type_name.clone(), internal_type_name.clone());
        }
        let mut fields = Vec::new();
        let mut names = HashSet::new();
        while !self.check(&Token::RBrace) {
            if self.check(&Token::Eof) {
                return Err(Self::error_at(
                    format!("unterminated {keyword} declaration for '{type_name}'"),
                    self.peek_located(),
                ));
            }
            self.consume_alignment_specifiers()?;
            let leading_const = self.consume_type_qualifiers();
            let decl_type = if matches!(self.peek(), Token::Struct | Token::Union) {
                let field_kind = match self.advance().kind {
                    Token::Struct => AggregateKind::Struct,
                    Token::Union => AggregateKind::Union,
                    token => unreachable!("expected aggregate field type, found {token:?}"),
                };
                let field_keyword = field_kind.keyword();
                let field_type_name =
                    self.expect_ident_after(&format!("{field_keyword} field type"))?;
                let Some(field_internal_type_name) = self.resolve_aggregate_type(&field_type_name)
                else {
                    return Err(CustError::new(format!(
                        "undefined {field_keyword} type '{field_type_name}'"
                    )));
                };
                DeclType::Struct(field_internal_type_name)
            } else {
                self.parse_decl_type(&format!("{keyword} field type"))?
            };
            let has_explicit_star = self.matches(&Token::Star);
            let post_star_const = has_explicit_star && self.consume_type_qualifiers();
            if has_explicit_star && self.check(&Token::Star) {
                return Err(Self::error_at(
                    format!("pointer-to-pointer {keyword} fields are not supported"),
                    self.peek_located(),
                ));
            }
            let is_pointer = has_explicit_star || matches!(decl_type, DeclType::Pointer { .. });
            let (is_const, points_to_const) = if is_pointer {
                if has_explicit_star {
                    (post_star_const, leading_const)
                } else {
                    (leading_const, Self::decl_type_points_to_const(&decl_type))
                }
            } else {
                (leading_const, false)
            };
            let ty = match decl_type {
                DeclType::Scalar(ty) if has_explicit_star => {
                    StructFieldType::Pointer(PointeeType::Scalar(ty))
                }
                DeclType::Struct(type_name) if has_explicit_star => {
                    StructFieldType::Pointer(PointeeType::Struct(type_name))
                }
                DeclType::Pointer { .. } if has_explicit_star => {
                    return Err(Self::error_at(
                        format!("pointer-to-pointer {keyword} fields are not supported"),
                        self.previous(),
                    ));
                }
                DeclType::Pointer { pointee, .. } => StructFieldType::Pointer(pointee),
                DeclType::Array(PointeeType::Scalar(ty), len) => StructFieldType::Array(ty, len),
                DeclType::Array(PointeeType::Struct(type_name), len) => {
                    StructFieldType::StructArray(type_name, len)
                }
                DeclType::Scalar(ty) => StructFieldType::Scalar(ty),
                DeclType::Struct(type_name) => StructFieldType::Struct(type_name),
            };
            let name = self.expect_ident_after(&format!("{keyword} field name after type"))?;
            if !names.insert(name.clone()) {
                return Err(Self::error_at(
                    format!("duplicate {keyword} field '{name}'"),
                    self.previous(),
                ));
            }
            let ty = if self.matches(&Token::LBracket) {
                match ty {
                    StructFieldType::Scalar(elem_type) => {
                        let len = self.expect_array_len()?;
                        self.expect_closing_bracket_after("struct array field length")?;
                        if self.check(&Token::LBracket) {
                            return Err(Self::error_at(
                                format!(
                                    "multidimensional array {keyword} fields are not supported"
                                ),
                                self.peek_located(),
                            ));
                        }
                        StructFieldType::Array(elem_type, len)
                    }
                    StructFieldType::Struct(type_name) => {
                        let len = self.expect_array_len()?;
                        self.expect_closing_bracket_after("struct array field length")?;
                        if self.check(&Token::LBracket) {
                            return Err(Self::error_at(
                                format!(
                                    "multidimensional array {keyword} fields are not supported"
                                ),
                                self.peek_located(),
                            ));
                        }
                        StructFieldType::StructArray(type_name, len)
                    }
                    StructFieldType::Pointer(_) => {
                        return Err(Self::error_at(
                            format!("pointer array {keyword} fields are not supported"),
                            self.previous(),
                        ));
                    }
                    StructFieldType::Array(_, _) | StructFieldType::StructArray(_, _) => {
                        unreachable!("array field not built yet")
                    }
                }
            } else {
                ty
            };
            self.expect_semicolon_after("struct field declaration")?;
            fields.push(StructFieldDef {
                name,
                ty,
                is_const,
                points_to_const,
            });
        }
        self.expect(Token::RBrace)?;
        if require_semicolon {
            self.expect_semicolon_after(&format!("{keyword} declaration"))?;
        }
        self.struct_types.insert(
            internal_type_name.clone(),
            StructTypeDef {
                fields,
                kind,
                display_name: type_name.clone(),
            },
        );
        Ok((internal_type_name, kind, is_anonymous))
    }

    fn parse_aggregate_var_decl(&mut self) -> CustResult<Stmt> {
        self.last_decl_had_initializer = false;
        self.consume_alignment_specifiers()?;
        let kind = match self.advance().kind {
            Token::Struct => AggregateKind::Struct,
            Token::Union => AggregateKind::Union,
            token => unreachable!("expected aggregate keyword, found {token:?}"),
        };
        let keyword = kind.keyword();
        let type_name = self.expect_ident_after(&format!("{keyword} type name"))?;
        let Some(internal_type_name) = self.resolve_aggregate_type(&type_name) else {
            return Err(CustError::new(format!(
                "undefined struct type '{type_name}'"
            )));
        };
        let type_name = internal_type_name;
        let points_to_const = self.consume_type_qualifiers();
        if self.matches(&Token::Star) {
            let is_const = self.consume_type_qualifiers();
            let name = self.expect_ident_after("struct pointer name after '*'")?;
            let expr = if self.matches(&Token::Assign) {
                self.last_decl_had_initializer = true;
                self.parse_expr()?
            } else if self.check(&Token::Semi) {
                Expr::Number(0)
            } else {
                self.expect_assign_after("struct pointer declaration")?;
                unreachable!("expect_assign_after only returns Ok after consuming '='")
            };
            self.expect_semicolon_after("struct pointer declaration")?;
            return Ok(Stmt::PointerDecl {
                name,
                ty: PointeeType::Struct(type_name),
                expr,
                is_const,
                points_to_const,
            });
        }
        let name = self.expect_ident_after("struct variable name")?;
        if self.matches(&Token::LBracket) {
            let len = self.expect_array_len()?;
            self.expect_closing_bracket_after("struct array length")?;
            let init = if self.matches(&Token::Assign) {
                self.last_decl_had_initializer = true;
                self.parse_struct_array_initializer(&name, &type_name, len)?
            } else {
                Vec::new()
            };
            self.expect_semicolon_after("struct array declaration")?;
            return Ok(Stmt::StructArrayDecl {
                type_name,
                name,
                len,
                init,
                is_const: points_to_const,
            });
        }
        let init = if self.matches(&Token::Assign) {
            self.last_decl_had_initializer = true;
            if self.check(&Token::LBrace) {
                Some(StructVarInitializer::Fields(
                    self.parse_struct_initializer(&type_name)?,
                ))
            } else {
                Some(StructVarInitializer::Expr(self.parse_expr()?))
            }
        } else {
            None
        };
        self.expect_semicolon_after("struct variable declaration")?;
        Ok(Stmt::StructVarDecl {
            type_name,
            name,
            init,
            is_const: points_to_const,
        })
    }

    fn parse_enum_decl(&mut self) -> CustResult<Stmt> {
        let constants = self.parse_enum_decl_body(true)?;
        Ok(Stmt::EnumDecl { constants })
    }

    fn parse_enum_decl_body(&mut self, require_semicolon: bool) -> CustResult<Vec<EnumConstant>> {
        self.expect(Token::Enum)?;
        let enum_tag = if let Token::Ident(name) = self.peek().clone() {
            self.advance();
            Some(name)
        } else {
            None
        };
        self.expect_opening_brace_after("enum")?;

        let mut constants = Vec::new();
        let mut local_constants = HashMap::new();
        let mut names = HashSet::new();
        let mut next_value = 0;
        while !self.check(&Token::RBrace) {
            if self.check(&Token::Eof) {
                let eof = self.peek_located().clone();
                return Err(Self::error_at(
                    "unterminated enum declaration".to_string(),
                    &eof,
                ));
            }

            let name_token = self.advance();
            let name = match name_token.kind.clone() {
                Token::Ident(name) => name,
                token => {
                    return Err(Self::error_at(
                        format!("expected enum constant name, found {token:?}"),
                        &name_token,
                    ));
                }
            };
            if !names.insert(name.clone()) {
                return Err(Self::error_at(
                    format!("duplicate enum constant '{name}'"),
                    &name_token,
                ));
            }

            let value = if self.matches(&Token::Assign) {
                self.last_decl_had_initializer = true;
                self.parse_enum_constant_value(&local_constants)?
            } else {
                next_value
            };
            next_value = value + 1;
            local_constants.insert(name.clone(), value);
            constants.push(EnumConstant { name, value });

            if self.matches(&Token::Comma) {
                if self.check(&Token::RBrace) {
                    break;
                }
            } else if self.check(&Token::RBrace) {
                break;
            } else {
                return Err(Self::error_at(
                    format!(
                        "expected ',' or '}}' after enum constant, found {:?}",
                        self.peek()
                    ),
                    self.peek_located(),
                ));
            }
        }
        self.expect(Token::RBrace)?;
        if require_semicolon {
            self.expect_semicolon_after("enum declaration")?;
        }
        match enum_tag {
            Some(enum_tag)
                if !self
                    .enum_type_scopes
                    .last_mut()
                    .expect("parser always has an enum type scope")
                    .insert(enum_tag.clone()) =>
            {
                return Err(CustError::new(format!(
                    "enum '{enum_tag}' already declared"
                )));
            }
            _ => {}
        }
        self.enum_constant_scopes
            .last_mut()
            .expect("parser always has an enum constant scope")
            .extend(local_constants);
        Ok(constants)
    }

    fn parse_enum_constant_value(
        &mut self,
        local_constants: &HashMap<String, i64>,
    ) -> CustResult<i64> {
        let (value, _) = self.parse_integer_constant_expr(
            local_constants,
            "expected integer constant after enum constant '='",
        )?;
        Ok(value)
    }

    fn parse_integer_constant_expr(
        &mut self,
        local_constants: &HashMap<String, i64>,
        context: &str,
    ) -> CustResult<(i64, LocatedToken)> {
        self.parse_integer_constant_bitwise_or(local_constants, context)
    }

    fn parse_integer_constant_bitwise_or(
        &mut self,
        local_constants: &HashMap<String, i64>,
        context: &str,
    ) -> CustResult<(i64, LocatedToken)> {
        let (mut value, first_token) =
            self.parse_integer_constant_bitwise_xor(local_constants, context)?;
        while self.matches(&Token::Pipe) {
            let (rhs, _) = self.parse_integer_constant_bitwise_xor(
                local_constants,
                "expected integer constant in integer constant expression",
            )?;
            value |= rhs;
        }
        Ok((value, first_token))
    }

    fn parse_integer_constant_bitwise_xor(
        &mut self,
        local_constants: &HashMap<String, i64>,
        context: &str,
    ) -> CustResult<(i64, LocatedToken)> {
        let (mut value, first_token) =
            self.parse_integer_constant_bitwise_and(local_constants, context)?;
        while self.matches(&Token::Caret) {
            let (rhs, _) = self.parse_integer_constant_bitwise_and(
                local_constants,
                "expected integer constant in integer constant expression",
            )?;
            value ^= rhs;
        }
        Ok((value, first_token))
    }

    fn parse_integer_constant_bitwise_and(
        &mut self,
        local_constants: &HashMap<String, i64>,
        context: &str,
    ) -> CustResult<(i64, LocatedToken)> {
        let (mut value, first_token) =
            self.parse_integer_constant_shift(local_constants, context)?;
        while self.matches(&Token::Amp) {
            let (rhs, _) = self.parse_integer_constant_shift(
                local_constants,
                "expected integer constant in integer constant expression",
            )?;
            value &= rhs;
        }
        Ok((value, first_token))
    }

    fn parse_integer_constant_shift(
        &mut self,
        local_constants: &HashMap<String, i64>,
        context: &str,
    ) -> CustResult<(i64, LocatedToken)> {
        let (mut value, first_token) =
            self.parse_integer_constant_additive(local_constants, context)?;
        while self.matches(&Token::ShiftLeft) || self.matches(&Token::ShiftRight) {
            let op = self.previous().kind.clone();
            let op_token = self.previous().clone();
            let (rhs, _) = self.parse_integer_constant_additive(
                local_constants,
                "expected integer constant in integer constant expression",
            )?;
            if rhs < 0 {
                return Err(Self::error_at(
                    "shift count must be non-negative".to_string(),
                    &op_token,
                ));
            }
            match op {
                Token::ShiftLeft => value <<= rhs,
                Token::ShiftRight => value >>= rhs,
                _ => unreachable!("only shift operators are matched above"),
            }
        }
        Ok((value, first_token))
    }

    fn parse_integer_constant_additive(
        &mut self,
        local_constants: &HashMap<String, i64>,
        context: &str,
    ) -> CustResult<(i64, LocatedToken)> {
        let (mut value, first_token) =
            self.parse_integer_constant_multiplicative(local_constants, context)?;
        while self.matches(&Token::Plus) || self.matches(&Token::Minus) {
            let op = self.previous().kind.clone();
            let (rhs, _) = self.parse_integer_constant_multiplicative(
                local_constants,
                "expected integer constant in integer constant expression",
            )?;
            match op {
                Token::Plus => value += rhs,
                Token::Minus => value -= rhs,
                _ => unreachable!("only plus/minus are matched above"),
            }
        }
        Ok((value, first_token))
    }

    fn parse_integer_constant_multiplicative(
        &mut self,
        local_constants: &HashMap<String, i64>,
        context: &str,
    ) -> CustResult<(i64, LocatedToken)> {
        let (mut value, first_token) =
            self.parse_integer_constant_unary(local_constants, context)?;
        while self.matches(&Token::Star)
            || self.matches(&Token::Slash)
            || self.matches(&Token::Percent)
        {
            let op = self.previous().kind.clone();
            let op_token = self.previous().clone();
            let (rhs, _) = self.parse_integer_constant_unary(
                local_constants,
                "expected integer constant in integer constant expression",
            )?;
            match op {
                Token::Star => value *= rhs,
                Token::Slash => {
                    if rhs == 0 {
                        return Err(Self::error_at("division by zero".to_string(), &op_token));
                    }
                    value /= rhs;
                }
                Token::Percent => {
                    if rhs == 0 {
                        return Err(Self::error_at("division by zero".to_string(), &op_token));
                    }
                    value %= rhs;
                }
                _ => unreachable!("only multiplicative operators are matched above"),
            }
        }
        Ok((value, first_token))
    }

    fn parse_integer_constant_unary(
        &mut self,
        local_constants: &HashMap<String, i64>,
        context: &str,
    ) -> CustResult<(i64, LocatedToken)> {
        if self.matches(&Token::Plus)
            || self.matches(&Token::Minus)
            || self.matches(&Token::Tilde)
            || self.matches(&Token::Bang)
        {
            let op_token = self.previous().clone();
            let (value, _) = self.parse_integer_constant_unary(
                local_constants,
                "expected integer constant in integer constant expression",
            )?;
            let value = match op_token.kind {
                Token::Plus => value,
                Token::Minus => -value,
                Token::Tilde => !value,
                Token::Bang => i64::from(value == 0),
                _ => unreachable!("only unary constant operators are matched above"),
            };
            return Ok((value, op_token));
        }
        self.parse_integer_constant_primary(local_constants, context)
    }

    fn parse_integer_constant_primary(
        &mut self,
        local_constants: &HashMap<String, i64>,
        context: &str,
    ) -> CustResult<(i64, LocatedToken)> {
        if self.matches(&Token::LParen) {
            let opening = self.previous().clone();
            let (value, _) = self.parse_integer_constant_expr(
                local_constants,
                "expected integer constant in parenthesized integer constant expression",
            )?;
            self.expect_closing_paren_after("integer constant expression")?;
            return Ok((value, opening));
        }

        let found = self.advance();
        match &found.kind {
            Token::Number(value) => Ok((*value, found)),
            Token::Ident(name) => local_constants
                .get(name)
                .copied()
                .or_else(|| self.lookup_enum_constant(name))
                .map(|value| (value, found.clone()))
                .ok_or_else(|| {
                    Self::error_at(format!("{context}, found {:?}", found.kind), &found)
                }),
            token => Err(Self::error_at(
                format!("{context}, found {token:?}"),
                &found,
            )),
        }
    }

    fn expect_array_len(&mut self) -> CustResult<usize> {
        let found = self.advance();
        match &found.kind {
            Token::Number(value) if *value > 0 => usize::try_from(*value)
                .map_err(|_| Self::error_at("array length is too large".to_string(), &found)),
            Token::Number(_) | Token::Minus => Err(Self::error_at(
                "array length must be positive".to_string(),
                &found,
            )),
            Token::RBracket => Err(Self::error_at(
                "expected array length before ']'".to_string(),
                &found,
            )),
            token => Err(Self::error_at(
                format!("expected array length, found {token:?}"),
                &found,
            )),
        }
    }

    fn parse_assign(&mut self) -> CustResult<Stmt> {
        self.parse_assign_with_semi(true)
    }

    fn parse_deref_assign(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Star)?;
        let pointer = self.parse_unary()?;
        if let Some(op) = self.compound_assignment_op() {
            self.advance();
            let value = self.parse_expr()?;
            self.expect_semicolon_after("assignment")?;
            return Ok(Stmt::Expr(Expr::DerefCompoundSet {
                pointer: Box::new(pointer),
                op,
                value: Box::new(value),
            }));
        }
        self.expect_assign_after("assignment")?;
        let value = self.parse_expr()?;
        self.expect_semicolon_after("assignment")?;
        Ok(Stmt::DerefAssign { pointer, value })
    }

    fn parse_assign_with_semi(&mut self, require_semi: bool) -> CustResult<Stmt> {
        let name = self.expect_ident()?;
        if self.matches(&Token::Dot) {
            let mut fields = vec![self.expect_ident_after("struct field name after '.'")?];
            while self.matches(&Token::Dot) {
                fields.push(self.expect_ident_after("struct field name after '.'")?);
            }
            if self.matches(&Token::LBracket) {
                let index = self.parse_index_expr()?;
                self.expect_closing_bracket_after("array index")?;
                if let Some(op) = self.compound_assignment_op() {
                    self.advance();
                    let value = self.parse_expr()?;
                    if require_semi {
                        self.expect_semicolon_after("assignment")?;
                    }
                    return Ok(Stmt::Expr(Expr::StructArrayCompoundSet {
                        name,
                        fields,
                        index: Box::new(index),
                        op,
                        value: Box::new(value),
                    }));
                }
                self.expect_assign_after("struct array field assignment")?;
                let value = self.parse_expr()?;
                if require_semi {
                    self.expect_semicolon_after("assignment")?;
                }
                return Ok(Stmt::Expr(Expr::StructArraySet {
                    name,
                    fields,
                    index: Box::new(index),
                    value: Box::new(value),
                }));
            }
            if let Some(op) = self.compound_assignment_op() {
                self.advance();
                let value = self.parse_expr()?;
                if require_semi {
                    self.expect_semicolon_after("assignment")?;
                }
                return Ok(Stmt::Expr(Expr::StructCompoundSet {
                    name,
                    fields,
                    op,
                    value: Box::new(value),
                }));
            }
            self.expect_assign_after("struct field assignment")?;
            let value = self.parse_expr()?;
            if require_semi {
                self.expect_semicolon_after("assignment")?;
            }
            return Ok(Stmt::StructAssign {
                name,
                fields,
                value,
            });
        }
        if self.matches(&Token::LBracket) {
            let index = self.parse_index_expr()?;
            self.expect_closing_bracket_after("array index")?;
            if let Some(op) = self.compound_assignment_op() {
                self.advance();
                let value = self.parse_expr()?;
                if require_semi {
                    self.expect_semicolon_after("assignment")?;
                }
                return Ok(Stmt::Expr(Expr::ArrayCompoundSet {
                    name,
                    index: Box::new(index),
                    op,
                    value: Box::new(value),
                }));
            }
            self.expect_assign_after("assignment")?;
            let value = self.parse_expr()?;
            if require_semi {
                self.expect_semicolon_after("assignment")?;
            }
            return Ok(Stmt::ArrayAssign { name, index, value });
        }
        if let Some(op) = self.compound_assignment_op() {
            self.advance();
            let value = self.parse_expr()?;
            if require_semi {
                self.expect_semicolon_after("assignment")?;
            }
            return Ok(Stmt::Expr(Expr::CompoundAssign {
                name,
                op,
                value: Box::new(value),
            }));
        }
        self.expect_assign_after("assignment")?;
        let expr = self.parse_expr()?;
        if require_semi {
            self.expect_semicolon_after("assignment")?;
        }
        Ok(Stmt::Assign(name, expr))
    }

    fn parse_expr_stmt_with_semi(&mut self, require_semi: bool) -> CustResult<Stmt> {
        let expr = self.parse_expr()?;
        if require_semi {
            self.expect_semicolon_after("expression statement")?;
        }
        Ok(Stmt::Expr(expr))
    }

    fn parse_return(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Return)?;
        if self.matches(&Token::Semi) {
            return Ok(Stmt::Return(None));
        }
        let expr = self.parse_expr()?;
        self.expect_semicolon_after("return statement")?;
        Ok(Stmt::Return(Some(expr)))
    }

    fn parse_break(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Break)?;
        self.expect_semicolon_after("break statement")?;
        Ok(Stmt::Break)
    }

    fn parse_continue(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Continue)?;
        self.expect_semicolon_after("continue statement")?;
        Ok(Stmt::Continue)
    }

    fn parse_if(&mut self) -> CustResult<Stmt> {
        self.expect(Token::If)?;
        self.expect_opening_paren_after("if")?;
        let cond = self.parse_expr()?;
        self.expect_closing_paren_after("if condition")?;
        let then_branch = self.parse_control_body_after("if condition")?;
        let else_branch = if self.matches(&Token::Else) {
            self.parse_control_body_after("else")?
        } else {
            Vec::new()
        };
        Ok(Stmt::If {
            cond,
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> CustResult<Stmt> {
        self.expect(Token::While)?;
        self.expect_opening_paren_after("while")?;
        let cond = self.parse_expr()?;
        self.expect_closing_paren_after("while condition")?;
        let body = self.parse_control_body_after("while condition")?;
        Ok(Stmt::While { cond, body })
    }

    fn parse_do_while(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Do)?;
        let body = self.parse_control_body_after("do")?;
        self.expect_keyword_after(&Token::While, "do body")?;
        self.expect_opening_paren_after("do-while")?;
        let cond = self.parse_expr()?;
        self.expect_closing_paren_after("do-while condition")?;
        self.expect_semicolon_after("do-while condition")?;
        Ok(Stmt::DoWhile { body, cond })
    }

    fn parse_for(&mut self) -> CustResult<Stmt> {
        self.expect(Token::For)?;
        self.expect_opening_paren_after("for")?;

        let init = if self.matches(&Token::Semi) {
            None
        } else if matches!(self.peek(), Token::Auto | Token::Register) {
            Some(Box::new(self.parse_auto_register_local_decl()?))
        } else if matches!(
            self.peek(),
            Token::Int
                | Token::Char
                | Token::Bool
                | Token::Signed
                | Token::Unsigned
                | Token::Long
                | Token::Short
                | Token::Const
                | Token::Volatile
                | Token::Restrict
                | Token::Atomic
                | Token::Alignas
        ) || self.current_alias().is_some()
        {
            Some(Box::new(self.parse_var_decl()?))
        } else if self.starts_assignment_stmt() {
            Some(Box::new(self.parse_assign()?))
        } else if self.starts_expr() {
            Some(Box::new(self.parse_expr_stmt_with_semi(true)?))
        } else if matches!(self.peek(), Token::Break | Token::Continue) {
            let control = self.loop_control_keyword();
            return Err(Self::error_at(
                format!("{control} is not allowed in for initializer"),
                self.peek_located(),
            ));
        } else {
            return Err(Self::error_at(
                format!("unexpected token in for initializer: {:?}", self.peek()),
                self.peek_located(),
            ));
        };

        let cond = if self.matches(&Token::Semi) {
            None
        } else {
            let expr = self.parse_expr()?;
            self.expect_semicolon_after("for condition")?;
            Some(expr)
        };

        let increment = if self.check(&Token::RParen) {
            None
        } else if self.starts_assignment_stmt() {
            Some(Box::new(self.parse_assign_with_semi(false)?))
        } else if self.starts_expr() {
            Some(Box::new(self.parse_expr_stmt_with_semi(false)?))
        } else if matches!(self.peek(), Token::Break | Token::Continue) {
            let control = self.loop_control_keyword();
            return Err(Self::error_at(
                format!("{control} is not allowed in for increment"),
                self.peek_located(),
            ));
        } else {
            return Err(Self::error_at(
                format!("unexpected token in for increment: {:?}", self.peek()),
                self.peek_located(),
            ));
        };
        self.expect_closing_paren_after("for clauses")?;

        let body = self.parse_control_body_after("for clauses")?;
        Ok(Stmt::For {
            init,
            cond,
            increment,
            body,
        })
    }

    fn parse_switch(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Switch)?;
        self.expect_opening_paren_after("switch")?;
        let expr = self.parse_expr()?;
        self.expect_closing_paren_after("switch expression")?;
        self.expect_opening_brace_after("switch expression")?;

        let mut sections = Vec::new();
        let mut seen_cases = HashSet::new();
        let mut seen_default = false;
        while !self.check(&Token::RBrace) {
            if self.check(&Token::Eof) {
                let eof = self.peek_located().clone();
                return Err(Self::error_at(
                    "unterminated block after switch expression".to_string(),
                    &eof,
                ));
            }

            let label = if self.matches(&Token::Case) {
                let (value, value_token) = self.parse_switch_case_value()?;
                if !seen_cases.insert(value) {
                    return Err(Self::error_at(
                        format!("duplicate switch case label {value}"),
                        &value_token,
                    ));
                }
                self.expect_colon_after("switch case label")?;
                SwitchLabel::Case(value)
            } else if self.matches(&Token::Default) {
                let default_token = self.previous().clone();
                if seen_default {
                    return Err(Self::error_at(
                        "duplicate switch default label".to_string(),
                        &default_token,
                    ));
                }
                seen_default = true;
                self.expect_colon_after("switch default label")?;
                SwitchLabel::Default
            } else {
                return Err(Self::error_at(
                    format!(
                        "expected switch case or default label, found {:?}",
                        self.peek()
                    ),
                    self.peek_located(),
                ));
            };

            let mut statements = Vec::new();
            while !matches!(
                self.peek(),
                Token::Case | Token::Default | Token::RBrace | Token::Eof
            ) {
                statements.push(self.parse_stmt()?);
            }
            sections.push(SwitchSection { label, statements });
        }

        self.expect(Token::RBrace)?;
        Ok(Stmt::Switch { expr, sections })
    }

    fn parse_switch_case_value(&mut self) -> CustResult<(i64, LocatedToken)> {
        let empty_constants = HashMap::new();
        self.parse_integer_constant_expr(
            &empty_constants,
            "expected integer constant after switch case",
        )
    }

    fn parse_expr(&mut self) -> CustResult<Expr> {
        self.parse_comma_expr()
    }

    fn parse_comma_expr(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_assignment_expr()?;
        while self.matches(&Token::Comma) {
            let rhs = self.parse_assignment_expr()?;
            expr = Expr::Comma(Box::new(expr), Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_index_expr(&mut self) -> CustResult<Expr> {
        self.parse_logical_or()
    }

    fn parse_assignment_expr(&mut self) -> CustResult<Expr> {
        let target = self.parse_conditional_expr()?;
        if let Some(op) = self.compound_assignment_op() {
            let operator = self.advance();
            let value = self.parse_assignment_expr()?;
            match target {
                Expr::Var(name) => Ok(Expr::CompoundAssign {
                    name,
                    op,
                    value: Box::new(value),
                }),
                Expr::ArrayGet { name, index } => Ok(Expr::ArrayCompoundSet {
                    name,
                    index,
                    op,
                    value: Box::new(value),
                }),
                Expr::StructArrayGet {
                    name,
                    fields,
                    index,
                } => Ok(Expr::StructArrayCompoundSet {
                    name,
                    fields,
                    index,
                    op,
                    value: Box::new(value),
                }),
                Expr::StructFieldArrayElementGet {
                    name,
                    array_fields,
                    index,
                    fields,
                } => Ok(Expr::StructFieldArrayElementCompoundSet {
                    name,
                    array_fields,
                    index,
                    fields,
                    op,
                    value: Box::new(value),
                }),
                Expr::StructElementGet {
                    name,
                    index,
                    fields,
                } => Ok(Expr::StructElementCompoundSet {
                    name,
                    index,
                    fields,
                    op,
                    value: Box::new(value),
                }),
                Expr::StructElementArrayGet {
                    name,
                    index,
                    fields,
                    array_index,
                } => Ok(Expr::StructElementArrayCompoundSet {
                    name,
                    index,
                    fields,
                    array_index,
                    op,
                    value: Box::new(value),
                }),
                Expr::StructPtrArrayGet {
                    pointer,
                    fields,
                    index,
                } => Ok(Expr::DerefCompoundSet {
                    pointer: Box::new(Expr::AddressOfStructPtrArrayField {
                        pointer,
                        fields,
                        index,
                    }),
                    op,
                    value: Box::new(value),
                }),
                Expr::Deref(pointer) => Ok(Expr::DerefCompoundSet {
                    pointer,
                    op,
                    value: Box::new(value),
                }),
                Expr::StructGet { name, fields } => Ok(Expr::StructCompoundSet {
                    name,
                    fields,
                    op,
                    value: Box::new(value),
                }),
                Expr::StructPtrGet { pointer, fields } => Ok(Expr::StructPtrCompoundSet {
                    pointer,
                    fields,
                    op,
                    value: Box::new(value),
                }),
                Expr::ScalarLiteral { ty, init } => Ok(Expr::ScalarLiteralCompoundSet {
                    ty,
                    init,
                    op,
                    value: Box::new(value),
                }),
                Expr::AggregateFieldGet { aggregate, fields } => {
                    Ok(Expr::AggregateFieldCompoundSet {
                        aggregate,
                        fields,
                        op,
                        value: Box::new(value),
                    })
                }
                _ => Err(Self::error_at(
                    "invalid compound assignment target".to_string(),
                    &operator,
                )),
            }
        } else if self.check(&Token::Assign) {
            let equals = self.advance();
            let value = self.parse_assignment_expr()?;
            match target {
                Expr::Var(name) => Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                }),
                Expr::ArrayGet { name, index } => Ok(Expr::ArraySet {
                    name,
                    index,
                    value: Box::new(value),
                }),
                Expr::StructArrayGet {
                    name,
                    fields,
                    index,
                } => Ok(Expr::StructArraySet {
                    name,
                    fields,
                    index,
                    value: Box::new(value),
                }),
                Expr::StructFieldArrayElementGet {
                    name,
                    array_fields,
                    index,
                    fields,
                } => Ok(Expr::StructFieldArrayElementSet {
                    name,
                    array_fields,
                    index,
                    fields,
                    value: Box::new(value),
                }),
                Expr::StructElementGet {
                    name,
                    index,
                    fields,
                } => Ok(Expr::StructElementSet {
                    name,
                    index,
                    fields,
                    value: Box::new(value),
                }),
                Expr::StructElementArrayGet {
                    name,
                    index,
                    fields,
                    array_index,
                } => Ok(Expr::StructElementArraySet {
                    name,
                    index,
                    fields,
                    array_index,
                    value: Box::new(value),
                }),
                Expr::StructPtrArrayGet {
                    pointer,
                    fields,
                    index,
                } => Ok(Expr::DerefSet {
                    pointer: Box::new(Expr::AddressOfStructPtrArrayField {
                        pointer,
                        fields,
                        index,
                    }),
                    value: Box::new(value),
                }),
                Expr::Deref(pointer) => Ok(Expr::DerefSet {
                    pointer,
                    value: Box::new(value),
                }),
                Expr::StructGet { name, fields } => Ok(Expr::StructSet {
                    name,
                    fields,
                    value: Box::new(value),
                }),
                Expr::StructPtrGet { pointer, fields } => Ok(Expr::StructPtrSet {
                    pointer,
                    fields,
                    value: Box::new(value),
                }),
                Expr::ScalarLiteral { ty, init } => Ok(Expr::ScalarLiteralSet {
                    ty,
                    init,
                    value: Box::new(value),
                }),
                Expr::AggregateFieldGet { aggregate, fields } => Ok(Expr::AggregateFieldSet {
                    aggregate,
                    fields,
                    value: Box::new(value),
                }),
                _ => Err(Self::error_at(
                    "invalid assignment target".to_string(),
                    &equals,
                )),
            }
        } else {
            Ok(target)
        }
    }

    fn parse_conditional_expr(&mut self) -> CustResult<Expr> {
        let cond = self.parse_logical_or()?;
        if self.matches(&Token::Question) {
            let then_expr = self.parse_expr()?;
            self.expect_colon_after("conditional then-expression")?;
            let else_expr = self.parse_conditional_expr()?;
            Ok(Expr::Conditional {
                cond: Box::new(cond),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            })
        } else {
            Ok(cond)
        }
    }

    fn parse_logical_or(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_logical_and()?;
        while self.matches(&Token::OrOr) {
            let rhs = self.parse_logical_and()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::LogicalOr, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_bitwise_or()?;
        while self.matches(&Token::AndAnd) {
            let rhs = self.parse_bitwise_or()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::LogicalAnd, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_bitwise_or(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_bitwise_xor()?;
        while self.matches(&Token::Pipe) {
            let rhs = self.parse_bitwise_xor()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::BitOr, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_bitwise_xor(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_bitwise_and()?;
        while self.matches(&Token::Caret) {
            let rhs = self.parse_bitwise_and()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::BitXor, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_bitwise_and(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_equality()?;
        while self.matches(&Token::Amp) {
            let rhs = self.parse_equality()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::BitAnd, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_comparison()?;
        loop {
            let op = if self.matches(&Token::Eq) {
                BinaryOp::Eq
            } else if self.matches(&Token::Ne) {
                BinaryOp::Ne
            } else {
                break;
            };
            let rhs = self.parse_comparison()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_shift()?;
        loop {
            let op = if self.matches(&Token::Lt) {
                BinaryOp::Lt
            } else if self.matches(&Token::Le) {
                BinaryOp::Le
            } else if self.matches(&Token::Gt) {
                BinaryOp::Gt
            } else if self.matches(&Token::Ge) {
                BinaryOp::Ge
            } else {
                break;
            };
            let rhs = self.parse_shift()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_shift(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_term()?;
        loop {
            let op = if self.matches(&Token::ShiftLeft) {
                BinaryOp::ShiftLeft
            } else if self.matches(&Token::ShiftRight) {
                BinaryOp::ShiftRight
            } else {
                break;
            };
            let rhs = self.parse_term()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_factor()?;
        loop {
            let op = if self.matches(&Token::Plus) {
                BinaryOp::Add
            } else if self.matches(&Token::Minus) {
                BinaryOp::Sub
            } else {
                break;
            };
            let rhs = self.parse_factor()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_unary()?;
        loop {
            let op = if self.matches(&Token::Star) {
                BinaryOp::Mul
            } else if self.matches(&Token::Slash) {
                BinaryOp::Div
            } else if self.matches(&Token::Percent) {
                BinaryOp::Rem
            } else {
                break;
            };
            let rhs = self.parse_unary()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> CustResult<Expr> {
        if self.matches(&Token::PlusPlus) {
            let operator = self.previous().clone();
            let target = self.parse_unary()?;
            Self::increment_expr(target, IncrementOp::Inc, true, &operator)
        } else if self.matches(&Token::MinusMinus) {
            let operator = self.previous().clone();
            let target = self.parse_unary()?;
            Self::increment_expr(target, IncrementOp::Dec, true, &operator)
        } else if self.matches(&Token::Plus) {
            Ok(Expr::UnaryPlus(Box::new(self.parse_unary()?)))
        } else if self.matches(&Token::Minus) {
            Ok(Expr::UnaryMinus(Box::new(self.parse_unary()?)))
        } else if self.matches(&Token::Tilde) {
            Ok(Expr::BitwiseNot(Box::new(self.parse_unary()?)))
        } else if self.matches(&Token::Bang) {
            Ok(Expr::LogicalNot(Box::new(self.parse_unary()?)))
        } else if self.matches(&Token::Sizeof) {
            self.parse_sizeof()
        } else if self.matches(&Token::Alignof) {
            self.parse_alignof()
        } else if self.matches(&Token::Star) {
            Ok(Expr::Deref(Box::new(self.parse_unary()?)))
        } else if self.matches(&Token::Amp) {
            let operator = self.previous().clone();
            let target = self.parse_unary()?;
            Self::address_of_expr(target, &operator)
        } else if self.check(&Token::LParen) && self.starts_cast_type_after_lparen() {
            let expr = self.parse_cast()?;
            self.parse_postfix_suffix(expr)
        } else {
            self.parse_postfix()
        }
    }

    fn starts_cast_type_after_lparen(&self) -> bool {
        match self.tokens.get(self.pos + 1).map(|token| &token.kind) {
            Some(
                Token::Int
                | Token::Char
                | Token::Bool
                | Token::Signed
                | Token::Unsigned
                | Token::Long
                | Token::Short
                | Token::Struct
                | Token::Union
                | Token::Const
                | Token::Volatile
                | Token::Restrict
                | Token::Atomic,
            ) => true,
            Some(Token::Ident(name)) => self.lookup_type_alias(name).is_some(),
            _ => false,
        }
    }

    fn parse_cast(&mut self) -> CustResult<Expr> {
        self.expect(Token::LParen)?;
        self.consume_type_qualifiers();
        let type_token = self.peek_located().clone();
        let decl_type = self.parse_decl_type("cast type")?;
        if self.matches(&Token::Star) {
            return Err(Self::error_at(
                "pointer casts are not supported".to_string(),
                self.previous(),
            ));
        }
        let ty = match decl_type {
            DeclType::Scalar(ty) => ty,
            DeclType::Struct(_) => {
                let DeclType::Struct(type_name) = decl_type else {
                    unreachable!("decl_type was already matched as aggregate")
                };
                if self.matches(&Token::LBracket) {
                    let len = if self.check(&Token::RBracket) {
                        None
                    } else {
                        Some(self.expect_array_len()?)
                    };
                    self.expect_closing_bracket_after("aggregate array compound literal type")?;
                    self.expect_closing_paren_after("cast type")?;
                    return Ok(Expr::AggregateArrayLiteral {
                        init: self.parse_aggregate_array_compound_initializer(&type_name, len)?,
                        len,
                        type_name,
                    });
                }
                self.expect_closing_paren_after("cast type")?;
                if !self.check(&Token::LBrace) {
                    return Err(Self::error_at(
                        "aggregate casts are not supported".to_string(),
                        &type_token,
                    ));
                }
                return Ok(Expr::AggregateLiteral {
                    init: self.parse_struct_initializer(&type_name)?,
                    type_name,
                });
            }
            DeclType::Pointer { .. } | DeclType::Array(_, _) => {
                return Err(Self::error_at(
                    "pointer casts are not supported".to_string(),
                    &type_token,
                ));
            }
        };
        if self.matches(&Token::LBracket) {
            let len = if self.check(&Token::RBracket) {
                None
            } else {
                Some(self.expect_array_len()?)
            };
            self.expect_closing_bracket_after("array compound literal type")?;
            self.expect_closing_paren_after("cast type")?;
            return Ok(Expr::ArrayLiteral {
                elem_type: ty,
                len,
                init: self.parse_array_compound_initializer(len, ty)?,
            });
        }
        self.expect_closing_paren_after("cast type")?;
        if self.check(&Token::LBrace) {
            return Ok(Expr::ScalarLiteral {
                ty,
                init: Box::new(self.parse_scalar_initializer_expr("scalar compound literal")?),
            });
        }
        Ok(Expr::Cast {
            ty,
            expr: Box::new(self.parse_unary()?),
        })
    }

    fn address_of_expr(target: Expr, operator: &LocatedToken) -> CustResult<Expr> {
        match target {
            Expr::Var(name) => Ok(Expr::AddressOf(name)),
            Expr::ArrayGet { name, index } => Ok(Expr::AddressOfArray { name, index }),
            Expr::StructGet { name, fields } => Ok(Expr::AddressOfStructField { name, fields }),
            Expr::StructElementGet {
                name,
                index,
                fields,
            } => Ok(Expr::AddressOfStructElementField {
                name,
                index,
                fields,
            }),
            Expr::StructArrayGet {
                name,
                fields,
                index,
            } => Ok(Expr::AddressOfStructArrayField {
                name,
                fields,
                index,
            }),
            Expr::StructElementArrayGet {
                name,
                index,
                fields,
                array_index,
            } => Ok(Expr::AddressOfStructElementArrayField {
                name,
                index,
                fields,
                array_index,
            }),
            Expr::StructPtrArrayGet {
                pointer,
                fields,
                index,
            } => Ok(Expr::AddressOfStructPtrArrayField {
                pointer,
                fields,
                index,
            }),
            Expr::StructFieldArrayElementGet {
                name,
                array_fields,
                index,
                fields,
            } => Ok(Expr::AddressOfStructPtrField {
                pointer: Box::new(Expr::AddressOfStructArrayField {
                    name,
                    fields: array_fields,
                    index,
                }),
                fields,
            }),
            Expr::StructPtrGet { pointer, fields } => {
                Ok(Expr::AddressOfStructPtrField { pointer, fields })
            }
            Expr::ScalarLiteral { ty, init } => Ok(Expr::AddressOfScalarLiteral { ty, init }),
            Expr::AggregateLiteral { type_name, init } => {
                Ok(Expr::AddressOfAggregateLiteral { type_name, init })
            }
            Expr::AggregateFieldGet { aggregate, fields } => {
                Ok(Expr::AddressOfAggregateField { aggregate, fields })
            }
            Expr::StringGet { values, index } => Ok(Expr::Binary(
                Box::new(Expr::StringLiteral(values)),
                BinaryOp::Add,
                index,
            )),
            Expr::Deref(pointer) => Ok(*pointer),
            _ => Err(Self::error_at(
                "invalid address-of target".to_string(),
                operator,
            )),
        }
    }

    fn parse_sizeof(&mut self) -> CustResult<Expr> {
        if self.matches(&Token::LParen) {
            if self.is_type_name_start() {
                let sizeof_type = self.parse_sizeof_like_type_name("sizeof")?;
                self.expect_closing_paren_after("sizeof type")?;
                Ok(Expr::SizeOfType(sizeof_type))
            } else {
                let expr = self.parse_expr()?;
                self.expect_closing_paren_after("sizeof expression")?;
                Ok(Expr::SizeOfValue(Box::new(expr)))
            }
        } else {
            Ok(Expr::SizeOfValue(Box::new(self.parse_unary()?)))
        }
    }

    fn parse_alignof(&mut self) -> CustResult<Expr> {
        self.expect_opening_paren_after("_Alignof")?;
        if !self.is_type_name_start() {
            let found = self.peek_located().clone();
            return Err(Self::error_at(
                format!("expected _Alignof type, found {:?}", found.kind),
                &found,
            ));
        }
        let alignof_type = self.parse_sizeof_like_type_name("_Alignof")?;
        self.expect_closing_paren_after("_Alignof type")?;
        Ok(Expr::AlignOfType(alignof_type))
    }

    fn is_type_name_start(&self) -> bool {
        matches!(
            self.peek(),
            Token::Int
                | Token::Char
                | Token::Bool
                | Token::Signed
                | Token::Unsigned
                | Token::Long
                | Token::Short
                | Token::Struct
                | Token::Union
                | Token::Enum
                | Token::Const
                | Token::Volatile
                | Token::Restrict
                | Token::Atomic
                | Token::Void
        ) || self.current_alias().is_some()
    }

    fn parse_sizeof_like_type_name(&mut self, operator: &str) -> CustResult<SizeOfType> {
        let is_const_qualified = self.consume_type_qualifiers();
        if self.check(&Token::Void) {
            let type_token = self.advance();
            return Err(Self::error_at(
                format!("{operator}(void) is not supported"),
                &type_token,
            ));
        }
        if is_const_qualified
            && !matches!(
                self.peek(),
                Token::Int
                    | Token::Char
                    | Token::Bool
                    | Token::Signed
                    | Token::Unsigned
                    | Token::Long
                    | Token::Short
                    | Token::Struct
                    | Token::Union
                    | Token::Enum
            )
            && self.current_alias().is_none()
        {
            let found = self.peek_located().clone();
            return Err(Self::error_at(
                format!(
                    "expected {operator} type after const, found {:?}",
                    found.kind
                ),
                &found,
            ));
        }
        match self.parse_decl_type(&format!("{operator} struct type name"))? {
            DeclType::Scalar(ty) => {
                if self.matches(&Token::Star) {
                    Ok(SizeOfType::Pointer)
                } else if let Some(len) = self.parse_sizeof_array_type_len(operator)? {
                    Ok(SizeOfType::Array(PointeeType::Scalar(ty), len))
                } else {
                    Ok(SizeOfType::Scalar(ty))
                }
            }
            DeclType::Struct(type_name) => {
                if self.matches(&Token::Star) {
                    Ok(SizeOfType::Pointer)
                } else if let Some(len) = self.parse_sizeof_array_type_len(operator)? {
                    Ok(SizeOfType::Array(PointeeType::Struct(type_name), len))
                } else {
                    Ok(SizeOfType::Struct(type_name))
                }
            }
            DeclType::Pointer { .. } => {
                if self.matches(&Token::Star) {
                    return Err(Self::error_at(
                        format!("pointer-to-pointer {operator} types are not supported"),
                        self.previous(),
                    ));
                }
                if self.matches(&Token::LBracket) {
                    return Err(Self::error_at(
                        format!("pointer array {operator} types are not supported"),
                        self.previous(),
                    ));
                }
                Ok(SizeOfType::Pointer)
            }
            DeclType::Array(element_type, len) => {
                if self.matches(&Token::Star) {
                    return Err(Self::error_at(
                        format!("pointer-to-array {operator} types are not supported"),
                        self.previous(),
                    ));
                }
                if self.check(&Token::LBracket) {
                    return Err(Self::error_at(
                        format!("multidimensional {operator} array types are not supported"),
                        self.peek_located(),
                    ));
                }
                Ok(SizeOfType::Array(element_type, len))
            }
        }
    }

    fn parse_sizeof_array_type_len(&mut self, operator: &str) -> CustResult<Option<usize>> {
        if !self.matches(&Token::LBracket) {
            return Ok(None);
        }
        let len = self.expect_array_len()?;
        self.expect_closing_bracket_after(&format!("{operator} array type"))?;
        if self.check(&Token::LBracket) {
            return Err(Self::error_at(
                format!("multidimensional {operator} array types are not supported"),
                self.peek_located(),
            ));
        }
        Ok(Some(len))
    }

    fn parse_postfix(&mut self) -> CustResult<Expr> {
        let expr = self.parse_primary()?;
        self.parse_postfix_suffix(expr)
    }

    fn parse_postfix_suffix(&mut self, mut expr: Expr) -> CustResult<Expr> {
        loop {
            if self.matches(&Token::LBracket) {
                let index = self.parse_index_expr()?;
                self.expect_closing_bracket_after("array index")?;
                expr = match expr {
                    Expr::Var(name) => Expr::ArrayGet {
                        name,
                        index: Box::new(index),
                    },
                    Expr::StructGet { name, fields } => Expr::StructArrayGet {
                        name,
                        fields,
                        index: Box::new(index),
                    },
                    Expr::StructElementGet {
                        name,
                        index: element_index,
                        fields,
                    } => Expr::StructElementArrayGet {
                        name,
                        index: element_index,
                        fields,
                        array_index: Box::new(index),
                    },
                    Expr::StructPtrGet { pointer, fields } => Expr::StructPtrArrayGet {
                        pointer,
                        fields,
                        index: Box::new(index),
                    },
                    Expr::AggregateFieldGet { .. } => Expr::Deref(Box::new(Expr::Binary(
                        Box::new(expr),
                        BinaryOp::Add,
                        Box::new(index),
                    ))),
                    _ => Expr::Deref(Box::new(Expr::Binary(
                        Box::new(expr),
                        BinaryOp::Add,
                        Box::new(index),
                    ))),
                };
            } else if self.matches(&Token::Dot) {
                let field = self.expect_ident_after("struct field name after '.'")?;
                expr = match expr {
                    Expr::Var(name) => Expr::StructGet {
                        name,
                        fields: vec![field],
                    },
                    Expr::StructGet { name, mut fields } => {
                        fields.push(field);
                        Expr::StructGet { name, fields }
                    }
                    Expr::ArrayGet { name, index } => Expr::StructElementGet {
                        name,
                        index,
                        fields: vec![field],
                    },
                    Expr::StructElementGet {
                        name,
                        index,
                        mut fields,
                    } => {
                        fields.push(field);
                        Expr::StructElementGet {
                            name,
                            index,
                            fields,
                        }
                    }
                    Expr::StructArrayGet {
                        name,
                        fields: array_fields,
                        index,
                    } => Expr::StructFieldArrayElementGet {
                        name,
                        array_fields,
                        index,
                        fields: vec![field],
                    },
                    Expr::StructFieldArrayElementGet {
                        name,
                        array_fields,
                        index,
                        mut fields,
                    } => {
                        fields.push(field);
                        Expr::StructFieldArrayElementGet {
                            name,
                            array_fields,
                            index,
                            fields,
                        }
                    }
                    Expr::Deref(pointer) => Expr::StructPtrGet {
                        pointer,
                        fields: vec![field],
                    },
                    Expr::StructPtrGet {
                        pointer,
                        mut fields,
                    } => {
                        fields.push(field);
                        Expr::StructPtrGet { pointer, fields }
                    }
                    Expr::StructPtrArrayGet {
                        pointer,
                        fields,
                        index,
                    } => Expr::StructPtrGet {
                        pointer: Box::new(Expr::StructPtrArrayGet {
                            pointer,
                            fields,
                            index,
                        }),
                        fields: vec![field],
                    },
                    Expr::AggregateLiteral { .. }
                    | Expr::Assign { .. }
                    | Expr::DerefSet { .. }
                    | Expr::Conditional { .. }
                    | Expr::Comma(_, _)
                    | Expr::Call { .. } => Expr::AggregateFieldGet {
                        aggregate: Box::new(expr),
                        fields: vec![field],
                    },
                    Expr::AggregateFieldGet {
                        aggregate,
                        mut fields,
                    } => {
                        fields.push(field);
                        Expr::AggregateFieldGet { aggregate, fields }
                    }
                    _ => {
                        return Err(Self::error_at(
                            "invalid struct field access target".to_string(),
                            self.previous(),
                        ));
                    }
                };
            } else if self.matches(&Token::Arrow) {
                let field = self.expect_ident_after("struct field name after '->'")?;
                expr = Expr::StructPtrGet {
                    pointer: Box::new(expr),
                    fields: vec![field],
                };
            } else if self.matches(&Token::PlusPlus) {
                let operator = self.previous().clone();
                expr = Self::increment_expr(expr, IncrementOp::Inc, false, &operator)?;
            } else if self.matches(&Token::MinusMinus) {
                let operator = self.previous().clone();
                expr = Self::increment_expr(expr, IncrementOp::Dec, false, &operator)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn increment_expr(
        target: Expr,
        op: IncrementOp,
        prefix: bool,
        operator: &LocatedToken,
    ) -> CustResult<Expr> {
        match target {
            Expr::Var(_)
            | Expr::ArrayGet { .. }
            | Expr::StructArrayGet { .. }
            | Expr::StructElementGet { .. }
            | Expr::StructElementArrayGet { .. }
            | Expr::StructPtrArrayGet { .. }
            | Expr::Deref(_)
            | Expr::StructGet { .. }
            | Expr::StructPtrGet { .. }
            | Expr::ScalarLiteral { .. }
            | Expr::AggregateFieldGet { .. } => Ok(Expr::Increment {
                target: Box::new(target),
                op,
                prefix,
            }),
            _ => Err(Self::error_at(
                "invalid increment/decrement target".to_string(),
                operator,
            )),
        }
    }

    fn parse_primary(&mut self) -> CustResult<Expr> {
        let found = self.advance();
        match found.kind.clone() {
            Token::Number(value) => Ok(Expr::Number(value)),
            Token::StringLiteral(values) => {
                let values = self.concatenate_adjacent_string_literals(values);
                if self.matches(&Token::LBracket) {
                    let index = self.parse_index_expr()?;
                    self.expect_closing_bracket_after("string literal index")?;
                    Ok(Expr::StringGet {
                        values,
                        index: Box::new(index),
                    })
                } else {
                    Ok(Expr::StringLiteral(values))
                }
            }
            Token::Ident(name) => {
                if self.matches(&Token::LParen) {
                    let args = self.parse_call_args()?;
                    self.expect_closing_paren_after("function call arguments")?;
                    Ok(Expr::Call { name, args })
                } else if self.matches(&Token::LBracket) {
                    let index = self.parse_index_expr()?;
                    self.expect_closing_bracket_after("array index")?;
                    Ok(Expr::ArrayGet {
                        name,
                        index: Box::new(index),
                    })
                } else if self.matches(&Token::Dot) {
                    let field = self.expect_ident_after("struct field name after '.'")?;
                    Ok(Expr::StructGet {
                        name,
                        fields: vec![field],
                    })
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Token::LParen => {
                let expr = self.parse_expr()?;
                self.expect_closing_paren_after("grouped expression")?;
                Ok(expr)
            }
            token => Err(Self::error_at(
                format!("expected expression, found {token:?}"),
                &found,
            )),
        }
    }

    fn concatenate_adjacent_string_literals(&mut self, mut values: Vec<i64>) -> Vec<i64> {
        while let Token::StringLiteral(next_values) = self.peek().clone() {
            self.advance();
            if values.last() == Some(&0) {
                values.pop();
            }
            values.extend(next_values);
        }
        values
    }

    fn parse_call_args(&mut self) -> CustResult<Vec<Expr>> {
        let mut args = Vec::new();
        if self.check(&Token::RParen) {
            return Ok(args);
        }

        loop {
            args.push(self.parse_assignment_expr()?);

            if self.matches(&Token::Comma) {
                if matches!(
                    self.peek(),
                    Token::RParen | Token::Semi | Token::LBrace | Token::RBrace | Token::Eof
                ) {
                    return Err(Self::error_at(
                        format!(
                            "expected function call argument after ',', found {:?}",
                            self.peek()
                        ),
                        self.peek_located(),
                    ));
                }
            } else if matches!(
                self.peek(),
                Token::RParen | Token::Semi | Token::LBrace | Token::RBrace | Token::Eof
            ) {
                break;
            } else {
                return Err(Self::error_at(
                    format!(
                        "expected ',' or ')' after function call argument, found {:?}",
                        self.peek()
                    ),
                    self.peek_located(),
                ));
            }
        }
        Ok(args)
    }

    fn expect(&mut self, expected: Token) -> CustResult<()> {
        let found = self.advance();
        if found.kind == expected {
            Ok(())
        } else {
            Err(Self::error_at(
                format!("expected {expected:?}, found {:?}", found.kind),
                &found,
            ))
        }
    }

    fn expect_opening_paren_after(&mut self, context: &str) -> CustResult<()> {
        let found = self.advance();
        if found.kind == Token::LParen {
            Ok(())
        } else {
            Err(Self::error_at(
                format!("expected '(' after {context}, found {:?}", found.kind),
                &found,
            ))
        }
    }

    fn expect_keyword_after(&mut self, expected: &Token, context: &str) -> CustResult<()> {
        let found = self.advance();
        if &found.kind == expected {
            Ok(())
        } else {
            Err(Self::error_at(
                format!(
                    "expected {expected:?} after {context}, found {:?}",
                    found.kind
                ),
                &found,
            ))
        }
    }

    fn expect_opening_brace_after(&mut self, context: &str) -> CustResult<()> {
        let found = self.advance();
        if found.kind == Token::LBrace {
            Ok(())
        } else {
            Err(Self::error_at(
                format!("expected '{{' after {context}, found {:?}", found.kind),
                &found,
            ))
        }
    }

    fn expect_closing_bracket_after(&mut self, context: &str) -> CustResult<()> {
        let found = self.advance();
        if found.kind == Token::RBracket {
            Ok(())
        } else {
            Err(Self::error_at(
                format!("expected ']' after {context}, found {:?}", found.kind),
                &found,
            ))
        }
    }

    fn expect_closing_brace_after(&mut self, context: &str) -> CustResult<()> {
        let found = self.advance();
        if found.kind == Token::RBrace {
            Ok(())
        } else {
            Err(Self::error_at(
                format!("expected '}}' after {context}, found {:?}", found.kind),
                &found,
            ))
        }
    }

    fn expect_closing_paren_after(&mut self, context: &str) -> CustResult<()> {
        let found = self.advance();
        if found.kind == Token::RParen {
            Ok(())
        } else {
            Err(Self::error_at(
                format!("expected ')' after {context}, found {:?}", found.kind),
                &found,
            ))
        }
    }

    fn expect_semicolon_after(&mut self, context: &str) -> CustResult<()> {
        let found = self.advance();
        if found.kind == Token::Semi {
            Ok(())
        } else {
            Err(Self::error_at(
                format!("expected ';' after {context}, found {:?}", found.kind),
                &found,
            ))
        }
    }

    fn expect_assign_after(&mut self, context: &str) -> CustResult<()> {
        let found = self.advance();
        if found.kind == Token::Assign {
            Ok(())
        } else {
            Err(Self::error_at(
                format!("expected '=' after {context}, found {:?}", found.kind),
                &found,
            ))
        }
    }

    fn expect_colon_after(&mut self, context: &str) -> CustResult<()> {
        let found = self.advance();
        if found.kind == Token::Colon {
            Ok(())
        } else {
            Err(Self::error_at(
                format!("expected ':' after {context}, found {:?}", found.kind),
                &found,
            ))
        }
    }

    fn expect_ident(&mut self) -> CustResult<String> {
        let found = self.advance();
        match found.kind.clone() {
            Token::Ident(name) => Ok(name),
            token => Err(Self::error_at(
                format!("expected identifier, found {token:?}"),
                &found,
            )),
        }
    }

    fn expect_ident_after(&mut self, context: &str) -> CustResult<String> {
        let found = self.advance();
        match found.kind.clone() {
            Token::Ident(name) => Ok(name),
            token => Err(Self::error_at(
                format!("expected {context}, found {token:?}"),
                &found,
            )),
        }
    }

    fn matches(&mut self, expected: &Token) -> bool {
        if self.check(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn check(&self, expected: &Token) -> bool {
        self.peek() == expected
    }

    fn compound_assignment_op(&self) -> Option<CompoundOp> {
        match self.peek() {
            Token::PlusAssign => Some(CompoundOp::Add),
            Token::MinusAssign => Some(CompoundOp::Sub),
            Token::StarAssign => Some(CompoundOp::Mul),
            Token::SlashAssign => Some(CompoundOp::Div),
            Token::PercentAssign => Some(CompoundOp::Rem),
            Token::AmpAssign => Some(CompoundOp::BitAnd),
            Token::PipeAssign => Some(CompoundOp::BitOr),
            Token::CaretAssign => Some(CompoundOp::BitXor),
            Token::ShiftLeftAssign => Some(CompoundOp::ShiftLeft),
            Token::ShiftRightAssign => Some(CompoundOp::ShiftRight),
            _ => None,
        }
    }

    fn is_assignment_operator(token: &Token) -> bool {
        matches!(
            token,
            Token::Assign
                | Token::PlusAssign
                | Token::MinusAssign
                | Token::StarAssign
                | Token::SlashAssign
                | Token::PercentAssign
                | Token::AmpAssign
                | Token::PipeAssign
                | Token::CaretAssign
                | Token::ShiftLeftAssign
                | Token::ShiftRightAssign
        )
    }

    fn peek(&self) -> &Token {
        &self.peek_located().kind
    }

    fn peek_next(&self) -> &Token {
        &self
            .tokens
            .get(self.pos + 1)
            .unwrap_or_else(|| self.peek_located())
            .kind
    }

    fn starts_expr(&self) -> bool {
        Self::is_expr_start_token(self.peek())
    }

    fn is_expr_start_token(token: &Token) -> bool {
        matches!(
            token,
            Token::Ident(_)
                | Token::Number(_)
                | Token::StringLiteral(_)
                | Token::Plus
                | Token::Minus
                | Token::PlusPlus
                | Token::MinusMinus
                | Token::Bang
                | Token::Sizeof
                | Token::Star
                | Token::Amp
                | Token::LParen
        )
    }

    fn is_primary_assignment_value_start(token: &Token) -> bool {
        matches!(
            token,
            Token::Ident(_)
                | Token::Number(_)
                | Token::StringLiteral(_)
                | Token::Star
                | Token::Amp
                | Token::LParen
        )
    }

    fn starts_deref_assignment_stmt(&self) -> bool {
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;

        for token in self.tokens.iter().skip(self.pos + 1) {
            match &token.kind {
                Token::LParen => paren_depth += 1,
                Token::RParen => paren_depth = paren_depth.saturating_sub(1),
                Token::LBracket => bracket_depth += 1,
                Token::RBracket => bracket_depth = bracket_depth.saturating_sub(1),
                token
                    if paren_depth == 0
                        && bracket_depth == 0
                        && Self::is_assignment_operator(token) =>
                {
                    return true;
                }
                Token::Semi | Token::Eof if paren_depth == 0 && bracket_depth == 0 => return false,
                _ => {}
            }
        }

        false
    }

    fn starts_assignment_stmt(&self) -> bool {
        if !matches!(self.peek(), Token::Ident(_)) {
            return false;
        }
        if self.peek_next() == &Token::Dot {
            let mut index = self.pos + 1;
            while self
                .tokens
                .get(index)
                .is_some_and(|token| token.kind == Token::Dot)
            {
                if !self
                    .tokens
                    .get(index + 1)
                    .is_some_and(|token| matches!(token.kind, Token::Ident(_)))
                {
                    return false;
                }
                index += 2;
            }
            return self
                .tokens
                .get(index)
                .is_some_and(|token| Self::is_assignment_operator(&token.kind));
        }
        if Self::is_assignment_operator(self.peek_next()) {
            return true;
        }
        if self.peek_next() != &Token::LBracket {
            return false;
        }

        let mut depth = 0usize;
        for index in (self.pos + 1)..self.tokens.len() {
            match &self.tokens[index].kind {
                Token::LBracket => depth += 1,
                Token::RBracket => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return self.tokens.get(index + 1).is_some_and(|candidate| {
                            Self::is_assignment_operator(&candidate.kind)
                        });
                    }
                }
                Token::Eof if depth > 0 => return false,
                _ => {}
            }
        }
        false
    }

    fn starts_missing_assignment_operator_stmt(&self) -> bool {
        if !matches!(self.peek(), Token::Ident(_)) {
            return false;
        }

        if self.peek_next() != &Token::LParen
            && Self::is_primary_assignment_value_start(self.peek_next())
        {
            return true;
        }

        if self.peek_next() != &Token::LBracket {
            return false;
        }

        let mut depth = 0usize;
        for index in (self.pos + 1)..self.tokens.len() {
            match &self.tokens[index].kind {
                Token::LBracket => depth += 1,
                Token::RBracket => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return self.tokens.get(index + 1).is_some_and(|candidate| {
                            Self::is_primary_assignment_value_start(&candidate.kind)
                        });
                    }
                }
                Token::Semi | Token::Eof if depth == 0 => return false,
                Token::Eof if depth > 0 => return false,
                _ => {}
            }
        }
        false
    }

    fn starts_missing_deref_assignment_operator_stmt(&self) -> bool {
        match self.tokens.get(self.pos + 1).map(|token| &token.kind) {
            Some(Token::Ident(_)) => {
                let after_target = if self
                    .tokens
                    .get(self.pos + 2)
                    .is_some_and(|token| token.kind == Token::LBracket)
                {
                    self.index_after_balanced_brackets(self.pos + 2)
                        .map(|index| index + 1)
                } else {
                    Some(self.pos + 2)
                };
                after_target
                    .and_then(|index| self.tokens.get(index))
                    .is_some_and(|token| Self::is_primary_assignment_value_start(&token.kind))
            }
            Some(Token::LParen) => self
                .index_after_balanced_parens(self.pos + 1)
                .and_then(|index| self.tokens.get(index + 1))
                .is_some_and(|token| Self::is_primary_assignment_value_start(&token.kind)),
            _ => false,
        }
    }

    fn index_after_balanced_brackets(&self, start: usize) -> Option<usize> {
        let mut depth = 0usize;
        for index in start..self.tokens.len() {
            match &self.tokens[index].kind {
                Token::LBracket => depth += 1,
                Token::RBracket => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return Some(index);
                    }
                }
                Token::Eof => return None,
                _ => {}
            }
        }
        None
    }

    fn index_after_balanced_parens(&self, start: usize) -> Option<usize> {
        let mut depth = 0usize;
        for index in start..self.tokens.len() {
            match &self.tokens[index].kind {
                Token::LParen => depth += 1,
                Token::RParen => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return Some(index);
                    }
                }
                Token::Eof => return None,
                _ => {}
            }
        }
        None
    }

    fn loop_control_keyword(&self) -> &'static str {
        match self.peek() {
            Token::Break => "break",
            Token::Continue => "continue",
            _ => "loop control statement",
        }
    }

    fn peek_located(&self) -> &LocatedToken {
        self.tokens
            .get(self.pos)
            .expect("lexer always appends an EOF token")
    }

    fn previous(&self) -> &LocatedToken {
        self.tokens
            .get(self.pos.saturating_sub(1))
            .expect("previous token exists after a successful match")
    }

    fn advance(&mut self) -> LocatedToken {
        let token = self.peek_located().clone();
        if !matches!(token.kind, Token::Eof) {
            self.pos += 1;
        }
        token
    }

    fn error_at(message: String, token: &LocatedToken) -> CustError {
        CustError::new(format!(
            "{message} at line {}, column {}",
            token.line, token.column
        ))
    }
}

const MAX_CALL_DEPTH: usize = 32;

struct Interpreter {
    scopes: Vec<Scope>,
    static_locals: HashMap<usize, StaticLocalStorage>,
    live_scope_ids: HashSet<usize>,
    next_scope_id: usize,
    functions: HashMap<String, Function>,
    struct_types: HashMap<String, StructTypeDef>,
    call_depth: usize,
    return_type_stack: Vec<ReturnType>,
    max_loop_iterations: Option<usize>,
    loop_iterations: usize,
    next_compound_literal_id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scope {
    id: usize,
    values: HashMap<String, Value>,
    static_local_ids: HashMap<String, usize>,
    enum_constants: HashMap<String, i64>,
    const_variables: HashSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StaticLocalStorage {
    scope_id: usize,
    name: String,
    value: Value,
    is_const: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExecFlow {
    None,
    Return(Option<ReturnValue>),
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReturnValue {
    Scalar(i64),
    Pointer {
        pointer: PointerValue,
        ty: PointeeType,
        points_to_const: bool,
    },
    Struct {
        type_name: String,
        fields: HashMap<String, StructFieldValue>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Scalar {
        value: i64,
        ty: CType,
    },
    Array(Rc<RefCell<ArrayValue>>),
    Pointer {
        pointer: PointerValue,
        ty: PointeeType,
        points_to_const: bool,
    },
    Struct {
        type_name: String,
        fields: HashMap<String, StructFieldValue>,
    },
    StructArray {
        type_name: String,
        elements: Vec<HashMap<String, StructFieldValue>>,
        read_only: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StructFieldValue {
    Scalar {
        value: i64,
        ty: CType,
        is_const: bool,
    },
    Array {
        value: Rc<RefCell<ArrayValue>>,
        is_const: bool,
    },
    Struct {
        type_name: String,
        fields: HashMap<String, StructFieldValue>,
        is_const: bool,
    },
    StructArray {
        type_name: String,
        elements: Vec<HashMap<String, StructFieldValue>>,
        is_const: bool,
    },
    Pointer {
        pointer: PointerValue,
        ty: PointeeType,
        is_const: bool,
        points_to_const: bool,
    },
}

impl StructFieldValue {
    fn is_const(&self) -> bool {
        match self {
            StructFieldValue::Scalar { is_const, .. }
            | StructFieldValue::Array { is_const, .. }
            | StructFieldValue::Struct { is_const, .. }
            | StructFieldValue::StructArray { is_const, .. }
            | StructFieldValue::Pointer { is_const, .. } => *is_const,
        }
    }

    fn deep_clone(&self) -> Self {
        match self {
            StructFieldValue::Scalar {
                value,
                ty,
                is_const,
            } => StructFieldValue::Scalar {
                value: *value,
                ty: *ty,
                is_const: *is_const,
            },
            StructFieldValue::Array { value, is_const } => StructFieldValue::Array {
                value: Rc::new(RefCell::new(value.borrow().clone())),
                is_const: *is_const,
            },
            StructFieldValue::Struct {
                type_name,
                fields,
                is_const,
            } => StructFieldValue::Struct {
                type_name: type_name.clone(),
                fields: Self::deep_clone_fields(fields),
                is_const: *is_const,
            },
            StructFieldValue::StructArray {
                type_name,
                elements,
                is_const,
            } => StructFieldValue::StructArray {
                type_name: type_name.clone(),
                elements: elements.iter().map(Self::deep_clone_fields).collect(),
                is_const: *is_const,
            },
            StructFieldValue::Pointer {
                pointer,
                ty,
                is_const,
                points_to_const,
            } => StructFieldValue::Pointer {
                pointer: pointer.clone(),
                ty: ty.clone(),
                is_const: *is_const,
                points_to_const: *points_to_const,
            },
        }
    }

    fn deep_clone_fields(
        fields: &HashMap<String, StructFieldValue>,
    ) -> HashMap<String, StructFieldValue> {
        fields
            .iter()
            .map(|(name, value)| (name.clone(), value.deep_clone()))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PointerValue {
    Null,
    Scalar {
        scope_id: usize,
        name: String,
    },
    Struct {
        scope_id: usize,
        name: String,
    },
    StructElement {
        scope_id: usize,
        name: String,
        index: usize,
    },
    StructFieldElement {
        scope_id: usize,
        name: String,
        element_index: Option<usize>,
        fields: Vec<String>,
        index: usize,
    },
    StructField {
        scope_id: usize,
        name: String,
        element_index: Option<usize>,
        fields: Vec<String>,
    },
    StructFieldElementField {
        scope_id: usize,
        name: String,
        element_index: Option<usize>,
        array_fields: Vec<String>,
        index: usize,
        fields: Vec<String>,
    },
    ArrayBase {
        array: Rc<RefCell<ArrayValue>>,
        source_name: Option<String>,
    },
    ArrayElement {
        array: Rc<RefCell<ArrayValue>>,
        source_name: Option<String>,
        index: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArrayValue {
    elements: Vec<i64>,
    elem_type: CType,
    read_only: bool,
}

impl ArrayValue {
    fn mutable_zeroed(len: usize, elem_type: CType) -> Self {
        Self {
            elements: vec![0; len],
            elem_type,
            read_only: false,
        }
    }

    fn read_only(elements: Vec<i64>) -> Self {
        Self {
            elements,
            elem_type: CType::Char,
            read_only: true,
        }
    }
}

impl Interpreter {
    fn new(options: InterpretOptions) -> Self {
        Self {
            scopes: Vec::new(),
            static_locals: HashMap::new(),
            live_scope_ids: HashSet::new(),
            next_scope_id: 0,
            functions: HashMap::new(),
            struct_types: HashMap::new(),
            call_depth: 0,
            return_type_stack: Vec::new(),
            max_loop_iterations: options.max_loop_iterations,
            loop_iterations: 0,
            next_compound_literal_id: 0,
        }
    }

    fn aggregate_kind_label(&self, type_name: &str) -> &'static str {
        self.struct_types
            .get(type_name)
            .map(|aggregate| aggregate.kind.keyword())
            .unwrap_or("struct")
    }

    fn run(&mut self, program: &Program) -> CustResult<i64> {
        self.functions = program.functions.clone();
        self.struct_types = program.struct_types.clone();
        self.push_scope();
        for global in &program.globals {
            match self.exec_stmt(global)? {
                ExecFlow::None => {}
                ExecFlow::Return(_) => return Err(CustError::new("return outside function")),
                ExecFlow::Break => return Err(CustError::new("break outside loop")),
                ExecFlow::Continue => return Err(CustError::new("continue outside loop")),
            }
        }

        let result = match self.call_function("main", &[])? {
            Some(ReturnValue::Scalar(value)) => Ok(value),
            Some(ReturnValue::Pointer { .. }) => Err(CustError::new(
                "pointer function 'main' used as program entry point",
            )),
            Some(ReturnValue::Struct { .. }) => Err(CustError::new(
                "struct function 'main' used as program entry point",
            )),
            None => Err(CustError::new(
                "int function 'main' returned without a value",
            )),
        };
        self.pop_scope();
        result
    }

    fn call_function(&mut self, name: &str, arg_exprs: &[Expr]) -> CustResult<Option<ReturnValue>> {
        let function = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| CustError::new(format!("undefined function '{name}'")))?;

        if function.params.len() != arg_exprs.len() {
            return Err(CustError::new(format!(
                "function '{name}' expected {} arguments, got {}",
                function.params.len(),
                arg_exprs.len()
            )));
        }

        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(CustError::new(format!(
                "function call depth limit exceeded while calling '{name}'"
            )));
        }

        let mut param_scope = HashMap::new();
        let mut const_params = HashSet::new();
        for (param, arg_expr) in function.params.iter().zip(arg_exprs) {
            let arg = match param.kind {
                ParamKind::Scalar => {
                    let ParamType::Scalar(ty) = &param.ty else {
                        return Err(CustError::new("internal scalar parameter type mismatch"));
                    };
                    let ty = *ty;
                    Value::Scalar {
                        value: self.eval(arg_expr)?,
                        ty,
                    }
                }
                ParamKind::Pointer => {
                    let ty = match &param.ty {
                        ParamType::Scalar(ty) => PointeeType::Scalar(*ty),
                        ParamType::Struct(type_name) => PointeeType::Struct(type_name.clone()),
                    };
                    self.ensure_pointer_conversion_preserves_const(
                        param.points_to_const,
                        arg_expr,
                    )?;
                    let pointer = self.eval_pointer(arg_expr)?;
                    self.ensure_pointer_type_matches(&ty, &pointer)?;
                    Value::Pointer {
                        pointer,
                        ty,
                        points_to_const: param.points_to_const,
                    }
                }
                ParamKind::Struct => {
                    let ParamType::Struct(type_name) = &param.ty else {
                        return Err(CustError::new("internal struct parameter type mismatch"));
                    };
                    self.eval_struct_argument(name, &param.name, type_name, arg_expr)?
                }
            };
            if param_scope.insert(param.name.clone(), arg).is_some() {
                return Err(CustError::new(format!(
                    "parameter '{}' already declared in this function",
                    param.name
                )));
            }
            if param.is_const {
                const_params.insert(param.name.clone());
            }
        }

        self.call_depth += 1;
        self.return_type_stack.push(function.return_type.clone());
        self.push_scope_with_values_and_consts(param_scope, const_params);
        let result = match self.exec_block(&function.body) {
            Ok(ExecFlow::Return(value)) => {
                self.validate_return_value(name, &function.return_type, value)
            }
            Ok(ExecFlow::None) => match &function.return_type {
                ReturnType::Scalar(_) => Err(CustError::new(format!(
                    "function '{name}' finished without return"
                ))),
                ReturnType::Pointer { .. } => Err(CustError::new(format!(
                    "function '{name}' finished without return"
                ))),
                ReturnType::Struct(_) => Err(CustError::new(format!(
                    "function '{name}' finished without return"
                ))),
                ReturnType::Void => Ok(None),
            },
            Ok(ExecFlow::Break) => Err(CustError::new("break outside loop")),
            Ok(ExecFlow::Continue) => Err(CustError::new("continue outside loop")),
            Err(error) => Err(error),
        };
        self.pop_scope();
        self.return_type_stack.pop();
        self.call_depth -= 1;
        result
    }

    fn validate_return_value(
        &self,
        function_name: &str,
        return_type: &ReturnType,
        value: Option<ReturnValue>,
    ) -> CustResult<Option<ReturnValue>> {
        match (return_type, value) {
            (ReturnType::Scalar(_), Some(ReturnValue::Scalar(value))) => {
                Ok(Some(ReturnValue::Scalar(value)))
            }
            (ReturnType::Scalar(return_type), None) => Err(CustError::new(format!(
                "{} function '{function_name}' returned without a value",
                ReturnType::Scalar(*return_type).value_return_label()
            ))),
            (ReturnType::Scalar(_), Some(ReturnValue::Struct { .. })) => Err(CustError::new(
                format!("struct value returned from scalar function '{function_name}'"),
            )),
            (ReturnType::Scalar(_), Some(ReturnValue::Pointer { .. })) => Err(CustError::new(
                format!("pointer value returned from scalar function '{function_name}'"),
            )),
            (
                ReturnType::Pointer {
                    ty: expected_ty,
                    points_to_const: expected_const,
                },
                Some(ReturnValue::Pointer {
                    pointer,
                    ty,
                    points_to_const,
                }),
            ) if &ty == expected_ty && (!points_to_const || *expected_const) => {
                Ok(Some(ReturnValue::Pointer {
                    pointer,
                    ty,
                    points_to_const,
                }))
            }
            (
                ReturnType::Pointer {
                    ty: expected_ty, ..
                },
                Some(ReturnValue::Pointer { ty, .. }),
            ) if &ty != expected_ty => Err(CustError::new(format!(
                "cannot convert pointer to {} to pointer to {}",
                self.pointee_label(&ty),
                self.pointee_label(expected_ty)
            ))),
            (ReturnType::Pointer { .. }, Some(ReturnValue::Pointer { .. })) => Err(CustError::new(
                "cannot discard const qualifier from pointer target",
            )),
            (ReturnType::Pointer { .. }, Some(ReturnValue::Scalar(_))) => Err(CustError::new(
                format!("pointer function '{function_name}' requires a pointer return value"),
            )),
            (ReturnType::Pointer { .. }, Some(ReturnValue::Struct { .. })) => Err(CustError::new(
                format!("pointer function '{function_name}' requires a pointer return value"),
            )),
            (ReturnType::Pointer { .. }, None) => Err(CustError::new(format!(
                "pointer function '{function_name}' returned without a value"
            ))),
            (
                ReturnType::Struct(expected_type),
                Some(ReturnValue::Struct { type_name, fields }),
            ) if &type_name == expected_type => Ok(Some(ReturnValue::Struct { type_name, fields })),
            (ReturnType::Struct(expected_type), Some(ReturnValue::Struct { type_name, .. })) => {
                Err(CustError::new(format!(
                    "struct function '{function_name}' expected return struct '{}', got struct '{}'",
                    self.aggregate_label(expected_type),
                    self.aggregate_label(&type_name)
                )))
            }
            (ReturnType::Struct(_), Some(ReturnValue::Scalar(_))) => Err(CustError::new(format!(
                "struct function '{function_name}' requires a struct return value"
            ))),
            (ReturnType::Struct(_), Some(ReturnValue::Pointer { .. })) => Err(CustError::new(
                format!("struct function '{function_name}' requires a struct return value"),
            )),
            (ReturnType::Struct(_), None) => Err(CustError::new(format!(
                "struct function '{function_name}' returned without a value"
            ))),
            (ReturnType::Void, Some(_)) => Err(CustError::new(format!(
                "void function '{function_name}' returned a value"
            ))),
            (ReturnType::Void, None) => Ok(None),
        }
    }

    fn eval_struct_argument(
        &mut self,
        function_name: &str,
        param_name: &str,
        expected_type: &str,
        arg_expr: &Expr,
    ) -> CustResult<Value> {
        let struct_value = match arg_expr {
            Expr::Var(arg_name) => match self.find_variable(arg_name).cloned() {
                Some(Value::Struct { type_name, fields }) => Some((type_name, fields)),
                Some(_) => None,
                None => return Err(CustError::new(format!("undefined variable '{arg_name}'"))),
            },
            Expr::StructGet { name, fields: path } => match self.find_variable(name).cloned() {
                Some(Value::Struct { type_name, fields }) => {
                    let (_, field_value) = Self::nested_field_value(&type_name, &fields, path)?;
                    match field_value {
                        StructFieldValue::Struct {
                            type_name, fields, ..
                        } => Some((
                            type_name.clone(),
                            StructFieldValue::deep_clone_fields(fields),
                        )),
                        StructFieldValue::Scalar { .. }
                        | StructFieldValue::Array { .. }
                        | StructFieldValue::Pointer { .. }
                        | StructFieldValue::StructArray { .. } => None,
                    }
                }
                Some(_) => None,
                None => return Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::ArrayGet { name, index } => {
                if let Some((type_name, fields)) = self.indexed_struct_pointer_value(name, index)? {
                    Some((type_name, fields))
                } else {
                    let index = self.checked_struct_element_index(name, index)?;
                    match self.find_variable(name).cloned() {
                        Some(Value::StructArray {
                            type_name,
                            elements,
                            ..
                        }) => Some((
                            type_name,
                            StructFieldValue::deep_clone_fields(&elements[index]),
                        )),
                        Some(_) => None,
                        None => return Err(CustError::new(format!("undefined variable '{name}'"))),
                    }
                }
            }
            Expr::Deref(_)
            | Expr::Call { .. }
            | Expr::Conditional { .. }
            | Expr::Comma(_, _)
            | Expr::AggregateLiteral { .. }
            | Expr::AggregateFieldGet { .. } => match self.eval_struct_expr(arg_expr)? {
                ReturnValue::Struct { type_name, fields } => Some((type_name, fields)),
                ReturnValue::Scalar(_) | ReturnValue::Pointer { .. } => None,
            },
            _ => None,
        };

        match struct_value {
            Some((type_name, fields)) if type_name == expected_type => {
                Ok(Value::Struct { type_name, fields })
            }
            Some((type_name, _)) => Err(CustError::new(format!(
                "function '{function_name}' struct parameter '{param_name}' expected struct '{}', got struct '{}'",
                self.aggregate_label(expected_type),
                self.aggregate_label(&type_name)
            ))),
            None => Err(CustError::new(format!(
                "function '{function_name}' struct parameter '{param_name}' requires a struct argument"
            ))),
        }
    }

    fn exec_block(&mut self, statements: &[Stmt]) -> CustResult<ExecFlow> {
        self.push_scope();
        for stmt in statements {
            match self.exec_stmt(stmt) {
                Ok(ExecFlow::None) => {}
                Ok(flow) => {
                    self.pop_scope();
                    return Ok(flow);
                }
                Err(error) => {
                    self.pop_scope();
                    return Err(error);
                }
            }
        }
        self.pop_scope();
        Ok(ExecFlow::None)
    }

    fn exec_control_body(&mut self, statements: &[Stmt]) -> CustResult<ExecFlow> {
        if statements.len() == 1 {
            self.exec_stmt(&statements[0])
        } else {
            self.exec_block(statements)
        }
    }

    fn push_scope(&mut self) {
        self.push_scope_with_values(HashMap::new());
    }

    fn push_scope_with_values(&mut self, values: HashMap<String, Value>) {
        self.push_scope_with_values_and_consts(values, HashSet::new());
    }

    fn push_scope_with_values_and_consts(
        &mut self,
        values: HashMap<String, Value>,
        const_variables: HashSet<String>,
    ) {
        let id = self.next_scope_id;
        self.next_scope_id += 1;
        self.live_scope_ids.insert(id);
        self.scopes.push(Scope {
            id,
            values,
            static_local_ids: HashMap::new(),
            enum_constants: HashMap::new(),
            const_variables,
        });
    }

    fn mark_current_variable_const(&mut self, name: &str) {
        self.scopes
            .last_mut()
            .expect("exec_block always creates a current scope")
            .const_variables
            .insert(name.to_string());
    }

    fn is_const_variable(&self, name: &str) -> bool {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| {
                if scope.values.contains_key(name) {
                    Some(scope.const_variables.contains(name))
                } else {
                    scope.static_local_ids.get(name).map(|id| {
                        self.static_locals
                            .get(id)
                            .is_some_and(|storage| storage.is_const)
                    })
                }
            })
            .unwrap_or(false)
    }

    fn ensure_variable_mutable(&self, name: &str) -> CustResult<()> {
        if self.is_const_variable(name) {
            Err(CustError::new(format!(
                "cannot assign to const variable '{name}'"
            )))
        } else {
            Ok(())
        }
    }

    fn pointer_variable_points_to_const(&self, name: &str) -> bool {
        match self.find_variable(name) {
            Some(Value::Pointer {
                points_to_const, ..
            }) => *points_to_const,
            Some(Value::Array(array)) => array.borrow().read_only,
            Some(Value::StructArray { read_only, .. }) => *read_only,
            _ => false,
        }
    }

    fn ensure_pointer_variable_pointee_mutable(&self, name: &str) -> CustResult<()> {
        if self.pointer_variable_points_to_const(name) {
            Err(CustError::new("cannot assign through pointer to const"))
        } else {
            Ok(())
        }
    }

    fn aggregate_literal_field_metadata(
        &self,
        aggregate: &Expr,
        path: &[String],
    ) -> CustResult<Option<(StructFieldType, bool, bool)>> {
        let Expr::AggregateLiteral { type_name, .. } = aggregate else {
            return Ok(None);
        };
        let mut current_type_name = type_name;
        for (index, field_name) in path.iter().enumerate() {
            let Some(struct_type) = self.struct_types.get(current_type_name) else {
                return Err(CustError::new(format!(
                    "undefined struct type '{current_type_name}'"
                )));
            };
            let Some(field) = struct_type
                .fields
                .iter()
                .find(|field| field.name == *field_name)
            else {
                return Ok(None);
            };
            if index + 1 == path.len() {
                return Ok(Some((
                    field.ty.clone(),
                    field.is_const,
                    field.points_to_const,
                )));
            }
            match &field.ty {
                StructFieldType::Struct(nested_type) => current_type_name = nested_type,
                _ => return Ok(None),
            }
        }
        Ok(None)
    }

    fn pointer_expr_pointee_type(&self, expr: &Expr) -> CustResult<Option<PointeeType>> {
        match expr {
            Expr::Var(name) | Expr::Assign { name, .. } | Expr::CompoundAssign { name, .. } => {
                match self.find_variable(name) {
                    Some(Value::Pointer { ty, .. }) => Ok(Some(ty.clone())),
                    Some(Value::Array(array)) => {
                        Ok(Some(PointeeType::Scalar(array.borrow().elem_type)))
                    }
                    Some(Value::StructArray { type_name, .. }) => {
                        Ok(Some(PointeeType::Struct(type_name.clone())))
                    }
                    _ => Ok(None),
                }
            }
            Expr::AddressOf(name) => match self.find_variable(name) {
                Some(Value::Scalar { ty, .. }) => Ok(Some(PointeeType::Scalar(*ty))),
                Some(Value::Struct { type_name, .. }) => {
                    Ok(Some(PointeeType::Struct(type_name.clone())))
                }
                _ => Ok(None),
            },
            Expr::AddressOfScalarLiteral { ty, .. } => Ok(Some(PointeeType::Scalar(*ty))),
            Expr::AddressOfAggregateLiteral { type_name, .. } => {
                Ok(Some(PointeeType::Struct(type_name.clone())))
            }
            Expr::AddressOfAggregateField { aggregate, fields } => {
                match self.aggregate_literal_field_metadata(aggregate, fields)? {
                    Some((StructFieldType::Scalar(ty), _, _)) => Ok(Some(PointeeType::Scalar(ty))),
                    Some((StructFieldType::Array(elem_type, _), _, _)) => {
                        Ok(Some(PointeeType::Scalar(elem_type)))
                    }
                    Some((StructFieldType::Struct(type_name), _, _)) => {
                        Ok(Some(PointeeType::Struct(type_name)))
                    }
                    Some((StructFieldType::StructArray(type_name, _), _, _)) => {
                        Ok(Some(PointeeType::Struct(type_name)))
                    }
                    Some((StructFieldType::Pointer(ty), _, _)) => Ok(Some(ty)),
                    None => Ok(None),
                }
            }
            Expr::AggregateFieldGet { aggregate, fields } => {
                match self.aggregate_literal_field_metadata(aggregate, fields)? {
                    Some((StructFieldType::Pointer(ty), _, _)) => Ok(Some(ty)),
                    Some((StructFieldType::Array(elem_type, _), _, _)) => {
                        Ok(Some(PointeeType::Scalar(elem_type)))
                    }
                    Some((StructFieldType::StructArray(type_name, _), _, _)) => {
                        Ok(Some(PointeeType::Struct(type_name)))
                    }
                    _ => Ok(None),
                }
            }
            Expr::AddressOfArray { name, .. } => match self.find_variable(name) {
                Some(Value::Array(array)) => {
                    Ok(Some(PointeeType::Scalar(array.borrow().elem_type)))
                }
                Some(Value::Pointer { ty, .. }) => Ok(Some(ty.clone())),
                _ => Ok(None),
            },
            Expr::AddressOfStructField { name, fields }
            | Expr::AddressOfStructArrayField { name, fields, .. }
            | Expr::StructGet { name, fields }
            | Expr::StructSet { name, fields, .. } => match self.find_variable(name) {
                Some(Value::Struct {
                    type_name,
                    fields: field_map,
                }) => match Self::nested_field_value(type_name, field_map, fields)? {
                    (_, StructFieldValue::Scalar { ty, .. }) => Ok(Some(PointeeType::Scalar(*ty))),
                    (_, StructFieldValue::Array { value, .. }) => {
                        Ok(Some(PointeeType::Scalar(value.borrow().elem_type)))
                    }
                    (_, StructFieldValue::Struct { type_name, .. }) => {
                        Ok(Some(PointeeType::Struct(type_name.clone())))
                    }
                    (_, StructFieldValue::Pointer { ty, .. }) => Ok(Some(ty.clone())),
                    (_, StructFieldValue::StructArray { type_name, .. }) => {
                        Ok(Some(PointeeType::Struct(type_name.clone())))
                    }
                },
                _ => Ok(None),
            },
            Expr::AddressOfStructElementField { name, fields, .. }
            | Expr::AddressOfStructElementArrayField { name, fields, .. } => {
                match self.find_variable(name) {
                    Some(Value::StructArray {
                        type_name,
                        elements,
                        ..
                    }) => {
                        let Some(first_element) = elements.first() else {
                            return Ok(None);
                        };
                        match Self::nested_field_value(type_name, first_element, fields)? {
                            (_, StructFieldValue::Scalar { ty, .. }) => {
                                Ok(Some(PointeeType::Scalar(*ty)))
                            }
                            (_, StructFieldValue::Array { value, .. }) => {
                                Ok(Some(PointeeType::Scalar(value.borrow().elem_type)))
                            }
                            (_, StructFieldValue::Struct { type_name, .. }) => {
                                Ok(Some(PointeeType::Struct(type_name.clone())))
                            }
                            (_, StructFieldValue::Pointer { ty, .. }) => Ok(Some(ty.clone())),
                            (_, StructFieldValue::StructArray { type_name, .. }) => {
                                Ok(Some(PointeeType::Struct(type_name.clone())))
                            }
                        }
                    }
                    _ => Ok(None),
                }
            }
            Expr::StructPtrGet { pointer, fields }
            | Expr::AddressOfStructPtrField { pointer, fields }
            | Expr::AddressOfStructPtrArrayField {
                pointer, fields, ..
            }
            | Expr::StructPtrArrayGet {
                pointer, fields, ..
            } => {
                let Some(PointeeType::Struct(mut current_type_name)) =
                    self.pointer_expr_pointee_type(pointer)?
                else {
                    return Ok(None);
                };
                for (index, field_name) in fields.iter().enumerate() {
                    let Some(struct_type) = self.struct_types.get(&current_type_name) else {
                        return Ok(None);
                    };
                    let Some(field) = struct_type
                        .fields
                        .iter()
                        .find(|field| field.name == *field_name)
                    else {
                        return Ok(None);
                    };
                    let is_last = index + 1 == fields.len();
                    match &field.ty {
                        StructFieldType::Scalar(ty) if is_last => {
                            return Ok(Some(PointeeType::Scalar(*ty)));
                        }
                        StructFieldType::Array(elem_type, _) if is_last => {
                            return Ok(Some(PointeeType::Scalar(*elem_type)));
                        }
                        StructFieldType::Struct(nested_type) if is_last => {
                            return Ok(Some(PointeeType::Struct(nested_type.clone())));
                        }
                        StructFieldType::Struct(nested_type) => {
                            current_type_name = nested_type.clone();
                        }
                        StructFieldType::StructArray(type_name, _) if is_last => {
                            return Ok(Some(PointeeType::Struct(type_name.clone())));
                        }
                        StructFieldType::Pointer(ty) if is_last => return Ok(Some(ty.clone())),
                        _ => return Ok(None),
                    }
                }
                Ok(None)
            }
            Expr::StringLiteral(_) => Ok(Some(PointeeType::Scalar(CType::Char))),
            Expr::Call { name, .. } => match self
                .functions
                .get(name)
                .map(|function| &function.return_type)
            {
                Some(ReturnType::Pointer { ty, .. }) => Ok(Some(ty.clone())),
                _ => Ok(None),
            },
            Expr::Conditional {
                then_expr,
                else_expr,
                ..
            } => {
                let then_type = self.pointer_expr_pointee_type(then_expr)?;
                let else_type = self.pointer_expr_pointee_type(else_expr)?;
                Ok(then_type.or(else_type))
            }
            Expr::Comma(_, right) => self.pointer_expr_pointee_type(right),
            Expr::Binary(left, BinaryOp::Add, right) => {
                let left_type = self.pointer_expr_pointee_type(left)?;
                if left_type.is_some() {
                    Ok(left_type)
                } else {
                    self.pointer_expr_pointee_type(right)
                }
            }
            Expr::Binary(left, BinaryOp::Sub, right) => {
                let left_type = self.pointer_expr_pointee_type(left)?;
                let right_type = self.pointer_expr_pointee_type(right)?;
                if left_type.is_some() && right_type.is_none() {
                    Ok(left_type)
                } else {
                    Ok(None)
                }
            }
            Expr::Increment { target, .. } => self.pointer_expr_pointee_type(target),
            _ => Ok(None),
        }
    }

    fn pointer_expr_points_to_const(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Var(name) => self.pointer_variable_points_to_const(name),
            Expr::AddressOf(name) => self.is_const_variable(name),
            Expr::AddressOfStructField { name, fields }
            | Expr::AddressOfStructArrayField { name, fields, .. } => self
                .find_variable_scope_id(name)
                .and_then(|scope_id| {
                    self.struct_field_points_to_const(scope_id, name, None, fields)
                        .ok()
                })
                .unwrap_or(false),
            Expr::AddressOfStructElementField { name, fields, .. }
            | Expr::AddressOfStructElementArrayField { name, fields, .. } => self
                .find_variable_scope_id(name)
                .and_then(|scope_id| {
                    self.struct_field_points_to_const(scope_id, name, Some(0), fields)
                        .ok()
                })
                .unwrap_or(false),
            Expr::StructGet { name, fields } => {
                self.direct_struct_array_field_points_to_const(name, fields)
                    || self.struct_pointer_field_points_to_const(name, fields)
            }
            Expr::AggregateFieldGet { aggregate, fields } => self
                .aggregate_literal_field_metadata(aggregate, fields)
                .ok()
                .flatten()
                .map(|(field_type, is_const, points_to_const)| match field_type {
                    StructFieldType::Array(_, _) | StructFieldType::StructArray(_, _) => is_const,
                    StructFieldType::Pointer(_) => points_to_const,
                    StructFieldType::Scalar(_) | StructFieldType::Struct(_) => false,
                })
                .unwrap_or(false),
            Expr::AddressOfAggregateField { aggregate, fields } => self
                .aggregate_literal_field_metadata(aggregate, fields)
                .ok()
                .flatten()
                .map(|(field_type, is_const, points_to_const)| match field_type {
                    StructFieldType::Array(_, _) | StructFieldType::StructArray(_, _) => is_const,
                    StructFieldType::Pointer(_) => points_to_const,
                    StructFieldType::Scalar(_) | StructFieldType::Struct(_) => is_const,
                })
                .unwrap_or(false),
            Expr::StructElementGet { name, fields, .. } => {
                self.struct_array_element_field_points_to_const(name, fields)
            }
            Expr::StructPtrGet { pointer, .. }
            | Expr::StructPtrArrayGet { pointer, .. }
            | Expr::AddressOfStructPtrField { pointer, .. }
            | Expr::AddressOfStructPtrArrayField { pointer, .. } => {
                self.pointer_expr_points_to_const(pointer)
            }
            Expr::AddressOfArray { name, .. } => match self.find_variable(name) {
                Some(Value::Array(array)) => array.borrow().read_only,
                Some(Value::Pointer {
                    points_to_const, ..
                }) => *points_to_const,
                _ => false,
            },
            Expr::Comma(_, right) => self.pointer_expr_points_to_const(right),
            Expr::Conditional {
                then_expr,
                else_expr,
                ..
            } => {
                self.pointer_expr_points_to_const(then_expr)
                    || self.pointer_expr_points_to_const(else_expr)
            }
            Expr::Binary(left, BinaryOp::Add | BinaryOp::Sub, right) => {
                self.pointer_expr_points_to_const(left) || self.pointer_expr_points_to_const(right)
            }
            Expr::Assign { name, value } => {
                self.pointer_variable_points_to_const(name)
                    || self.pointer_expr_points_to_const(value)
            }
            Expr::Call { name, .. } => self
                .functions
                .get(name)
                .and_then(|function| match function.return_type {
                    ReturnType::Pointer {
                        points_to_const, ..
                    } => Some(points_to_const),
                    _ => None,
                })
                .unwrap_or(false),
            Expr::StructSet {
                name,
                fields,
                value,
            } => {
                self.struct_pointer_field_points_to_const(name, fields)
                    || self.pointer_expr_points_to_const(value)
            }
            Expr::CompoundAssign { name, .. } => self.pointer_variable_points_to_const(name),
            Expr::Increment { target, .. } => match target.as_ref() {
                Expr::Var(name) => self.pointer_variable_points_to_const(name),
                _ => false,
            },
            _ => false,
        }
    }

    fn ensure_pointer_conversion_preserves_const(
        &self,
        target_points_to_const: bool,
        value: &Expr,
    ) -> CustResult<()> {
        if !target_points_to_const && self.pointer_expr_points_to_const(value) {
            return Err(CustError::new(
                "cannot discard const qualifier from pointer target",
            ));
        }
        Ok(())
    }

    fn aggregate_label(&self, type_name: &str) -> String {
        Self::aggregate_label_from(&self.struct_types, type_name)
    }

    fn aggregate_label_from(
        struct_types: &HashMap<String, StructTypeDef>,
        type_name: &str,
    ) -> String {
        struct_types
            .get(type_name)
            .map(|aggregate| aggregate.display_name.clone())
            .unwrap_or_else(|| type_name.to_string())
    }

    fn pointee_label(&self, ty: &PointeeType) -> String {
        match ty {
            PointeeType::Scalar(CType::Int) => "int".to_string(),
            PointeeType::Scalar(CType::Char) => "char".to_string(),
            PointeeType::Scalar(CType::Bool) => "_Bool".to_string(),
            PointeeType::Struct(type_name) => {
                let aggregate = self.struct_types.get(type_name);
                let keyword = aggregate
                    .map(|aggregate| aggregate.kind.keyword())
                    .unwrap_or("struct");
                let display_name = aggregate
                    .map(|aggregate| aggregate.display_name.as_str())
                    .unwrap_or(type_name);
                format!("{keyword} '{display_name}'")
            }
        }
    }

    fn pointer_value_type(&self, pointer: &PointerValue) -> CustResult<Option<PointeeType>> {
        match pointer {
            PointerValue::Null => Ok(None),
            PointerValue::Scalar { scope_id, name } => {
                let value = self
                    .scopes
                    .iter()
                    .find(|scope| scope.id == *scope_id)
                    .and_then(|scope| scope.values.get(name))
                    .or_else(|| self.static_value_by_scope(*scope_id, name));
                match value {
                    Some(Value::Scalar { ty, .. }) => Ok(Some(PointeeType::Scalar(*ty))),
                    _ => Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    ))),
                }
            }
            PointerValue::ArrayBase { array, .. } | PointerValue::ArrayElement { array, .. } => {
                Ok(Some(PointeeType::Scalar(array.borrow().elem_type)))
            }
            PointerValue::Struct { scope_id, name } => {
                let value = self
                    .scopes
                    .iter()
                    .find(|scope| scope.id == *scope_id)
                    .and_then(|scope| scope.values.get(name))
                    .or_else(|| self.static_value_by_scope(*scope_id, name));
                match value {
                    Some(Value::Struct { type_name, .. }) => {
                        Ok(Some(PointeeType::Struct(type_name.clone())))
                    }
                    _ => Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    ))),
                }
            }
            PointerValue::StructElement {
                scope_id,
                name,
                index: _,
            } => {
                let value = self
                    .scopes
                    .iter()
                    .find(|scope| scope.id == *scope_id)
                    .and_then(|scope| scope.values.get(name))
                    .or_else(|| self.static_value_by_scope(*scope_id, name));
                match value {
                    Some(Value::StructArray { type_name, .. }) => {
                        Ok(Some(PointeeType::Struct(type_name.clone())))
                    }
                    _ => Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    ))),
                }
            }
            PointerValue::StructFieldElement {
                scope_id,
                name,
                element_index,
                fields,
                ..
            } => match self.struct_field_by_scope(*scope_id, name, *element_index, fields)? {
                StructFieldValue::StructArray { type_name, .. } => {
                    Ok(Some(PointeeType::Struct(type_name.clone())))
                }
                _ => Ok(None),
            },
            PointerValue::StructFieldElementField {
                scope_id,
                name,
                element_index,
                array_fields,
                index,
                fields,
            } => {
                let (type_name, element_fields) = self.struct_field_array_element_fields(
                    *scope_id,
                    name,
                    *element_index,
                    array_fields,
                    *index,
                )?;
                match Self::nested_field_value(&type_name, element_fields, fields)?.1 {
                    StructFieldValue::Scalar { ty, .. } => Ok(Some(PointeeType::Scalar(*ty))),
                    StructFieldValue::Array { value, .. } => {
                        Ok(Some(PointeeType::Scalar(value.borrow().elem_type)))
                    }
                    StructFieldValue::Struct { type_name, .. } => {
                        Ok(Some(PointeeType::Struct(type_name.clone())))
                    }
                    StructFieldValue::Pointer { ty, .. } => Ok(Some(ty.clone())),
                    StructFieldValue::StructArray { type_name, .. } => {
                        Ok(Some(PointeeType::Struct(type_name.clone())))
                    }
                }
            }
            PointerValue::StructField {
                scope_id,
                name,
                element_index,
                fields,
            } => match self.struct_field_by_scope(*scope_id, name, *element_index, fields)? {
                StructFieldValue::Scalar { ty, .. } => Ok(Some(PointeeType::Scalar(*ty))),
                StructFieldValue::Array { value, .. } => {
                    Ok(Some(PointeeType::Scalar(value.borrow().elem_type)))
                }
                StructFieldValue::Struct { type_name, .. } => {
                    Ok(Some(PointeeType::Struct(type_name.clone())))
                }
                StructFieldValue::Pointer { ty, .. } => Ok(Some(ty.clone())),
                StructFieldValue::StructArray { type_name, .. } => {
                    Ok(Some(PointeeType::Struct(type_name.clone())))
                }
            },
        }
    }

    fn ensure_pointer_type_matches(
        &self,
        expected: &PointeeType,
        pointer: &PointerValue,
    ) -> CustResult<()> {
        let Some(actual) = self.pointer_value_type(pointer)? else {
            return Ok(());
        };
        if &actual != expected {
            return Err(CustError::new(format!(
                "cannot convert pointer to {} to pointer to {}",
                self.pointee_label(&actual),
                self.pointee_label(expected)
            )));
        }
        Ok(())
    }

    fn ensure_pointer_expr_pointee_mutable(&self, expr: &Expr) -> CustResult<()> {
        if self.pointer_expr_points_to_const(expr) {
            Err(CustError::new("cannot assign through pointer to const"))
        } else {
            Ok(())
        }
    }

    fn ensure_struct_pointer_target_mutable(&self, pointer: &PointerValue) -> CustResult<()> {
        match pointer {
            PointerValue::Struct { scope_id, name }
                if self
                    .scopes
                    .iter()
                    .find(|scope| scope.id == *scope_id)
                    .is_some_and(|scope| scope.const_variables.contains(name))
                    || self.static_locals.values().any(|storage| {
                        storage.scope_id == *scope_id && storage.name == *name && storage.is_const
                    }) =>
            {
                Err(CustError::new("cannot assign through pointer to const"))
            }
            _ => Ok(()),
        }
    }

    fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            self.live_scope_ids.remove(&scope.id);
        }
    }

    fn current_scope_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut self
            .scopes
            .last_mut()
            .expect("exec_block always creates a current scope")
            .values
    }

    fn current_scope_has_identifier(&self, name: &str) -> bool {
        let scope = self
            .scopes
            .last()
            .expect("exec_block always creates a current scope");
        scope.values.contains_key(name)
            || scope.static_local_ids.contains_key(name)
            || scope.enum_constants.contains_key(name)
    }

    fn insert_enum_constant(&mut self, name: String, value: i64) -> CustResult<()> {
        if self.current_scope_has_identifier(&name) {
            return Err(CustError::new(format!(
                "variable '{name}' already declared in this scope"
            )));
        }
        self.scopes
            .last_mut()
            .expect("exec_block always creates a current scope")
            .enum_constants
            .insert(name, value);
        Ok(())
    }

    fn find_enum_constant(&self, name: &str) -> Option<i64> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.enum_constants.get(name).copied())
    }

    fn find_variable(&self, name: &str) -> Option<&Value> {
        self.scopes.iter().rev().find_map(|scope| {
            scope.values.get(name).or_else(|| {
                scope
                    .static_local_ids
                    .get(name)
                    .and_then(|id| self.static_locals.get(id))
                    .map(|storage| &storage.value)
            })
        })
    }

    fn find_variable_mut(&mut self, name: &str) -> Option<&mut Value> {
        let mut static_id = None;
        for scope in self.scopes.iter().rev() {
            if scope.values.contains_key(name) {
                break;
            }
            if let Some(id) = scope.static_local_ids.get(name) {
                static_id = Some(*id);
                break;
            }
        }
        if let Some(id) = static_id {
            return self
                .static_locals
                .get_mut(&id)
                .map(|storage| &mut storage.value);
        }
        self.scopes
            .iter_mut()
            .rev()
            .find_map(|scope| scope.values.get_mut(name))
    }

    fn find_scalar(&self, name: &str) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::Scalar { value, .. }) => Ok(*value),
            Some(Value::Array(_)) => Err(CustError::new(format!("array '{name}' used as scalar"))),
            Some(Value::Pointer { .. }) => {
                Err(CustError::new(format!("pointer '{name}' used as scalar")))
            }
            Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => Err(CustError::new(
                format!("struct variable '{name}' used as scalar"),
            )),
            None => self
                .find_enum_constant(name)
                .ok_or_else(|| CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn find_array(&self, name: &str) -> CustResult<Rc<RefCell<ArrayValue>>> {
        match self.find_variable(name) {
            Some(Value::Array(values)) => Ok(Rc::clone(values)),
            Some(Value::Scalar { .. }) => {
                Err(CustError::new(format!("variable '{name}' is not an array")))
            }
            Some(Value::Pointer { .. }) => {
                Err(CustError::new(format!("pointer '{name}' is not an array")))
            }
            Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => Err(CustError::new(
                format!("struct variable '{name}' is not an array"),
            )),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn make_array_value(
        &mut self,
        len: usize,
        elem_type: CType,
        init: &[ArrayInitializer],
        read_only: bool,
    ) -> CustResult<Value> {
        let mut array = ArrayValue::mutable_zeroed(len, elem_type);
        let mut next_positional_index = 0usize;
        for initializer in init {
            match initializer {
                ArrayInitializer::Expr(expr) => {
                    array.elements[next_positional_index] = self.eval(expr)?;
                    next_positional_index += 1;
                }
                ArrayInitializer::Designated { index, value } => {
                    array.elements[*index] = self.eval(value)?;
                    next_positional_index = *index + 1;
                }
                ArrayInitializer::StringLiteral(values) => {
                    for (index, value) in values.iter().take(len).enumerate() {
                        array.elements[index] = *value;
                    }
                    next_positional_index = values.len().min(len);
                }
            }
        }
        array.read_only = read_only;
        Ok(Value::Array(Rc::new(RefCell::new(array))))
    }

    fn make_array_compound_literal(
        &mut self,
        len: Option<usize>,
        elem_type: CType,
        init: &[ArrayInitializer],
    ) -> CustResult<Rc<RefCell<ArrayValue>>> {
        let len = len.unwrap_or_else(|| Self::infer_array_initializer_len(init));
        let Value::Array(array) = self.make_array_value(len, elem_type, init, false)? else {
            unreachable!("make_array_value always returns Value::Array")
        };
        Ok(array)
    }

    fn infer_array_initializer_len(init: &[ArrayInitializer]) -> usize {
        let mut len = 0usize;
        let mut next_positional_index = 0usize;
        for initializer in init {
            match initializer {
                ArrayInitializer::Expr(_) => {
                    len = len.max(next_positional_index + 1);
                    next_positional_index += 1;
                }
                ArrayInitializer::Designated { index, .. } => {
                    len = len.max(index + 1);
                    next_positional_index = index + 1;
                }
                ArrayInitializer::StringLiteral(values) => {
                    len = len.max(values.len());
                    next_positional_index = values.len();
                }
            }
        }
        len
    }

    fn infer_struct_array_initializer_len(init: &[StructArrayInitializer]) -> usize {
        let mut len = 0usize;
        let mut next_positional_index = 0usize;
        for initializer in init {
            match initializer {
                StructArrayInitializer::Element(_) => {
                    len = len.max(next_positional_index + 1);
                    next_positional_index += 1;
                }
                StructArrayInitializer::Designated { index, .. } => {
                    len = len.max(index + 1);
                    next_positional_index = index + 1;
                }
            }
        }
        len
    }

    fn make_struct_value(
        &mut self,
        type_name: &str,
        init: Option<&StructVarInitializer>,
    ) -> CustResult<Value> {
        let fields = match init {
            Some(StructVarInitializer::Fields(init)) => self.make_struct_fields(type_name, init)?,
            Some(StructVarInitializer::Expr(expr)) => match self.eval_struct_expr(expr)? {
                ReturnValue::Struct {
                    type_name: rhs_type,
                    fields,
                } if rhs_type == type_name => fields,
                ReturnValue::Struct {
                    type_name: rhs_type,
                    ..
                } => {
                    return Err(CustError::new(format!(
                        "cannot assign struct '{}' to struct '{}'",
                        self.aggregate_label(&rhs_type),
                        self.aggregate_label(type_name)
                    )));
                }
                ReturnValue::Scalar(_) => {
                    return Err(CustError::new("struct initializer requires struct value"));
                }
                ReturnValue::Pointer { .. } => {
                    return Err(CustError::new("struct initializer requires struct value"));
                }
            },
            None => self.make_struct_fields(type_name, &[])?,
        };
        Ok(Value::Struct {
            type_name: type_name.to_string(),
            fields,
        })
    }

    fn make_struct_array_value(
        &mut self,
        type_name: &str,
        len: usize,
        init: &[StructArrayInitializer],
        read_only: bool,
    ) -> CustResult<Value> {
        let mut elements = Vec::with_capacity(len);
        for _ in 0..len {
            elements.push(self.make_struct_fields(type_name, &[])?);
        }
        let mut next_positional_index = 0usize;
        for initializer in init {
            match initializer {
                StructArrayInitializer::Element(element_init) => {
                    if next_positional_index == len {
                        return Err(CustError::new("too many initializers for struct array"));
                    }
                    elements[next_positional_index] =
                        self.make_struct_fields(type_name, element_init.as_slice())?;
                    next_positional_index += 1;
                }
                StructArrayInitializer::Designated { index, value } => {
                    elements[*index] = self.make_struct_fields(type_name, value.as_slice())?;
                    next_positional_index = index + 1;
                }
            }
        }
        Ok(Value::StructArray {
            type_name: type_name.to_string(),
            elements,
            read_only,
        })
    }

    fn make_aggregate_array_compound_literal(
        &mut self,
        type_name: &str,
        len: Option<usize>,
        init: &[StructArrayInitializer],
    ) -> CustResult<PointerValue> {
        let len = len.unwrap_or_else(|| Self::infer_struct_array_initializer_len(init));
        let value = self.make_struct_array_value(type_name, len, init, false)?;
        let scope_id = self
            .scopes
            .last()
            .expect("compound literal evaluation requires a current scope")
            .id;
        let name = format!(
            "__cust_compound_aggregate_array#{}",
            self.next_compound_literal_id
        );
        self.next_compound_literal_id += 1;
        self.current_scope_mut().insert(name.clone(), value);
        Ok(PointerValue::StructElement {
            scope_id,
            name,
            index: 0,
        })
    }

    fn make_scalar_compound_literal_pointer(
        &mut self,
        ty: CType,
        init: &Expr,
    ) -> CustResult<PointerValue> {
        let value = self.eval(init)?;
        let scope_id = self
            .scopes
            .last()
            .expect("compound literal evaluation requires a current scope")
            .id;
        let name = format!("__cust_compound_scalar#{}", self.next_compound_literal_id);
        self.next_compound_literal_id += 1;
        self.current_scope_mut()
            .insert(name.clone(), Value::Scalar { value, ty });
        Ok(PointerValue::Scalar { scope_id, name })
    }

    fn make_aggregate_compound_literal_pointer(
        &mut self,
        type_name: &str,
        init: &[StructInitializer],
    ) -> CustResult<PointerValue> {
        let fields = self.make_struct_fields(type_name, init)?;
        let scope_id = self
            .scopes
            .last()
            .expect("compound literal evaluation requires a current scope")
            .id;
        let name = format!(
            "__cust_compound_aggregate#{}",
            self.next_compound_literal_id
        );
        self.next_compound_literal_id += 1;
        self.current_scope_mut().insert(
            name.clone(),
            Value::Struct {
                type_name: type_name.to_string(),
                fields,
            },
        );
        Ok(PointerValue::Struct { scope_id, name })
    }

    fn make_aggregate_compound_literal_field_pointer(
        &mut self,
        aggregate: &Expr,
        fields: &[String],
    ) -> CustResult<PointerValue> {
        let Expr::AggregateLiteral { type_name, init } = aggregate else {
            return Err(CustError::new(
                "invalid address-of target for aggregate expression field",
            ));
        };
        match self.make_aggregate_compound_literal_pointer(type_name, init)? {
            PointerValue::Struct { scope_id, name } => {
                match self.struct_field_by_scope(scope_id, &name, None, fields)? {
                    StructFieldValue::Scalar { .. } | StructFieldValue::Struct { .. } => {
                        Ok(PointerValue::StructField {
                            scope_id,
                            name,
                            element_index: None,
                            fields: fields.to_vec(),
                        })
                    }
                    StructFieldValue::Array { value, .. } => Ok(PointerValue::ArrayBase {
                        array: Rc::clone(value),
                        source_name: Some(Self::field_path_label(fields).to_string()),
                    }),
                    StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                        "struct field '{}' requires indexed field access",
                        Self::field_path_label(fields)
                    ))),
                    StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                        "pointer field '{}' cannot be addressed in this pointer milestone",
                        Self::field_path_label(fields)
                    ))),
                }
            }
            _ => Err(CustError::new(
                "aggregate compound literal did not produce a struct pointer",
            )),
        }
    }

    fn make_struct_fields(
        &mut self,
        type_name: &str,
        init: &[StructInitializer],
    ) -> CustResult<HashMap<String, StructFieldValue>> {
        let struct_type = self
            .struct_types
            .get(type_name)
            .cloned()
            .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'")))?;
        let mut fields = HashMap::new();
        for field in &struct_type.fields {
            fields.insert(field.name.clone(), self.default_struct_field_value(field)?);
        }
        for initializer in init {
            let StructInitializer::Designated { field, value } = initializer else {
                unreachable!("struct initializer parsing resolves positional entries to fields")
            };
            let field_def = struct_type
                .fields
                .iter()
                .find(|field_def| field_def.name == *field)
                .ok_or_else(|| {
                    CustError::new(format!("struct '{type_name}' has no field '{field}'"))
                })?;
            self.apply_struct_field_initializer(&mut fields, field_def, value)?;
            if struct_type.kind == AggregateKind::Union {
                Self::sync_union_scalar_fields_from_active(&mut fields, field)?;
            }
        }
        Ok(fields)
    }

    fn sync_union_scalar_fields_from_active(
        fields: &mut HashMap<String, StructFieldValue>,
        active_field: &str,
    ) -> CustResult<()> {
        let active_value = match fields.get(active_field) {
            Some(StructFieldValue::Scalar { value, .. }) => Some(*value),
            Some(_) => None,
            None => {
                return Err(CustError::new(format!(
                    "missing union field '{active_field}'"
                )));
            }
        };
        if let Some(active_value) = active_value {
            for field_value in fields.values_mut() {
                if let StructFieldValue::Scalar { value, .. } = field_value {
                    *value = active_value;
                }
            }
        }
        Ok(())
    }

    fn default_struct_field_value(
        &mut self,
        field: &StructFieldDef,
    ) -> CustResult<StructFieldValue> {
        match &field.ty {
            StructFieldType::Scalar(ty) => Ok(StructFieldValue::Scalar {
                value: 0,
                ty: *ty,
                is_const: field.is_const,
            }),
            StructFieldType::Array(elem_type, len) => {
                let mut array = ArrayValue::mutable_zeroed(*len, *elem_type);
                array.read_only = field.is_const;
                Ok(StructFieldValue::Array {
                    value: Rc::new(RefCell::new(array)),
                    is_const: field.is_const,
                })
            }
            StructFieldType::Struct(nested_type) => Ok(StructFieldValue::Struct {
                type_name: nested_type.clone(),
                fields: self.make_struct_fields(nested_type, &[])?,
                is_const: field.is_const,
            }),
            StructFieldType::StructArray(nested_type, len) => {
                let mut elements = Vec::with_capacity(*len);
                for _ in 0..*len {
                    elements.push(self.make_struct_fields(nested_type, &[])?);
                }
                Ok(StructFieldValue::StructArray {
                    type_name: nested_type.clone(),
                    elements,
                    is_const: field.is_const,
                })
            }
            StructFieldType::Pointer(pointee) => Ok(StructFieldValue::Pointer {
                pointer: PointerValue::Null,
                ty: pointee.clone(),
                is_const: field.is_const,
                points_to_const: field.points_to_const,
            }),
        }
    }

    fn apply_struct_field_initializer(
        &mut self,
        fields: &mut HashMap<String, StructFieldValue>,
        field: &StructFieldDef,
        initializer: &StructInitializer,
    ) -> CustResult<()> {
        let field_value = fields
            .get_mut(&field.name)
            .ok_or_else(|| CustError::new(format!("missing struct field '{}'", field.name)))?;
        match (&field.ty, field_value, initializer) {
            (
                StructFieldType::Scalar(_),
                StructFieldValue::Scalar { value, .. },
                StructInitializer::Expr(expr),
            ) => {
                *value = self.eval(expr)?;
            }
            (
                StructFieldType::Scalar(_),
                _,
                StructInitializer::Array(_) | StructInitializer::Struct(_),
            ) => {
                return Err(CustError::new(format!(
                    "nested initializer for scalar field '{}' is not supported",
                    field.name
                )));
            }
            (
                StructFieldType::Array(_, _),
                StructFieldValue::Array { value, .. },
                StructInitializer::Array(array_init),
            ) => {
                let mut array = value.borrow_mut();
                let mut next_positional_index = 0usize;
                for initializer in array_init {
                    match initializer {
                        ArrayInitializer::Expr(expr) => {
                            array.elements[next_positional_index] = self.eval(expr)?;
                            next_positional_index += 1;
                        }
                        ArrayInitializer::Designated { index, value } => {
                            array.elements[*index] = self.eval(value)?;
                            next_positional_index = *index + 1;
                        }
                        ArrayInitializer::StringLiteral(values) => {
                            for (index, value) in
                                values.iter().take(array.elements.len()).enumerate()
                            {
                                array.elements[index] = *value;
                            }
                            next_positional_index = values.len().min(array.elements.len());
                        }
                    }
                }
            }
            (
                StructFieldType::Array(_, _),
                _,
                StructInitializer::Expr(_) | StructInitializer::Struct(_),
            ) => {
                return Err(CustError::new(format!(
                    "expected array initializer for struct field '{}'",
                    field.name
                )));
            }
            (
                StructFieldType::Struct(nested_type),
                StructFieldValue::Struct {
                    fields: nested_fields,
                    ..
                },
                StructInitializer::Struct(nested_init),
            ) => {
                let nested_type_def =
                    self.struct_types.get(nested_type).cloned().ok_or_else(|| {
                        CustError::new(format!("undefined struct type '{nested_type}'"))
                    })?;
                for nested_initializer in nested_init {
                    let StructInitializer::Designated { field, value } = nested_initializer else {
                        unreachable!(
                            "nested struct initializer parsing resolves positional entries to fields"
                        )
                    };
                    let nested_field_def = nested_type_def
                        .fields
                        .iter()
                        .find(|field_def| field_def.name == *field)
                        .ok_or_else(|| {
                            CustError::new(format!("struct '{nested_type}' has no field '{field}'"))
                        })?;
                    self.apply_struct_field_initializer(nested_fields, nested_field_def, value)?;
                    if nested_type_def.kind == AggregateKind::Union {
                        Self::sync_union_scalar_fields_from_active(nested_fields, field)?;
                    }
                }
            }
            (StructFieldType::Struct(_), _, StructInitializer::Array(_)) => {
                return Err(CustError::new(format!(
                    "array initializer for struct field '{}' is not supported",
                    field.name
                )));
            }
            (
                StructFieldType::StructArray(type_name, len),
                field_value,
                StructInitializer::StructArray(init),
            ) => {
                let Value::StructArray { elements, .. } =
                    self.make_struct_array_value(type_name, *len, init, field.is_const)?
                else {
                    unreachable!("make_struct_array_value returns a struct array value")
                };
                *field_value = StructFieldValue::StructArray {
                    type_name: type_name.clone(),
                    elements,
                    is_const: field.is_const,
                };
            }
            (
                StructFieldType::StructArray(_, _),
                _,
                StructInitializer::Expr(_)
                | StructInitializer::Array(_)
                | StructInitializer::Struct(_),
            ) => {
                return Err(CustError::new(format!(
                    "expected struct array initializer for struct field '{}'",
                    field.name
                )));
            }
            (StructFieldType::Struct(nested_type), field_value, StructInitializer::Expr(expr)) => {
                match self.eval_struct_expr(expr)? {
                    ReturnValue::Struct { type_name, fields } if type_name == *nested_type => {
                        *field_value = StructFieldValue::Struct {
                            type_name,
                            fields,
                            is_const: field.is_const,
                        };
                    }
                    ReturnValue::Struct { type_name, .. } => {
                        return Err(CustError::new(format!(
                            "cannot initialize struct field '{}' of type '{}' with struct '{}'",
                            field.name, nested_type, type_name
                        )));
                    }
                    ReturnValue::Scalar(_) => {
                        unreachable!("eval_struct_expr only returns struct values or errors")
                    }
                    ReturnValue::Pointer { .. } => {
                        unreachable!("eval_struct_expr only returns struct values or errors")
                    }
                }
            }
            (
                StructFieldType::Pointer(_),
                StructFieldValue::Pointer { pointer, .. },
                StructInitializer::Expr(expr),
            ) => {
                self.ensure_pointer_conversion_preserves_const(field.points_to_const, expr)?;
                let assigned = self.eval_pointer(expr)?;
                if let StructFieldType::Pointer(expected_ty) = &field.ty {
                    self.ensure_pointer_type_matches(expected_ty, &assigned)?;
                }
                *pointer = assigned;
            }
            (
                StructFieldType::Pointer(_),
                _,
                StructInitializer::Array(_) | StructInitializer::Struct(_),
            ) => {
                return Err(CustError::new(format!(
                    "nested initializer for pointer field '{}' is not supported",
                    field.name
                )));
            }
            (_, _, StructInitializer::Designated { .. }) => {
                unreachable!("designated struct initializers are resolved before evaluation")
            }
            _ => unreachable!("struct field definition and value variants must match"),
        }
        Ok(())
    }

    fn field_path_label(fields: &[String]) -> &str {
        fields.last().map(String::as_str).unwrap_or("<missing>")
    }

    fn nested_field_value<'a>(
        type_name: &'a str,
        fields_map: &'a HashMap<String, StructFieldValue>,
        path: &[String],
    ) -> CustResult<(&'a str, &'a StructFieldValue)> {
        let Some((field, rest)) = path.split_first() else {
            return Err(CustError::new("expected struct field"));
        };
        let value = fields_map.get(field).ok_or_else(|| {
            CustError::new(format!("struct '{type_name}' has no field '{field}'"))
        })?;
        if rest.is_empty() {
            return Ok((type_name, value));
        }
        match value {
            StructFieldValue::Struct {
                type_name, fields, ..
            } => Self::nested_field_value(type_name, fields, rest),
            StructFieldValue::Scalar { .. }
            | StructFieldValue::Array { .. }
            | StructFieldValue::StructArray { .. }
            | StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                "struct field '{field}' is not a struct"
            ))),
        }
    }

    fn nested_field_value_mut<'a>(
        type_name: &'a str,
        fields_map: &'a mut HashMap<String, StructFieldValue>,
        path: &[String],
    ) -> CustResult<(&'a str, &'a mut StructFieldValue)> {
        let Some((field, rest)) = path.split_first() else {
            return Err(CustError::new("expected struct field"));
        };
        let value = fields_map.get_mut(field).ok_or_else(|| {
            CustError::new(format!("struct '{type_name}' has no field '{field}'"))
        })?;
        if rest.is_empty() {
            return Ok((type_name, value));
        }
        match value {
            StructFieldValue::Struct {
                type_name, fields, ..
            } => Self::nested_field_value_mut(type_name, fields, rest),
            StructFieldValue::Scalar { .. }
            | StructFieldValue::Array { .. }
            | StructFieldValue::StructArray { .. }
            | StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                "struct field '{field}' is not a struct"
            ))),
        }
    }

    fn nested_struct_field<'a>(
        type_name: &str,
        fields_map: &'a HashMap<String, StructFieldValue>,
        path: &[String],
    ) -> CustResult<(String, &'a HashMap<String, StructFieldValue>)> {
        let Some((field, rest)) = path.split_first() else {
            return Err(CustError::new("expected struct field"));
        };
        let value = fields_map.get(field).ok_or_else(|| {
            CustError::new(format!("struct '{type_name}' has no field '{field}'"))
        })?;
        match value {
            StructFieldValue::Struct {
                type_name, fields, ..
            } if rest.is_empty() => Ok((type_name.clone(), fields)),
            StructFieldValue::Struct {
                type_name, fields, ..
            } => Self::nested_struct_field(type_name, fields, rest),
            StructFieldValue::Scalar { .. }
            | StructFieldValue::Array { .. }
            | StructFieldValue::StructArray { .. }
            | StructFieldValue::Pointer { .. } => {
                if rest.is_empty() {
                    Err(CustError::new("pointer does not reference a struct"))
                } else {
                    Err(CustError::new(format!(
                        "struct field '{field}' is not a struct"
                    )))
                }
            }
        }
    }

    fn nested_struct_field_mut<'a>(
        type_name: &str,
        fields_map: &'a mut HashMap<String, StructFieldValue>,
        path: &[String],
    ) -> CustResult<(String, &'a mut HashMap<String, StructFieldValue>)> {
        let Some((field, rest)) = path.split_first() else {
            return Err(CustError::new("expected struct field"));
        };
        let value = fields_map.get_mut(field).ok_or_else(|| {
            CustError::new(format!("struct '{type_name}' has no field '{field}'"))
        })?;
        if rest.is_empty() {
            return match value {
                StructFieldValue::Struct {
                    type_name, fields, ..
                } => Ok((type_name.clone(), fields)),
                StructFieldValue::Scalar { .. }
                | StructFieldValue::Array { .. }
                | StructFieldValue::StructArray { .. }
                | StructFieldValue::Pointer { .. } => {
                    Err(CustError::new("pointer does not reference a struct"))
                }
            };
        }
        match value {
            StructFieldValue::Struct {
                type_name, fields, ..
            } => Self::nested_struct_field_mut(type_name, fields, rest),
            StructFieldValue::Scalar { .. }
            | StructFieldValue::Array { .. }
            | StructFieldValue::StructArray { .. }
            | StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                "struct field '{field}' is not a struct"
            ))),
        }
    }

    fn read_struct_field(&self, name: &str, path: &[String]) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::Struct { type_name, fields }) => {
                let (_, field_value) = Self::nested_field_value(type_name, fields, path)?;
                match field_value {
                    StructFieldValue::Scalar { value, .. } => Ok(*value),
                    StructFieldValue::Array { .. } => Err(CustError::new(format!(
                        "struct field '{}' is an array",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::Struct { type_name, .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct '{type_name}'",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct array",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                        "pointer field '{}' used as scalar",
                        Self::field_path_label(path)
                    ))),
                }
            }
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn assign_scalar_field_in_map(
        struct_types: &HashMap<String, StructTypeDef>,
        type_name: &str,
        fields_map: &mut HashMap<String, StructFieldValue>,
        path: &[String],
        value: i64,
    ) -> CustResult<()> {
        let Some((field, rest)) = path.split_first() else {
            return Err(CustError::new("expected struct field"));
        };
        if rest.is_empty() {
            let field_value = fields_map.get_mut(field).ok_or_else(|| {
                CustError::new(format!("struct '{type_name}' has no field '{field}'"))
            })?;
            if field_value.is_const() {
                return Err(CustError::new(format!(
                    "cannot assign to const struct field '{}'",
                    Self::field_path_label(path)
                )));
            }
            match field_value {
                StructFieldValue::Scalar { value: slot, .. } => {
                    *slot = value;
                    if struct_types
                        .get(type_name)
                        .is_some_and(|aggregate| aggregate.kind == AggregateKind::Union)
                    {
                        Self::sync_union_scalar_fields_from_active(fields_map, field)?;
                    }
                    Ok(())
                }
                StructFieldValue::Array { .. } => Err(CustError::new(format!(
                    "struct field '{}' is an array",
                    Self::field_path_label(path)
                ))),
                StructFieldValue::Struct { type_name, .. } => Err(CustError::new(format!(
                    "struct field '{}' is a struct '{type_name}'",
                    Self::field_path_label(path)
                ))),
                StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                    "struct field '{}' is a struct array",
                    Self::field_path_label(path)
                ))),
                StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                    "pointer field '{}' used as scalar",
                    Self::field_path_label(path)
                ))),
            }
        } else {
            let field_value = fields_map.get_mut(field).ok_or_else(|| {
                CustError::new(format!("struct '{type_name}' has no field '{field}'"))
            })?;
            match field_value {
                StructFieldValue::Struct {
                    type_name: nested_type,
                    fields: nested_fields,
                    ..
                } => Self::assign_scalar_field_in_map(
                    struct_types,
                    nested_type,
                    nested_fields,
                    rest,
                    value,
                ),
                StructFieldValue::Scalar { .. }
                | StructFieldValue::Array { .. }
                | StructFieldValue::StructArray { .. }
                | StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                    "struct field '{field}' is not a struct"
                ))),
            }
        }
    }

    fn assign_struct_field(&mut self, name: &str, path: &[String], value: i64) -> CustResult<()> {
        self.ensure_variable_mutable(name)?;
        let struct_types = self.struct_types.clone();
        match self.find_variable_mut(name) {
            Some(Value::Struct { type_name, fields }) => {
                Self::assign_scalar_field_in_map(&struct_types, type_name, fields, path, value)
            }
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn nested_field_path_is_const(
        fields_map: &HashMap<String, StructFieldValue>,
        path: &[String],
    ) -> CustResult<bool> {
        let Some((field, rest)) = path.split_first() else {
            return Err(CustError::new("expected struct field"));
        };
        let value = fields_map
            .get(field)
            .ok_or_else(|| CustError::new(format!("struct has no field '{field}'")))?;
        if value.is_const() {
            return Ok(true);
        }
        if rest.is_empty() {
            return Ok(false);
        }
        match value {
            StructFieldValue::Struct { fields, .. } => {
                Self::nested_field_path_is_const(fields, rest)
            }
            StructFieldValue::Scalar { .. }
            | StructFieldValue::Array { .. }
            | StructFieldValue::StructArray { .. }
            | StructFieldValue::Pointer { .. } => Ok(false),
        }
    }

    fn direct_struct_array_field_points_to_const(&self, name: &str, path: &[String]) -> bool {
        let root_is_const = self.is_const_variable(name);
        match self.find_variable(name) {
            Some(Value::Struct { fields, .. }) => {
                root_is_const || Self::nested_field_path_is_const(fields, path).unwrap_or(false)
            }
            _ => false,
        }
    }

    fn struct_array_element_field_points_to_const(&self, name: &str, path: &[String]) -> bool {
        match self.find_variable(name) {
            Some(Value::StructArray {
                elements,
                read_only,
                ..
            }) => {
                *read_only
                    || elements.first().is_some_and(|fields| {
                        Self::nested_field_path_is_const(fields, path).unwrap_or(false)
                    })
            }
            _ => false,
        }
    }

    fn struct_pointer_field_points_to_const(&self, name: &str, path: &[String]) -> bool {
        match self.find_variable(name) {
            Some(Value::Struct { type_name, fields }) => {
                match Self::nested_field_value(type_name, fields, path) {
                    Ok((
                        _,
                        StructFieldValue::Pointer {
                            points_to_const, ..
                        },
                    )) => *points_to_const,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn struct_field_is_pointer(&self, name: &str, path: &[String]) -> bool {
        matches!(
            self.find_variable(name),
            Some(Value::Struct { type_name, fields })
                if matches!(
                    Self::nested_field_value(type_name, fields, path),
                    Ok((_, StructFieldValue::Pointer { .. }))
                )
        )
    }

    fn read_direct_struct_pointer_field(
        &self,
        name: &str,
        path: &[String],
    ) -> CustResult<PointerValue> {
        match self.find_variable(name) {
            Some(Value::Struct { type_name, fields }) => {
                let (_, field_value) = Self::nested_field_value(type_name, fields, path)?;
                match field_value {
                    StructFieldValue::Pointer { pointer, .. } => Ok(pointer.clone()),
                    StructFieldValue::Scalar { .. } => Err(CustError::new(format!(
                        "struct field '{}' is not a pointer",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::Array { .. } => Err(CustError::new(format!(
                        "struct field '{}' is an array",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::Struct { type_name, .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct '{type_name}'",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct array",
                        Self::field_path_label(path)
                    ))),
                }
            }
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn assign_direct_struct_pointer_field(
        &mut self,
        name: &str,
        path: &[String],
        value: PointerValue,
    ) -> CustResult<()> {
        self.ensure_variable_mutable(name)?;
        if let Some(expected) = self.direct_struct_pointer_field_type(name, path)? {
            self.ensure_pointer_type_matches(&expected, &value)?;
        }
        match self.find_variable_mut(name) {
            Some(Value::Struct { type_name, fields }) => {
                let (_, field_value) = Self::nested_field_value_mut(type_name, fields, path)?;
                if field_value.is_const() {
                    return Err(CustError::new(format!(
                        "cannot assign to const struct field '{}'",
                        Self::field_path_label(path)
                    )));
                }
                match field_value {
                    StructFieldValue::Pointer { pointer, .. } => {
                        *pointer = value;
                        Ok(())
                    }
                    StructFieldValue::Scalar { .. } => Err(CustError::new(format!(
                        "struct field '{}' is not a pointer",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::Array { .. } => Err(CustError::new(format!(
                        "struct field '{}' is an array",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::Struct { type_name, .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct '{type_name}'",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct array",
                        Self::field_path_label(path)
                    ))),
                }
            }
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn direct_struct_pointer_field_type(
        &self,
        name: &str,
        path: &[String],
    ) -> CustResult<Option<PointeeType>> {
        match self.find_variable(name) {
            Some(Value::Struct { type_name, fields }) => {
                let (_, field_value) = Self::nested_field_value(type_name, fields, path)?;
                match field_value {
                    StructFieldValue::Pointer { ty, .. } => Ok(Some(ty.clone())),
                    _ => Ok(None),
                }
            }
            Some(_) => Ok(None),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn offset_direct_struct_pointer_field(
        &mut self,
        name: &str,
        path: &[String],
        offset: i64,
    ) -> CustResult<(PointerValue, PointerValue)> {
        let current = self.read_direct_struct_pointer_field(name, path)?;
        let updated = self.offset_array_pointer(&current, offset)?;
        self.assign_direct_struct_pointer_field(name, path, updated.clone())?;
        Ok((current, updated))
    }

    fn compound_assign_direct_struct_pointer_field(
        &mut self,
        name: &str,
        path: &[String],
        op: CompoundOp,
        value: &Expr,
    ) -> CustResult<PointerValue> {
        let offset = self.eval(value)?;
        let offset = match op {
            CompoundOp::Add => offset,
            CompoundOp::Sub => -offset,
            CompoundOp::Mul
            | CompoundOp::Div
            | CompoundOp::Rem
            | CompoundOp::BitAnd
            | CompoundOp::BitOr
            | CompoundOp::BitXor
            | CompoundOp::ShiftLeft
            | CompoundOp::ShiftRight => return Err(Self::pointer_compound_error(op)),
        };
        let (_, updated) = self.offset_direct_struct_pointer_field(name, path, offset)?;
        Ok(updated)
    }

    fn increment_direct_struct_pointer_field(
        &mut self,
        name: &str,
        path: &[String],
        op: IncrementOp,
        prefix: bool,
    ) -> CustResult<PointerValue> {
        let offset = match op {
            IncrementOp::Inc => 1,
            IncrementOp::Dec => -1,
        };
        let (current, updated) = self.offset_direct_struct_pointer_field(name, path, offset)?;
        if prefix { Ok(updated) } else { Ok(current) }
    }

    fn find_struct_array_field(
        &self,
        name: &str,
        path: &[String],
    ) -> CustResult<Rc<RefCell<ArrayValue>>> {
        match self.find_variable(name) {
            Some(Value::Struct { type_name, fields }) => {
                let (_, field_value) = Self::nested_field_value(type_name, fields, path)?;
                match field_value {
                    StructFieldValue::Array { value, .. } => Ok(Rc::clone(value)),
                    StructFieldValue::Scalar { .. } => Err(CustError::new(format!(
                        "struct field '{}' is not an array",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::Struct { type_name, .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct '{type_name}'",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct array",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                        "struct field '{}' is not an array",
                        Self::field_path_label(path)
                    ))),
                }
            }
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn checked_struct_array_index(
        &mut self,
        name: &str,
        fields: &[String],
        index: &Expr,
    ) -> CustResult<(Rc<RefCell<ArrayValue>>, usize)> {
        let index_value = self.eval(index)?;
        let array = self.find_struct_array_field(name, fields)?;
        let len = array.borrow().elements.len();
        let Ok(index) = usize::try_from(index_value) else {
            return Err(CustError::new(format!(
                "array '{}' index {index_value} out of bounds for length {len}",
                Self::field_path_label(fields)
            )));
        };
        if index >= len {
            return Err(CustError::new(format!(
                "array '{}' index {index_value} out of bounds for length {len}",
                Self::field_path_label(fields)
            )));
        }
        Ok((array, index))
    }

    fn read_struct_field_array_element_field(
        &mut self,
        name: &str,
        array_fields: &[String],
        index: &Expr,
        fields: &[String],
    ) -> CustResult<i64> {
        let index_value = self.eval(index)?;
        match self.find_variable(name) {
            Some(Value::Struct {
                type_name,
                fields: field_map,
            }) => {
                let (_, array_value) =
                    Self::nested_field_value(type_name, field_map, array_fields)?;
                let StructFieldValue::StructArray {
                    type_name: element_type,
                    elements,
                    ..
                } = array_value
                else {
                    return Err(CustError::new(format!(
                        "struct field '{}' is not a struct array",
                        Self::field_path_label(array_fields)
                    )));
                };
                let len = elements.len();
                let Ok(index) = usize::try_from(index_value) else {
                    return Err(CustError::new(format!(
                        "struct array field '{}' index {index_value} out of bounds for length {len}",
                        Self::field_path_label(array_fields)
                    )));
                };
                let element = elements.get(index).ok_or_else(|| {
                    CustError::new(format!(
                        "struct array field '{}' index {index_value} out of bounds for length {len}",
                        Self::field_path_label(array_fields)
                    ))
                })?;
                let (_, field_value) = Self::nested_field_value(element_type, element, fields)?;
                match field_value {
                    StructFieldValue::Scalar { value, .. } => Ok(*value),
                    StructFieldValue::Array { .. } => Err(CustError::new(format!(
                        "struct field '{}' is an array",
                        Self::field_path_label(fields)
                    ))),
                    StructFieldValue::Struct { type_name, .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct '{type_name}'",
                        Self::field_path_label(fields)
                    ))),
                    StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct array",
                        Self::field_path_label(fields)
                    ))),
                    StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                        "pointer field '{}' used as scalar",
                        Self::field_path_label(fields)
                    ))),
                }
            }
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn assign_struct_field_array_element_field(
        &mut self,
        name: &str,
        array_fields: &[String],
        index: &Expr,
        fields: &[String],
        value: i64,
    ) -> CustResult<()> {
        self.ensure_variable_mutable(name)?;
        let index_value = self.eval(index)?;
        let struct_types = self.struct_types.clone();
        match self.find_variable_mut(name) {
            Some(Value::Struct {
                type_name,
                fields: field_map,
            }) => {
                let (_, array_value) =
                    Self::nested_field_value_mut(type_name, field_map, array_fields)?;
                let StructFieldValue::StructArray {
                    type_name: element_type,
                    elements,
                    is_const,
                } = array_value
                else {
                    return Err(CustError::new(format!(
                        "struct field '{}' is not a struct array",
                        Self::field_path_label(array_fields)
                    )));
                };
                if *is_const {
                    return Err(CustError::new(format!(
                        "cannot assign to const struct field '{}'",
                        Self::field_path_label(array_fields)
                    )));
                }
                let len = elements.len();
                let Ok(index) = usize::try_from(index_value) else {
                    return Err(CustError::new(format!(
                        "struct array field '{}' index {index_value} out of bounds for length {len}",
                        Self::field_path_label(array_fields)
                    )));
                };
                let element = elements.get_mut(index).ok_or_else(|| {
                    CustError::new(format!(
                        "struct array field '{}' index {index_value} out of bounds for length {len}",
                        Self::field_path_label(array_fields)
                    ))
                })?;
                Self::assign_scalar_field_in_map(
                    &struct_types,
                    element_type,
                    element,
                    fields,
                    value,
                )
            }
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn find_struct_array_field_pointer(
        &mut self,
        name: &str,
        fields: &[String],
        index: &Expr,
    ) -> CustResult<PointerValue> {
        if self
            .direct_struct_aggregate_array_field_type(name, fields)?
            .is_some()
        {
            let index =
                self.checked_struct_aggregate_array_field_index(name, None, fields, index)?;
            let scope_id = self
                .find_variable_scope_id(name)
                .ok_or_else(|| CustError::new(format!("undefined variable '{name}'")))?;
            return Ok(PointerValue::StructFieldElement {
                scope_id,
                name: name.to_string(),
                element_index: None,
                fields: fields.to_vec(),
                index,
            });
        }
        let (array, index) = self.checked_struct_array_index(name, fields, index)?;
        Ok(PointerValue::ArrayElement {
            array,
            source_name: Some(Self::field_path_label(fields).to_string()),
            index,
        })
    }

    fn find_struct_array_field_base_pointer(
        &self,
        name: &str,
        fields: &[String],
    ) -> CustResult<PointerValue> {
        if self
            .direct_struct_aggregate_array_field_type(name, fields)?
            .is_some()
        {
            let scope_id = self
                .find_variable_scope_id(name)
                .ok_or_else(|| CustError::new(format!("undefined variable '{name}'")))?;
            return Ok(PointerValue::StructFieldElement {
                scope_id,
                name: name.to_string(),
                element_index: None,
                fields: fields.to_vec(),
                index: 0,
            });
        }
        let array = self.find_struct_array_field(name, fields)?;
        Ok(PointerValue::ArrayBase {
            array,
            source_name: Some(Self::field_path_label(fields).to_string()),
        })
    }

    fn find_struct_element_array_field_pointer(
        &mut self,
        name: &str,
        index: &Expr,
        fields: &[String],
        array_index: &Expr,
    ) -> CustResult<PointerValue> {
        let (array, index) =
            self.checked_struct_element_array_index(name, index, fields, array_index)?;
        Ok(PointerValue::ArrayElement {
            array,
            source_name: Some(Self::field_path_label(fields).to_string()),
            index,
        })
    }

    fn find_struct_element_array_field_base_pointer(
        &mut self,
        name: &str,
        index: &Expr,
        fields: &[String],
    ) -> CustResult<PointerValue> {
        let array = self.find_struct_element_array_field(name, index, fields)?;
        Ok(PointerValue::ArrayBase {
            array,
            source_name: Some(Self::field_path_label(fields).to_string()),
        })
    }

    fn direct_struct_aggregate_array_field_type(
        &self,
        name: &str,
        fields: &[String],
    ) -> CustResult<Option<String>> {
        let Some(scope_id) = self.find_variable_scope_id(name) else {
            return Err(CustError::new(format!("undefined variable '{name}'")));
        };
        match self.struct_field_by_scope(scope_id, name, None, fields)? {
            StructFieldValue::StructArray { type_name, .. } => Ok(Some(type_name.clone())),
            _ => Ok(None),
        }
    }

    fn checked_struct_aggregate_array_field_index(
        &mut self,
        name: &str,
        element_index: Option<usize>,
        fields: &[String],
        index: &Expr,
    ) -> CustResult<usize> {
        let scope_id = self
            .find_variable_scope_id(name)
            .ok_or_else(|| CustError::new(format!("undefined variable '{name}'")))?;
        let index_value = self.eval(index)?;
        let len = match self.struct_field_by_scope(scope_id, name, element_index, fields)? {
            StructFieldValue::StructArray { elements, .. } => elements.len(),
            _ => {
                return Err(CustError::new(format!(
                    "struct field '{}' is not a struct array",
                    Self::field_path_label(fields)
                )));
            }
        };
        let Ok(index) = usize::try_from(index_value) else {
            return Err(CustError::new(format!(
                "struct array field '{}' index {index_value} out of bounds for length {len}",
                Self::field_path_label(fields)
            )));
        };
        if index >= len {
            return Err(CustError::new(format!(
                "struct array field '{}' index {index_value} out of bounds for length {len}",
                Self::field_path_label(fields)
            )));
        }
        Ok(index)
    }

    fn struct_field_array_pointer_at(
        &self,
        scope_id: usize,
        name: &str,
        element_index: Option<usize>,
        fields: &[String],
        index: i64,
    ) -> CustResult<PointerValue> {
        let field_value = self.struct_field_by_scope(scope_id, name, element_index, fields)?;
        let len = match field_value {
            StructFieldValue::StructArray { elements, .. } => elements.len(),
            _ => {
                return Err(CustError::new(format!(
                    "struct field '{}' is not a struct array",
                    Self::field_path_label(fields)
                )));
            }
        };
        let Ok(index_usize) = usize::try_from(index) else {
            return Err(CustError::new(format!(
                "struct array field pointer index {index} out of bounds for length {len}"
            )));
        };
        if index_usize >= len {
            return Err(CustError::new(format!(
                "struct array field pointer index {index} out of bounds for length {len}"
            )));
        }
        Ok(PointerValue::StructFieldElement {
            scope_id,
            name: name.to_string(),
            element_index,
            fields: fields.to_vec(),
            index: index_usize,
        })
    }

    fn struct_field_array_element_fields(
        &self,
        scope_id: usize,
        name: &str,
        element_index: Option<usize>,
        fields: &[String],
        index: usize,
    ) -> CustResult<(String, &HashMap<String, StructFieldValue>)> {
        match self.struct_field_by_scope(scope_id, name, element_index, fields)? {
            StructFieldValue::StructArray {
                type_name,
                elements,
                ..
            } => elements
                .get(index)
                .map(|element| (type_name.clone(), element))
                .ok_or_else(|| {
                    CustError::new(format!(
                        "struct array field pointer index {index} out of bounds for length {}",
                        elements.len()
                    ))
                }),
            _ => Err(CustError::new(format!(
                "struct field '{}' is not a struct array",
                Self::field_path_label(fields)
            ))),
        }
    }

    fn struct_field_array_element_fields_mut(
        &mut self,
        scope_id: usize,
        name: &str,
        element_index: Option<usize>,
        fields: &[String],
        index: usize,
    ) -> CustResult<(String, &mut HashMap<String, StructFieldValue>)> {
        match self.struct_field_by_scope_mut(scope_id, name, element_index, fields)? {
            StructFieldValue::StructArray {
                type_name,
                elements,
                ..
            } => {
                let len = elements.len();
                elements
                    .get_mut(index)
                    .map(|element| (type_name.clone(), element))
                    .ok_or_else(|| {
                        CustError::new(format!(
                            "struct array field pointer index {index} out of bounds for length {len}"
                        ))
                    })
            }
            _ => Err(CustError::new(format!(
                "struct field '{}' is not a struct array",
                Self::field_path_label(fields)
            ))),
        }
    }

    fn find_struct_pointer_array_field(
        &self,
        pointer: &PointerValue,
        fields: &[String],
    ) -> CustResult<Rc<RefCell<ArrayValue>>> {
        let (type_name, field_map) = self.find_struct_pointer_fields(pointer)?;
        let (_, field_value) = Self::nested_field_value(&type_name, field_map, fields)?;
        match field_value {
            StructFieldValue::Array { value, .. } => Ok(Rc::clone(value)),
            StructFieldValue::Scalar { .. } | StructFieldValue::Pointer { .. } => {
                Err(CustError::new(format!(
                    "struct field '{}' is not an array",
                    Self::field_path_label(fields)
                )))
            }
            StructFieldValue::Struct { type_name, .. } => Err(CustError::new(format!(
                "struct field '{}' is a struct '{type_name}'",
                Self::field_path_label(fields)
            ))),
            StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                "struct field '{}' is a struct array",
                Self::field_path_label(fields)
            ))),
        }
    }

    fn struct_pointer_source(
        pointer: &PointerValue,
    ) -> CustResult<Option<(usize, String, Option<usize>)>> {
        match pointer {
            PointerValue::Struct { scope_id, name } => Ok(Some((*scope_id, name.clone(), None))),
            PointerValue::StructElement {
                scope_id,
                name,
                index,
            } => Ok(Some((*scope_id, name.clone(), Some(*index)))),
            PointerValue::StructFieldElement { .. } => Ok(None),
            PointerValue::Null => Err(CustError::new("null pointer dereference")),
            _ => Err(CustError::new("pointer does not reference a struct")),
        }
    }

    fn find_struct_pointer_aggregate_array_field_base_pointer(
        &self,
        pointer: &PointerValue,
        fields: &[String],
    ) -> CustResult<Option<PointerValue>> {
        let Some((scope_id, name, element_index)) = Self::struct_pointer_source(pointer)? else {
            return Ok(None);
        };
        match self.struct_field_by_scope(scope_id, &name, element_index, fields)? {
            StructFieldValue::StructArray { .. } => Ok(Some(PointerValue::StructFieldElement {
                scope_id,
                name,
                element_index,
                fields: fields.to_vec(),
                index: 0,
            })),
            _ => Ok(None),
        }
    }

    fn find_struct_pointer_aggregate_array_field_pointer(
        &mut self,
        pointer: &PointerValue,
        fields: &[String],
        index: &Expr,
    ) -> CustResult<Option<PointerValue>> {
        let Some((scope_id, name, element_index)) = Self::struct_pointer_source(pointer)? else {
            return Ok(None);
        };
        let index_value = self.eval(index)?;
        let len = match self.struct_field_by_scope(scope_id, &name, element_index, fields)? {
            StructFieldValue::StructArray { elements, .. } => elements.len(),
            _ => return Ok(None),
        };
        let Ok(index) = usize::try_from(index_value) else {
            return Err(CustError::new(format!(
                "struct array field '{}' index {index_value} out of bounds for length {len}",
                Self::field_path_label(fields)
            )));
        };
        if index >= len {
            return Err(CustError::new(format!(
                "struct array field '{}' index {index_value} out of bounds for length {len}",
                Self::field_path_label(fields)
            )));
        }
        Ok(Some(PointerValue::StructFieldElement {
            scope_id,
            name,
            element_index,
            fields: fields.to_vec(),
            index,
        }))
    }

    fn checked_struct_pointer_array_index(
        &mut self,
        pointer: &PointerValue,
        fields: &[String],
        index: &Expr,
    ) -> CustResult<(Rc<RefCell<ArrayValue>>, usize)> {
        let index_value = self.eval(index)?;
        let array = self.find_struct_pointer_array_field(pointer, fields)?;
        let len = array.borrow().elements.len();
        let Ok(index) = usize::try_from(index_value) else {
            return Err(CustError::new(format!(
                "array '{}' index {index_value} out of bounds for length {len}",
                Self::field_path_label(fields)
            )));
        };
        if index >= len {
            return Err(CustError::new(format!(
                "array '{}' index {index_value} out of bounds for length {len}",
                Self::field_path_label(fields)
            )));
        }
        Ok((array, index))
    }

    fn find_struct_pointer_array_field_pointer(
        &mut self,
        pointer: &PointerValue,
        fields: &[String],
        index: &Expr,
    ) -> CustResult<PointerValue> {
        if let Some(pointer) =
            self.find_struct_pointer_aggregate_array_field_pointer(pointer, fields, index)?
        {
            return Ok(pointer);
        }
        let (array, index) = self.checked_struct_pointer_array_index(pointer, fields, index)?;
        Ok(PointerValue::ArrayElement {
            array,
            source_name: Some(Self::field_path_label(fields).to_string()),
            index,
        })
    }

    fn find_struct_pointer_array_field_base_pointer(
        &self,
        pointer: &PointerValue,
        fields: &[String],
    ) -> CustResult<PointerValue> {
        if let Some(pointer) =
            self.find_struct_pointer_aggregate_array_field_base_pointer(pointer, fields)?
        {
            return Ok(pointer);
        }
        let array = self.find_struct_pointer_array_field(pointer, fields)?;
        Ok(PointerValue::ArrayBase {
            array,
            source_name: Some(Self::field_path_label(fields).to_string()),
        })
    }

    fn checked_struct_element_index(&mut self, name: &str, index: &Expr) -> CustResult<usize> {
        let index_value = self.eval(index)?;
        let len = match self.find_variable(name) {
            Some(Value::StructArray { elements, .. }) => elements.len(),
            Some(_) => {
                return Err(CustError::new(format!(
                    "variable '{name}' is not a struct array"
                )));
            }
            None => return Err(CustError::new(format!("undefined variable '{name}'"))),
        };
        let Ok(index) = usize::try_from(index_value) else {
            return Err(CustError::new(format!(
                "struct array '{name}' index {index_value} out of bounds for length {len}"
            )));
        };
        if index >= len {
            return Err(CustError::new(format!(
                "struct array '{name}' index {index_value} out of bounds for length {len}"
            )));
        }
        Ok(index)
    }

    fn indexed_struct_pointer(
        &mut self,
        name: &str,
        index: &Expr,
    ) -> CustResult<Option<PointerValue>> {
        let Some(Value::Pointer { pointer, ty, .. }) = self.find_variable(name).cloned() else {
            return Ok(None);
        };
        if !matches!(ty, PointeeType::Struct(_)) {
            return Ok(None);
        }
        let index_value = self.eval(index)?;
        let pointer = self.offset_array_pointer(&pointer, index_value)?;
        match pointer {
            PointerValue::StructElement { .. } | PointerValue::StructFieldElement { .. } => {
                Ok(Some(pointer))
            }
            _ => Err(CustError::new("struct pointer is not indexable")),
        }
    }

    fn indexed_struct_pointer_value(
        &mut self,
        name: &str,
        index: &Expr,
    ) -> CustResult<Option<(String, HashMap<String, StructFieldValue>)>> {
        let Some(pointer) = self.indexed_struct_pointer(name, index)? else {
            return Ok(None);
        };
        let (type_name, fields) = self.find_struct_pointer_fields(&pointer)?;
        Ok(Some((
            type_name,
            StructFieldValue::deep_clone_fields(fields),
        )))
    }

    fn find_variable_scope_id(&self, name: &str) -> Option<usize> {
        for scope in self.scopes.iter().rev() {
            if scope.values.contains_key(name) {
                return Some(scope.id);
            }
            if let Some(storage) = scope
                .static_local_ids
                .get(name)
                .and_then(|id| self.static_locals.get(id))
            {
                return Some(storage.scope_id);
            }
        }
        None
    }

    fn find_struct_element_pointer(
        &mut self,
        name: &str,
        index: &Expr,
    ) -> CustResult<PointerValue> {
        let index = self.checked_struct_element_index(name, index)?;
        let scope_id = self
            .find_variable_scope_id(name)
            .ok_or_else(|| CustError::new(format!("undefined variable '{name}'")))?;
        Ok(PointerValue::StructElement {
            scope_id,
            name: name.to_string(),
            index,
        })
    }

    fn struct_field_points_to_const(
        &self,
        scope_id: usize,
        name: &str,
        element_index: Option<usize>,
        path: &[String],
    ) -> CustResult<bool> {
        let root_is_const = self
            .scopes
            .iter()
            .find(|scope| scope.id == scope_id)
            .is_some_and(|scope| scope.const_variables.contains(name))
            || self.static_locals.values().any(|storage| {
                storage.scope_id == scope_id && storage.name == name && storage.is_const
            });
        let field_is_const =
            match self.struct_field_by_scope(scope_id, name, element_index, path)? {
                StructFieldValue::Scalar { is_const, .. }
                | StructFieldValue::Array { is_const, .. }
                | StructFieldValue::Struct { is_const, .. }
                | StructFieldValue::StructArray { is_const, .. }
                | StructFieldValue::Pointer { is_const, .. } => *is_const,
            };
        Ok(root_is_const || field_is_const)
    }

    fn find_struct_field_pointer(
        &mut self,
        name: &str,
        fields: &[String],
    ) -> CustResult<PointerValue> {
        let scope_id = self
            .find_variable_scope_id(name)
            .ok_or_else(|| CustError::new(format!("undefined variable '{name}'")))?;
        match self.struct_field_by_scope(scope_id, name, None, fields)? {
            StructFieldValue::Scalar { .. } => Ok(PointerValue::StructField {
                scope_id,
                name: name.to_string(),
                element_index: None,
                fields: fields.to_vec(),
            }),
            StructFieldValue::Array { value, .. } => Ok(PointerValue::ArrayBase {
                array: Rc::clone(value),
                source_name: Some(Self::field_path_label(fields).to_string()),
            }),
            StructFieldValue::Struct { .. } => Err(CustError::new(format!(
                "struct field '{}' requires field access",
                Self::field_path_label(fields)
            ))),
            StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                "struct field '{}' requires indexed field access",
                Self::field_path_label(fields)
            ))),
            StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                "pointer field '{}' cannot be addressed in this pointer milestone",
                Self::field_path_label(fields)
            ))),
        }
    }

    fn find_struct_element_field_pointer(
        &mut self,
        name: &str,
        index: &Expr,
        fields: &[String],
    ) -> CustResult<PointerValue> {
        let index = self.checked_struct_element_index(name, index)?;
        let scope_id = self
            .find_variable_scope_id(name)
            .ok_or_else(|| CustError::new(format!("undefined variable '{name}'")))?;
        match self.struct_field_by_scope(scope_id, name, Some(index), fields)? {
            StructFieldValue::Scalar { .. } | StructFieldValue::Struct { .. } => {
                Ok(PointerValue::StructField {
                    scope_id,
                    name: name.to_string(),
                    element_index: Some(index),
                    fields: fields.to_vec(),
                })
            }
            StructFieldValue::Array { value, .. } => Ok(PointerValue::ArrayBase {
                array: Rc::clone(value),
                source_name: Some(Self::field_path_label(fields).to_string()),
            }),
            StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                "struct field '{}' requires indexed field access",
                Self::field_path_label(fields)
            ))),
            StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                "pointer field '{}' cannot be addressed in this pointer milestone",
                Self::field_path_label(fields)
            ))),
        }
    }

    fn append_field_path(base: &[String], suffix: &[String]) -> Vec<String> {
        base.iter()
            .chain(suffix.iter())
            .cloned()
            .collect::<Vec<_>>()
    }

    fn find_struct_pointer_field_pointer(
        &mut self,
        pointer: &PointerValue,
        fields: &[String],
    ) -> CustResult<PointerValue> {
        let (scope_id, name, element_index, field_path) = match pointer {
            PointerValue::Struct { scope_id, name } => {
                (*scope_id, name.clone(), None, fields.to_vec())
            }
            PointerValue::StructElement {
                scope_id,
                name,
                index,
            } => (*scope_id, name.clone(), Some(*index), fields.to_vec()),
            PointerValue::StructField {
                scope_id,
                name,
                element_index,
                fields: base_fields,
            } => (
                *scope_id,
                name.clone(),
                *element_index,
                Self::append_field_path(base_fields, fields),
            ),
            PointerValue::StructFieldElement {
                scope_id,
                name,
                element_index,
                fields: array_fields,
                index,
            } => {
                let (type_name, element_fields) = self.struct_field_array_element_fields(
                    *scope_id,
                    name,
                    *element_index,
                    array_fields,
                    *index,
                )?;
                match Self::nested_field_value(&type_name, element_fields, fields)?.1 {
                    StructFieldValue::Scalar { .. } | StructFieldValue::Struct { .. } => {
                        return Ok(PointerValue::StructFieldElementField {
                            scope_id: *scope_id,
                            name: name.clone(),
                            element_index: *element_index,
                            array_fields: array_fields.clone(),
                            index: *index,
                            fields: fields.to_vec(),
                        });
                    }
                    StructFieldValue::Array { value, .. } => {
                        return Ok(PointerValue::ArrayBase {
                            array: Rc::clone(value),
                            source_name: Some(Self::field_path_label(fields).to_string()),
                        });
                    }
                    StructFieldValue::StructArray { .. } => {
                        return Err(CustError::new(format!(
                            "struct field '{}' requires indexed field access",
                            Self::field_path_label(fields)
                        )));
                    }
                    StructFieldValue::Pointer { .. } => {
                        return Err(CustError::new(format!(
                            "pointer field '{}' cannot be addressed in this pointer milestone",
                            Self::field_path_label(fields)
                        )));
                    }
                }
            }
            PointerValue::Null => return Err(CustError::new("null pointer dereference")),
            _ => return Err(CustError::new("pointer does not reference a struct")),
        };

        match self.struct_field_by_scope(scope_id, &name, element_index, &field_path)? {
            StructFieldValue::Scalar { .. } | StructFieldValue::Struct { .. } => {
                Ok(PointerValue::StructField {
                    scope_id,
                    name,
                    element_index,
                    fields: field_path,
                })
            }
            StructFieldValue::Array { value, .. } => Ok(PointerValue::ArrayBase {
                array: Rc::clone(value),
                source_name: Some(Self::field_path_label(&field_path).to_string()),
            }),
            StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                "struct field '{}' requires indexed field access",
                Self::field_path_label(&field_path)
            ))),
            StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                "pointer field '{}' cannot be addressed in this pointer milestone",
                Self::field_path_label(&field_path)
            ))),
        }
    }

    fn struct_field_by_scope(
        &self,
        scope_id: usize,
        name: &str,
        element_index: Option<usize>,
        path: &[String],
    ) -> CustResult<&StructFieldValue> {
        if !self.live_scope_ids.contains(&scope_id) {
            return Err(CustError::new(format!(
                "pointer to out-of-scope variable '{name}'"
            )));
        }
        let value = self
            .scopes
            .iter()
            .find(|scope| scope.id == scope_id)
            .and_then(|scope| scope.values.get(name))
            .or_else(|| self.static_value_by_scope(scope_id, name));
        match (value, element_index) {
            (Some(Value::Struct { type_name, fields }), None) => {
                Self::nested_field_value(type_name, fields, path).map(|(_, field)| field)
            }
            (
                Some(Value::StructArray {
                    type_name,
                    elements,
                    ..
                }),
                Some(index),
            ) => {
                Self::nested_field_value(type_name, &elements[index], path).map(|(_, field)| field)
            }
            _ => Err(CustError::new(format!(
                "pointer to out-of-scope variable '{name}'"
            ))),
        }
    }

    fn struct_field_by_scope_mut(
        &mut self,
        scope_id: usize,
        name: &str,
        element_index: Option<usize>,
        path: &[String],
    ) -> CustResult<&mut StructFieldValue> {
        if !self.live_scope_ids.contains(&scope_id) {
            return Err(CustError::new(format!(
                "pointer to out-of-scope variable '{name}'"
            )));
        }
        if let Some(pos) = self.scopes.iter().position(|scope| scope.id == scope_id) {
            let value = self.scopes[pos].values.get_mut(name);
            return match (value, element_index) {
                (Some(Value::Struct { type_name, fields }), None) => {
                    Self::nested_field_value_mut(type_name, fields, path).map(|(_, field)| field)
                }
                (
                    Some(Value::StructArray {
                        type_name,
                        elements,
                        ..
                    }),
                    Some(index),
                ) => Self::nested_field_value_mut(type_name, &mut elements[index], path)
                    .map(|(_, field)| field),
                _ => Err(CustError::new(format!(
                    "pointer to out-of-scope variable '{name}'"
                ))),
            };
        }
        match (
            self.static_value_by_scope_mut(scope_id, name),
            element_index,
        ) {
            (Some(Value::Struct { type_name, fields }), None) => {
                Self::nested_field_value_mut(type_name, fields, path).map(|(_, field)| field)
            }
            (
                Some(Value::StructArray {
                    type_name,
                    elements,
                    ..
                }),
                Some(index),
            ) => Self::nested_field_value_mut(type_name, &mut elements[index], path)
                .map(|(_, field)| field),
            _ => Err(CustError::new(format!(
                "pointer to out-of-scope variable '{name}'"
            ))),
        }
    }

    fn read_struct_field_pointer(
        &self,
        scope_id: usize,
        name: &str,
        element_index: Option<usize>,
        fields: &[String],
    ) -> CustResult<i64> {
        match self.struct_field_by_scope(scope_id, name, element_index, fields)? {
            StructFieldValue::Scalar { value, .. } => Ok(*value),
            _ => Err(CustError::new(format!(
                "struct field '{}' used as non-scalar pointer target",
                Self::field_path_label(fields)
            ))),
        }
    }

    fn assign_struct_field_pointer(
        &mut self,
        scope_id: usize,
        name: &str,
        element_index: Option<usize>,
        fields: &[String],
        value: i64,
    ) -> CustResult<()> {
        if self.struct_field_points_to_const(scope_id, name, element_index, fields)? {
            return Err(CustError::new("cannot assign through pointer to const"));
        }
        match self.struct_field_by_scope_mut(scope_id, name, element_index, fields)? {
            StructFieldValue::Scalar { value: slot, .. } => {
                *slot = value;
                Ok(())
            }
            _ => Err(CustError::new(format!(
                "struct field '{}' used as non-scalar pointer target",
                Self::field_path_label(fields)
            ))),
        }
    }

    fn read_struct_field_element_field_pointer(
        &self,
        scope_id: usize,
        name: &str,
        element_index: Option<usize>,
        array_fields: &[String],
        index: usize,
        fields: &[String],
    ) -> CustResult<i64> {
        let (type_name, element_fields) = self.struct_field_array_element_fields(
            scope_id,
            name,
            element_index,
            array_fields,
            index,
        )?;
        match Self::nested_field_value(&type_name, element_fields, fields)?.1 {
            StructFieldValue::Scalar { value, .. } => Ok(*value),
            _ => Err(CustError::new(format!(
                "struct field '{}' used as non-scalar pointer target",
                Self::field_path_label(fields)
            ))),
        }
    }

    fn assign_struct_field_element_field_pointer(
        &mut self,
        pointer: &PointerValue,
        value: i64,
    ) -> CustResult<()> {
        let PointerValue::StructFieldElementField {
            scope_id,
            name,
            element_index,
            array_fields,
            index,
            fields,
        } = pointer
        else {
            return Err(CustError::new(
                "pointer does not reference an embedded struct field",
            ));
        };
        if self.struct_field_points_to_const(*scope_id, name, *element_index, array_fields)? {
            return Err(CustError::new("cannot assign through pointer to const"));
        }
        let (type_name, element_fields) = self.struct_field_array_element_fields_mut(
            *scope_id,
            name,
            *element_index,
            array_fields,
            *index,
        )?;
        match Self::nested_field_value_mut(&type_name, element_fields, fields)?.1 {
            StructFieldValue::Scalar {
                value: slot,
                is_const,
                ..
            } => {
                if *is_const {
                    return Err(CustError::new("cannot assign through pointer to const"));
                }
                *slot = value;
                Ok(())
            }
            _ => Err(CustError::new(format!(
                "struct field '{}' used as non-scalar pointer target",
                Self::field_path_label(fields)
            ))),
        }
    }

    fn read_struct_element_field(
        &mut self,
        name: &str,
        index: &Expr,
        path: &[String],
    ) -> CustResult<i64> {
        if let Some(pointer) = self.indexed_struct_pointer(name, index)? {
            return self.read_struct_pointer_field(&pointer, path);
        }
        let index = self.checked_struct_element_index(name, index)?;
        match self.find_variable(name) {
            Some(Value::StructArray {
                type_name,
                elements,
                ..
            }) => {
                let (_, field_value) = Self::nested_field_value(type_name, &elements[index], path)?;
                match field_value {
                    StructFieldValue::Scalar { value, .. } => Ok(*value),
                    StructFieldValue::Array { .. } => Err(CustError::new(format!(
                        "struct field '{}' is an array",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::Struct { type_name, .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct '{type_name}'",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                        "struct field '{}' is a struct array",
                        Self::field_path_label(path)
                    ))),
                    StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                        "pointer field '{}' used as scalar",
                        Self::field_path_label(path)
                    ))),
                }
            }
            Some(_) => Err(CustError::new(format!(
                "variable '{name}' is not a struct array"
            ))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn assign_struct_element_field(
        &mut self,
        name: &str,
        index: &Expr,
        path: &[String],
        value: i64,
    ) -> CustResult<()> {
        if let Some(pointer) = self.indexed_struct_pointer(name, index)? {
            if self.pointer_variable_points_to_const(name) {
                return Err(CustError::new("cannot assign through pointer to const"));
            }
            return self.assign_struct_pointer_field(&pointer, path, value);
        }
        self.ensure_variable_mutable(name)?;
        let index = self.checked_struct_element_index(name, index)?;
        let struct_types = self.struct_types.clone();
        match self.find_variable_mut(name) {
            Some(Value::StructArray {
                type_name,
                elements,
                read_only,
            }) => {
                if *read_only {
                    return Err(CustError::new(format!(
                        "cannot assign to const variable '{name}'"
                    )));
                }
                Self::assign_scalar_field_in_map(
                    &struct_types,
                    type_name,
                    &mut elements[index],
                    path,
                    value,
                )
            }
            Some(_) => Err(CustError::new(format!(
                "variable '{name}' is not a struct array"
            ))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn find_struct_element_array_field(
        &mut self,
        name: &str,
        index: &Expr,
        path: &[String],
    ) -> CustResult<Rc<RefCell<ArrayValue>>> {
        let index = self.checked_struct_element_index(name, index)?;
        match self.find_variable(name) {
            Some(Value::StructArray {
                type_name,
                elements,
                ..
            }) => {
                let (_, field_value) = Self::nested_field_value(type_name, &elements[index], path)?;
                match field_value {
                    StructFieldValue::Array { value, .. } => Ok(Rc::clone(value)),
                    _ => Err(CustError::new(format!(
                        "struct field '{}' is not an array",
                        Self::field_path_label(path)
                    ))),
                }
            }
            Some(_) => Err(CustError::new(format!(
                "variable '{name}' is not a struct array"
            ))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn checked_struct_element_array_index(
        &mut self,
        name: &str,
        index: &Expr,
        fields: &[String],
        array_index: &Expr,
    ) -> CustResult<(Rc<RefCell<ArrayValue>>, usize)> {
        let array = self.find_struct_element_array_field(name, index, fields)?;
        let index_value = self.eval(array_index)?;
        let len = array.borrow().elements.len();
        let Ok(index) = usize::try_from(index_value) else {
            return Err(CustError::new(format!(
                "array '{}' index {index_value} out of bounds for length {len}",
                Self::field_path_label(fields)
            )));
        };
        if index >= len {
            return Err(CustError::new(format!(
                "array '{}' index {index_value} out of bounds for length {len}",
                Self::field_path_label(fields)
            )));
        }
        Ok((array, index))
    }

    fn eval_struct_array_set(
        &mut self,
        name: &str,
        fields: &[String],
        index: &Expr,
        value: &Expr,
    ) -> CustResult<i64> {
        self.ensure_variable_mutable(name)?;
        let value = self.eval(value)?;
        let (array, index) = self.checked_struct_array_index(name, fields, index)?;
        let mut array = array.borrow_mut();
        if array.read_only {
            return Err(CustError::new(format!(
                "cannot modify read-only array '{}'",
                Self::field_path_label(fields)
            )));
        }
        array.elements[index] = value;
        Ok(value)
    }

    fn eval_struct_array_compound_set(
        &mut self,
        name: &str,
        fields: &[String],
        index: &Expr,
        op: CompoundOp,
        value: &Expr,
    ) -> CustResult<i64> {
        self.ensure_variable_mutable(name)?;
        let (array, index) = self.checked_struct_array_index(name, fields, index)?;
        let current = array.borrow().elements[index];
        let rhs = self.eval(value)?;
        let result = Self::apply_compound_op(current, op, rhs)?;
        let mut array = array.borrow_mut();
        if array.read_only {
            return Err(CustError::new(format!(
                "cannot modify read-only array '{}'",
                Self::field_path_label(fields)
            )));
        }
        array.elements[index] = result;
        Ok(result)
    }

    fn static_value_by_scope(&self, scope_id: usize, name: &str) -> Option<&Value> {
        self.static_locals.values().find_map(|storage| {
            (storage.scope_id == scope_id && storage.name == name).then_some(&storage.value)
        })
    }

    fn static_value_by_scope_mut(&mut self, scope_id: usize, name: &str) -> Option<&mut Value> {
        self.static_locals.values_mut().find_map(|storage| {
            (storage.scope_id == scope_id && storage.name == name).then_some(&mut storage.value)
        })
    }

    fn find_struct_pointer_fields(
        &self,
        pointer: &PointerValue,
    ) -> CustResult<(String, &HashMap<String, StructFieldValue>)> {
        match pointer {
            PointerValue::Null => Err(CustError::new("null pointer dereference")),
            PointerValue::Struct { scope_id, name } => {
                if !self.live_scope_ids.contains(scope_id) {
                    return Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    )));
                }
                let value = self
                    .scopes
                    .iter()
                    .find(|scope| scope.id == *scope_id)
                    .and_then(|scope| scope.values.get(name))
                    .or_else(|| self.static_value_by_scope(*scope_id, name));
                match value {
                    Some(Value::Struct { type_name, fields }) => Ok((type_name.clone(), fields)),
                    _ => Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    ))),
                }
            }
            PointerValue::StructElement {
                scope_id,
                name,
                index,
            } => {
                if !self.live_scope_ids.contains(scope_id) {
                    return Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    )));
                }
                let value = self
                    .scopes
                    .iter()
                    .find(|scope| scope.id == *scope_id)
                    .and_then(|scope| scope.values.get(name))
                    .or_else(|| self.static_value_by_scope(*scope_id, name));
                match value {
                    Some(Value::StructArray {
                        type_name,
                        elements,
                        ..
                    }) => Ok((type_name.clone(), &elements[*index])),
                    _ => Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    ))),
                }
            }
            PointerValue::StructFieldElement {
                scope_id,
                name,
                element_index,
                fields,
                index,
            } => self.struct_field_array_element_fields(
                *scope_id,
                name,
                *element_index,
                fields,
                *index,
            ),
            PointerValue::StructField {
                scope_id,
                name,
                element_index,
                fields,
            } => match self.struct_field_by_scope(*scope_id, name, *element_index, fields)? {
                StructFieldValue::Struct {
                    type_name, fields, ..
                } => Ok((type_name.clone(), fields)),
                _ => Err(CustError::new("pointer does not reference a struct")),
            },
            PointerValue::StructFieldElementField {
                scope_id,
                name,
                element_index,
                array_fields,
                index,
                fields,
            } => {
                let (element_type_name, element_fields) = self.struct_field_array_element_fields(
                    *scope_id,
                    name,
                    *element_index,
                    array_fields,
                    *index,
                )?;
                Self::nested_struct_field(&element_type_name, element_fields, fields)
            }
            _ => Err(CustError::new("pointer does not reference a struct")),
        }
    }

    fn find_struct_pointer_fields_mut(
        &mut self,
        pointer: &PointerValue,
    ) -> CustResult<(String, &mut HashMap<String, StructFieldValue>)> {
        match pointer {
            PointerValue::Null => Err(CustError::new("null pointer dereference")),
            PointerValue::Struct { scope_id, name } => {
                if !self.live_scope_ids.contains(scope_id) {
                    return Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    )));
                }
                if let Some(pos) = self.scopes.iter().position(|scope| scope.id == *scope_id) {
                    return match self.scopes[pos].values.get_mut(name) {
                        Some(Value::Struct { type_name, fields }) => {
                            Ok((type_name.clone(), fields))
                        }
                        _ => Err(CustError::new(format!(
                            "pointer to out-of-scope variable '{name}'"
                        ))),
                    };
                }
                match self.static_value_by_scope_mut(*scope_id, name) {
                    Some(Value::Struct { type_name, fields }) => Ok((type_name.clone(), fields)),
                    _ => Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    ))),
                }
            }
            PointerValue::StructElement {
                scope_id,
                name,
                index,
            } => {
                if !self.live_scope_ids.contains(scope_id) {
                    return Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    )));
                }
                if let Some(pos) = self.scopes.iter().position(|scope| scope.id == *scope_id) {
                    return match self.scopes[pos].values.get_mut(name) {
                        Some(Value::StructArray {
                            type_name,
                            elements,
                            ..
                        }) => Ok((type_name.clone(), &mut elements[*index])),
                        _ => Err(CustError::new(format!(
                            "pointer to out-of-scope variable '{name}'"
                        ))),
                    };
                }
                match self.static_value_by_scope_mut(*scope_id, name) {
                    Some(Value::StructArray {
                        type_name,
                        elements,
                        ..
                    }) => Ok((type_name.clone(), &mut elements[*index])),
                    _ => Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    ))),
                }
            }
            PointerValue::StructFieldElement {
                scope_id,
                name,
                element_index,
                fields,
                index,
            } => self.struct_field_array_element_fields_mut(
                *scope_id,
                name,
                *element_index,
                fields,
                *index,
            ),
            PointerValue::StructField {
                scope_id,
                name,
                element_index,
                fields,
            } => match self.struct_field_by_scope_mut(*scope_id, name, *element_index, fields)? {
                StructFieldValue::Struct {
                    type_name, fields, ..
                } => Ok((type_name.clone(), fields)),
                _ => Err(CustError::new("pointer does not reference a struct")),
            },
            PointerValue::StructFieldElementField {
                scope_id,
                name,
                element_index,
                array_fields,
                index,
                fields,
            } => {
                let (element_type_name, element_fields) = self
                    .struct_field_array_element_fields_mut(
                        *scope_id,
                        name,
                        *element_index,
                        array_fields,
                        *index,
                    )?;
                Self::nested_struct_field_mut(&element_type_name, element_fields, fields)
            }
            _ => Err(CustError::new("pointer does not reference a struct")),
        }
    }

    fn read_struct_pointer_field(
        &self,
        pointer: &PointerValue,
        path: &[String],
    ) -> CustResult<i64> {
        let (type_name, fields) = self.find_struct_pointer_fields(pointer)?;
        let (_, field_value) = Self::nested_field_value(&type_name, fields, path)?;
        match field_value {
            StructFieldValue::Scalar { value, .. } => Ok(*value),
            StructFieldValue::Array { .. } => Err(CustError::new(format!(
                "struct field '{}' is an array",
                Self::field_path_label(path)
            ))),
            StructFieldValue::Struct { type_name, .. } => Err(CustError::new(format!(
                "struct field '{}' is a struct '{type_name}'",
                Self::field_path_label(path)
            ))),
            StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                "struct field '{}' is a struct array",
                Self::field_path_label(path)
            ))),
            StructFieldValue::Pointer { .. } => Err(CustError::new(format!(
                "pointer field '{}' used as scalar",
                Self::field_path_label(path)
            ))),
        }
    }

    fn read_struct_pointer_pointer_field(
        &self,
        pointer: &PointerValue,
        path: &[String],
    ) -> CustResult<PointerValue> {
        let (type_name, fields) = self.find_struct_pointer_fields(pointer)?;
        let (_, field_value) = Self::nested_field_value(&type_name, fields, path)?;
        match field_value {
            StructFieldValue::Pointer { pointer, .. } => Ok(pointer.clone()),
            StructFieldValue::Scalar { .. } => Err(CustError::new(format!(
                "struct field '{}' is not a pointer",
                Self::field_path_label(path)
            ))),
            StructFieldValue::Array { .. } => Err(CustError::new(format!(
                "struct field '{}' is an array",
                Self::field_path_label(path)
            ))),
            StructFieldValue::Struct { type_name, .. } => Err(CustError::new(format!(
                "struct field '{}' is a struct '{type_name}'",
                Self::field_path_label(path)
            ))),
            StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                "struct field '{}' is a struct array",
                Self::field_path_label(path)
            ))),
        }
    }

    fn struct_pointer_pointer_field_points_to_const(
        &self,
        pointer: &PointerValue,
        path: &[String],
    ) -> CustResult<bool> {
        let (type_name, fields) = self.find_struct_pointer_fields(pointer)?;
        let (_, field_value) = Self::nested_field_value(&type_name, fields, path)?;
        match field_value {
            StructFieldValue::Pointer {
                points_to_const, ..
            } => Ok(*points_to_const),
            _ => Ok(false),
        }
    }

    fn struct_pointer_pointer_field_type(
        &self,
        pointer: &PointerValue,
        path: &[String],
    ) -> CustResult<Option<PointeeType>> {
        let (type_name, fields) = self.find_struct_pointer_fields(pointer)?;
        let (_, field_value) = Self::nested_field_value(&type_name, fields, path)?;
        match field_value {
            StructFieldValue::Pointer { ty, .. } => Ok(Some(ty.clone())),
            _ => Ok(None),
        }
    }

    fn assign_struct_pointer_pointer_field(
        &mut self,
        pointer: &PointerValue,
        path: &[String],
        value: PointerValue,
    ) -> CustResult<()> {
        self.ensure_struct_pointer_target_mutable(pointer)?;
        if let Some(expected) = self.struct_pointer_pointer_field_type(pointer, path)? {
            self.ensure_pointer_type_matches(&expected, &value)?;
        }
        let (type_name, fields) = self.find_struct_pointer_fields_mut(pointer)?;
        let (_, field_value) = Self::nested_field_value_mut(&type_name, fields, path)?;
        if field_value.is_const() {
            return Err(CustError::new(format!(
                "cannot assign to const struct field '{}'",
                Self::field_path_label(path)
            )));
        }
        match field_value {
            StructFieldValue::Pointer { pointer, .. } => {
                *pointer = value;
                Ok(())
            }
            StructFieldValue::Scalar { .. } => Err(CustError::new(format!(
                "struct field '{}' is not a pointer",
                Self::field_path_label(path)
            ))),
            StructFieldValue::Array { .. } => Err(CustError::new(format!(
                "struct field '{}' is an array",
                Self::field_path_label(path)
            ))),
            StructFieldValue::Struct { type_name, .. } => Err(CustError::new(format!(
                "struct field '{}' is a struct '{type_name}'",
                Self::field_path_label(path)
            ))),
            StructFieldValue::StructArray { .. } => Err(CustError::new(format!(
                "struct field '{}' is a struct array",
                Self::field_path_label(path)
            ))),
        }
    }

    fn offset_struct_pointer_pointer_field(
        &mut self,
        pointer: &PointerValue,
        path: &[String],
        offset: i64,
    ) -> CustResult<(PointerValue, PointerValue)> {
        let current = self.read_struct_pointer_pointer_field(pointer, path)?;
        let updated = self.offset_array_pointer(&current, offset)?;
        self.assign_struct_pointer_pointer_field(pointer, path, updated.clone())?;
        Ok((current, updated))
    }

    fn assign_struct_pointer_field(
        &mut self,
        pointer: &PointerValue,
        path: &[String],
        value: i64,
    ) -> CustResult<()> {
        self.ensure_struct_pointer_target_mutable(pointer)?;
        let struct_types = self.struct_types.clone();
        let (type_name, fields) = self.find_struct_pointer_fields_mut(pointer)?;
        Self::assign_scalar_field_in_map(&struct_types, &type_name, fields, path, value)
    }

    fn eval_struct_ptr_set(
        &mut self,
        pointer: &Expr,
        path: &[String],
        value: &Expr,
    ) -> CustResult<i64> {
        self.ensure_pointer_expr_pointee_mutable(pointer)?;
        let pointer = self.eval_pointer(pointer)?;
        let value = self.eval(value)?;
        self.assign_struct_pointer_field(&pointer, path, value)?;
        Ok(value)
    }

    fn eval_struct_ptr_compound_set(
        &mut self,
        pointer: &Expr,
        path: &[String],
        op: CompoundOp,
        value: &Expr,
    ) -> CustResult<i64> {
        self.ensure_pointer_expr_pointee_mutable(pointer)?;
        let pointer = self.eval_pointer(pointer)?;
        let current = self.read_struct_pointer_field(&pointer, path)?;
        let rhs = self.eval(value)?;
        let result = Self::apply_compound_op(current, op, rhs)?;
        self.assign_struct_pointer_field(&pointer, path, result)?;
        Ok(result)
    }

    fn assign_struct_copy(&mut self, name: &str, rhs: &Expr) -> CustResult<()> {
        self.eval_struct_assignment_expr(name, rhs).map(|_| ())
    }

    fn eval_struct_assignment_expr(&mut self, name: &str, rhs: &Expr) -> CustResult<ReturnValue> {
        self.ensure_variable_mutable(name)?;
        let (rhs_type, rhs_fields) = match self.eval_struct_expr(rhs)? {
            ReturnValue::Struct { type_name, fields } => (type_name, fields),
            ReturnValue::Scalar(_) | ReturnValue::Pointer { .. } => {
                return Err(CustError::new("struct assignment requires struct value"));
            }
        };

        let struct_types = self.struct_types.clone();
        match self.find_variable_mut(name) {
            Some(Value::Struct { type_name, fields }) if *type_name == rhs_type => {
                if fields.values().any(StructFieldValue::is_const) {
                    return Err(CustError::new(format!(
                        "cannot assign to struct '{type_name}' with const fields"
                    )));
                }
                *fields = StructFieldValue::deep_clone_fields(&rhs_fields);
                Ok(ReturnValue::Struct {
                    type_name: rhs_type,
                    fields: rhs_fields,
                })
            }
            Some(Value::Struct { type_name, .. }) => Err(CustError::new(format!(
                "cannot assign struct '{}' to struct '{}'",
                Self::aggregate_label_from(&struct_types, &rhs_type),
                Self::aggregate_label_from(&struct_types, type_name)
            ))),
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn assign_struct_pointer_copy(&mut self, pointer: &Expr, rhs: &Expr) -> CustResult<()> {
        self.eval_struct_pointer_assignment_expr(pointer, rhs)
            .map(|_| ())
    }

    fn eval_struct_pointer_assignment_expr(
        &mut self,
        pointer: &Expr,
        rhs: &Expr,
    ) -> CustResult<ReturnValue> {
        self.ensure_pointer_expr_pointee_mutable(pointer)?;
        let pointer = self.eval_pointer(pointer)?;
        let (rhs_type, rhs_fields) = match self.eval_struct_expr(rhs)? {
            ReturnValue::Struct { type_name, fields } => (type_name, fields),
            ReturnValue::Scalar(_) | ReturnValue::Pointer { .. } => {
                return Err(CustError::new("struct assignment requires struct value"));
            }
        };

        self.ensure_struct_pointer_target_mutable(&pointer)?;
        let struct_types = self.struct_types.clone();
        let (target_type, target_fields) = self.find_struct_pointer_fields_mut(&pointer)?;
        if target_type != rhs_type {
            return Err(CustError::new(format!(
                "cannot assign struct '{}' to struct '{}'",
                Self::aggregate_label_from(&struct_types, &rhs_type),
                Self::aggregate_label_from(&struct_types, &target_type)
            )));
        }
        if target_fields.values().any(StructFieldValue::is_const) {
            return Err(CustError::new(format!(
                "cannot assign to struct '{target_type}' with const fields"
            )));
        }
        *target_fields = StructFieldValue::deep_clone_fields(&rhs_fields);
        Ok(ReturnValue::Struct {
            type_name: rhs_type,
            fields: rhs_fields,
        })
    }

    fn eval_struct_set(&mut self, name: &str, fields: &[String], value: &Expr) -> CustResult<i64> {
        if self.struct_field_is_pointer(name, fields) {
            return Err(CustError::new("pointer value used as scalar"));
        }
        let value = self.eval(value)?;
        self.assign_struct_field(name, fields, value)?;
        Ok(value)
    }

    fn eval_struct_compound_set(
        &mut self,
        name: &str,
        fields: &[String],
        op: CompoundOp,
        value: &Expr,
    ) -> CustResult<i64> {
        let current = self.read_struct_field(name, fields)?;
        let rhs = self.eval(value)?;
        let result = Self::apply_compound_op(current, op, rhs)?;
        self.assign_struct_field(name, fields, result)?;
        Ok(result)
    }

    fn address_of_scalar(&self, name: &str) -> CustResult<PointerValue> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.values.get(name) {
                return match value {
                    Value::Scalar { .. } => Ok(PointerValue::Scalar {
                        scope_id: scope.id,
                        name: name.to_string(),
                    }),
                    Value::Array(_) => Err(CustError::new(format!(
                        "array '{name}' requires indexed address-of"
                    ))),
                    Value::Pointer { .. } => Err(CustError::new(format!(
                        "pointer '{name}' cannot be addressed in this pointer milestone"
                    ))),
                    Value::Struct { .. } => Ok(PointerValue::Struct {
                        scope_id: scope.id,
                        name: name.to_string(),
                    }),
                    Value::StructArray { .. } => Err(CustError::new(format!(
                        "struct array '{name}' requires indexed field access"
                    ))),
                };
            }
            if let Some(storage) = scope
                .static_local_ids
                .get(name)
                .and_then(|id| self.static_locals.get(id))
            {
                return match &storage.value {
                    Value::Scalar { .. } => Ok(PointerValue::Scalar {
                        scope_id: storage.scope_id,
                        name: name.to_string(),
                    }),
                    Value::Array(_) => Err(CustError::new(format!(
                        "array '{name}' requires indexed address-of"
                    ))),
                    Value::Pointer { .. } => Err(CustError::new(format!(
                        "pointer '{name}' cannot be addressed in this pointer milestone"
                    ))),
                    Value::Struct { .. } => Ok(PointerValue::Struct {
                        scope_id: storage.scope_id,
                        name: name.to_string(),
                    }),
                    Value::StructArray { .. } => Err(CustError::new(format!(
                        "struct array '{name}' requires indexed field access"
                    ))),
                };
            }
        }
        Err(CustError::new(format!("undefined variable '{name}'")))
    }

    fn eval_pointer(&mut self, expr: &Expr) -> CustResult<PointerValue> {
        match expr {
            Expr::Number(0) => Ok(PointerValue::Null),
            Expr::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                if self.eval_truthy(cond)? {
                    self.eval_pointer(then_expr)
                } else {
                    self.eval_pointer(else_expr)
                }
            }
            Expr::Comma(left, right) => {
                self.eval_discard(left)?;
                self.eval_pointer(right)
            }
            Expr::AddressOf(name) => self.address_of_scalar(name),
            Expr::AddressOfArray { name, index } => {
                if let Some(Value::StructArray { .. }) = self.find_variable(name) {
                    self.find_struct_element_pointer(name, index)
                } else if let Some(Value::Pointer { pointer, .. }) =
                    self.find_variable(name).cloned()
                {
                    let index_value = self.eval(index)?;
                    let (array, source_name, index) =
                        self.checked_pointer_value_index(&pointer, index_value)?;
                    Ok(PointerValue::ArrayElement {
                        array,
                        source_name,
                        index,
                    })
                } else {
                    let (array, index) = self.checked_array_index(name, index)?;
                    Ok(PointerValue::ArrayElement {
                        array,
                        source_name: Some(name.clone()),
                        index,
                    })
                }
            }
            Expr::AddressOfStructField { name, fields } => {
                self.find_struct_field_pointer(name, fields)
            }
            Expr::AddressOfStructElementField {
                name,
                index,
                fields,
            } => self.find_struct_element_field_pointer(name, index, fields),
            Expr::AddressOfStructArrayField {
                name,
                fields,
                index,
            } => self.find_struct_array_field_pointer(name, fields, index),
            Expr::AddressOfStructElementArrayField {
                name,
                index,
                fields,
                array_index,
            } => self.find_struct_element_array_field_pointer(name, index, fields, array_index),
            Expr::AddressOfStructPtrArrayField {
                pointer,
                fields,
                index,
            } => {
                let pointer = self.eval_pointer(pointer)?;
                self.find_struct_pointer_array_field_pointer(&pointer, fields, index)
            }
            Expr::AddressOfStructPtrField { pointer, fields } => {
                let pointer = self.eval_pointer(pointer)?;
                self.find_struct_pointer_field_pointer(&pointer, fields)
            }
            Expr::AddressOfScalarLiteral { ty, init } => {
                self.make_scalar_compound_literal_pointer(*ty, init)
            }
            Expr::AddressOfAggregateLiteral { type_name, init } => {
                self.make_aggregate_compound_literal_pointer(type_name, init)
            }
            Expr::AddressOfAggregateField { aggregate, fields } => {
                self.make_aggregate_compound_literal_field_pointer(aggregate, fields)
            }
            Expr::StringLiteral(values) => Ok(PointerValue::ArrayBase {
                array: Rc::new(RefCell::new(ArrayValue::read_only(values.clone()))),
                source_name: None,
            }),
            Expr::ArrayLiteral {
                elem_type,
                len,
                init,
            } => Ok(PointerValue::ArrayBase {
                array: self.make_array_compound_literal(*len, *elem_type, init)?,
                source_name: None,
            }),
            Expr::AggregateArrayLiteral {
                type_name,
                len,
                init,
            } => self.make_aggregate_array_compound_literal(type_name, *len, init),
            Expr::AggregateFieldGet { aggregate, fields } => {
                self.eval_aggregate_literal_field_pointer(aggregate, fields)
            }
            Expr::AggregateFieldSet {
                aggregate,
                fields,
                value,
            } => {
                let (_, expected_ty, is_const, points_to_const) =
                    self.eval_aggregate_literal_pointer_field_metadata(aggregate, fields)?;
                if is_const {
                    return Err(CustError::new(format!(
                        "cannot assign to const struct field '{}'",
                        Self::field_path_label(fields)
                    )));
                }
                self.ensure_pointer_conversion_preserves_const(points_to_const, value)?;
                let pointer = self.eval_pointer(value)?;
                self.ensure_pointer_type_matches(&expected_ty, &pointer)?;
                Ok(pointer)
            }
            Expr::AggregateFieldCompoundSet {
                aggregate,
                fields,
                op,
                value,
            } => {
                let (current, _, is_const, _) =
                    self.eval_aggregate_literal_pointer_field_metadata(aggregate, fields)?;
                if is_const {
                    return Err(CustError::new(format!(
                        "cannot assign to const struct field '{}'",
                        Self::field_path_label(fields)
                    )));
                }
                let offset = self.eval(value)?;
                let offset = match op {
                    CompoundOp::Add => offset,
                    CompoundOp::Sub => -offset,
                    CompoundOp::Mul
                    | CompoundOp::Div
                    | CompoundOp::Rem
                    | CompoundOp::BitAnd
                    | CompoundOp::BitOr
                    | CompoundOp::BitXor
                    | CompoundOp::ShiftLeft
                    | CompoundOp::ShiftRight => return Err(Self::pointer_compound_error(*op)),
                };
                self.offset_array_pointer(&current, offset)
            }
            Expr::Call { name, args } => match self.call_function(name, args)? {
                Some(ReturnValue::Pointer { pointer, .. }) => Ok(pointer),
                Some(ReturnValue::Scalar(_)) => Err(CustError::new(format!(
                    "scalar function '{name}' used as pointer expression"
                ))),
                Some(ReturnValue::Struct { .. }) => Err(CustError::new(format!(
                    "struct function '{name}' used as pointer expression"
                ))),
                None => Err(CustError::new(format!(
                    "void function '{name}' used as pointer expression"
                ))),
            },
            Expr::Assign { name, value } => match self.find_variable(name).cloned() {
                Some(Value::Pointer {
                    ty,
                    points_to_const,
                    ..
                }) => {
                    self.ensure_variable_mutable(name)?;
                    self.ensure_pointer_conversion_preserves_const(points_to_const, value)?;
                    let pointer = self.eval_pointer(value)?;
                    self.ensure_pointer_type_matches(&ty, &pointer)?;
                    if let Some(Value::Pointer { pointer: slot, .. }) = self.find_variable_mut(name)
                    {
                        *slot = pointer.clone();
                    }
                    Ok(pointer)
                }
                Some(Value::Scalar { .. }) => Err(CustError::new(format!(
                    "variable '{name}' is not a pointer"
                ))),
                Some(Value::Array(_)) => Err(CustError::new(format!(
                    "array '{name}' requires indexed address-of"
                ))),
                Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => Err(
                    CustError::new(format!("struct variable '{name}' used as pointer")),
                ),
                None => Err(CustError::new(format!(
                    "assignment to undeclared variable '{name}'"
                ))),
            },
            Expr::Var(name) => match self.find_variable(name) {
                Some(Value::Pointer { pointer, .. }) => Ok(pointer.clone()),
                Some(Value::Array(array)) => Ok(PointerValue::ArrayBase {
                    array: Rc::clone(array),
                    source_name: Some(name.clone()),
                }),
                Some(Value::StructArray { .. }) => {
                    let scope_id = self
                        .find_variable_scope_id(name)
                        .ok_or_else(|| CustError::new(format!("undefined variable '{name}'")))?;
                    Ok(PointerValue::StructElement {
                        scope_id,
                        name: name.clone(),
                        index: 0,
                    })
                }
                Some(Value::Scalar { .. }) => Err(CustError::new(format!(
                    "variable '{name}' is not a pointer"
                ))),
                Some(Value::Struct { .. }) => Err(CustError::new(format!(
                    "struct variable '{name}' used as pointer"
                ))),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::StructGet { name, fields } => {
                match self.find_struct_array_field_base_pointer(name, fields) {
                    Ok(pointer) => Ok(pointer),
                    Err(_) => self.read_direct_struct_pointer_field(name, fields),
                }
            }
            Expr::StructElementGet {
                name,
                index,
                fields,
            } => match self.find_struct_element_array_field_base_pointer(name, index, fields) {
                Ok(pointer) => Ok(pointer),
                Err(_) => Err(CustError::new(format!(
                    "struct field '{}' is not a pointer",
                    Self::field_path_label(fields)
                ))),
            },
            Expr::StructPtrGet { pointer, fields } => {
                let pointer = self.eval_pointer(pointer)?;
                match self.find_struct_pointer_array_field_base_pointer(&pointer, fields) {
                    Ok(pointer) => Ok(pointer),
                    Err(_) => self.read_struct_pointer_pointer_field(&pointer, fields),
                }
            }
            Expr::StructPtrArrayGet {
                pointer,
                fields,
                index,
            } => {
                let pointer = self.eval_pointer(pointer)?;
                self.find_struct_pointer_array_field_pointer(&pointer, fields, index)
            }
            Expr::StructSet {
                name,
                fields,
                value,
            } => {
                self.ensure_pointer_conversion_preserves_const(
                    self.struct_pointer_field_points_to_const(name, fields),
                    value,
                )?;
                let pointer = self.eval_pointer(value)?;
                self.assign_direct_struct_pointer_field(name, fields, pointer.clone())?;
                Ok(pointer)
            }
            Expr::StructPtrSet {
                pointer,
                fields,
                value,
            } => {
                let target_pointer = self.eval_pointer(pointer)?;
                let target_points_to_const =
                    self.struct_pointer_pointer_field_points_to_const(&target_pointer, fields)?;
                self.ensure_pointer_conversion_preserves_const(target_points_to_const, value)?;
                let value_pointer = self.eval_pointer(value)?;
                self.assign_struct_pointer_pointer_field(
                    &target_pointer,
                    fields,
                    value_pointer.clone(),
                )?;
                Ok(value_pointer)
            }
            Expr::StructCompoundSet {
                name,
                fields,
                op,
                value,
            } => self.compound_assign_direct_struct_pointer_field(name, fields, *op, value),
            Expr::StructPtrCompoundSet {
                pointer,
                fields,
                op,
                value,
            } => {
                let offset = self.eval(value)?;
                let offset = match op {
                    CompoundOp::Add => offset,
                    CompoundOp::Sub => -offset,
                    CompoundOp::Mul
                    | CompoundOp::Div
                    | CompoundOp::Rem
                    | CompoundOp::BitAnd
                    | CompoundOp::BitOr
                    | CompoundOp::BitXor
                    | CompoundOp::ShiftLeft
                    | CompoundOp::ShiftRight => return Err(Self::pointer_compound_error(*op)),
                };
                let target_pointer = self.eval_pointer(pointer)?;
                let (_, updated) =
                    self.offset_struct_pointer_pointer_field(&target_pointer, fields, offset)?;
                Ok(updated)
            }
            Expr::CompoundAssign { name, op, value } => {
                let pointer = match self.find_variable(name).cloned() {
                    Some(Value::Pointer { pointer, .. }) => pointer,
                    Some(Value::Scalar { .. }) => {
                        return Err(CustError::new(format!(
                            "variable '{name}' is not a pointer"
                        )));
                    }
                    Some(Value::Array(_)) => {
                        return Err(CustError::new(format!("array '{name}' is not a pointer")));
                    }
                    Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => {
                        return Err(CustError::new(format!(
                            "struct variable '{name}' used as pointer"
                        )));
                    }
                    None => return Err(CustError::new(format!("undefined variable '{name}'"))),
                };
                let offset = self.eval(value)?;
                self.ensure_variable_mutable(name)?;
                let pointer = match op {
                    CompoundOp::Add => self.offset_array_pointer(&pointer, offset)?,
                    CompoundOp::Sub => self.offset_array_pointer(&pointer, -offset)?,
                    CompoundOp::Mul
                    | CompoundOp::Div
                    | CompoundOp::Rem
                    | CompoundOp::BitAnd
                    | CompoundOp::BitOr
                    | CompoundOp::BitXor
                    | CompoundOp::ShiftLeft
                    | CompoundOp::ShiftRight => return Err(Self::pointer_compound_error(*op)),
                };
                if let Some(Value::Pointer { pointer: slot, .. }) = self.find_variable_mut(name) {
                    *slot = pointer.clone();
                }
                Ok(pointer)
            }
            Expr::Increment { target, op, prefix } => match target.as_ref() {
                Expr::StructGet { name, fields } => {
                    self.increment_direct_struct_pointer_field(name, fields, *op, *prefix)
                }
                Expr::StructPtrGet { pointer, fields } => {
                    let target_pointer = self.eval_pointer(pointer)?;
                    let offset = match op {
                        IncrementOp::Inc => 1,
                        IncrementOp::Dec => -1,
                    };
                    let (current, updated) =
                        self.offset_struct_pointer_pointer_field(&target_pointer, fields, offset)?;
                    if *prefix { Ok(updated) } else { Ok(current) }
                }
                Expr::AggregateFieldGet { aggregate, fields } => {
                    let (current, _, is_const, _) =
                        self.eval_aggregate_literal_pointer_field_metadata(aggregate, fields)?;
                    if is_const {
                        return Err(CustError::new(format!(
                            "cannot assign to const struct field '{}'",
                            Self::field_path_label(fields)
                        )));
                    }
                    let offset = match op {
                        IncrementOp::Inc => 1,
                        IncrementOp::Dec => -1,
                    };
                    let updated = self.offset_array_pointer(&current, offset)?;
                    if *prefix { Ok(updated) } else { Ok(current) }
                }
                Expr::Var(name) => {
                    let pointer = match self.find_variable(name).cloned() {
                        Some(Value::Pointer { pointer, .. }) => pointer,
                        Some(Value::Scalar { .. }) => {
                            return Err(CustError::new(format!(
                                "variable '{name}' is not a pointer"
                            )));
                        }
                        Some(Value::Array(_)) => {
                            return Err(CustError::new(format!("array '{name}' is not a pointer")));
                        }
                        Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => {
                            return Err(CustError::new(format!(
                                "struct variable '{name}' used as pointer"
                            )));
                        }
                        None => return Err(CustError::new(format!("undefined variable '{name}'"))),
                    };
                    self.ensure_variable_mutable(name)?;
                    let offset = match op {
                        IncrementOp::Inc => 1,
                        IncrementOp::Dec => -1,
                    };
                    let updated = self.offset_array_pointer(&pointer, offset)?;
                    if let Some(Value::Pointer { pointer: slot, .. }) = self.find_variable_mut(name)
                    {
                        *slot = updated.clone();
                    }
                    if *prefix { Ok(updated) } else { Ok(pointer) }
                }
                _ => Err(CustError::new("invalid increment/decrement target")),
            },
            Expr::Binary(left, op @ (BinaryOp::Add | BinaryOp::Sub), right) => {
                self.eval_pointer_arithmetic(left, *op, right)
            }
            _ => Err(CustError::new("expected pointer expression")),
        }
    }

    fn eval_pointer_arithmetic(
        &mut self,
        left: &Expr,
        op: BinaryOp,
        right: &Expr,
    ) -> CustResult<PointerValue> {
        if let Expr::Number(offset) = right {
            let pointer = self.eval_pointer(left)?;
            return match op {
                BinaryOp::Add => self.offset_array_pointer(&pointer, *offset),
                BinaryOp::Sub => self.offset_array_pointer(&pointer, -*offset),
                _ => unreachable!("only pointer add/sub reach pointer arithmetic"),
            };
        }
        if let (BinaryOp::Add, Expr::Number(offset)) = (op, left) {
            let pointer = self.eval_pointer(right)?;
            return self.offset_array_pointer(&pointer, *offset);
        }
        match (self.eval_pointer(left), self.eval_pointer(right)) {
            (Ok(pointer), Err(_)) => {
                let offset = self.eval(right)?;
                match op {
                    BinaryOp::Add => self.offset_array_pointer(&pointer, offset),
                    BinaryOp::Sub => self.offset_array_pointer(&pointer, -offset),
                    _ => unreachable!("only pointer add/sub reach pointer arithmetic"),
                }
            }
            (Err(_), Ok(pointer)) if op == BinaryOp::Add => {
                let offset = self.eval(left)?;
                self.offset_array_pointer(&pointer, offset)
            }
            (Ok(_), Ok(_)) if op == BinaryOp::Add => Err(CustError::new("cannot add two pointers")),
            (Ok(_), Ok(_)) | (Err(_), Ok(_)) | (Err(_), Err(_)) => {
                Err(CustError::new("expected pointer expression"))
            }
        }
    }

    fn offset_array_pointer(
        &self,
        pointer: &PointerValue,
        offset: i64,
    ) -> CustResult<PointerValue> {
        match pointer {
            PointerValue::Null => Err(CustError::new("null pointer arithmetic is not supported")),
            PointerValue::Scalar { .. }
            | PointerValue::Struct { .. }
            | PointerValue::StructField { .. }
            | PointerValue::StructFieldElementField { .. } => {
                Err(CustError::new("scalar pointer arithmetic is not supported"))
            }
            PointerValue::StructElement {
                scope_id,
                name,
                index,
            } => self.struct_array_pointer_at(*scope_id, name, *index as i64 + offset),
            PointerValue::StructFieldElement {
                scope_id,
                name,
                element_index,
                fields,
                index,
            } => self.struct_field_array_pointer_at(
                *scope_id,
                name,
                *element_index,
                fields,
                *index as i64 + offset,
            ),
            PointerValue::ArrayBase { array, source_name } => {
                self.array_pointer_at(array, source_name.clone(), offset)
            }
            PointerValue::ArrayElement {
                array,
                source_name,
                index,
            } => self.array_pointer_at(array, source_name.clone(), *index as i64 + offset),
        }
    }

    fn struct_array_pointer_at(
        &self,
        scope_id: usize,
        name: &str,
        index: i64,
    ) -> CustResult<PointerValue> {
        if !self.live_scope_ids.contains(&scope_id) {
            return Err(CustError::new(format!(
                "pointer to out-of-scope variable '{name}'"
            )));
        }
        let value = self
            .scopes
            .iter()
            .find(|scope| scope.id == scope_id)
            .and_then(|scope| scope.values.get(name))
            .or_else(|| self.static_value_by_scope(scope_id, name));
        let len = match value {
            Some(Value::StructArray { elements, .. }) => elements.len(),
            _ => {
                return Err(CustError::new(format!(
                    "pointer to out-of-scope variable '{name}'"
                )));
            }
        };
        let Ok(index_usize) = usize::try_from(index) else {
            return Err(CustError::new(format!(
                "struct array pointer index {index} out of bounds for length {len}"
            )));
        };
        if index_usize >= len {
            return Err(CustError::new(format!(
                "struct array pointer index {index} out of bounds for length {len}"
            )));
        }
        Ok(PointerValue::StructElement {
            scope_id,
            name: name.to_string(),
            index: index_usize,
        })
    }

    fn array_pointer_at(
        &self,
        array: &Rc<RefCell<ArrayValue>>,
        source_name: Option<String>,
        index: i64,
    ) -> CustResult<PointerValue> {
        let len = array.borrow().elements.len();
        let Ok(index_usize) = usize::try_from(index) else {
            return Err(CustError::new(format!(
                "array pointer index {index} out of bounds for length {len}"
            )));
        };
        if index_usize >= len {
            return Err(CustError::new(format!(
                "array pointer index {index} out of bounds for length {len}"
            )));
        }
        Ok(PointerValue::ArrayElement {
            array: Rc::clone(array),
            source_name,
            index: index_usize,
        })
    }

    fn deref_pointer(&self, pointer: &PointerValue) -> CustResult<i64> {
        match pointer {
            PointerValue::Null => Err(CustError::new("null pointer dereference")),
            PointerValue::Scalar { scope_id, name } => {
                if !self.live_scope_ids.contains(scope_id) {
                    return Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    )));
                }
                let value = self
                    .scopes
                    .iter()
                    .find(|scope| scope.id == *scope_id)
                    .and_then(|scope| scope.values.get(name))
                    .or_else(|| self.static_value_by_scope(*scope_id, name));
                match value {
                    Some(Value::Scalar { value, .. }) => Ok(*value),
                    _ => Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    ))),
                }
            }
            PointerValue::Struct { .. }
            | PointerValue::StructElement { .. }
            | PointerValue::StructFieldElement { .. } => {
                Err(CustError::new("struct pointer used as scalar"))
            }
            PointerValue::StructFieldElementField {
                scope_id,
                name,
                element_index,
                array_fields,
                index,
                fields,
            } => self.read_struct_field_element_field_pointer(
                *scope_id,
                name,
                *element_index,
                array_fields,
                *index,
                fields,
            ),
            PointerValue::StructField {
                scope_id,
                name,
                element_index,
                fields,
            } => self.read_struct_field_pointer(*scope_id, name, *element_index, fields),
            PointerValue::ArrayBase { array, .. } => {
                let array = array.borrow();
                array.elements.first().copied().ok_or_else(|| {
                    CustError::new("array pointer index 0 out of bounds for length 0")
                })
            }
            PointerValue::ArrayElement { array, index, .. } => Ok(array.borrow().elements[*index]),
        }
    }

    fn assign_deref_pointer(&mut self, pointer: &PointerValue, value: i64) -> CustResult<()> {
        match pointer {
            PointerValue::Null => Err(CustError::new("null pointer dereference")),
            PointerValue::Scalar { scope_id, name } => {
                if !self.live_scope_ids.contains(scope_id) {
                    return Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    )));
                }
                if let Some(scope) = self.scopes.iter().find(|scope| scope.id == *scope_id) {
                    if scope.const_variables.contains(name) {
                        return Err(CustError::new(format!(
                            "cannot assign to const variable '{name}'"
                        )));
                    }
                } else if self.static_locals.values().any(|storage| {
                    storage.scope_id == *scope_id && storage.name == *name && storage.is_const
                }) {
                    return Err(CustError::new(format!(
                        "cannot assign to const variable '{name}'"
                    )));
                }
                match self
                    .scopes
                    .iter_mut()
                    .find(|scope| scope.id == *scope_id)
                    .and_then(|scope| scope.values.get_mut(name))
                {
                    Some(Value::Scalar { value: slot, .. }) => {
                        *slot = value;
                        Ok(())
                    }
                    Some(_) => Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    ))),
                    None => match self.static_value_by_scope_mut(*scope_id, name) {
                        Some(Value::Scalar { value: slot, .. }) => {
                            *slot = value;
                            Ok(())
                        }
                        _ => Err(CustError::new(format!(
                            "pointer to out-of-scope variable '{name}'"
                        ))),
                    },
                }
            }
            PointerValue::ArrayBase { .. } | PointerValue::ArrayElement { .. } => {
                self.assign_pointer_index(pointer, 0, value)
            }
            PointerValue::Struct { .. }
            | PointerValue::StructElement { .. }
            | PointerValue::StructFieldElement { .. } => {
                Err(CustError::new("struct pointer used as scalar"))
            }
            PointerValue::StructFieldElementField { .. } => {
                self.assign_struct_field_element_field_pointer(pointer, value)
            }
            PointerValue::StructField {
                scope_id,
                name,
                element_index,
                fields,
            } => self.assign_struct_field_pointer(*scope_id, name, *element_index, fields, value),
        }
    }

    fn checked_pointer_index(
        &mut self,
        name: &str,
        index: &Expr,
    ) -> CustResult<(Rc<RefCell<ArrayValue>>, Option<String>, usize)> {
        let index_value = self.eval(index)?;
        let pointer = match self.find_variable(name) {
            Some(Value::Pointer { pointer, .. }) => pointer.clone(),
            Some(Value::Scalar { .. }) => {
                return Err(CustError::new(format!(
                    "variable '{name}' is not a pointer"
                )));
            }
            Some(Value::Array(_)) => {
                return Err(CustError::new(format!("array '{name}' is not a pointer")));
            }
            Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => {
                return Err(CustError::new(format!(
                    "struct variable '{name}' used as pointer"
                )));
            }
            None => return Err(CustError::new(format!("undefined variable '{name}'"))),
        };
        self.checked_pointer_value_index(&pointer, index_value)
    }

    fn checked_pointer_value_index(
        &self,
        pointer: &PointerValue,
        index_value: i64,
    ) -> CustResult<(Rc<RefCell<ArrayValue>>, Option<String>, usize)> {
        match pointer {
            PointerValue::Null => Err(CustError::new("null pointer dereference")),
            PointerValue::Scalar { .. }
            | PointerValue::StructField { .. }
            | PointerValue::StructFieldElementField { .. } => {
                Err(CustError::new("scalar pointer is not indexable"))
            }
            PointerValue::Struct { .. }
            | PointerValue::StructElement { .. }
            | PointerValue::StructFieldElement { .. } => {
                Err(CustError::new("struct pointer is not indexable"))
            }
            PointerValue::ArrayBase { array, source_name } => {
                let len = array.borrow().elements.len();
                let Ok(index) = usize::try_from(index_value) else {
                    return Err(CustError::new(format!(
                        "array pointer index {index_value} out of bounds for length {len}"
                    )));
                };
                if index >= len {
                    return Err(CustError::new(format!(
                        "array pointer index {index_value} out of bounds for length {len}"
                    )));
                }
                Ok((Rc::clone(array), source_name.clone(), index))
            }
            PointerValue::ArrayElement {
                array,
                source_name,
                index: base_index,
            } => {
                let len = array.borrow().elements.len();
                let candidate = *base_index as i64 + index_value;
                let Ok(index) = usize::try_from(candidate) else {
                    return Err(CustError::new(format!(
                        "array pointer index {candidate} out of bounds for length {len}"
                    )));
                };
                if index >= len {
                    return Err(CustError::new(format!(
                        "array pointer index {candidate} out of bounds for length {len}"
                    )));
                }
                Ok((Rc::clone(array), source_name.clone(), index))
            }
        }
    }

    fn assign_pointer_index(
        &mut self,
        pointer: &PointerValue,
        index_value: i64,
        value: i64,
    ) -> CustResult<()> {
        let (array, source_name, index) = self.checked_pointer_value_index(pointer, index_value)?;
        let mut array = array.borrow_mut();
        if array.read_only {
            return Err(CustError::new(match source_name {
                Some(name) => format!("cannot modify read-only array '{name}'"),
                None => "cannot modify read-only array through pointer".to_string(),
            }));
        }
        array.elements[index] = value;
        Ok(())
    }

    fn checked_array_index(
        &mut self,
        name: &str,
        index: &Expr,
    ) -> CustResult<(Rc<RefCell<ArrayValue>>, usize)> {
        let index_value = self.eval(index)?;
        let array = self.find_array(name)?;
        let len = array.borrow().elements.len();
        let Ok(index) = usize::try_from(index_value) else {
            return Err(CustError::new(format!(
                "array '{name}' index {index_value} out of bounds for length {len}"
            )));
        };
        if index >= len {
            return Err(CustError::new(format!(
                "array '{name}' index {index_value} out of bounds for length {len}"
            )));
        }
        Ok((array, index))
    }

    fn consume_loop_iteration(&mut self) -> CustResult<()> {
        match self.max_loop_iterations {
            Some(max) if self.loop_iterations >= max => {
                return Err(CustError::new(format!(
                    "execution step limit exceeded after {max} loop iterations"
                )));
            }
            Some(_) | None => {}
        }
        self.loop_iterations += 1;
        Ok(())
    }

    fn pointer_truthy(pointer: &PointerValue) -> bool {
        !matches!(pointer, PointerValue::Null)
    }

    fn expr_is_pointer_value(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Number(_) => false,
            Expr::AddressOf(_)
            | Expr::AddressOfArray { .. }
            | Expr::AddressOfStructField { .. }
            | Expr::AddressOfStructElementField { .. }
            | Expr::AddressOfStructPtrField { .. }
            | Expr::StringLiteral(_)
            | Expr::ArrayLiteral { .. }
            | Expr::AggregateArrayLiteral { .. } => true,
            Expr::Var(name) | Expr::Assign { name, .. } | Expr::CompoundAssign { name, .. } => {
                matches!(
                    self.find_variable(name),
                    Some(Value::Pointer { .. } | Value::Array(_) | Value::StructArray { .. })
                )
            }
            Expr::StructGet { name, fields } => self.struct_field_is_pointer(name, fields),
            Expr::AggregateFieldGet { aggregate, fields } => matches!(
                self.aggregate_literal_field_metadata(aggregate, fields),
                Ok(Some((
                    StructFieldType::Pointer(_)
                        | StructFieldType::Array(_, _)
                        | StructFieldType::StructArray(_, _),
                    _,
                    _
                )))
            ),
            Expr::StructSet { name, fields, .. } => self.struct_field_is_pointer(name, fields),
            Expr::Increment { target, .. } => self.expr_is_pointer_value(target),
            Expr::Call { name, .. } => matches!(
                self.functions
                    .get(name)
                    .map(|function| &function.return_type),
                Some(ReturnType::Pointer { .. })
            ),
            Expr::Conditional {
                then_expr,
                else_expr,
                ..
            } => self.expr_is_pointer_value(then_expr) || self.expr_is_pointer_value(else_expr),
            Expr::Comma(_, right) => self.expr_is_pointer_value(right),
            Expr::Binary(left, BinaryOp::Add | BinaryOp::Sub, right) => {
                self.expr_is_pointer_value(left) || self.expr_is_pointer_value(right)
            }
            _ => false,
        }
    }

    fn pointer_eq(left: &PointerValue, right: &PointerValue) -> bool {
        match (left, right) {
            (PointerValue::Null, PointerValue::Null) => true,
            (PointerValue::Null, _) | (_, PointerValue::Null) => false,
            (
                PointerValue::Scalar {
                    scope_id: left_scope,
                    name: left_name,
                },
                PointerValue::Scalar {
                    scope_id: right_scope,
                    name: right_name,
                },
            ) => left_scope == right_scope && left_name == right_name,
            (
                PointerValue::Struct {
                    scope_id: left_scope,
                    name: left_name,
                },
                PointerValue::Struct {
                    scope_id: right_scope,
                    name: right_name,
                },
            ) => left_scope == right_scope && left_name == right_name,
            (
                PointerValue::StructElement {
                    scope_id: left_scope,
                    name: left_name,
                    index: left_index,
                },
                PointerValue::StructElement {
                    scope_id: right_scope,
                    name: right_name,
                    index: right_index,
                },
            ) => left_scope == right_scope && left_name == right_name && left_index == right_index,
            (
                PointerValue::StructFieldElement {
                    scope_id: left_scope,
                    name: left_name,
                    element_index: left_element,
                    fields: left_fields,
                    index: left_index,
                },
                PointerValue::StructFieldElement {
                    scope_id: right_scope,
                    name: right_name,
                    element_index: right_element,
                    fields: right_fields,
                    index: right_index,
                },
            ) => {
                left_scope == right_scope
                    && left_name == right_name
                    && left_element == right_element
                    && left_fields == right_fields
                    && left_index == right_index
            }
            (
                PointerValue::StructField {
                    scope_id: left_scope,
                    name: left_name,
                    element_index: left_element,
                    fields: left_fields,
                },
                PointerValue::StructField {
                    scope_id: right_scope,
                    name: right_name,
                    element_index: right_element,
                    fields: right_fields,
                },
            ) => {
                left_scope == right_scope
                    && left_name == right_name
                    && left_element == right_element
                    && left_fields == right_fields
            }
            (
                PointerValue::StructFieldElementField {
                    scope_id: left_scope,
                    name: left_name,
                    element_index: left_element,
                    array_fields: left_array_fields,
                    index: left_index,
                    fields: left_fields,
                },
                PointerValue::StructFieldElementField {
                    scope_id: right_scope,
                    name: right_name,
                    element_index: right_element,
                    array_fields: right_array_fields,
                    index: right_index,
                    fields: right_fields,
                },
            ) => {
                left_scope == right_scope
                    && left_name == right_name
                    && left_element == right_element
                    && left_array_fields == right_array_fields
                    && left_index == right_index
                    && left_fields == right_fields
            }
            (PointerValue::Struct { .. }, PointerValue::StructElement { .. })
            | (PointerValue::StructElement { .. }, PointerValue::Struct { .. })
            | (PointerValue::Struct { .. }, PointerValue::StructField { .. })
            | (PointerValue::StructField { .. }, PointerValue::Struct { .. })
            | (PointerValue::StructElement { .. }, PointerValue::StructField { .. })
            | (PointerValue::StructField { .. }, PointerValue::StructElement { .. }) => false,
            (
                PointerValue::ArrayBase { array: left, .. },
                PointerValue::ArrayBase { array: right, .. },
            ) => Rc::ptr_eq(left, right),
            (
                PointerValue::ArrayBase { array: left, .. },
                PointerValue::ArrayElement {
                    array: right,
                    index,
                    ..
                },
            )
            | (
                PointerValue::ArrayElement {
                    array: right,
                    index,
                    ..
                },
                PointerValue::ArrayBase { array: left, .. },
            ) => *index == 0 && Rc::ptr_eq(left, right),
            (
                PointerValue::ArrayElement {
                    array: left,
                    index: left_index,
                    ..
                },
                PointerValue::ArrayElement {
                    array: right,
                    index: right_index,
                    ..
                },
            ) => left_index == right_index && Rc::ptr_eq(left, right),
            (PointerValue::Scalar { .. }, PointerValue::ArrayBase { .. })
            | (PointerValue::Scalar { .. }, PointerValue::ArrayElement { .. })
            | (PointerValue::Scalar { .. }, PointerValue::Struct { .. })
            | (PointerValue::ArrayBase { .. }, PointerValue::Scalar { .. })
            | (PointerValue::ArrayElement { .. }, PointerValue::Scalar { .. })
            | (PointerValue::ArrayBase { .. }, PointerValue::Struct { .. })
            | (PointerValue::ArrayElement { .. }, PointerValue::Struct { .. })
            | (PointerValue::Struct { .. }, PointerValue::Scalar { .. })
            | (PointerValue::Struct { .. }, PointerValue::ArrayBase { .. })
            | (PointerValue::Struct { .. }, PointerValue::ArrayElement { .. }) => false,
            _ => false,
        }
    }

    fn array_pointer_index(pointer: &PointerValue) -> CustResult<(&Rc<RefCell<ArrayValue>>, i64)> {
        match pointer {
            PointerValue::ArrayBase { array, .. } => Ok((array, 0)),
            PointerValue::ArrayElement { array, index, .. } => Ok((array, *index as i64)),
            PointerValue::Null => Err(CustError::new("null pointer arithmetic is not supported")),
            PointerValue::Scalar { .. }
            | PointerValue::Struct { .. }
            | PointerValue::StructElement { .. }
            | PointerValue::StructFieldElement { .. }
            | PointerValue::StructField { .. }
            | PointerValue::StructFieldElementField { .. } => {
                Err(CustError::new("scalar pointer arithmetic is not supported"))
            }
        }
    }

    fn pointer_ordering(
        &self,
        left: &PointerValue,
        op: BinaryOp,
        right: &PointerValue,
    ) -> CustResult<i64> {
        let difference = self.pointer_difference(left, right).map_err(|error| {
            let message = error.to_string();
            if message == "cannot subtract pointers to different arrays" {
                CustError::new("cannot compare pointers to different arrays")
            } else if message == "scalar pointer arithmetic is not supported"
                || message == "null pointer arithmetic is not supported"
            {
                CustError::new("pointer ordering comparisons are not supported")
            } else {
                error
            }
        })?;
        let ordering = difference.cmp(&0);
        let result = match op {
            BinaryOp::Lt => ordering.is_lt(),
            BinaryOp::Le => !ordering.is_gt(),
            BinaryOp::Gt => ordering.is_gt(),
            BinaryOp::Ge => !ordering.is_lt(),
            _ => unreachable!("only ordering operators reach pointer_ordering"),
        };
        Ok(result as i64)
    }

    fn pointer_difference(&self, left: &PointerValue, right: &PointerValue) -> CustResult<i64> {
        match (left, right) {
            (
                PointerValue::StructFieldElement {
                    scope_id: left_scope,
                    name: left_name,
                    element_index: left_element_index,
                    fields: left_fields,
                    index: left_index,
                },
                PointerValue::StructFieldElement {
                    scope_id: right_scope,
                    name: right_name,
                    element_index: right_element_index,
                    fields: right_fields,
                    index: right_index,
                },
            ) => {
                if left_scope != right_scope
                    || left_name != right_name
                    || left_element_index != right_element_index
                    || left_fields != right_fields
                {
                    return Err(CustError::new(
                        "cannot subtract pointers to different arrays",
                    ));
                }
                self.struct_field_array_pointer_at(
                    *left_scope,
                    left_name,
                    *left_element_index,
                    left_fields,
                    *left_index as i64,
                )?;
                self.struct_field_array_pointer_at(
                    *right_scope,
                    right_name,
                    *right_element_index,
                    right_fields,
                    *right_index as i64,
                )?;
                Ok(*left_index as i64 - *right_index as i64)
            }
            (PointerValue::StructFieldElement { .. }, _)
            | (_, PointerValue::StructFieldElement { .. }) => Err(CustError::new(
                "cannot subtract pointers to different arrays",
            )),
            (
                PointerValue::StructElement {
                    scope_id: left_scope,
                    name: left_name,
                    index: left_index,
                },
                PointerValue::StructElement {
                    scope_id: right_scope,
                    name: right_name,
                    index: right_index,
                },
            ) => {
                if left_scope != right_scope || left_name != right_name {
                    return Err(CustError::new(
                        "cannot subtract pointers to different arrays",
                    ));
                }
                self.struct_array_pointer_at(*left_scope, left_name, *left_index as i64)?;
                self.struct_array_pointer_at(*right_scope, right_name, *right_index as i64)?;
                Ok(*left_index as i64 - *right_index as i64)
            }
            (PointerValue::StructElement { .. }, _) | (_, PointerValue::StructElement { .. }) => {
                Err(CustError::new(
                    "cannot subtract pointers to different arrays",
                ))
            }
            _ => {
                let (left_array, left_index) = Self::array_pointer_index(left)?;
                let (right_array, right_index) = Self::array_pointer_index(right)?;
                if !Rc::ptr_eq(left_array, right_array) {
                    return Err(CustError::new(
                        "cannot subtract pointers to different arrays",
                    ));
                }
                Ok(left_index - right_index)
            }
        }
    }

    fn eval_truthy(&mut self, expr: &Expr) -> CustResult<bool> {
        match expr {
            Expr::Comma(left, right) => {
                self.eval_discard(left)?;
                self.eval_truthy(right)
            }
            Expr::AddressOf(_)
            | Expr::AddressOfArray { .. }
            | Expr::AddressOfStructField { .. }
            | Expr::AddressOfStructElementField { .. }
            | Expr::AddressOfStructArrayField { .. }
            | Expr::AddressOfStructElementArrayField { .. }
            | Expr::AddressOfStructPtrField { .. }
            | Expr::AddressOfStructPtrArrayField { .. }
            | Expr::AddressOfScalarLiteral { .. }
            | Expr::AddressOfAggregateLiteral { .. }
            | Expr::AddressOfAggregateField { .. }
            | Expr::StringLiteral(_)
            | Expr::ArrayLiteral { .. }
            | Expr::AggregateArrayLiteral { .. } => {
                Ok(Self::pointer_truthy(&self.eval_pointer(expr)?))
            }
            Expr::Assign { name, .. } => match self.find_variable(name).cloned() {
                Some(Value::Pointer { .. }) => Ok(Self::pointer_truthy(&self.eval_pointer(expr)?)),
                Some(Value::Scalar { .. }) => Ok(self.eval(expr)? != 0),
                Some(Value::Array(_)) => Err(CustError::new(format!(
                    "array '{name}' requires indexed assignment"
                ))),
                Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => Err(
                    CustError::new(format!("struct variable '{name}' used as scalar")),
                ),
                None => Err(CustError::new(format!(
                    "assignment to undeclared variable '{name}'"
                ))),
            },
            Expr::Var(name) => match self.find_variable(name).cloned() {
                Some(Value::Pointer { pointer, .. }) => Ok(Self::pointer_truthy(&pointer)),
                Some(Value::Array(array)) => Ok(!array.borrow().elements.is_empty()),
                Some(Value::Scalar { value, .. }) => Ok(value != 0),
                Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => Err(
                    CustError::new(format!("struct variable '{name}' used as scalar")),
                ),
                None => self
                    .find_enum_constant(name)
                    .map(|value| value != 0)
                    .ok_or_else(|| CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::Deref(_)
            | Expr::DerefSet { .. }
            | Expr::DerefCompoundSet { .. }
            | Expr::ArrayGet { .. }
            | Expr::ArraySet { .. }
            | Expr::ArrayCompoundSet { .. }
            | Expr::StructArrayGet { .. }
            | Expr::StructFieldArrayElementGet { .. }
            | Expr::StructFieldArrayElementSet { .. }
            | Expr::StructFieldArrayElementCompoundSet { .. }
            | Expr::StructArraySet { .. }
            | Expr::StructArrayCompoundSet { .. }
            | Expr::StructElementGet { .. }
            | Expr::StructElementArrayGet { .. }
            | Expr::StructPtrArrayGet { .. }
            | Expr::StructElementSet { .. }
            | Expr::StructElementArraySet { .. }
            | Expr::StructElementCompoundSet { .. }
            | Expr::StructElementArrayCompoundSet { .. }
            | Expr::StructGet { .. }
            | Expr::StructSet { .. }
            | Expr::StructCompoundSet { .. }
            | Expr::StructPtrGet { .. }
            | Expr::StructPtrSet { .. }
            | Expr::StructPtrCompoundSet { .. }
            | Expr::StringGet { .. } => Ok(self.eval(expr)? != 0),
            Expr::Call { name, .. }
                if matches!(
                    self.functions
                        .get(name)
                        .map(|function| &function.return_type),
                    Some(ReturnType::Pointer { .. })
                ) =>
            {
                Ok(Self::pointer_truthy(&self.eval_pointer(expr)?))
            }
            Expr::Call { .. } => Ok(self.eval(expr)? != 0),
            Expr::Number(value) => Ok(*value != 0),
            Expr::Binary(_, BinaryOp::Add | BinaryOp::Sub, _)
                if self.expr_is_pointer_value(expr) =>
            {
                match self.eval_pointer(expr) {
                    Ok(pointer) => Ok(Self::pointer_truthy(&pointer)),
                    Err(error) => Err(error),
                }
            }
            Expr::Binary(_, BinaryOp::Add | BinaryOp::Sub, _) => Ok(self.eval(expr)? != 0),
            Expr::UnaryPlus(_)
            | Expr::UnaryMinus(_)
            | Expr::BitwiseNot(_)
            | Expr::LogicalNot(_)
            | Expr::Cast { .. }
            | Expr::ScalarLiteral { .. }
            | Expr::AggregateLiteral { .. }
            | Expr::AggregateFieldGet { .. }
            | Expr::AggregateFieldSet { .. }
            | Expr::AggregateFieldCompoundSet { .. }
            | Expr::SizeOfType(_)
            | Expr::SizeOfValue(_)
            | Expr::AlignOfType(_)
            | Expr::CompoundAssign { .. }
            | Expr::ScalarLiteralSet { .. }
            | Expr::ScalarLiteralCompoundSet { .. }
            | Expr::Increment { .. }
            | Expr::Conditional { .. }
            | Expr::Binary(_, _, _) => Ok(self.eval(expr)? != 0),
        }
    }

    fn eval_discard(&mut self, expr: &Expr) -> CustResult<()> {
        if let Expr::Call { name, args } = expr {
            self.call_function(name, args)?;
            return Ok(());
        }
        match expr {
            Expr::Assign { name, value }
                if matches!(self.find_variable(name), Some(Value::Struct { .. })) =>
            {
                self.assign_struct_copy(name, value)?;
                return Ok(());
            }
            Expr::DerefSet { pointer, value } => {
                if match pointer.as_ref() {
                    Expr::Var(name) => matches!(
                        self.find_variable(name),
                        Some(Value::Pointer {
                            ty: PointeeType::Struct(_),
                            ..
                        })
                    ),
                    _ => false,
                } {
                    self.assign_struct_pointer_copy(pointer, value)?;
                    return Ok(());
                }
            }
            _ => {}
        }
        if matches!(
            expr,
            Expr::StructPtrSet { .. }
                | Expr::CompoundAssign {
                    op: CompoundOp::Add | CompoundOp::Sub,
                    ..
                }
                | Expr::StructCompoundSet {
                    op: CompoundOp::Add | CompoundOp::Sub,
                    ..
                }
                | Expr::StructPtrCompoundSet {
                    op: CompoundOp::Add | CompoundOp::Sub,
                    ..
                }
                | Expr::Increment { .. }
        ) && self.eval_pointer(expr).is_ok()
        {
            return Ok(());
        }
        match self.eval(expr) {
            Ok(_) => Ok(()),
            Err(error) if error.to_string() == "pointer value used as scalar" => {
                self.eval_pointer(expr).map(|_| ())
            }
            Err(error) => Err(error),
        }
    }

    fn eval_assignment_expr(&mut self, name: &str, value: &Expr) -> CustResult<i64> {
        match self.find_variable(name).cloned() {
            Some(Value::Scalar { .. }) => {
                self.ensure_variable_mutable(name)?;
                let value = self.eval(value)?;
                if let Some(Value::Scalar { value: slot, .. }) = self.find_variable_mut(name) {
                    *slot = value;
                }
                Ok(value)
            }
            Some(Value::Pointer { .. }) => Err(CustError::new("pointer value used as scalar")),
            Some(Value::Array(_)) => Err(CustError::new(format!(
                "array '{name}' requires indexed assignment"
            ))),
            Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => Err(CustError::new(
                format!("struct variable '{name}' assignment is not supported"),
            )),
            None if self.find_enum_constant(name).is_some() => Err(CustError::new(format!(
                "cannot assign to enum constant '{name}'"
            ))),
            None => Err(CustError::new(format!(
                "assignment to undeclared variable '{name}'"
            ))),
        }
    }

    fn eval_compound_assignment_expr(
        &mut self,
        name: &str,
        op: CompoundOp,
        value: &Expr,
    ) -> CustResult<i64> {
        match self.find_variable(name).cloned() {
            Some(Value::Scalar { value: current, .. }) => {
                self.ensure_variable_mutable(name)?;
                let rhs = self.eval(value)?;
                let result = Self::apply_compound_op(current, op, rhs)?;
                if let Some(Value::Scalar { value: slot, .. }) = self.find_variable_mut(name) {
                    *slot = result;
                }
                Ok(result)
            }
            Some(Value::Pointer { .. }) => Err(Self::pointer_compound_error(op)),
            Some(Value::Array(_)) => Err(CustError::new(format!(
                "array '{name}' requires indexed assignment"
            ))),
            Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => Err(CustError::new(
                format!("struct variable '{name}' assignment is not supported"),
            )),
            None if self.find_enum_constant(name).is_some() => Err(CustError::new(format!(
                "cannot assign to enum constant '{name}'"
            ))),
            None => Err(CustError::new(format!(
                "assignment to undeclared variable '{name}'"
            ))),
        }
    }

    fn apply_compound_op(lhs: i64, op: CompoundOp, rhs: i64) -> CustResult<i64> {
        match op {
            CompoundOp::Add => Ok(lhs + rhs),
            CompoundOp::Sub => Ok(lhs - rhs),
            CompoundOp::Mul => Ok(lhs * rhs),
            CompoundOp::Div if rhs == 0 => Err(CustError::new("division by zero")),
            CompoundOp::Div => Ok(lhs / rhs),
            CompoundOp::Rem if rhs == 0 => Err(CustError::new("division by zero")),
            CompoundOp::Rem => Ok(lhs % rhs),
            CompoundOp::BitAnd => Ok(lhs & rhs),
            CompoundOp::BitOr => Ok(lhs | rhs),
            CompoundOp::BitXor => Ok(lhs ^ rhs),
            CompoundOp::ShiftLeft => Self::checked_shift_left(lhs, rhs),
            CompoundOp::ShiftRight => Self::checked_shift_right(lhs, rhs),
        }
    }

    fn pointer_compound_error(op: CompoundOp) -> CustError {
        match op {
            CompoundOp::Add
            | CompoundOp::Sub
            | CompoundOp::Mul
            | CompoundOp::Div
            | CompoundOp::Rem => CustError::new("pointer arithmetic is not supported"),
            CompoundOp::BitAnd
            | CompoundOp::BitOr
            | CompoundOp::BitXor
            | CompoundOp::ShiftLeft
            | CompoundOp::ShiftRight => {
                CustError::new("pointer bitwise operations are not supported")
            }
        }
    }

    fn apply_increment_op(current: i64, op: IncrementOp) -> i64 {
        match op {
            IncrementOp::Inc => current + 1,
            IncrementOp::Dec => current - 1,
        }
    }

    fn increment_result(current: i64, updated: i64, prefix: bool) -> i64 {
        if prefix { updated } else { current }
    }

    fn eval_aggregate_literal_field_scalar(
        &mut self,
        aggregate: &Expr,
        path: &[String],
    ) -> CustResult<(i64, bool)> {
        match self.eval_struct_expr(aggregate)? {
            ReturnValue::Struct { type_name, fields } => {
                let (_, field_value) = Self::nested_field_value(&type_name, &fields, path)?;
                match field_value {
                    StructFieldValue::Scalar {
                        value, is_const, ..
                    } => Ok((*value, *is_const)),
                    StructFieldValue::Array { .. }
                    | StructFieldValue::Struct { .. }
                    | StructFieldValue::StructArray { .. }
                    | StructFieldValue::Pointer { .. } => {
                        Err(CustError::new("struct field used as scalar expression"))
                    }
                }
            }
            _ => Err(CustError::new("expected struct expression")),
        }
    }

    fn eval_aggregate_literal_pointer_field_metadata(
        &mut self,
        aggregate: &Expr,
        path: &[String],
    ) -> CustResult<(PointerValue, PointeeType, bool, bool)> {
        match self.eval_struct_expr(aggregate)? {
            ReturnValue::Struct { type_name, fields } => {
                let (_, field_value) = Self::nested_field_value(&type_name, &fields, path)?;
                match field_value {
                    StructFieldValue::Pointer {
                        pointer,
                        ty,
                        is_const,
                        points_to_const,
                    } => Ok((pointer.clone(), ty.clone(), *is_const, *points_to_const)),
                    _ => Err(CustError::new("struct field is not a pointer")),
                }
            }
            _ => Err(CustError::new("expected struct expression")),
        }
    }

    fn eval_aggregate_literal_field_pointer(
        &mut self,
        aggregate: &Expr,
        path: &[String],
    ) -> CustResult<PointerValue> {
        match self.eval_struct_expr(aggregate)? {
            ReturnValue::Struct { type_name, fields } => {
                let (_, field_value) = Self::nested_field_value(&type_name, &fields, path)?;
                match field_value {
                    StructFieldValue::Pointer { pointer, .. } => Ok(pointer.clone()),
                    StructFieldValue::Array { value, .. } => Ok(PointerValue::ArrayBase {
                        array: Rc::clone(value),
                        source_name: None,
                    }),
                    StructFieldValue::StructArray {
                        type_name,
                        elements,
                        is_const,
                    } => {
                        let scope_id = self
                            .scopes
                            .last()
                            .expect("compound literal evaluation requires a current scope")
                            .id;
                        let name = format!(
                            "__cust_compound_aggregate_field_array#{}",
                            self.next_compound_literal_id
                        );
                        self.next_compound_literal_id += 1;
                        self.current_scope_mut().insert(
                            name.clone(),
                            Value::StructArray {
                                type_name: type_name.clone(),
                                elements: elements
                                    .iter()
                                    .map(StructFieldValue::deep_clone_fields)
                                    .collect(),
                                read_only: *is_const,
                            },
                        );
                        Ok(PointerValue::StructElement {
                            scope_id,
                            name,
                            index: 0,
                        })
                    }
                    StructFieldValue::Scalar { .. } | StructFieldValue::Struct { .. } => {
                        Err(CustError::new("struct field is not a pointer"))
                    }
                }
            }
            _ => Err(CustError::new("expected struct expression")),
        }
    }

    fn checked_shift_left(lhs: i64, rhs: i64) -> CustResult<i64> {
        let shift = Self::checked_shift_count(rhs)?;
        lhs.checked_shl(shift)
            .ok_or_else(|| CustError::new("shift count too large"))
    }

    fn checked_shift_right(lhs: i64, rhs: i64) -> CustResult<i64> {
        let shift = Self::checked_shift_count(rhs)?;
        lhs.checked_shr(shift)
            .ok_or_else(|| CustError::new("shift count too large"))
    }

    fn checked_shift_count(rhs: i64) -> CustResult<u32> {
        if rhs < 0 {
            return Err(CustError::new("shift count must be non-negative"));
        }
        u32::try_from(rhs).map_err(|_| CustError::new("shift count too large"))
    }

    fn sizeof_expr(&self, expr: &Expr) -> CustResult<i64> {
        match expr {
            Expr::Number(_) => Ok(INT_SIZE),
            Expr::StringLiteral(values) => Ok(values.len() as i64 * CHAR_SIZE),
            Expr::ArrayLiteral {
                elem_type,
                len,
                init,
            } => {
                let len = len.unwrap_or_else(|| Self::infer_array_initializer_len(init));
                Ok(len as i64 * elem_type.size())
            }
            Expr::AggregateArrayLiteral {
                type_name,
                len,
                init,
            } => {
                let len = len.unwrap_or_else(|| Self::infer_struct_array_initializer_len(init));
                let element_size = self
                    .struct_types
                    .get(type_name)
                    .map(|struct_type| struct_type.size(&self.struct_types))
                    .transpose()?
                    .ok_or_else(|| {
                        CustError::new(format!("undefined struct type '{type_name}'"))
                    })?;
                Ok(len as i64 * element_size)
            }
            Expr::SizeOfType(_) | Expr::SizeOfValue(_) | Expr::AlignOfType(_) => Ok(INT_SIZE),
            Expr::Var(name) => self.sizeof_variable(name),
            Expr::StructGet { name, fields } => self.sizeof_struct_field(name, fields),
            Expr::StructArrayGet { name, fields, .. } => {
                self.sizeof_struct_array_indexed_value(name, fields)
            }
            Expr::StructFieldArrayElementGet {
                name,
                array_fields,
                index,
                fields,
            }
            | Expr::StructFieldArrayElementSet {
                name,
                array_fields,
                index,
                fields,
                ..
            }
            | Expr::StructFieldArrayElementCompoundSet {
                name,
                array_fields,
                index,
                fields,
                ..
            } => {
                let _ = (name, array_fields, index, fields);
                Ok(INT_SIZE)
            }
            Expr::StructElementGet { name, fields, .. } => {
                self.sizeof_struct_element_field(name, fields)
            }
            Expr::StructElementArrayGet { name, fields, .. } => {
                self.sizeof_struct_element_array_indexed_value(name, fields)
            }
            Expr::StructPtrGet { pointer, fields } => {
                self.sizeof_struct_pointer_field(pointer, fields)
            }
            Expr::StructPtrArrayGet {
                pointer, fields, ..
            } => {
                let pointee = self.pointer_expr_pointee_type(&Expr::StructPtrGet {
                    pointer: pointer.clone(),
                    fields: fields.clone(),
                })?;
                match pointee {
                    Some(PointeeType::Scalar(ty)) => Ok(ty.size()),
                    Some(PointeeType::Struct(type_name)) => self
                        .struct_types
                        .get(&type_name)
                        .map(|struct_type| struct_type.size(&self.struct_types))
                        .transpose()?
                        .ok_or_else(|| {
                            CustError::new(format!("undefined struct type '{type_name}'"))
                        }),
                    None => Err(CustError::new("expected pointer expression")),
                }
            }
            Expr::ArrayGet { name, .. } => match self.find_variable(name) {
                Some(Value::StructArray { type_name, .. }) => self
                    .struct_types
                    .get(type_name)
                    .map(|struct_type| struct_type.size(&self.struct_types))
                    .transpose()?
                    .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
                _ => self.sizeof_indexed_value(name),
            },
            Expr::StringGet { .. } => Ok(CHAR_SIZE),
            Expr::AddressOf(_)
            | Expr::AddressOfArray { .. }
            | Expr::AddressOfStructField { .. }
            | Expr::AddressOfStructElementField { .. }
            | Expr::AddressOfStructArrayField { .. }
            | Expr::AddressOfStructElementArrayField { .. }
            | Expr::AddressOfStructPtrField { .. }
            | Expr::AddressOfStructPtrArrayField { .. }
            | Expr::AddressOfScalarLiteral { .. }
            | Expr::AddressOfAggregateLiteral { .. }
            | Expr::AddressOfAggregateField { .. } => Ok(POINTER_SIZE),
            Expr::Deref(pointer) => self.sizeof_deref(pointer),
            Expr::Assign { name, .. } | Expr::CompoundAssign { name, .. } => {
                self.sizeof_assignment_result(name)
            }
            Expr::ArraySet { name, .. } | Expr::ArrayCompoundSet { name, .. } => {
                self.sizeof_indexed_value(name)
            }
            Expr::DerefSet { pointer, .. } | Expr::DerefCompoundSet { pointer, .. } => {
                self.sizeof_deref(pointer)
            }
            Expr::StructSet { name, fields, .. } | Expr::StructCompoundSet { name, fields, .. } => {
                self.sizeof_struct_field(name, fields)
            }
            Expr::StructArraySet { name, fields, .. }
            | Expr::StructArrayCompoundSet { name, fields, .. } => {
                self.sizeof_struct_array_indexed_value(name, fields)
            }
            Expr::StructElementSet { name, fields, .. }
            | Expr::StructElementCompoundSet { name, fields, .. } => {
                self.sizeof_struct_element_field(name, fields)
            }
            Expr::StructElementArraySet { name, fields, .. }
            | Expr::StructElementArrayCompoundSet { name, fields, .. } => {
                self.sizeof_struct_element_array_indexed_value(name, fields)
            }
            Expr::StructPtrSet {
                pointer, fields, ..
            }
            | Expr::StructPtrCompoundSet {
                pointer, fields, ..
            } => self.sizeof_struct_pointer_field(pointer, fields),
            Expr::Increment { target, .. } => self.sizeof_expr(target),
            Expr::Cast { ty, .. }
            | Expr::ScalarLiteral { ty, .. }
            | Expr::ScalarLiteralSet { ty, .. }
            | Expr::ScalarLiteralCompoundSet { ty, .. } => Ok(ty.size()),
            Expr::AggregateLiteral { type_name, .. } => self
                .struct_types
                .get(type_name)
                .map(|struct_type| struct_type.size(&self.struct_types))
                .transpose()?
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
            Expr::AggregateFieldGet { aggregate, fields }
            | Expr::AggregateFieldSet {
                aggregate, fields, ..
            }
            | Expr::AggregateFieldCompoundSet {
                aggregate, fields, ..
            } => {
                let type_name = self.aggregate_expr_type_name(aggregate)?;
                self.sizeof_aggregate_field_type(&type_name, fields)
            }
            Expr::Call { name, .. } => match self.functions.get(name) {
                Some(function) => function
                    .return_type
                    .size(&self.struct_types)
                    .ok_or_else(|| {
                        CustError::new(format!("void function '{name}' used as scalar expression"))
                    }),
                None => Err(CustError::new(format!("undefined function '{name}'"))),
            },
            Expr::UnaryPlus(_)
            | Expr::UnaryMinus(_)
            | Expr::BitwiseNot(_)
            | Expr::LogicalNot(_)
            | Expr::Conditional { .. }
            | Expr::Comma(_, _)
            | Expr::Binary(_, _, _) => Ok(INT_SIZE),
        }
    }

    fn aggregate_expr_type_name(&self, expr: &Expr) -> CustResult<String> {
        match expr {
            Expr::AggregateLiteral { type_name, .. } => Ok(type_name.clone()),
            Expr::AggregateFieldGet { aggregate, fields } => {
                let type_name = self.aggregate_expr_type_name(aggregate)?;
                self.aggregate_field_type_name(&type_name, fields)
            }
            Expr::Var(name) | Expr::Assign { name, .. } => match self.find_variable(name) {
                Some(Value::Struct { type_name, .. }) => Ok(type_name.clone()),
                Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::StructGet { name, fields } => match self.find_variable(name) {
                Some(Value::Struct { type_name, .. }) => {
                    self.aggregate_field_type_name(type_name, fields)
                }
                Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::ArrayGet { name, .. } => match self.find_variable(name) {
                Some(Value::StructArray { type_name, .. }) => Ok(type_name.clone()),
                Some(Value::Pointer {
                    ty: PointeeType::Struct(type_name),
                    ..
                }) => Ok(type_name.clone()),
                _ => Err(CustError::new("expected struct expression")),
            },
            Expr::Deref(pointer) | Expr::DerefSet { pointer, .. } => {
                match self.pointer_expr_pointee_type(pointer)? {
                    Some(PointeeType::Struct(type_name)) => Ok(type_name),
                    _ => Err(CustError::new("expected struct expression")),
                }
            }
            Expr::Call { name, .. } => match self.functions.get(name) {
                Some(function) => match &function.return_type {
                    ReturnType::Struct(type_name) => Ok(type_name.clone()),
                    ReturnType::Scalar(_) => Err(CustError::new(format!(
                        "scalar function '{name}' used as struct expression"
                    ))),
                    ReturnType::Pointer { .. } => Err(CustError::new(format!(
                        "pointer function '{name}' used as struct expression"
                    ))),
                    ReturnType::Void => Err(CustError::new(format!(
                        "void function '{name}' used as struct expression"
                    ))),
                },
                None => Err(CustError::new(format!("undefined function '{name}'"))),
            },
            Expr::Conditional {
                then_expr,
                else_expr,
                ..
            } => {
                let then_type = self.aggregate_expr_type_name(then_expr)?;
                let else_type = self.aggregate_expr_type_name(else_expr)?;
                if then_type == else_type {
                    Ok(then_type)
                } else {
                    Err(CustError::new(format!(
                        "conditional branches have mismatched aggregate types: {} vs {}",
                        self.aggregate_label(&then_type),
                        self.aggregate_label(&else_type)
                    )))
                }
            }
            Expr::Comma(_, right) => self.aggregate_expr_type_name(right),
            _ => Err(CustError::new("expected struct expression")),
        }
    }

    fn aggregate_field_type_name(&self, type_name: &str, path: &[String]) -> CustResult<String> {
        let mut current_type_name = type_name.to_string();
        for (index, field_name) in path.iter().enumerate() {
            let struct_type = self.struct_types.get(&current_type_name).ok_or_else(|| {
                CustError::new(format!("undefined struct type '{current_type_name}'"))
            })?;
            let field = struct_type
                .fields
                .iter()
                .find(|field| field.name == *field_name)
                .ok_or_else(|| {
                    CustError::new(format!(
                        "struct '{current_type_name}' has no field '{field_name}'"
                    ))
                })?;
            let is_last = index + 1 == path.len();
            match &field.ty {
                StructFieldType::Struct(nested_type) if is_last => return Ok(nested_type.clone()),
                StructFieldType::Struct(nested_type) => current_type_name = nested_type.clone(),
                _ => return Err(CustError::new("expected struct expression")),
            }
        }
        Ok(current_type_name)
    }

    fn sizeof_aggregate_field_type(&self, type_name: &str, path: &[String]) -> CustResult<i64> {
        let mut current_type_name = type_name.to_string();
        for (index, field_name) in path.iter().enumerate() {
            let struct_type = self.struct_types.get(&current_type_name).ok_or_else(|| {
                CustError::new(format!("undefined struct type '{current_type_name}'"))
            })?;
            let field = struct_type
                .fields
                .iter()
                .find(|field| field.name == *field_name)
                .ok_or_else(|| {
                    CustError::new(format!(
                        "struct '{current_type_name}' has no field '{field_name}'"
                    ))
                })?;
            let is_last = index + 1 == path.len();
            match &field.ty {
                StructFieldType::Scalar(ty) if is_last => return Ok(ty.size()),
                StructFieldType::Array(elem_type, len) if is_last => {
                    return Ok(*len as i64 * elem_type.size());
                }
                StructFieldType::Struct(nested_type) if is_last => {
                    return self
                        .struct_types
                        .get(nested_type)
                        .map(|struct_type| struct_type.size(&self.struct_types))
                        .transpose()?
                        .ok_or_else(|| {
                            CustError::new(format!("undefined struct type '{nested_type}'"))
                        });
                }
                StructFieldType::Struct(nested_type) => current_type_name = nested_type.clone(),
                StructFieldType::Pointer(_) if is_last => return Ok(POINTER_SIZE),
                StructFieldType::StructArray(element_type, len) if is_last => {
                    let element_size = self
                        .struct_types
                        .get(element_type)
                        .map(|struct_type| struct_type.size(&self.struct_types))
                        .transpose()?
                        .ok_or_else(|| {
                            CustError::new(format!("undefined struct type '{element_type}'"))
                        })?;
                    return Ok(element_size * *len as i64);
                }
                _ => return Err(CustError::new("expected struct expression")),
            }
        }
        self.struct_types
            .get(type_name)
            .map(|struct_type| struct_type.size(&self.struct_types))
            .transpose()?
            .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'")))
    }

    fn struct_field_value_size(&self, field_value: &StructFieldValue) -> CustResult<i64> {
        match field_value {
            StructFieldValue::Scalar { ty, .. } => Ok(ty.size()),
            StructFieldValue::Array { value, .. } => {
                let array = value.borrow();
                Ok(array.elements.len() as i64 * array.elem_type.size())
            }
            StructFieldValue::Struct { type_name, .. } => self
                .struct_types
                .get(type_name)
                .map(|struct_type| struct_type.size(&self.struct_types))
                .transpose()?
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
            StructFieldValue::StructArray {
                type_name,
                elements,
                ..
            } => self
                .struct_types
                .get(type_name)
                .map(|struct_type| {
                    struct_type
                        .size(&self.struct_types)
                        .map(|size| size * elements.len() as i64)
                })
                .transpose()?
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
            StructFieldValue::Pointer { .. } => Ok(POINTER_SIZE),
        }
    }

    fn sizeof_variable(&self, name: &str) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::Scalar { ty, .. }) => Ok(ty.size()),
            Some(Value::Array(array)) => {
                let array = array.borrow();
                Ok(array.elements.len() as i64 * array.elem_type.size())
            }
            Some(Value::Pointer { .. }) => Ok(POINTER_SIZE),
            Some(Value::Struct { type_name, .. }) => self
                .struct_types
                .get(type_name)
                .map(|struct_type| struct_type.size(&self.struct_types))
                .transpose()?
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
            Some(Value::StructArray {
                type_name,
                elements,
                ..
            }) => self
                .struct_types
                .get(type_name)
                .map(|struct_type| struct_type.size(&self.struct_types))
                .transpose()?
                .map(|element_size| element_size * elements.len() as i64)
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn sizeof_struct_field(&self, name: &str, path: &[String]) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::Struct { type_name, fields }) => {
                let (_, field_value) = Self::nested_field_value(type_name, fields, path)?;
                self.struct_field_value_size(field_value)
            }
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn sizeof_struct_pointer_field(&self, pointer: &Expr, path: &[String]) -> CustResult<i64> {
        let pointer = self.clone_for_sizeof_pointer(pointer)?;
        let (type_name, fields) = self.find_struct_pointer_fields(&pointer)?;
        let (_, field_value) = Self::nested_field_value(&type_name, fields, path)?;
        self.struct_field_value_size(field_value)
    }

    fn sizeof_struct_array_indexed_value(&self, name: &str, path: &[String]) -> CustResult<i64> {
        let array = self.find_struct_array_field(name, path)?;
        Ok(array.borrow().elem_type.size())
    }

    fn sizeof_struct_element_field(&self, name: &str, path: &[String]) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::StructArray {
                type_name,
                elements,
                ..
            }) => {
                let Some(first_element) = elements.first() else {
                    return Err(CustError::new(format!(
                        "struct array '{name}' has no elements"
                    )));
                };
                let (_, field_value) = Self::nested_field_value(type_name, first_element, path)?;
                self.struct_field_value_size(field_value)
            }
            Some(_) => Err(CustError::new(format!(
                "variable '{name}' is not a struct array"
            ))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn sizeof_struct_element_array_indexed_value(
        &self,
        name: &str,
        path: &[String],
    ) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::StructArray {
                type_name,
                elements,
                ..
            }) => {
                let Some(first_element) = elements.first() else {
                    return Err(CustError::new(format!(
                        "struct array '{name}' has no elements"
                    )));
                };
                let (_, field_value) = Self::nested_field_value(type_name, first_element, path)?;
                match field_value {
                    StructFieldValue::Array { value, .. } => Ok(value.borrow().elem_type.size()),
                    _ => Err(CustError::new(format!(
                        "struct field '{}' is not an array",
                        Self::field_path_label(path)
                    ))),
                }
            }
            Some(_) => Err(CustError::new(format!(
                "variable '{name}' is not a struct array"
            ))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn clone_for_sizeof_pointer(&self, pointer: &Expr) -> CustResult<PointerValue> {
        match pointer {
            Expr::Var(name) => match self.find_variable(name) {
                Some(Value::Pointer { pointer, .. }) => Ok(pointer.clone()),
                Some(Value::Struct { .. }) => self.address_of_scalar(name),
                Some(_) => Err(CustError::new(format!(
                    "variable '{name}' is not a pointer"
                ))),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::AddressOf(name) => self.address_of_scalar(name),
            Expr::Deref(inner) => self.clone_for_sizeof_pointer(inner),
            _ => Err(CustError::new("expected pointer expression")),
        }
    }

    fn sizeof_assignment_result(&self, name: &str) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::Scalar { ty, .. }) => Ok(ty.size()),
            Some(Value::Pointer { .. }) => Ok(POINTER_SIZE),
            Some(Value::Array(_)) => Err(CustError::new(format!(
                "array '{name}' requires indexed assignment"
            ))),
            Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => Err(CustError::new(
                format!("struct variable '{name}' assignment is not supported"),
            )),
            None if self.find_enum_constant(name).is_some() => Err(CustError::new(format!(
                "cannot assign to enum constant '{name}'"
            ))),
            None => Err(CustError::new(format!(
                "assignment to undeclared variable '{name}'"
            ))),
        }
    }

    fn sizeof_indexed_value(&self, name: &str) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::Array(array)) => Ok(array.borrow().elem_type.size()),
            Some(Value::Pointer { ty, .. }) => Ok(ty.size(&self.struct_types)?),
            Some(Value::Scalar { .. }) => {
                Err(CustError::new(format!("variable '{name}' is not an array")))
            }
            Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => Err(CustError::new(
                format!("struct variable '{name}' is not an array"),
            )),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn sizeof_deref(&self, pointer: &Expr) -> CustResult<i64> {
        if let Some(ty) = self.pointer_expr_pointee_type(pointer)? {
            return ty.size(&self.struct_types);
        }
        match pointer {
            Expr::Var(name) => match self.find_variable(name) {
                Some(Value::Scalar { .. }) => Err(CustError::new(format!(
                    "variable '{name}' is not a pointer"
                ))),
                Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => Err(
                    CustError::new(format!("struct variable '{name}' used as pointer")),
                ),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
                Some(Value::Pointer { .. } | Value::Array(_)) => {
                    unreachable!("pointer-like variables should have returned a pointee type")
                }
            },
            _ => Ok(INT_SIZE),
        }
    }

    fn eval_increment_expr(
        &mut self,
        target: &Expr,
        op: IncrementOp,
        prefix: bool,
    ) -> CustResult<i64> {
        match target {
            Expr::Var(name) => match self.find_variable(name).cloned() {
                Some(Value::Scalar { value: current, .. }) => {
                    self.ensure_variable_mutable(name)?;
                    let updated = Self::apply_increment_op(current, op);
                    if let Some(Value::Scalar { value: slot, .. }) = self.find_variable_mut(name) {
                        *slot = updated;
                    }
                    Ok(Self::increment_result(current, updated, prefix))
                }
                Some(Value::Pointer { .. }) => {
                    Err(CustError::new("pointer arithmetic is not supported"))
                }
                Some(Value::Array(_)) => Err(CustError::new(format!(
                    "array '{name}' requires indexed assignment"
                ))),
                Some(Value::Struct { .. }) | Some(Value::StructArray { .. }) => {
                    Err(CustError::new(format!(
                        "struct variable '{name}' assignment is not supported"
                    )))
                }
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::ArrayGet { name, index } => {
                let (array, index) = match self.find_variable(name).cloned() {
                    Some(Value::Pointer { pointer, .. }) => {
                        self.ensure_pointer_variable_pointee_mutable(name)?;
                        let index_value = self.eval(index)?;
                        let (array, _, index) =
                            self.checked_pointer_value_index(&pointer, index_value)?;
                        (array, index)
                    }
                    Some(_) | None => self.checked_array_index(name, index)?,
                };
                let current = array.borrow().elements[index];
                let updated = Self::apply_increment_op(current, op);
                let mut array = array.borrow_mut();
                if array.read_only {
                    return Err(CustError::new(format!(
                        "cannot modify read-only array '{name}'"
                    )));
                }
                array.elements[index] = updated;
                Ok(Self::increment_result(current, updated, prefix))
            }
            Expr::StructArrayGet {
                name,
                fields,
                index,
            } => {
                self.ensure_variable_mutable(name)?;
                let (array, index) = self.checked_struct_array_index(name, fields, index)?;
                let current = array.borrow().elements[index];
                let updated = Self::apply_increment_op(current, op);
                let mut array = array.borrow_mut();
                if array.read_only {
                    return Err(CustError::new(format!(
                        "cannot modify read-only array '{}'",
                        Self::field_path_label(fields)
                    )));
                }
                array.elements[index] = updated;
                Ok(Self::increment_result(current, updated, prefix))
            }
            Expr::StructElementGet {
                name,
                index,
                fields,
            } => {
                let current = self.read_struct_element_field(name, index, fields)?;
                let updated = Self::apply_increment_op(current, op);
                self.assign_struct_element_field(name, index, fields, updated)?;
                Ok(Self::increment_result(current, updated, prefix))
            }
            Expr::StructElementArrayGet {
                name,
                index,
                fields,
                array_index,
            } => {
                let (array, index) =
                    self.checked_struct_element_array_index(name, index, fields, array_index)?;
                let current = array.borrow().elements[index];
                let updated = Self::apply_increment_op(current, op);
                let mut array = array.borrow_mut();
                if array.read_only {
                    return Err(CustError::new(format!(
                        "cannot modify read-only array '{}'",
                        Self::field_path_label(fields)
                    )));
                }
                array.elements[index] = updated;
                Ok(Self::increment_result(current, updated, prefix))
            }
            Expr::StructGet { name, fields } => {
                let current = self.read_struct_field(name, fields)?;
                let updated = Self::apply_increment_op(current, op);
                self.assign_struct_field(name, fields, updated)?;
                Ok(Self::increment_result(current, updated, prefix))
            }
            Expr::StructPtrGet { pointer, fields } => {
                self.ensure_pointer_expr_pointee_mutable(pointer)?;
                let pointer = self.eval_pointer(pointer)?;
                let current = self.read_struct_pointer_field(&pointer, fields)?;
                let updated = Self::apply_increment_op(current, op);
                self.assign_struct_pointer_field(&pointer, fields, updated)?;
                Ok(Self::increment_result(current, updated, prefix))
            }
            Expr::Deref(pointer) => {
                self.ensure_pointer_expr_pointee_mutable(pointer)?;
                let pointer = self.eval_pointer(pointer)?;
                let current = self.deref_pointer(&pointer)?;
                let updated = Self::apply_increment_op(current, op);
                self.assign_deref_pointer(&pointer, updated)?;
                Ok(Self::increment_result(current, updated, prefix))
            }
            Expr::ScalarLiteral { init, .. } => {
                let current = self.eval(init)?;
                let updated = Self::apply_increment_op(current, op);
                Ok(Self::increment_result(current, updated, prefix))
            }
            Expr::AggregateFieldGet { aggregate, fields } => {
                let (current, is_const) =
                    self.eval_aggregate_literal_field_scalar(aggregate, fields)?;
                if is_const {
                    return Err(CustError::new(format!(
                        "cannot assign to const struct field '{}'",
                        Self::field_path_label(fields)
                    )));
                }
                let updated = Self::apply_increment_op(current, op);
                Ok(Self::increment_result(current, updated, prefix))
            }
            _ => Err(CustError::new("invalid increment/decrement target")),
        }
    }

    fn eval_array_compound_set(
        &mut self,
        name: &str,
        index: &Expr,
        op: CompoundOp,
        value: &Expr,
    ) -> CustResult<i64> {
        let (array, index) = match self.find_variable(name).cloned() {
            Some(Value::Pointer { pointer, .. }) => {
                self.ensure_pointer_variable_pointee_mutable(name)?;
                let index_value = self.eval(index)?;
                let (array, _, index) = self.checked_pointer_value_index(&pointer, index_value)?;
                (array, index)
            }
            Some(_) | None => self.checked_array_index(name, index)?,
        };
        let current = array.borrow().elements[index];
        let rhs = self.eval(value)?;
        let result = Self::apply_compound_op(current, op, rhs)?;
        let mut array = array.borrow_mut();
        if array.read_only {
            return Err(CustError::new(format!(
                "cannot modify read-only array '{name}'"
            )));
        }
        array.elements[index] = result;
        Ok(result)
    }

    fn eval_deref_compound_set(
        &mut self,
        pointer: &Expr,
        op: CompoundOp,
        value: &Expr,
    ) -> CustResult<i64> {
        self.ensure_pointer_expr_pointee_mutable(pointer)?;
        let pointer = self.eval_pointer(pointer)?;
        let current = self.deref_pointer(&pointer)?;
        let rhs = self.eval(value)?;
        let result = Self::apply_compound_op(current, op, rhs)?;
        self.assign_deref_pointer(&pointer, result)?;
        Ok(result)
    }

    fn eval_equality(&mut self, left: &Expr, op: &BinaryOp, right: &Expr) -> CustResult<i64> {
        match (self.eval_pointer(left), self.eval_pointer(right)) {
            (Ok(left_pointer), Ok(right_pointer)) => {
                let equal = Self::pointer_eq(&left_pointer, &right_pointer);
                Ok((*op == BinaryOp::Eq && equal || *op == BinaryOp::Ne && !equal) as i64)
            }
            (Ok(_), Err(_)) if matches!(right, Expr::Number(value) if *value != 0) => Err(
                CustError::new("cannot compare pointer with nonzero integer"),
            ),
            (Err(_), Ok(_)) if matches!(left, Expr::Number(value) if *value != 0) => Err(
                CustError::new("cannot compare pointer with nonzero integer"),
            ),
            (Ok(_), Err(error)) if !matches!(left, Expr::Number(0)) => Err(error),
            (Err(error), Ok(_)) if !matches!(right, Expr::Number(0)) => Err(error),
            (Ok(_), Err(_)) | (Err(_), Ok(_)) | (Err(_), Err(_)) => {
                let lhs = self.eval(left)?;
                let rhs = self.eval(right)?;
                match op {
                    BinaryOp::Eq => Ok((lhs == rhs) as i64),
                    BinaryOp::Ne => Ok((lhs != rhs) as i64),
                    _ => unreachable!("only equality operators use eval_equality"),
                }
            }
        }
    }

    fn eval_struct_expr(&mut self, expr: &Expr) -> CustResult<ReturnValue> {
        match expr {
            Expr::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                if self.eval_truthy(cond)? {
                    self.eval_struct_expr(then_expr)
                } else {
                    self.eval_struct_expr(else_expr)
                }
            }
            Expr::Comma(left, right) => {
                self.eval_discard(left)?;
                self.eval_struct_expr(right)
            }
            Expr::Assign { name, value } => self.eval_struct_assignment_expr(name, value),
            Expr::DerefSet { pointer, value } => {
                self.eval_struct_pointer_assignment_expr(pointer, value)
            }
            Expr::AggregateLiteral { type_name, init } => Ok(ReturnValue::Struct {
                type_name: type_name.clone(),
                fields: self.make_struct_fields(type_name, init)?,
            }),
            Expr::AggregateFieldGet {
                aggregate,
                fields: path,
            } => match self.eval_struct_expr(aggregate)? {
                ReturnValue::Struct { type_name, fields } => {
                    let (_, field_value) = Self::nested_field_value(&type_name, &fields, path)?;
                    match field_value {
                        StructFieldValue::Struct {
                            type_name, fields, ..
                        } => Ok(ReturnValue::Struct {
                            type_name: type_name.clone(),
                            fields: StructFieldValue::deep_clone_fields(fields),
                        }),
                        StructFieldValue::Scalar { .. }
                        | StructFieldValue::Array { .. }
                        | StructFieldValue::StructArray { .. }
                        | StructFieldValue::Pointer { .. } => {
                            Err(CustError::new("expected struct expression"))
                        }
                    }
                }
                _ => Err(CustError::new("expected struct expression")),
            },
            Expr::Var(name) => match self.find_variable(name).cloned() {
                Some(Value::Struct { type_name, fields }) => Ok(ReturnValue::Struct {
                    type_name,
                    fields: StructFieldValue::deep_clone_fields(&fields),
                }),
                Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::StructGet { name, fields: path } => match self.find_variable(name).cloned() {
                Some(Value::Struct { type_name, fields }) => {
                    let (_, field_value) = Self::nested_field_value(&type_name, &fields, path)?;
                    match field_value {
                        StructFieldValue::Struct {
                            type_name, fields, ..
                        } => Ok(ReturnValue::Struct {
                            type_name: type_name.clone(),
                            fields: StructFieldValue::deep_clone_fields(fields),
                        }),
                        StructFieldValue::Scalar { .. }
                        | StructFieldValue::Array { .. }
                        | StructFieldValue::StructArray { .. }
                        | StructFieldValue::Pointer { .. } => {
                            Err(CustError::new("expected struct expression"))
                        }
                    }
                }
                Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::ArrayGet { name, index } => {
                if let Some((type_name, fields)) = self.indexed_struct_pointer_value(name, index)? {
                    Ok(ReturnValue::Struct { type_name, fields })
                } else {
                    let index = self.checked_struct_element_index(name, index)?;
                    match self.find_variable(name).cloned() {
                        Some(Value::StructArray {
                            type_name,
                            elements,
                            ..
                        }) => Ok(ReturnValue::Struct {
                            type_name,
                            fields: StructFieldValue::deep_clone_fields(&elements[index]),
                        }),
                        Some(_) => Err(CustError::new(format!(
                            "variable '{name}' is not a struct array"
                        ))),
                        None => Err(CustError::new(format!("undefined variable '{name}'"))),
                    }
                }
            }
            Expr::Deref(pointer) => {
                let pointer = self.eval_pointer(pointer)?;
                let (type_name, fields) = self.find_struct_pointer_fields(&pointer)?;
                Ok(ReturnValue::Struct {
                    type_name,
                    fields: StructFieldValue::deep_clone_fields(fields),
                })
            }
            Expr::Call { name, args } => match self.call_function(name, args)? {
                Some(ReturnValue::Struct { type_name, fields }) => {
                    Ok(ReturnValue::Struct { type_name, fields })
                }
                Some(ReturnValue::Scalar(_)) => Err(CustError::new(format!(
                    "scalar function '{name}' used as struct expression"
                ))),
                Some(ReturnValue::Pointer { .. }) => Err(CustError::new(format!(
                    "pointer function '{name}' used as struct expression"
                ))),
                None => Err(CustError::new(format!(
                    "void function '{name}' used as struct expression"
                ))),
            },
            _ => Err(CustError::new("expected struct expression")),
        }
    }

    fn eval_return_value(&mut self, expr: Option<&Expr>) -> CustResult<Option<ReturnValue>> {
        let Some(expr) = expr else {
            return Ok(None);
        };
        match self.return_type_stack.last().cloned() {
            Some(ReturnType::Struct(_)) => Ok(Some(self.eval_struct_expr(expr)?)),
            Some(ReturnType::Pointer {
                ty,
                points_to_const,
            }) => {
                self.ensure_pointer_conversion_preserves_const(points_to_const, expr)?;
                let pointer = self.eval_pointer(expr)?;
                self.ensure_pointer_type_matches(&ty, &pointer)?;
                Ok(Some(ReturnValue::Pointer {
                    pointer,
                    ty,
                    points_to_const,
                }))
            }
            Some(ReturnType::Scalar(_)) | Some(ReturnType::Void) => {
                Ok(Some(ReturnValue::Scalar(self.eval(expr)?)))
            }
            None => Ok(Some(ReturnValue::Scalar(self.eval(expr)?))),
        }
    }

    fn static_local_name_and_const(decl: &Stmt) -> CustResult<(&str, bool)> {
        match decl {
            Stmt::VarDecl { name, is_const, .. }
            | Stmt::PointerDecl { name, is_const, .. }
            | Stmt::ArrayDecl { name, is_const, .. }
            | Stmt::StructVarDecl { name, is_const, .. }
            | Stmt::StructArrayDecl { name, is_const, .. } => Ok((name, *is_const)),
            _ => Err(CustError::new(
                "static local declarations must declare variables",
            )),
        }
    }

    fn initialize_static_local(&mut self, decl: &Stmt) -> CustResult<Value> {
        match decl {
            Stmt::VarDecl { ty, expr, .. } => Ok(Value::Scalar {
                value: self.eval(expr)?,
                ty: *ty,
            }),
            Stmt::PointerDecl {
                ty,
                expr,
                points_to_const,
                ..
            } => {
                self.ensure_pointer_conversion_preserves_const(*points_to_const, expr)?;
                let pointer = self.eval_pointer(expr)?;
                self.ensure_pointer_type_matches(ty, &pointer)?;
                Ok(Value::Pointer {
                    pointer,
                    ty: ty.clone(),
                    points_to_const: *points_to_const,
                })
            }
            Stmt::ArrayDecl {
                elem_type,
                len,
                init,
                is_const,
                ..
            } => self.make_array_value(*len, *elem_type, init, *is_const),
            Stmt::StructVarDecl {
                type_name, init, ..
            } => self.make_struct_value(type_name, init.as_ref()),
            Stmt::StructArrayDecl {
                type_name,
                len,
                init,
                is_const,
                ..
            } => self.make_struct_array_value(type_name, *len, init, *is_const),
            _ => Err(CustError::new(
                "static local declarations must declare variables",
            )),
        }
    }

    fn exec_static_local(&mut self, id: usize, decl: &Stmt) -> CustResult<ExecFlow> {
        let (name, is_const) = Self::static_local_name_and_const(decl)?;
        if self.current_scope_has_identifier(name) {
            return Err(CustError::new(format!(
                "variable '{name}' already declared in this scope"
            )));
        }
        if !self.static_locals.contains_key(&id) {
            let value = self.initialize_static_local(decl)?;
            let scope_id = self.next_scope_id;
            self.next_scope_id += 1;
            self.live_scope_ids.insert(scope_id);
            self.static_locals.insert(
                id,
                StaticLocalStorage {
                    scope_id,
                    name: name.to_string(),
                    value,
                    is_const,
                },
            );
        }
        self.scopes
            .last_mut()
            .expect("exec_block always creates a current scope")
            .static_local_ids
            .insert(name.to_string(), id);
        Ok(ExecFlow::None)
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> CustResult<ExecFlow> {
        match stmt {
            Stmt::Empty => Ok(ExecFlow::None),
            Stmt::StaticLocal { id, decl } => self.exec_static_local(*id, decl),
            Stmt::VarDecl {
                name,
                ty,
                expr,
                is_const,
            } => {
                let value = self.eval(expr)?;
                if self.current_scope_has_identifier(name) {
                    return Err(CustError::new(format!(
                        "variable '{name}' already declared in this scope"
                    )));
                }
                let scope = self.current_scope_mut();
                scope.insert(name.clone(), Value::Scalar { value, ty: *ty });
                if *is_const {
                    self.mark_current_variable_const(name);
                }
                Ok(ExecFlow::None)
            }
            Stmt::PointerDecl {
                name,
                ty,
                expr,
                is_const,
                points_to_const,
            } => {
                self.ensure_pointer_conversion_preserves_const(*points_to_const, expr)?;
                let pointer = self.eval_pointer(expr)?;
                self.ensure_pointer_type_matches(ty, &pointer)?;
                if self.current_scope_has_identifier(name) {
                    return Err(CustError::new(format!(
                        "variable '{name}' already declared in this scope"
                    )));
                }
                let scope = self.current_scope_mut();
                scope.insert(
                    name.clone(),
                    Value::Pointer {
                        pointer,
                        ty: ty.clone(),
                        points_to_const: *points_to_const,
                    },
                );
                if *is_const {
                    self.mark_current_variable_const(name);
                }
                Ok(ExecFlow::None)
            }
            Stmt::ArrayDecl {
                name,
                elem_type,
                len,
                init,
                is_const,
            } => {
                if self.current_scope_has_identifier(name) {
                    return Err(CustError::new(format!(
                        "variable '{name}' already declared in this scope"
                    )));
                }
                let value = self.make_array_value(*len, *elem_type, init, *is_const)?;
                self.current_scope_mut().insert(name.clone(), value);
                if *is_const {
                    self.mark_current_variable_const(name);
                }
                Ok(ExecFlow::None)
            }
            Stmt::StructVarDecl {
                type_name,
                name,
                init,
                is_const,
            } => {
                if self.current_scope_has_identifier(name) {
                    return Err(CustError::new(format!(
                        "variable '{name}' already declared in this scope"
                    )));
                }
                let value = self.make_struct_value(type_name, init.as_ref())?;
                self.current_scope_mut().insert(name.clone(), value);
                if *is_const {
                    self.mark_current_variable_const(name);
                }
                Ok(ExecFlow::None)
            }
            Stmt::StructArrayDecl {
                type_name,
                name,
                len,
                init,
                is_const,
            } => {
                if self.current_scope_has_identifier(name) {
                    return Err(CustError::new(format!(
                        "variable '{name}' already declared in this scope"
                    )));
                }
                let value = self.make_struct_array_value(type_name, *len, init, *is_const)?;
                self.current_scope_mut().insert(name.clone(), value);
                if *is_const {
                    self.mark_current_variable_const(name);
                }
                Ok(ExecFlow::None)
            }
            Stmt::EnumDecl { constants } => {
                for constant in constants {
                    self.insert_enum_constant(constant.name.clone(), constant.value)?;
                }
                Ok(ExecFlow::None)
            }
            Stmt::StaticAssert { condition, message } => {
                if self.eval_truthy(condition)? {
                    Ok(ExecFlow::None)
                } else {
                    Err(CustError::new(format!(
                        "static assertion failed: {message}"
                    )))
                }
            }
            Stmt::Assign(name, expr) => {
                let existing = self.find_variable(name).cloned();
                match existing {
                    Some(Value::Scalar { .. }) => {
                        self.ensure_variable_mutable(name)?;
                        let value = self.eval(expr)?;
                        if let Some(Value::Scalar { value: slot, .. }) =
                            self.find_variable_mut(name)
                        {
                            *slot = value;
                        }
                        Ok(ExecFlow::None)
                    }
                    Some(Value::Pointer {
                        ty,
                        points_to_const,
                        ..
                    }) => {
                        self.ensure_variable_mutable(name)?;
                        self.ensure_pointer_conversion_preserves_const(points_to_const, expr)?;
                        let pointer = self.eval_pointer(expr)?;
                        self.ensure_pointer_type_matches(&ty, &pointer)?;
                        if let Some(Value::Pointer { pointer: slot, .. }) =
                            self.find_variable_mut(name)
                        {
                            *slot = pointer;
                        }
                        Ok(ExecFlow::None)
                    }
                    Some(Value::Array(_)) => Err(CustError::new(format!(
                        "array '{name}' requires indexed assignment"
                    ))),
                    Some(Value::Struct { .. }) => {
                        self.assign_struct_copy(name, expr)?;
                        Ok(ExecFlow::None)
                    }
                    Some(Value::StructArray { .. }) => Err(CustError::new(format!(
                        "struct array '{name}' assignment is not supported"
                    ))),
                    None if self.find_enum_constant(name).is_some() => Err(CustError::new(
                        format!("cannot assign to enum constant '{name}'"),
                    )),
                    None => Err(CustError::new(format!(
                        "assignment to undeclared variable '{name}'"
                    ))),
                }
            }
            Stmt::DerefAssign { pointer, value } => {
                if match pointer {
                    Expr::Var(name) => matches!(
                        self.find_variable(name),
                        Some(Value::Pointer {
                            ty: PointeeType::Struct(_),
                            ..
                        })
                    ),
                    _ => false,
                } {
                    self.assign_struct_pointer_copy(pointer, value)?;
                    return Ok(ExecFlow::None);
                }
                self.ensure_pointer_expr_pointee_mutable(pointer)?;
                let pointer = self.eval_pointer(pointer)?;
                let value = self.eval(value)?;
                self.assign_deref_pointer(&pointer, value)?;
                Ok(ExecFlow::None)
            }
            Stmt::ArrayAssign { name, index, value } => {
                let value = self.eval(value)?;
                match self.find_variable(name).cloned() {
                    Some(Value::Pointer { pointer, .. }) => {
                        self.ensure_pointer_variable_pointee_mutable(name)?;
                        let index_value = self.eval(index)?;
                        self.assign_pointer_index(&pointer, index_value, value)?;
                    }
                    Some(_) | None => {
                        let (array, index) = self.checked_array_index(name, index)?;
                        let mut array = array.borrow_mut();
                        if array.read_only {
                            return Err(CustError::new(format!(
                                "cannot modify read-only array '{name}'"
                            )));
                        }
                        array.elements[index] = value;
                    }
                }
                Ok(ExecFlow::None)
            }
            Stmt::StructAssign {
                name,
                fields,
                value,
            } => {
                if self.struct_field_is_pointer(name, fields) {
                    self.ensure_pointer_conversion_preserves_const(
                        self.struct_pointer_field_points_to_const(name, fields),
                        value,
                    )?;
                    let pointer = self.eval_pointer(value)?;
                    self.assign_direct_struct_pointer_field(name, fields, pointer)?;
                } else {
                    let value = self.eval(value)?;
                    self.assign_struct_field(name, fields, value)?;
                }
                Ok(ExecFlow::None)
            }
            Stmt::Expr(expr) => {
                self.eval_discard(expr)?;
                Ok(ExecFlow::None)
            }
            Stmt::Return(expr) => Ok(ExecFlow::Return(self.eval_return_value(expr.as_ref())?)),
            Stmt::Break => Ok(ExecFlow::Break),
            Stmt::Continue => Ok(ExecFlow::Continue),
            Stmt::Block(statements) => self.exec_block(statements),
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                if self.eval_truthy(cond)? {
                    self.exec_control_body(then_branch)
                } else {
                    self.exec_control_body(else_branch)
                }
            }
            Stmt::While { cond, body } => {
                let mut iterations = 0usize;
                while self.eval_truthy(cond)? {
                    self.consume_loop_iteration()?;
                    iterations += 1;
                    if iterations > 1_000_000 {
                        return Err(CustError::new("loop iteration limit exceeded"));
                    }
                    match self.exec_control_body(body)? {
                        ExecFlow::None | ExecFlow::Continue => {}
                        ExecFlow::Break => break,
                        ExecFlow::Return(value) => return Ok(ExecFlow::Return(value)),
                    }
                }
                Ok(ExecFlow::None)
            }
            Stmt::DoWhile { body, cond } => self.exec_do_while(body, cond),
            Stmt::For {
                init,
                cond,
                increment,
                body,
            } => self.exec_for(init.as_deref(), cond.as_ref(), increment.as_deref(), body),
            Stmt::Switch { expr, sections } => self.exec_switch(expr, sections),
        }
    }

    fn exec_switch(&mut self, expr: &Expr, sections: &[SwitchSection]) -> CustResult<ExecFlow> {
        let value = self.eval(expr)?;
        let default_index = sections
            .iter()
            .position(|section| matches!(section.label, SwitchLabel::Default));
        let start_index = sections
            .iter()
            .position(|section| matches!(section.label, SwitchLabel::Case(case_value) if case_value == value))
            .or(default_index);

        let Some(start_index) = start_index else {
            return Ok(ExecFlow::None);
        };

        self.push_scope();
        for section in &sections[start_index..] {
            for stmt in &section.statements {
                match self.exec_stmt(stmt) {
                    Ok(ExecFlow::None) => {}
                    Ok(ExecFlow::Break) => {
                        self.pop_scope();
                        return Ok(ExecFlow::None);
                    }
                    Ok(flow) => {
                        self.pop_scope();
                        return Ok(flow);
                    }
                    Err(error) => {
                        self.pop_scope();
                        return Err(error);
                    }
                }
            }
        }
        self.pop_scope();
        Ok(ExecFlow::None)
    }

    fn exec_for(
        &mut self,
        init: Option<&Stmt>,
        cond: Option<&Expr>,
        increment: Option<&Stmt>,
        body: &[Stmt],
    ) -> CustResult<ExecFlow> {
        self.push_scope();
        let result = self.exec_for_in_current_scope(init, cond, increment, body);
        self.pop_scope();
        result
    }

    fn exec_do_while(&mut self, body: &[Stmt], cond: &Expr) -> CustResult<ExecFlow> {
        let mut iterations = 0usize;
        loop {
            self.consume_loop_iteration()?;
            iterations += 1;
            if iterations > 1_000_000 {
                return Err(CustError::new("loop iteration limit exceeded"));
            }

            match self.exec_control_body(body)? {
                ExecFlow::None | ExecFlow::Continue => {}
                ExecFlow::Break => break,
                ExecFlow::Return(value) => return Ok(ExecFlow::Return(value)),
            }

            if !self.eval_truthy(cond)? {
                break;
            }
        }

        Ok(ExecFlow::None)
    }

    fn exec_for_in_current_scope(
        &mut self,
        init: Option<&Stmt>,
        cond: Option<&Expr>,
        increment: Option<&Stmt>,
        body: &[Stmt],
    ) -> CustResult<ExecFlow> {
        if let Some(init) = init {
            match self.exec_stmt(init)? {
                ExecFlow::None => {}
                ExecFlow::Return(value) => return Ok(ExecFlow::Return(value)),
                ExecFlow::Break => return Err(CustError::new("break outside loop")),
                ExecFlow::Continue => return Err(CustError::new("continue outside loop")),
            }
        }

        let mut iterations = 0usize;
        loop {
            match cond {
                Some(cond) if !self.eval_truthy(cond)? => break,
                Some(_) | None => {}
            }

            self.consume_loop_iteration()?;
            iterations += 1;
            if iterations > 1_000_000 {
                return Err(CustError::new("loop iteration limit exceeded"));
            }

            match self.exec_control_body(body)? {
                ExecFlow::None | ExecFlow::Continue => {}
                ExecFlow::Break => break,
                ExecFlow::Return(value) => return Ok(ExecFlow::Return(value)),
            }

            if let Some(increment) = increment {
                match self.exec_stmt(increment)? {
                    ExecFlow::None => {}
                    ExecFlow::Return(value) => return Ok(ExecFlow::Return(value)),
                    ExecFlow::Break => return Err(CustError::new("break outside loop")),
                    ExecFlow::Continue => return Err(CustError::new("continue outside loop")),
                }
            }
        }

        Ok(ExecFlow::None)
    }

    fn eval(&mut self, expr: &Expr) -> CustResult<i64> {
        match expr {
            Expr::Number(value) => Ok(*value),
            Expr::StringLiteral(_) => Err(CustError::new("string literal used as scalar")),
            Expr::ArrayLiteral { .. } | Expr::AggregateArrayLiteral { .. } => {
                Err(CustError::new("pointer value used as scalar"))
            }
            Expr::StructGet { name, fields } => self.read_struct_field(name, fields),
            Expr::StructArrayGet {
                name,
                fields,
                index,
            } => {
                let (array, index) = self.checked_struct_array_index(name, fields, index)?;
                Ok(array.borrow().elements[index])
            }
            Expr::StructFieldArrayElementGet {
                name,
                array_fields,
                index,
                fields,
            } => self.read_struct_field_array_element_field(name, array_fields, index, fields),
            Expr::StructFieldArrayElementSet {
                name,
                array_fields,
                index,
                fields,
                value,
            } => {
                let value = self.eval(value)?;
                self.assign_struct_field_array_element_field(
                    name,
                    array_fields,
                    index,
                    fields,
                    value,
                )?;
                Ok(value)
            }
            Expr::StructFieldArrayElementCompoundSet {
                name,
                array_fields,
                index,
                fields,
                op,
                value,
            } => {
                let current =
                    self.read_struct_field_array_element_field(name, array_fields, index, fields)?;
                let rhs = self.eval(value)?;
                let result = Self::apply_compound_op(current, *op, rhs)?;
                self.assign_struct_field_array_element_field(
                    name,
                    array_fields,
                    index,
                    fields,
                    result,
                )?;
                Ok(result)
            }
            Expr::StructElementGet {
                name,
                index,
                fields,
            } => self.read_struct_element_field(name, index, fields),
            Expr::StructElementArrayGet {
                name,
                index,
                fields,
                array_index,
            } => {
                let (array, index) =
                    self.checked_struct_element_array_index(name, index, fields, array_index)?;
                Ok(array.borrow().elements[index])
            }
            Expr::StructPtrArrayGet {
                pointer,
                fields,
                index,
            } => {
                let pointer = self.eval_pointer(pointer)?;
                let (array, index) =
                    self.checked_struct_pointer_array_index(&pointer, fields, index)?;
                Ok(array.borrow().elements[index])
            }
            Expr::StructPtrGet { pointer, fields } => {
                let pointer = self.eval_pointer(pointer)?;
                self.read_struct_pointer_field(&pointer, fields)
            }
            Expr::SizeOfType(sizeof_type) => Ok(sizeof_type.size(&self.struct_types)?),
            Expr::SizeOfValue(expr) => self.sizeof_expr(expr),
            Expr::AlignOfType(alignof_type) => Ok(alignof_type.alignment(&self.struct_types)?),
            Expr::Var(name) => self.find_scalar(name),
            Expr::AddressOf(_)
            | Expr::AddressOfArray { .. }
            | Expr::AddressOfStructField { .. }
            | Expr::AddressOfStructElementField { .. }
            | Expr::AddressOfStructArrayField { .. }
            | Expr::AddressOfStructElementArrayField { .. }
            | Expr::AddressOfStructPtrField { .. }
            | Expr::AddressOfStructPtrArrayField { .. }
            | Expr::AddressOfScalarLiteral { .. }
            | Expr::AddressOfAggregateLiteral { .. }
            | Expr::AddressOfAggregateField { .. } => {
                Err(CustError::new("pointer value used as scalar"))
            }
            Expr::Assign { name, value } => self.eval_assignment_expr(name, value),
            Expr::CompoundAssign { name, op, value } => {
                self.eval_compound_assignment_expr(name, *op, value)
            }
            Expr::ScalarLiteralSet { init, value, .. } => {
                self.eval(init)?;
                self.eval(value)
            }
            Expr::ScalarLiteralCompoundSet {
                init, op, value, ..
            } => {
                let current = self.eval(init)?;
                let rhs = self.eval(value)?;
                Self::apply_compound_op(current, *op, rhs)
            }
            Expr::AggregateFieldSet {
                aggregate,
                fields,
                value,
            } => {
                let (_, is_const) = self.eval_aggregate_literal_field_scalar(aggregate, fields)?;
                if is_const {
                    return Err(CustError::new(format!(
                        "cannot assign to const struct field '{}'",
                        Self::field_path_label(fields)
                    )));
                }
                self.eval(value)
            }
            Expr::AggregateFieldCompoundSet {
                aggregate,
                fields,
                op,
                value,
            } => {
                let (current, is_const) =
                    self.eval_aggregate_literal_field_scalar(aggregate, fields)?;
                if is_const {
                    return Err(CustError::new(format!(
                        "cannot assign to const struct field '{}'",
                        Self::field_path_label(fields)
                    )));
                }
                let rhs = self.eval(value)?;
                Self::apply_compound_op(current, *op, rhs)
            }
            Expr::Increment { target, op, prefix } => {
                self.eval_increment_expr(target, *op, *prefix)
            }
            Expr::ArraySet { name, index, value } => {
                let value = self.eval(value)?;
                match self.find_variable(name).cloned() {
                    Some(Value::Pointer { pointer, .. }) => {
                        self.ensure_pointer_variable_pointee_mutable(name)?;
                        let index_value = self.eval(index)?;
                        self.assign_pointer_index(&pointer, index_value, value)?;
                    }
                    Some(_) | None => {
                        let (array, index) = self.checked_array_index(name, index)?;
                        let mut array = array.borrow_mut();
                        if array.read_only {
                            return Err(CustError::new(format!(
                                "cannot modify read-only array '{name}'"
                            )));
                        }
                        array.elements[index] = value;
                    }
                }
                Ok(value)
            }
            Expr::ArrayCompoundSet {
                name,
                index,
                op,
                value,
            } => self.eval_array_compound_set(name, index, *op, value),
            Expr::StructArraySet {
                name,
                fields,
                index,
                value,
            } => self.eval_struct_array_set(name, fields, index, value),
            Expr::StructElementSet {
                name,
                index,
                fields,
                value,
            } => {
                let value = self.eval(value)?;
                self.assign_struct_element_field(name, index, fields, value)?;
                Ok(value)
            }
            Expr::StructElementArraySet {
                name,
                index,
                fields,
                array_index,
                value,
            } => {
                let value = self.eval(value)?;
                let (array, index) =
                    self.checked_struct_element_array_index(name, index, fields, array_index)?;
                let mut array = array.borrow_mut();
                if array.read_only {
                    return Err(CustError::new(format!(
                        "cannot modify read-only array '{}'",
                        Self::field_path_label(fields)
                    )));
                }
                array.elements[index] = value;
                Ok(value)
            }
            Expr::StructArrayCompoundSet {
                name,
                fields,
                index,
                op,
                value,
            } => self.eval_struct_array_compound_set(name, fields, index, *op, value),
            Expr::StructElementCompoundSet {
                name,
                index,
                fields,
                op,
                value,
            } => {
                let current = self.read_struct_element_field(name, index, fields)?;
                let rhs = self.eval(value)?;
                let result = Self::apply_compound_op(current, *op, rhs)?;
                self.assign_struct_element_field(name, index, fields, result)?;
                Ok(result)
            }
            Expr::StructElementArrayCompoundSet {
                name,
                index,
                fields,
                array_index,
                op,
                value,
            } => {
                let (array, index) =
                    self.checked_struct_element_array_index(name, index, fields, array_index)?;
                let current = array.borrow().elements[index];
                let rhs = self.eval(value)?;
                let result = Self::apply_compound_op(current, *op, rhs)?;
                let mut array = array.borrow_mut();
                if array.read_only {
                    return Err(CustError::new(format!(
                        "cannot modify read-only array '{}'",
                        Self::field_path_label(fields)
                    )));
                }
                array.elements[index] = result;
                Ok(result)
            }
            Expr::DerefSet { pointer, value } => {
                self.ensure_pointer_expr_pointee_mutable(pointer)?;
                let pointer = self.eval_pointer(pointer)?;
                let value = self.eval(value)?;
                self.assign_deref_pointer(&pointer, value)?;
                Ok(value)
            }
            Expr::DerefCompoundSet { pointer, op, value } => {
                self.eval_deref_compound_set(pointer, *op, value)
            }
            Expr::StructSet {
                name,
                fields,
                value,
            } => self.eval_struct_set(name, fields, value),
            Expr::StructPtrSet {
                pointer,
                fields,
                value,
            } => self.eval_struct_ptr_set(pointer, fields, value),
            Expr::StructCompoundSet {
                name,
                fields,
                op,
                value,
            } => self.eval_struct_compound_set(name, fields, *op, value),
            Expr::StructPtrCompoundSet {
                pointer,
                fields,
                op,
                value,
            } => self.eval_struct_ptr_compound_set(pointer, fields, *op, value),
            Expr::Deref(pointer) => {
                let pointer = self.eval_pointer(pointer)?;
                self.deref_pointer(&pointer)
            }
            Expr::ArrayGet { name, index } => match self.find_variable(name).cloned() {
                Some(Value::Pointer { .. }) => {
                    let (array, _, index) = self.checked_pointer_index(name, index)?;
                    Ok(array.borrow().elements[index])
                }
                Some(_) | None => {
                    let (array, index) = self.checked_array_index(name, index)?;
                    Ok(array.borrow().elements[index])
                }
            },
            Expr::StringGet { values, index } => {
                let index_value = self.eval(index)?;
                let Ok(index) = usize::try_from(index_value) else {
                    return Err(CustError::new(format!(
                        "string literal index {index_value} out of bounds for length {}",
                        values.len()
                    )));
                };
                values.get(index).copied().ok_or_else(|| {
                    CustError::new(format!(
                        "string literal index {index_value} out of bounds for length {}",
                        values.len()
                    ))
                })
            }
            Expr::Call { name, args } => match self.call_function(name, args)? {
                Some(ReturnValue::Scalar(value)) => Ok(value),
                Some(ReturnValue::Pointer { .. }) => Err(CustError::new(format!(
                    "pointer function '{name}' used as scalar expression"
                ))),
                Some(ReturnValue::Struct { type_name, .. }) => {
                    let aggregate_kind = self.aggregate_kind_label(&type_name);
                    Err(CustError::new(format!(
                        "{aggregate_kind} function '{name}' used as scalar expression"
                    )))
                }
                None => Err(CustError::new(format!(
                    "void function '{name}' used as scalar expression"
                ))),
            },
            Expr::Cast { expr, .. } => self.eval(expr),
            Expr::ScalarLiteral { init, .. } => self.eval(init),
            Expr::AggregateLiteral { type_name, .. } => {
                let aggregate_kind = self.aggregate_kind_label(type_name);
                Err(CustError::new(format!(
                    "{aggregate_kind} value used as scalar"
                )))
            }
            Expr::AggregateFieldGet {
                aggregate,
                fields: path,
            } => self
                .eval_aggregate_literal_field_scalar(aggregate, path)
                .map(|(value, _)| value),
            Expr::UnaryPlus(inner) => self.eval(inner),
            Expr::UnaryMinus(inner) => Ok(-self.eval(inner)?),
            Expr::BitwiseNot(inner) => Ok(!self.eval(inner)?),
            Expr::LogicalNot(inner) => Ok((!self.eval_truthy(inner)?) as i64),
            Expr::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                if self.eval_truthy(cond)? {
                    self.eval(then_expr)
                } else {
                    self.eval(else_expr)
                }
            }
            Expr::Comma(left, right) => {
                self.eval_discard(left)?;
                self.eval(right)
            }
            Expr::Binary(left, op, right) => match op {
                BinaryOp::LogicalAnd => {
                    if !self.eval_truthy(left)? {
                        Ok(0)
                    } else {
                        Ok(self.eval_truthy(right)? as i64)
                    }
                }
                BinaryOp::LogicalOr => {
                    if self.eval_truthy(left)? {
                        Ok(1)
                    } else {
                        Ok(self.eval_truthy(right)? as i64)
                    }
                }
                BinaryOp::Eq | BinaryOp::Ne => self.eval_equality(left, op, right),
                BinaryOp::Add | BinaryOp::Sub => {
                    if self.expr_is_pointer_value(left) || self.expr_is_pointer_value(right) {
                        match (self.eval_pointer(left), self.eval_pointer(right)) {
                            (Ok(left_pointer), Ok(right_pointer)) if *op == BinaryOp::Sub => {
                                return self.pointer_difference(&left_pointer, &right_pointer);
                            }
                            (Ok(_), Ok(_)) if *op == BinaryOp::Add => {
                                return Err(CustError::new("cannot add two pointers"));
                            }
                            (Ok(_), Err(_)) | (Err(_), Ok(_)) => {
                                return Err(CustError::new("pointer value used as scalar"));
                            }
                            (Err(error), Err(_)) => return Err(error),
                            (Ok(_), Ok(_)) => {}
                        }
                    }
                    let lhs = self.eval(left)?;
                    let rhs = self.eval(right)?;
                    match op {
                        BinaryOp::Add => Ok(lhs + rhs),
                        BinaryOp::Sub => Ok(lhs - rhs),
                        _ => unreachable!("only addition/subtraction handled in this branch"),
                    }
                }
                BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                    if self.expr_is_pointer_value(left) || self.expr_is_pointer_value(right) {
                        match (self.eval_pointer(left), self.eval_pointer(right)) {
                            (Ok(left_pointer), Ok(right_pointer)) => {
                                return self.pointer_ordering(&left_pointer, *op, &right_pointer);
                            }
                            (Ok(_), Err(_)) | (Err(_), Ok(_)) => {
                                return Err(CustError::new(
                                    "pointer ordering comparisons are not supported",
                                ));
                            }
                            (Err(error), Err(_)) => return Err(error),
                        }
                    }
                    let lhs = self.eval(left)?;
                    let rhs = self.eval(right)?;
                    match op {
                        BinaryOp::Lt => Ok((lhs < rhs) as i64),
                        BinaryOp::Le => Ok((lhs <= rhs) as i64),
                        BinaryOp::Gt => Ok((lhs > rhs) as i64),
                        BinaryOp::Ge => Ok((lhs >= rhs) as i64),
                        _ => unreachable!("only ordering operators handled in this branch"),
                    }
                }
                BinaryOp::ShiftLeft
                | BinaryOp::ShiftRight
                | BinaryOp::BitAnd
                | BinaryOp::BitXor
                | BinaryOp::BitOr => {
                    if self.expr_is_pointer_value(left) || self.expr_is_pointer_value(right) {
                        return Err(CustError::new(
                            "pointer bitwise operations are not supported",
                        ));
                    }
                    let lhs = self.eval(left)?;
                    let rhs = self.eval(right)?;
                    match op {
                        BinaryOp::BitAnd => Ok(lhs & rhs),
                        BinaryOp::BitXor => Ok(lhs ^ rhs),
                        BinaryOp::BitOr => Ok(lhs | rhs),
                        BinaryOp::ShiftLeft => Self::checked_shift_left(lhs, rhs),
                        BinaryOp::ShiftRight => Self::checked_shift_right(lhs, rhs),
                        _ => unreachable!("only bitwise operators handled in this branch"),
                    }
                }
                BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem => {
                    let lhs = self.eval(left)?;
                    let rhs = self.eval(right)?;
                    match op {
                        BinaryOp::Mul => Ok(lhs * rhs),
                        BinaryOp::Div if rhs == 0 => Err(CustError::new("division by zero")),
                        BinaryOp::Div => Ok(lhs / rhs),
                        BinaryOp::Rem if rhs == 0 => Err(CustError::new("division by zero")),
                        BinaryOp::Rem => Ok(lhs % rhs),
                        _ => unreachable!("only multiplicative operators handled in this branch"),
                    }
                }
            },
        }
    }
}
