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
    Const,
    Static,
    Void,
    Enum,
    Struct,
    Typedef,
    Sizeof,
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
    Var(String),
    StructGet {
        name: String,
        field: String,
    },
    StructPtrGet {
        pointer: Box<Expr>,
        field: String,
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
        field: String,
        value: Box<Expr>,
    },
    StructPtrSet {
        pointer: Box<Expr>,
        field: String,
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
        field: String,
        op: CompoundOp,
        value: Box<Expr>,
    },
    StructPtrCompoundSet {
        pointer: Box<Expr>,
        field: String,
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
    Pointer(PointeeType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DeclType {
    Scalar(CType),
    Struct(String),
    Pointer(PointeeType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructTypeDef {
    fields: Vec<StructFieldDef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructFieldDef {
    name: String,
    ty: CType,
    is_const: bool,
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
    Struct(String),
    Void,
}

impl ReturnType {
    fn value_return_label(&self) -> &'static str {
        match self {
            ReturnType::Scalar(CType::Int) => "int",
            ReturnType::Scalar(CType::Char) => "char",
            ReturnType::Struct(_) => "struct",
            ReturnType::Void => "void",
        }
    }

    fn size(&self, struct_types: &HashMap<String, StructTypeDef>) -> Option<i64> {
        match self {
            ReturnType::Scalar(ty) => Some(ty.size()),
            ReturnType::Struct(type_name) => struct_types
                .get(type_name)
                .map(|struct_type| struct_type.fields.iter().map(|field| field.ty.size()).sum()),
            ReturnType::Void => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CType {
    Int,
    Char,
}

impl CType {
    fn size(self) -> i64 {
        match self {
            CType::Int => INT_SIZE,
            CType::Char => CHAR_SIZE,
        }
    }
}

impl PointeeType {
    fn size(&self, struct_types: &HashMap<String, StructTypeDef>) -> CustResult<i64> {
        match self {
            PointeeType::Scalar(ty) => Ok(ty.size()),
            PointeeType::Struct(type_name) => struct_types
                .get(type_name)
                .map(|struct_type| struct_type.fields.iter().map(|field| field.ty.size()).sum())
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SizeOfType {
    Scalar(CType),
    Struct(String),
    Pointer,
}

impl SizeOfType {
    fn size(&self, struct_types: &HashMap<String, StructTypeDef>) -> CustResult<i64> {
        match self {
            SizeOfType::Scalar(ty) => Ok(ty.size()),
            SizeOfType::Struct(type_name) => struct_types
                .get(type_name)
                .map(|struct_type| struct_type.fields.iter().map(|field| field.ty.size()).sum())
                .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'"))),
            SizeOfType::Pointer => Ok(POINTER_SIZE),
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
    Array(usize),
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
        is_const: bool,
    },
    StructVarDecl {
        type_name: String,
        name: String,
        is_const: bool,
    },
    EnumDecl {
        constants: Vec<EnumConstant>,
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
        field: String,
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
/// - `int main() { ... }`
/// - top-level `int`/`char` scalar, array, and pointer globals initialized before `main()`
/// - `int name(int param, char param, struct Point param, ...) { ... }` function definitions and calls, including bounded recursion
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
                while i < chars.len() && chars[i].is_ascii_digit() {
                    advance_position(chars[i], &mut line, &mut column, &mut i);
                }
                let text: String = chars[start..i].iter().collect();
                let value = text.parse::<i64>().map_err(|_| {
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
                            let Some(escape_char) = chars.get(i).copied() else {
                                return Err(lexer_error_with_context(
                                    "unterminated string literal",
                                    source,
                                    start_line,
                                    start_column,
                                ));
                            };
                            let escaped = match escape_char {
                                'n' => '\n',
                                't' => '\t',
                                '0' => '\0',
                                '\\' => '\\',
                                '\'' => '\'',
                                '"' => '"',
                                other => {
                                    return Err(lexer_error_with_context(
                                        format!("unsupported string escape '\\{other}'"),
                                        source,
                                        start_line,
                                        start_column,
                                    ));
                                }
                            };
                            advance_position(escape_char, &mut line, &mut column, &mut i);
                            values.push(escaped as i64);
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
                        match chars.get(i).copied() {
                            Some('n') => {
                                advance_position('n', &mut line, &mut column, &mut i);
                                '\n' as i64
                            }
                            Some('t') => {
                                advance_position('t', &mut line, &mut column, &mut i);
                                '\t' as i64
                            }
                            Some('0') => {
                                advance_position('0', &mut line, &mut column, &mut i);
                                '\0' as i64
                            }
                            Some('\\') => {
                                advance_position('\\', &mut line, &mut column, &mut i);
                                '\\' as i64
                            }
                            Some('\'') => {
                                advance_position('\'', &mut line, &mut column, &mut i);
                                '\'' as i64
                            }
                            Some(other) => {
                                return Err(lexer_error_with_context(
                                    format!("unsupported character escape '\\{other}'"),
                                    source,
                                    start_line,
                                    start_column,
                                ));
                            }
                            None => {
                                return Err(lexer_error_with_context(
                                    "unterminated character literal",
                                    source,
                                    start_line,
                                    start_column,
                                ));
                            }
                        }
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
                    "const" => Token::Const,
                    "static" => Token::Static,
                    "void" => Token::Void,
                    "enum" => Token::Enum,
                    "struct" => Token::Struct,
                    "typedef" => Token::Typedef,
                    "sizeof" => Token::Sizeof,
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
            '*' => {
                push_token(&mut tokens, Token::Star, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '/' => {
                push_token(&mut tokens, Token::Slash, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
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
    enum_type_scopes: Vec<HashSet<String>>,
    type_alias_scopes: Vec<HashMap<String, TypeAlias>>,
    next_static_local_id: usize,
}

impl Parser {
    fn new(tokens: Vec<LocatedToken>) -> Self {
        Self {
            tokens,
            pos: 0,
            struct_types: HashMap::new(),
            enum_type_scopes: vec![HashSet::new()],
            type_alias_scopes: vec![HashMap::new()],
            next_static_local_id: 0,
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
            self.matches(&Token::Static);
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
            } else if matches!(self.peek(), Token::Int | Token::Char | Token::Const)
                || self.current_alias().is_some()
            {
                globals.push(self.parse_var_decl()?);
            } else if self.check(&Token::Typedef) {
                self.parse_typedef_decl()?;
            } else if self.check(&Token::Enum) {
                globals.push(self.parse_enum_decl()?);
            } else if self.check(&Token::Struct) {
                if self.is_struct_definition() {
                    self.parse_struct_definition()?;
                } else {
                    globals.push(self.parse_struct_var_decl()?);
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

    fn is_struct_definition(&self) -> bool {
        matches!(
            (
                self.tokens.get(self.pos + 1).map(|token| &token.kind),
                self.tokens.get(self.pos + 2).map(|token| &token.kind)
            ),
            (Some(Token::Ident(_)), Some(Token::LBrace))
        )
    }

    fn starts_struct_function_declaration(&self) -> bool {
        matches!(
            (
                self.peek(),
                self.tokens.get(self.pos + 1).map(|token| &token.kind),
                self.tokens.get(self.pos + 2).map(|token| &token.kind),
                self.tokens.get(self.pos + 3).map(|token| &token.kind)
            ),
            (
                Token::Struct,
                Some(Token::Ident(_)),
                Some(Token::Ident(_)),
                Some(Token::LParen)
            )
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

    fn lookup_type_alias(&self, name: &str) -> Option<&TypeAlias> {
        self.type_alias_scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
    }

    fn enum_type_is_declared(&self, name: &str) -> bool {
        self.enum_type_scopes
            .iter()
            .rev()
            .any(|scope| scope.contains(name))
    }

    fn parse_decl_type(&mut self, context: &str) -> CustResult<DeclType> {
        let found = self.advance();
        match found.kind.clone() {
            Token::Int => Ok(DeclType::Scalar(CType::Int)),
            Token::Char => Ok(DeclType::Scalar(CType::Char)),
            Token::Struct => {
                let type_name = self.expect_ident_after(context)?;
                if !self.struct_types.contains_key(&type_name) {
                    return Err(CustError::new(format!(
                        "undefined struct type '{type_name}'"
                    )));
                }
                Ok(DeclType::Struct(type_name))
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
                if !self.enum_type_is_declared(&type_name) {
                    return Err(Self::error_at(
                        format!("undefined enum type '{type_name}'"),
                        &type_name_token,
                    ));
                }
                Ok(DeclType::Scalar(CType::Int))
            }
            Token::Ident(name) => match self.lookup_type_alias(&name).cloned() {
                Some(TypeAlias::Scalar(ty)) => Ok(DeclType::Scalar(ty)),
                Some(TypeAlias::Struct(type_name)) => Ok(DeclType::Struct(type_name)),
                Some(TypeAlias::Pointer(pointee)) => Ok(DeclType::Pointer(pointee)),
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

    fn parse_const_qualified_decl_type(&mut self, context: &str) -> CustResult<(bool, DeclType)> {
        let is_const = self.matches(&Token::Const);
        let decl_type = self.parse_decl_type(context)?;
        Ok((is_const, decl_type))
    }

    fn decl_type_to_return_type(decl_type: DeclType) -> ReturnType {
        match decl_type {
            DeclType::Scalar(ty) => ReturnType::Scalar(ty),
            DeclType::Struct(type_name) => ReturnType::Struct(type_name),
            DeclType::Pointer(_) => unreachable!("pointer aliases cannot be function return types"),
        }
    }

    fn decl_type_to_param_type(decl_type: &DeclType) -> ParamType {
        match decl_type {
            DeclType::Scalar(ty) => ParamType::Scalar(*ty),
            DeclType::Struct(type_name) => ParamType::Struct(type_name.clone()),
            DeclType::Pointer(pointee) => match pointee {
                PointeeType::Scalar(ty) => ParamType::Scalar(*ty),
                PointeeType::Struct(type_name) => ParamType::Struct(type_name.clone()),
            },
        }
    }

    fn decl_type_to_pointee_type(decl_type: &DeclType) -> PointeeType {
        match decl_type {
            DeclType::Scalar(ty) => PointeeType::Scalar(*ty),
            DeclType::Struct(type_name) => PointeeType::Struct(type_name.clone()),
            DeclType::Pointer(pointee) => pointee.clone(),
        }
    }

    fn parse_typedef_decl(&mut self) -> CustResult<()> {
        self.expect(Token::Typedef)?;
        let base_type = self.parse_decl_type("typedef struct type name")?;
        let alias = if self.matches(&Token::Star) {
            if self.check(&Token::Star) {
                return Err(Self::error_at(
                    "pointer-to-pointer typedef aliases are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            match base_type {
                DeclType::Scalar(ty) => TypeAlias::Pointer(PointeeType::Scalar(ty)),
                DeclType::Struct(type_name) => TypeAlias::Pointer(PointeeType::Struct(type_name)),
                DeclType::Pointer(_) => {
                    return Err(Self::error_at(
                        "pointer-to-pointer typedef aliases are not supported".to_string(),
                        self.previous(),
                    ));
                }
            }
        } else {
            match base_type {
                DeclType::Scalar(ty) => TypeAlias::Scalar(ty),
                DeclType::Struct(type_name) => TypeAlias::Struct(type_name),
                DeclType::Pointer(pointee) => TypeAlias::Pointer(pointee),
            }
        };
        let alias_name = self.expect_ident_after("typedef alias name after type")?;
        self.expect_semicolon_after("typedef declaration")?;
        let current_scope = self
            .type_alias_scopes
            .last_mut()
            .expect("parser always has a typedef scope");
        if current_scope.insert(alias_name.clone(), alias).is_some() {
            return Err(CustError::new(format!(
                "typedef alias '{alias_name}' already declared"
            )));
        }
        Ok(())
    }

    fn starts_function_definition(&self) -> bool {
        if !matches!(self.peek(), Token::Int | Token::Char | Token::Void)
            && self.current_alias().is_none()
        {
            return false;
        }

        let mut index = self.pos + 1;
        if matches!(
            self.tokens.get(index).map(|token| &token.kind),
            Some(Token::Star)
        ) {
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

    fn starts_malformed_function_definition(&self) -> bool {
        if !matches!(self.peek(), Token::Int | Token::Char) {
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
        if self.check(&Token::Star) {
            return Err(Self::error_at(
                "pointer return types are not supported".to_string(),
                self.peek_located(),
            ));
        }
        let name = self.expect_ident_after("function name after return type")?;
        self.expect_opening_paren_after("function name")?;
        let params = self.parse_params()?;
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

    fn parse_function_return_type(&mut self) -> CustResult<ReturnType> {
        if self.check(&Token::Void) {
            self.advance();
            return Ok(ReturnType::Void);
        }
        let decl_type = self.parse_decl_type("struct return type name")?;
        if matches!(decl_type, DeclType::Pointer(_)) {
            return Err(Self::error_at(
                "pointer return types are not supported".to_string(),
                self.previous(),
            ));
        }
        Ok(Self::decl_type_to_return_type(decl_type))
    }

    fn parse_params(&mut self) -> CustResult<Vec<Param>> {
        let mut params = Vec::new();
        if self.check(&Token::RParen) {
            return Ok(params);
        }

        loop {
            let (leading_const, decl_type) =
                self.parse_const_qualified_decl_type("parameter type")?;
            let has_explicit_star = self.matches(&Token::Star);
            let post_star_const = has_explicit_star && self.matches(&Token::Const);
            if matches!(decl_type, DeclType::Pointer(_)) && has_explicit_star {
                return Err(Self::error_at(
                    "pointer-to-pointer parameters are not supported".to_string(),
                    self.previous(),
                ));
            }
            if has_explicit_star && self.check(&Token::Star) {
                return Err(Self::error_at(
                    "pointer-to-pointer parameters are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            let is_pointer = has_explicit_star || matches!(decl_type, DeclType::Pointer(_));
            let name = if has_explicit_star {
                match &decl_type {
                    DeclType::Scalar(_) => self.expect_ident_after("parameter name after '*'")?,
                    DeclType::Struct(_) => {
                        self.expect_ident_after("struct pointer parameter name after '*'")?
                    }
                    DeclType::Pointer(_) => {
                        unreachable!("pointer aliases with explicit stars return above")
                    }
                }
            } else {
                match &decl_type {
                    DeclType::Scalar(_) => self.expect_ident_after("parameter name after type")?,
                    DeclType::Struct(_) => {
                        self.expect_ident_after("struct parameter name after type")?
                    }
                    DeclType::Pointer(_) => {
                        self.expect_ident_after("pointer parameter name after type")?
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
                ParamKind::Struct
            } else if self.matches(&Token::LBracket) {
                let len = self.expect_array_len()?;
                self.expect_closing_bracket_after("array parameter length")?;
                ParamKind::Array(len)
            } else {
                ParamKind::Scalar
            };
            let (is_const, points_to_const) = if is_pointer {
                if has_explicit_star {
                    (post_star_const, leading_const)
                } else {
                    (leading_const, false)
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
        self.enum_type_scopes.push(HashSet::new());
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
        self.enum_type_scopes.pop();
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
            Token::Int | Token::Char | Token::Const => self.parse_var_decl(),
            Token::Ident(_) if self.current_alias().is_some() => self.parse_var_decl(),
            Token::Typedef => {
                self.parse_typedef_decl()?;
                Ok(Stmt::Empty)
            }
            Token::Enum => self.parse_enum_decl(),
            Token::Struct => self.parse_struct_var_decl(),
            Token::Return => self.parse_return(),
            Token::LBrace => Ok(Stmt::Block(self.parse_block_after("block statement")?)),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Do => self.parse_do_while(),
            Token::For => self.parse_for(),
            Token::Switch => self.parse_switch(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
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

    fn parse_static_local_decl(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Static)?;
        let id = self.next_static_local_id;
        self.next_static_local_id += 1;
        let decl = match self.peek() {
            Token::Int | Token::Char | Token::Const => self.parse_var_decl()?,
            Token::Ident(_) if self.current_alias().is_some() => self.parse_var_decl()?,
            Token::Struct => self.parse_struct_var_decl()?,
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

    fn parse_var_decl(&mut self) -> CustResult<Stmt> {
        self.parse_var_decl_with_semi(true)
    }

    fn parse_var_decl_with_semi(&mut self, require_semi: bool) -> CustResult<Stmt> {
        let (leading_const, decl_type) =
            self.parse_const_qualified_decl_type("struct type name")?;
        let has_explicit_star = self.matches(&Token::Star);
        let post_star_const = has_explicit_star && self.matches(&Token::Const);
        if matches!(decl_type, DeclType::Pointer(_)) && has_explicit_star {
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
        let is_pointer = has_explicit_star || matches!(decl_type, DeclType::Pointer(_));
        let name = if has_explicit_star {
            match &decl_type {
                DeclType::Scalar(_) => self.expect_ident_after("pointer name after '*'")?,
                DeclType::Struct(_) => self.expect_ident_after("struct pointer name after '*'")?,
                DeclType::Pointer(_) => {
                    unreachable!("pointer aliases with explicit stars return above")
                }
            }
        } else {
            match &decl_type {
                DeclType::Scalar(_) => self.expect_ident_after("variable name after type")?,
                DeclType::Struct(_) => self.expect_ident_after("struct variable name")?,
                DeclType::Pointer(_) => self.expect_ident_after("pointer name after type")?,
            }
        };
        if is_pointer {
            let (is_const, points_to_const) = if has_explicit_star {
                (post_star_const, leading_const)
            } else {
                (leading_const, false)
            };
            if self.check(&Token::LBracket) {
                return Err(Self::error_at(
                    "pointer array declarations are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            let expr = if self.matches(&Token::Assign) {
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
                    DeclType::Scalar(_) | DeclType::Pointer(PointeeType::Scalar(_)) => {
                        "pointer declaration"
                    }
                    DeclType::Struct(_) | DeclType::Pointer(PointeeType::Struct(_)) => {
                        "struct pointer declaration"
                    }
                };
                self.expect_assign_after(context)?;
                unreachable!("expect_assign_after only returns Ok after consuming '='")
            };
            if require_semi {
                let context = match &decl_type {
                    DeclType::Scalar(_) | DeclType::Pointer(PointeeType::Scalar(_)) => {
                        "pointer declaration"
                    }
                    DeclType::Struct(_) | DeclType::Pointer(PointeeType::Struct(_)) => {
                        "struct pointer declaration"
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
            if self.matches(&Token::Assign) {
                return Err(Self::error_at(
                    "struct variable initializers are not supported".to_string(),
                    self.previous(),
                ));
            }
            if require_semi {
                self.expect_semicolon_after("struct variable declaration")?;
            }
            if let DeclType::Struct(type_name) = decl_type {
                return Ok(Stmt::StructVarDecl {
                    type_name,
                    name,
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
            if require_semi {
                self.expect_semicolon_after("array declaration")?;
            }
            return Ok(Stmt::ArrayDecl {
                name,
                elem_type: ty,
                len,
                is_const: leading_const,
            });
        }
        let expr = if self.matches(&Token::Assign) {
            self.parse_expr()?
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

    fn parse_struct_definition(&mut self) -> CustResult<()> {
        self.expect(Token::Struct)?;
        let type_name = self.expect_ident_after("struct name")?;
        self.expect_opening_brace_after("struct name")?;
        let mut fields = Vec::new();
        let mut names = HashSet::new();
        while !self.check(&Token::RBrace) {
            if self.check(&Token::Eof) {
                return Err(Self::error_at(
                    format!("unterminated struct declaration for '{type_name}'"),
                    self.peek_located(),
                ));
            }
            let (is_const, decl_type) =
                self.parse_const_qualified_decl_type("struct field type")?;
            let ty = match decl_type {
                DeclType::Scalar(ty) => ty,
                DeclType::Struct(_) => {
                    return Err(Self::error_at(
                        "nested struct fields are not supported".to_string(),
                        self.previous(),
                    ));
                }
                DeclType::Pointer(_) => {
                    return Err(Self::error_at(
                        "pointer struct fields are not supported".to_string(),
                        self.previous(),
                    ));
                }
            };
            let name = self.expect_ident_after("struct field name after type")?;
            if !names.insert(name.clone()) {
                return Err(Self::error_at(
                    format!("duplicate struct field '{name}'"),
                    self.previous(),
                ));
            }
            self.expect_semicolon_after("struct field declaration")?;
            fields.push(StructFieldDef { name, ty, is_const });
        }
        self.expect(Token::RBrace)?;
        self.expect_semicolon_after("struct declaration")?;
        if self
            .struct_types
            .insert(type_name.clone(), StructTypeDef { fields })
            .is_some()
        {
            return Err(CustError::new(format!(
                "struct '{type_name}' already declared"
            )));
        }
        Ok(())
    }

    fn parse_struct_var_decl(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Struct)?;
        let type_name = self.expect_ident_after("struct type name")?;
        if !self.struct_types.contains_key(&type_name) {
            return Err(CustError::new(format!(
                "undefined struct type '{type_name}'"
            )));
        }
        if self.matches(&Token::Star) {
            let is_const = self.matches(&Token::Const);
            let name = self.expect_ident_after("struct pointer name after '*'")?;
            let expr = if self.matches(&Token::Assign) {
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
                points_to_const: false,
            });
        }
        let name = self.expect_ident_after("struct variable name")?;
        self.expect_semicolon_after("struct variable declaration")?;
        Ok(Stmt::StructVarDecl {
            type_name,
            name,
            is_const: false,
        })
    }

    fn parse_enum_decl(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Enum)?;
        let enum_tag = if let Token::Ident(name) = self.peek().clone() {
            self.advance();
            Some(name)
        } else {
            None
        };
        self.expect_opening_brace_after("enum")?;

        let mut constants = Vec::new();
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
                self.parse_enum_constant_value()?
            } else {
                next_value
            };
            next_value = value + 1;
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
        self.expect_semicolon_after("enum declaration")?;
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
        Ok(Stmt::EnumDecl { constants })
    }

    fn parse_enum_constant_value(&mut self) -> CustResult<i64> {
        let sign = if self.matches(&Token::Minus) { -1 } else { 1 };
        let found = self.advance();
        match &found.kind {
            Token::Number(value) => Ok(sign * *value),
            token => Err(Self::error_at(
                format!("expected integer constant after enum constant '=', found {token:?}"),
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
            let field = self.expect_ident_after("struct field name after '.'")?;
            if let Some(op) = self.compound_assignment_op() {
                self.advance();
                let value = self.parse_expr()?;
                if require_semi {
                    self.expect_semicolon_after("assignment")?;
                }
                return Ok(Stmt::Expr(Expr::StructCompoundSet {
                    name,
                    field,
                    op,
                    value: Box::new(value),
                }));
            }
            self.expect_assign_after("struct field assignment")?;
            let value = self.parse_expr()?;
            if require_semi {
                self.expect_semicolon_after("assignment")?;
            }
            return Ok(Stmt::StructAssign { name, field, value });
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
        } else if matches!(self.peek(), Token::Int | Token::Char | Token::Const)
            || self.current_alias().is_some()
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
        let sign = if self.matches(&Token::Minus) { -1 } else { 1 };
        let found = self.advance();
        match &found.kind {
            Token::Number(value) => Ok((sign * *value, found)),
            token => Err(Self::error_at(
                format!("expected integer constant after switch case, found {token:?}"),
                &found,
            )),
        }
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
                Expr::Deref(pointer) => Ok(Expr::DerefCompoundSet {
                    pointer,
                    op,
                    value: Box::new(value),
                }),
                Expr::StructGet { name, field } => Ok(Expr::StructCompoundSet {
                    name,
                    field,
                    op,
                    value: Box::new(value),
                }),
                Expr::StructPtrGet { pointer, field } => Ok(Expr::StructPtrCompoundSet {
                    pointer,
                    field,
                    op,
                    value: Box::new(value),
                }),
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
                Expr::Deref(pointer) => Ok(Expr::DerefSet {
                    pointer,
                    value: Box::new(value),
                }),
                Expr::StructGet { name, field } => Ok(Expr::StructSet {
                    name,
                    field,
                    value: Box::new(value),
                }),
                Expr::StructPtrGet { pointer, field } => Ok(Expr::StructPtrSet {
                    pointer,
                    field,
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
        } else if self.matches(&Token::Star) {
            Ok(Expr::Deref(Box::new(self.parse_unary()?)))
        } else if self.matches(&Token::Amp) {
            let found = self.advance();
            match found.kind.clone() {
                Token::Ident(name) => {
                    if self.matches(&Token::LBracket) {
                        let index = self.parse_index_expr()?;
                        self.expect_closing_bracket_after("array index")?;
                        Ok(Expr::AddressOfArray {
                            name,
                            index: Box::new(index),
                        })
                    } else {
                        Ok(Expr::AddressOf(name))
                    }
                }
                token => Err(Self::error_at(
                    format!("expected identifier after '&', found {token:?}"),
                    &found,
                )),
            }
        } else {
            self.parse_postfix()
        }
    }

    fn parse_sizeof(&mut self) -> CustResult<Expr> {
        if self.matches(&Token::LParen) {
            if matches!(
                self.peek(),
                Token::Int | Token::Char | Token::Const | Token::Void
            ) || self.current_alias().is_some()
            {
                let is_const_qualified = self.matches(&Token::Const);
                let sizeof_type = if self.check(&Token::Void) {
                    let type_token = self.advance();
                    return Err(Self::error_at(
                        "sizeof(void) is not supported".to_string(),
                        &type_token,
                    ));
                } else {
                    if is_const_qualified
                        && !matches!(self.peek(), Token::Int | Token::Char)
                        && self.current_alias().is_none()
                    {
                        let found = self.peek_located().clone();
                        return Err(Self::error_at(
                            format!("expected sizeof type after const, found {:?}", found.kind),
                            &found,
                        ));
                    }
                    match self.parse_decl_type("sizeof struct type name")? {
                        DeclType::Scalar(ty) => {
                            if self.matches(&Token::Star) {
                                SizeOfType::Pointer
                            } else {
                                SizeOfType::Scalar(ty)
                            }
                        }
                        DeclType::Struct(type_name) => {
                            if self.matches(&Token::Star) {
                                SizeOfType::Pointer
                            } else {
                                SizeOfType::Struct(type_name)
                            }
                        }
                        DeclType::Pointer(_) => {
                            if self.matches(&Token::Star) {
                                return Err(Self::error_at(
                                    "pointer-to-pointer sizeof types are not supported".to_string(),
                                    self.previous(),
                                ));
                            }
                            SizeOfType::Pointer
                        }
                    }
                };
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

    fn parse_postfix(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.matches(&Token::Dot) {
                let field = self.expect_ident_after("struct field name after '.'")?;
                expr = match expr {
                    Expr::Var(name) => Expr::StructGet { name, field },
                    Expr::Deref(pointer) => Expr::StructPtrGet { pointer, field },
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
                    field,
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
            | Expr::Deref(_)
            | Expr::StructGet { .. }
            | Expr::StructPtrGet { .. } => Ok(Expr::Increment {
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
                    Ok(Expr::StructGet { name, field })
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
            return self
                .tokens
                .get(self.pos + 3)
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

const MAX_CALL_DEPTH: usize = 64;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructFieldValue {
    value: i64,
    ty: CType,
    is_const: bool,
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
        }
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
                ParamKind::Array(expected_len) => {
                    self.eval_array_argument(name, &param.name, expected_len, arg_expr)?
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
                    Value::Pointer {
                        pointer: self.eval_pointer(arg_expr)?,
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
            (
                ReturnType::Struct(expected_type),
                Some(ReturnValue::Struct { type_name, fields }),
            ) if &type_name == expected_type => Ok(Some(ReturnValue::Struct { type_name, fields })),
            (ReturnType::Struct(expected_type), Some(ReturnValue::Struct { type_name, .. })) => {
                Err(CustError::new(format!(
                    "struct function '{function_name}' expected return struct '{expected_type}', got struct '{type_name}'"
                )))
            }
            (ReturnType::Struct(_), Some(ReturnValue::Scalar(_))) => Err(CustError::new(format!(
                "struct function '{function_name}' requires a struct return value"
            ))),
            (ReturnType::Struct(_), None) => Err(CustError::new(format!(
                "struct function '{function_name}' returned without a value"
            ))),
            (ReturnType::Void, Some(_)) => Err(CustError::new(format!(
                "void function '{function_name}' returned a value"
            ))),
            (ReturnType::Void, None) => Ok(None),
        }
    }

    fn eval_array_argument(
        &self,
        function_name: &str,
        param_name: &str,
        expected_len: usize,
        arg_expr: &Expr,
    ) -> CustResult<Value> {
        match arg_expr {
            Expr::Var(arg_name) => {
                let array = self.find_array(arg_name)?;
                let actual_len = array.borrow().elements.len();
                if actual_len != expected_len {
                    return Err(CustError::new(format!(
                        "function '{function_name}' array parameter '{param_name}' expected length {expected_len}, got {actual_len}"
                    )));
                }
                Ok(Value::Array(array))
            }
            Expr::StringLiteral(values) => {
                let actual_len = values.len();
                if actual_len != expected_len {
                    return Err(CustError::new(format!(
                        "function '{function_name}' array parameter '{param_name}' expected length {expected_len}, got {actual_len}"
                    )));
                }
                Ok(Value::Array(Rc::new(RefCell::new(ArrayValue::read_only(
                    values.clone(),
                )))))
            }
            _ => Err(CustError::new(format!(
                "function '{function_name}' array parameter '{param_name}' requires an array argument"
            ))),
        }
    }

    fn eval_struct_argument(
        &self,
        function_name: &str,
        param_name: &str,
        expected_type: &str,
        arg_expr: &Expr,
    ) -> CustResult<Value> {
        match arg_expr {
            Expr::Var(arg_name) => match self.find_variable(arg_name).cloned() {
                Some(Value::Struct { type_name, fields }) if type_name == expected_type => {
                    Ok(Value::Struct { type_name, fields })
                }
                Some(Value::Struct { type_name, .. }) => Err(CustError::new(format!(
                    "function '{function_name}' struct parameter '{param_name}' expected struct '{expected_type}', got struct '{type_name}'"
                ))),
                Some(_) => Err(CustError::new(format!(
                    "function '{function_name}' struct parameter '{param_name}' requires a struct argument"
                ))),
                None => Err(CustError::new(format!("undefined variable '{arg_name}'"))),
            },
            _ => Err(CustError::new(format!(
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
        matches!(
            self.find_variable(name),
            Some(Value::Pointer {
                points_to_const: true,
                ..
            })
        )
    }

    fn ensure_pointer_variable_pointee_mutable(&self, name: &str) -> CustResult<()> {
        if self.pointer_variable_points_to_const(name) {
            Err(CustError::new("cannot assign through pointer to const"))
        } else {
            Ok(())
        }
    }

    fn pointer_expr_points_to_const(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Var(name) => self.pointer_variable_points_to_const(name),
            Expr::AddressOf(name) => self.is_const_variable(name),
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
            Err(CustError::new(
                "cannot discard const qualifier from pointer target",
            ))
        } else {
            Ok(())
        }
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
            Some(Value::Struct { .. }) => Err(CustError::new(format!(
                "struct variable '{name}' used as scalar"
            ))),
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
            Some(Value::Struct { .. }) => Err(CustError::new(format!(
                "struct variable '{name}' is not an array"
            ))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn make_struct_value(&self, type_name: &str) -> CustResult<Value> {
        let struct_type = self
            .struct_types
            .get(type_name)
            .ok_or_else(|| CustError::new(format!("undefined struct type '{type_name}'")))?;
        let fields = struct_type
            .fields
            .iter()
            .map(|field| {
                (
                    field.name.clone(),
                    StructFieldValue {
                        value: 0,
                        ty: field.ty,
                        is_const: field.is_const,
                    },
                )
            })
            .collect();
        Ok(Value::Struct {
            type_name: type_name.to_string(),
            fields,
        })
    }

    fn read_struct_field(&self, name: &str, field: &str) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::Struct { type_name, fields }) => fields
                .get(field)
                .map(|field_value| field_value.value)
                .ok_or_else(|| {
                    CustError::new(format!("struct '{type_name}' has no field '{field}'"))
                }),
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn assign_struct_field(&mut self, name: &str, field: &str, value: i64) -> CustResult<()> {
        self.ensure_variable_mutable(name)?;
        match self.find_variable_mut(name) {
            Some(Value::Struct { type_name, fields }) => {
                let field_value = fields.get_mut(field).ok_or_else(|| {
                    CustError::new(format!("struct '{type_name}' has no field '{field}'"))
                })?;
                if field_value.is_const {
                    return Err(CustError::new(format!(
                        "cannot assign to const struct field '{field}'"
                    )));
                }
                field_value.value = value;
                Ok(())
            }
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
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
            _ => Err(CustError::new("pointer does not reference a struct")),
        }
    }

    fn read_struct_pointer_field(&self, pointer: &PointerValue, field: &str) -> CustResult<i64> {
        let (type_name, fields) = self.find_struct_pointer_fields(pointer)?;
        fields
            .get(field)
            .map(|field_value| field_value.value)
            .ok_or_else(|| CustError::new(format!("struct '{type_name}' has no field '{field}'")))
    }

    fn assign_struct_pointer_field(
        &mut self,
        pointer: &PointerValue,
        field: &str,
        value: i64,
    ) -> CustResult<()> {
        self.ensure_struct_pointer_target_mutable(pointer)?;
        let (type_name, fields) = self.find_struct_pointer_fields_mut(pointer)?;
        let field_value = fields.get_mut(field).ok_or_else(|| {
            CustError::new(format!("struct '{type_name}' has no field '{field}'"))
        })?;
        if field_value.is_const {
            return Err(CustError::new(format!(
                "cannot assign to const struct field '{field}'"
            )));
        }
        field_value.value = value;
        Ok(())
    }

    fn eval_struct_ptr_set(
        &mut self,
        pointer: &Expr,
        field: &str,
        value: &Expr,
    ) -> CustResult<i64> {
        self.ensure_pointer_expr_pointee_mutable(pointer)?;
        let pointer = self.eval_pointer(pointer)?;
        let value = self.eval(value)?;
        self.assign_struct_pointer_field(&pointer, field, value)?;
        Ok(value)
    }

    fn eval_struct_ptr_compound_set(
        &mut self,
        pointer: &Expr,
        field: &str,
        op: CompoundOp,
        value: &Expr,
    ) -> CustResult<i64> {
        self.ensure_pointer_expr_pointee_mutable(pointer)?;
        let pointer = self.eval_pointer(pointer)?;
        let current = self.read_struct_pointer_field(&pointer, field)?;
        let rhs = self.eval(value)?;
        let result = Self::apply_compound_op(current, op, rhs)?;
        self.assign_struct_pointer_field(&pointer, field, result)?;
        Ok(result)
    }

    fn assign_struct_copy(&mut self, name: &str, rhs: &Expr) -> CustResult<()> {
        self.ensure_variable_mutable(name)?;
        let (rhs_type, rhs_fields) = match self.eval_struct_expr(rhs)? {
            ReturnValue::Struct { type_name, fields } => (type_name, fields),
            ReturnValue::Scalar(_) => {
                return Err(CustError::new("struct assignment requires struct value"));
            }
        };

        match self.find_variable_mut(name) {
            Some(Value::Struct { type_name, fields }) if *type_name == rhs_type => {
                if fields.values().any(|field| field.is_const) {
                    return Err(CustError::new(format!(
                        "cannot assign to struct '{type_name}' with const fields"
                    )));
                }
                *fields = rhs_fields;
                Ok(())
            }
            Some(Value::Struct { type_name, .. }) => Err(CustError::new(format!(
                "cannot assign struct '{rhs_type}' to struct '{type_name}'"
            ))),
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn eval_struct_set(&mut self, name: &str, field: &str, value: &Expr) -> CustResult<i64> {
        let value = self.eval(value)?;
        self.assign_struct_field(name, field, value)?;
        Ok(value)
    }

    fn eval_struct_compound_set(
        &mut self,
        name: &str,
        field: &str,
        op: CompoundOp,
        value: &Expr,
    ) -> CustResult<i64> {
        let current = self.read_struct_field(name, field)?;
        let rhs = self.eval(value)?;
        let result = Self::apply_compound_op(current, op, rhs)?;
        self.assign_struct_field(name, field, result)?;
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
                if let Some(Value::Pointer { pointer, .. }) = self.find_variable(name).cloned() {
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
            Expr::StringLiteral(values) => Ok(PointerValue::ArrayBase {
                array: Rc::new(RefCell::new(ArrayValue::read_only(values.clone()))),
                source_name: None,
            }),
            Expr::Assign { name, value } => match self.find_variable(name).cloned() {
                Some(Value::Pointer {
                    points_to_const, ..
                }) => {
                    self.ensure_variable_mutable(name)?;
                    self.ensure_pointer_conversion_preserves_const(points_to_const, value)?;
                    let pointer = self.eval_pointer(value)?;
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
                Some(Value::Struct { .. }) => Err(CustError::new(format!(
                    "struct variable '{name}' used as pointer"
                ))),
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
                Some(Value::Scalar { .. }) => Err(CustError::new(format!(
                    "variable '{name}' is not a pointer"
                ))),
                Some(Value::Struct { .. }) => Err(CustError::new(format!(
                    "struct variable '{name}' used as pointer"
                ))),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
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
                    Some(Value::Struct { .. }) => {
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
                    CompoundOp::BitAnd
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
                        Some(Value::Struct { .. }) => {
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
            PointerValue::Scalar { .. } | PointerValue::Struct { .. } => {
                Err(CustError::new("scalar pointer arithmetic is not supported"))
            }
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
            PointerValue::Struct { .. } => Err(CustError::new("struct pointer used as scalar")),
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
            PointerValue::Struct { .. } => Err(CustError::new("struct pointer used as scalar")),
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
            Some(Value::Struct { .. }) => {
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
            PointerValue::Scalar { .. } => Err(CustError::new("scalar pointer is not indexable")),
            PointerValue::Struct { .. } => Err(CustError::new("struct pointer is not indexable")),
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

    fn expr_is_pointer_value(&mut self, expr: &Expr) -> bool {
        !matches!(expr, Expr::Number(_)) && self.eval_pointer(expr).is_ok()
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
        }
    }

    fn array_pointer_index(pointer: &PointerValue) -> CustResult<(&Rc<RefCell<ArrayValue>>, i64)> {
        match pointer {
            PointerValue::ArrayBase { array, .. } => Ok((array, 0)),
            PointerValue::ArrayElement { array, index, .. } => Ok((array, *index as i64)),
            PointerValue::Null => Err(CustError::new("null pointer arithmetic is not supported")),
            PointerValue::Scalar { .. } | PointerValue::Struct { .. } => {
                Err(CustError::new("scalar pointer arithmetic is not supported"))
            }
        }
    }

    fn pointer_difference(left: &PointerValue, right: &PointerValue) -> CustResult<i64> {
        let (left_array, left_index) = Self::array_pointer_index(left)?;
        let (right_array, right_index) = Self::array_pointer_index(right)?;
        if !Rc::ptr_eq(left_array, right_array) {
            return Err(CustError::new(
                "cannot subtract pointers to different arrays",
            ));
        }
        Ok(left_index - right_index)
    }

    fn eval_truthy(&mut self, expr: &Expr) -> CustResult<bool> {
        match expr {
            Expr::Comma(left, right) => {
                self.eval_discard(left)?;
                self.eval_truthy(right)
            }
            Expr::AddressOf(_) | Expr::AddressOfArray { .. } | Expr::StringLiteral(_) => {
                Ok(Self::pointer_truthy(&self.eval_pointer(expr)?))
            }
            Expr::Assign { name, .. } => match self.find_variable(name).cloned() {
                Some(Value::Pointer { .. }) => Ok(Self::pointer_truthy(&self.eval_pointer(expr)?)),
                Some(Value::Scalar { .. }) => Ok(self.eval(expr)? != 0),
                Some(Value::Array(_)) => Err(CustError::new(format!(
                    "array '{name}' requires indexed assignment"
                ))),
                Some(Value::Struct { .. }) => Err(CustError::new(format!(
                    "struct variable '{name}' used as scalar"
                ))),
                None => Err(CustError::new(format!(
                    "assignment to undeclared variable '{name}'"
                ))),
            },
            Expr::Var(name) => match self.find_variable(name).cloned() {
                Some(Value::Pointer { pointer, .. }) => Ok(Self::pointer_truthy(&pointer)),
                Some(Value::Array(array)) => Ok(!array.borrow().elements.is_empty()),
                Some(Value::Scalar { value, .. }) => Ok(value != 0),
                Some(Value::Struct { .. }) => Err(CustError::new(format!(
                    "struct variable '{name}' used as scalar"
                ))),
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
            | Expr::StructGet { .. }
            | Expr::StructSet { .. }
            | Expr::StructCompoundSet { .. }
            | Expr::StructPtrGet { .. }
            | Expr::StructPtrSet { .. }
            | Expr::StructPtrCompoundSet { .. }
            | Expr::StringGet { .. }
            | Expr::Call { .. } => Ok(self.eval(expr)? != 0),
            Expr::Number(value) => Ok(*value != 0),
            Expr::Binary(_, BinaryOp::Add | BinaryOp::Sub, _) => match self.eval_pointer(expr) {
                Ok(pointer) => Ok(Self::pointer_truthy(&pointer)),
                Err(error) if error.to_string() == "expected pointer expression" => {
                    Ok(self.eval(expr)? != 0)
                }
                Err(error) => Err(error),
            },
            Expr::UnaryPlus(_)
            | Expr::UnaryMinus(_)
            | Expr::BitwiseNot(_)
            | Expr::LogicalNot(_)
            | Expr::SizeOfType(_)
            | Expr::SizeOfValue(_)
            | Expr::CompoundAssign { .. }
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
        if matches!(
            expr,
            Expr::CompoundAssign {
                op: CompoundOp::Add | CompoundOp::Sub,
                ..
            } | Expr::Increment { .. }
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
            Some(Value::Struct { .. }) => Err(CustError::new(format!(
                "struct variable '{name}' assignment is not supported"
            ))),
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
            Some(Value::Struct { .. }) => Err(CustError::new(format!(
                "struct variable '{name}' assignment is not supported"
            ))),
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
            CompoundOp::BitAnd => Ok(lhs & rhs),
            CompoundOp::BitOr => Ok(lhs | rhs),
            CompoundOp::BitXor => Ok(lhs ^ rhs),
            CompoundOp::ShiftLeft => Self::checked_shift_left(lhs, rhs),
            CompoundOp::ShiftRight => Self::checked_shift_right(lhs, rhs),
        }
    }

    fn pointer_compound_error(op: CompoundOp) -> CustError {
        match op {
            CompoundOp::Add | CompoundOp::Sub => {
                CustError::new("pointer arithmetic is not supported")
            }
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
            Expr::SizeOfType(_) | Expr::SizeOfValue(_) => Ok(INT_SIZE),
            Expr::Var(name) => self.sizeof_variable(name),
            Expr::StructGet { name, field } => self.sizeof_struct_field(name, field),
            Expr::StructPtrGet { pointer, field } => {
                self.sizeof_struct_pointer_field(pointer, field)
            }
            Expr::ArrayGet { name, .. } => self.sizeof_indexed_value(name),
            Expr::StringGet { .. } => Ok(CHAR_SIZE),
            Expr::AddressOf(_) | Expr::AddressOfArray { .. } => Ok(POINTER_SIZE),
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
            Expr::StructSet { name, field, .. } | Expr::StructCompoundSet { name, field, .. } => {
                self.sizeof_struct_field(name, field)
            }
            Expr::StructPtrSet { pointer, field, .. }
            | Expr::StructPtrCompoundSet { pointer, field, .. } => {
                self.sizeof_struct_pointer_field(pointer, field)
            }
            Expr::Increment { target, .. } => self.sizeof_expr(target),
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

    fn sizeof_variable(&self, name: &str) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::Scalar { ty, .. }) => Ok(ty.size()),
            Some(Value::Array(array)) => {
                let array = array.borrow();
                Ok(array.elements.len() as i64 * array.elem_type.size())
            }
            Some(Value::Pointer { .. }) => Ok(POINTER_SIZE),
            Some(Value::Struct { fields, .. }) => {
                Ok(fields.values().map(|field| field.ty.size()).sum())
            }
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn sizeof_struct_field(&self, name: &str, field: &str) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::Struct { type_name, fields }) => fields
                .get(field)
                .map(|field_value| field_value.ty.size())
                .ok_or_else(|| {
                    CustError::new(format!("struct '{type_name}' has no field '{field}'"))
                }),
            Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn sizeof_struct_pointer_field(&self, pointer: &Expr, field: &str) -> CustResult<i64> {
        let pointer = self.clone_for_sizeof_pointer(pointer)?;
        let (type_name, fields) = self.find_struct_pointer_fields(&pointer)?;
        fields
            .get(field)
            .map(|field_value| field_value.ty.size())
            .ok_or_else(|| CustError::new(format!("struct '{type_name}' has no field '{field}'")))
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
            Some(Value::Struct { .. }) => Err(CustError::new(format!(
                "struct variable '{name}' assignment is not supported"
            ))),
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
            Some(Value::Struct { .. }) => Err(CustError::new(format!(
                "struct variable '{name}' is not an array"
            ))),
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn sizeof_deref(&self, pointer: &Expr) -> CustResult<i64> {
        match pointer {
            Expr::Var(name) => match self.find_variable(name) {
                Some(Value::Pointer { ty, .. }) => Ok(ty.size(&self.struct_types)?),
                Some(Value::Array(array)) => Ok(array.borrow().elem_type.size()),
                Some(Value::Scalar { .. }) => Err(CustError::new(format!(
                    "variable '{name}' is not a pointer"
                ))),
                Some(Value::Struct { .. }) => Err(CustError::new(format!(
                    "struct variable '{name}' used as pointer"
                ))),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::AddressOf(name) => self.sizeof_variable(name),
            Expr::AddressOfArray { name, .. } => self.sizeof_indexed_value(name),
            Expr::StringLiteral(_) => Ok(CHAR_SIZE),
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
                Some(Value::Struct { .. }) => Err(CustError::new(format!(
                    "struct variable '{name}' assignment is not supported"
                ))),
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
            Expr::StructGet { name, field } => {
                let current = self.read_struct_field(name, field)?;
                let updated = Self::apply_increment_op(current, op);
                self.assign_struct_field(name, field, updated)?;
                Ok(Self::increment_result(current, updated, prefix))
            }
            Expr::StructPtrGet { pointer, field } => {
                self.ensure_pointer_expr_pointee_mutable(pointer)?;
                let pointer = self.eval_pointer(pointer)?;
                let current = self.read_struct_pointer_field(&pointer, field)?;
                let updated = Self::apply_increment_op(current, op);
                self.assign_struct_pointer_field(&pointer, field, updated)?;
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
            Expr::Var(name) => match self.find_variable(name).cloned() {
                Some(Value::Struct { type_name, fields }) => {
                    Ok(ReturnValue::Struct { type_name, fields })
                }
                Some(_) => Err(CustError::new(format!("variable '{name}' is not a struct"))),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::Call { name, args } => self.call_function(name, args)?.ok_or_else(|| {
                CustError::new(format!("void function '{name}' used as struct expression"))
            }),
            _ => Err(CustError::new("expected struct expression")),
        }
    }

    fn eval_return_value(&mut self, expr: Option<&Expr>) -> CustResult<Option<ReturnValue>> {
        let Some(expr) = expr else {
            return Ok(None);
        };
        match self.return_type_stack.last().cloned() {
            Some(ReturnType::Struct(_)) => Ok(Some(self.eval_struct_expr(expr)?)),
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
            | Stmt::StructVarDecl { name, is_const, .. } => Ok((name, *is_const)),
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
                Ok(Value::Pointer {
                    pointer: self.eval_pointer(expr)?,
                    ty: ty.clone(),
                    points_to_const: *points_to_const,
                })
            }
            Stmt::ArrayDecl {
                elem_type,
                len,
                is_const,
                ..
            } => {
                let mut array = ArrayValue::mutable_zeroed(*len, *elem_type);
                array.read_only = *is_const;
                Ok(Value::Array(Rc::new(RefCell::new(array))))
            }
            Stmt::StructVarDecl { type_name, .. } => self.make_struct_value(type_name),
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
                is_const,
            } => {
                if self.current_scope_has_identifier(name) {
                    return Err(CustError::new(format!(
                        "variable '{name}' already declared in this scope"
                    )));
                }
                let scope = self.current_scope_mut();
                scope.insert(
                    name.clone(),
                    Value::Array(Rc::new(RefCell::new(ArrayValue::mutable_zeroed(
                        *len, *elem_type,
                    )))),
                );
                if *is_const {
                    if let Some(Value::Array(array)) = self.find_variable(name) {
                        array.borrow_mut().read_only = true;
                    }
                    self.mark_current_variable_const(name);
                }
                Ok(ExecFlow::None)
            }
            Stmt::StructVarDecl {
                type_name,
                name,
                is_const,
            } => {
                if self.current_scope_has_identifier(name) {
                    return Err(CustError::new(format!(
                        "variable '{name}' already declared in this scope"
                    )));
                }
                let value = self.make_struct_value(type_name)?;
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
                        points_to_const, ..
                    }) => {
                        self.ensure_variable_mutable(name)?;
                        self.ensure_pointer_conversion_preserves_const(points_to_const, expr)?;
                        let pointer = self.eval_pointer(expr)?;
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
                    None if self.find_enum_constant(name).is_some() => Err(CustError::new(
                        format!("cannot assign to enum constant '{name}'"),
                    )),
                    None => Err(CustError::new(format!(
                        "assignment to undeclared variable '{name}'"
                    ))),
                }
            }
            Stmt::DerefAssign { pointer, value } => {
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
            Stmt::StructAssign { name, field, value } => {
                let value = self.eval(value)?;
                self.assign_struct_field(name, field, value)?;
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
            Expr::StructGet { name, field } => self.read_struct_field(name, field),
            Expr::StructPtrGet { pointer, field } => {
                let pointer = self.eval_pointer(pointer)?;
                self.read_struct_pointer_field(&pointer, field)
            }
            Expr::SizeOfType(sizeof_type) => Ok(sizeof_type.size(&self.struct_types)?),
            Expr::SizeOfValue(expr) => self.sizeof_expr(expr),
            Expr::Var(name) => self.find_scalar(name),
            Expr::AddressOf(_) | Expr::AddressOfArray { .. } => {
                Err(CustError::new("pointer value used as scalar"))
            }
            Expr::Assign { name, value } => self.eval_assignment_expr(name, value),
            Expr::CompoundAssign { name, op, value } => {
                self.eval_compound_assignment_expr(name, *op, value)
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
            Expr::StructSet { name, field, value } => self.eval_struct_set(name, field, value),
            Expr::StructPtrSet {
                pointer,
                field,
                value,
            } => self.eval_struct_ptr_set(pointer, field, value),
            Expr::StructCompoundSet {
                name,
                field,
                op,
                value,
            } => self.eval_struct_compound_set(name, field, *op, value),
            Expr::StructPtrCompoundSet {
                pointer,
                field,
                op,
                value,
            } => self.eval_struct_ptr_compound_set(pointer, field, *op, value),
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
                Some(ReturnValue::Struct { .. }) => Err(CustError::new(format!(
                    "struct function '{name}' used as scalar expression"
                ))),
                None => Err(CustError::new(format!(
                    "void function '{name}' used as scalar expression"
                ))),
            },
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
                    match (self.eval_pointer(left), self.eval_pointer(right)) {
                        (Ok(left_pointer), Ok(right_pointer)) if *op == BinaryOp::Sub => {
                            return Self::pointer_difference(&left_pointer, &right_pointer);
                        }
                        (Ok(_), Ok(_)) if *op == BinaryOp::Add => {
                            return Err(CustError::new("cannot add two pointers"));
                        }
                        (Ok(_), Err(_)) | (Err(_), Ok(_)) => {
                            return Err(CustError::new("pointer value used as scalar"));
                        }
                        (Err(_), Err(_)) | (Ok(_), Ok(_)) => {}
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
                        return Err(CustError::new(
                            "pointer ordering comparisons are not supported",
                        ));
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
