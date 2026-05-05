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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Int,
    Char,
    Return,
    If,
    Else,
    While,
    Do,
    For,
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
    Star,
    Slash,
    Percent,
    Amp,
    AndAnd,
    OrOr,
    Bang,
    Assign,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semi,
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
    Var(String),
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
    LogicalNot(Box<Expr>),
    Conditional {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Program {
    functions: HashMap<String, Function>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Function {
    params: Vec<Param>,
    body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Param {
    name: String,
    kind: ParamKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParamKind {
    Scalar,
    Array(usize),
    Pointer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompoundOp {
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IncrementOp {
    Inc,
    Dec,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Stmt {
    Empty,
    VarDecl(String, Expr),
    PointerDecl(String, Expr),
    ArrayDecl {
        name: String,
        len: usize,
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
    Expr(Expr),
    Return(Expr),
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
}

/// Interpret a small, safe C subset and return `main()`'s integer exit value.
///
/// Supported v0.1 syntax:
/// - `int main() { ... }`
/// - `int name(int param, char param, ...) { ... }` function definitions and calls, including bounded recursion
/// - `int name = expression;` and `char name = expression;`
/// - `name = expression;`
/// - `return expression;`
/// - block statements: `{ ... }`
/// - `if (expression) { ... } else { ... }`
/// - `while (expression) { ... }`
/// - `do { ... } while (expression);`
/// - `for (initializer; condition; increment) { ... }`
/// - `break;` and `continue;` inside loops
/// - empty statements (`;`) and side-effect-free expression statements (`expr;`)
/// - integer, character, and string literals (string literals are read-only NUL-terminated byte arrays)
/// - integer arithmetic/comparisons/logical operators: `+ - * / % == != < <= > >= && || !`
/// - pointer truthiness and pointer equality/inequality for null, scalar pointers, and array-backed pointers
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
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "do" => Token::Do,
                    "for" => Token::For,
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
            '&' => {
                push_token(&mut tokens, Token::Amp, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '|' if chars.get(i + 1) == Some(&'|') => {
                push_token(&mut tokens, Token::OrOr, line, column);
                advance_position('|', &mut line, &mut column, &mut i);
                advance_position('|', &mut line, &mut column, &mut i);
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
            '<' if chars.get(i + 1) == Some(&'=') => {
                push_token(&mut tokens, Token::Le, line, column);
                advance_position('<', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
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
}

impl Parser {
    fn new(tokens: Vec<LocatedToken>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse_program(&mut self) -> CustResult<Program> {
        let mut functions = HashMap::new();
        while !self.check(&Token::Eof) {
            if self.check(&Token::RBrace) {
                return Err(Self::error_at(
                    "unmatched '}' at top level".to_string(),
                    self.peek_located(),
                ));
            }
            let (name, function) = self.parse_function()?;
            if functions.insert(name.clone(), function).is_some() {
                return Err(CustError::new(format!("function '{name}' already defined")));
            }
        }
        self.expect(Token::Eof)?;
        if !functions.contains_key("main") {
            return Err(CustError::new("missing main() function"));
        }
        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> CustResult<(String, Function)> {
        self.expect(Token::Int)?;
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
        let body = self.parse_block_after("function header")?;
        Ok((name, Function { params, body }))
    }

    fn parse_params(&mut self) -> CustResult<Vec<Param>> {
        let mut params = Vec::new();
        if self.check(&Token::RParen) {
            return Ok(params);
        }

        loop {
            self.expect_type_after("parameter type")?;
            let is_pointer = self.matches(&Token::Star);
            if is_pointer && self.check(&Token::Star) {
                return Err(Self::error_at(
                    "pointer-to-pointer parameters are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            let name = if is_pointer {
                self.expect_ident_after("parameter name after '*'")?
            } else {
                self.expect_ident_after("parameter name after type")?
            };
            let kind = if is_pointer {
                if self.check(&Token::LBracket) {
                    return Err(Self::error_at(
                        "pointer array parameters are not supported".to_string(),
                        self.peek_located(),
                    ));
                }
                ParamKind::Pointer
            } else if self.matches(&Token::LBracket) {
                let len = self.expect_array_len()?;
                self.expect_closing_bracket_after("array parameter length")?;
                ParamKind::Array(len)
            } else {
                ParamKind::Scalar
            };
            params.push(Param { name, kind });

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
        let mut statements = Vec::new();
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
    }

    fn parse_stmt(&mut self) -> CustResult<Stmt> {
        match self.peek() {
            Token::Semi => self.parse_empty(),
            Token::Int | Token::Char => self.parse_var_decl(),
            Token::Return => self.parse_return(),
            Token::LBrace => Ok(Stmt::Block(self.parse_block_after("block statement")?)),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Do => self.parse_do_while(),
            Token::For => self.parse_for(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
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

    fn parse_var_decl(&mut self) -> CustResult<Stmt> {
        self.parse_var_decl_with_semi(true)
    }

    fn parse_var_decl_with_semi(&mut self, require_semi: bool) -> CustResult<Stmt> {
        self.expect_type()?;
        let is_pointer = self.matches(&Token::Star);
        if is_pointer && self.check(&Token::Star) {
            return Err(Self::error_at(
                "pointer-to-pointer declarations are not supported".to_string(),
                self.peek_located(),
            ));
        }
        let name = if is_pointer {
            self.expect_ident_after("pointer name after '*'")?
        } else {
            self.expect_ident_after("variable name after type")?
        };
        if is_pointer {
            if self.check(&Token::LBracket) {
                return Err(Self::error_at(
                    "pointer array declarations are not supported".to_string(),
                    self.peek_located(),
                ));
            }
            self.expect_assign_after("pointer declaration")?;
            let expr = self.parse_expr()?;
            if require_semi {
                self.expect_semicolon_after("pointer declaration")?;
            }
            return Ok(Stmt::PointerDecl(name, expr));
        }
        if self.matches(&Token::LBracket) {
            let len = self.expect_array_len()?;
            self.expect_closing_bracket_after("array length")?;
            if require_semi {
                self.expect_semicolon_after("array declaration")?;
            }
            return Ok(Stmt::ArrayDecl { name, len });
        }
        self.expect_assign_after("variable declaration")?;
        let expr = self.parse_expr()?;
        if require_semi {
            self.expect_semicolon_after("variable declaration")?;
        }
        Ok(Stmt::VarDecl(name, expr))
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
        let expr = self.parse_expr()?;
        self.expect_semicolon_after("return statement")?;
        Ok(Stmt::Return(expr))
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
        let then_branch = self.parse_block_after("if condition")?;
        let else_branch = if self.matches(&Token::Else) {
            self.parse_block_after("else")?
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
        let body = self.parse_block_after("while condition")?;
        Ok(Stmt::While { cond, body })
    }

    fn parse_do_while(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Do)?;
        let body = self.parse_block_after("do")?;
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
        } else if matches!(self.peek(), Token::Int | Token::Char) {
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

        let body = self.parse_block_after("for clauses")?;
        Ok(Stmt::For {
            init,
            cond,
            increment,
            body,
        })
    }

    fn parse_expr(&mut self) -> CustResult<Expr> {
        self.parse_assignment_expr()
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
        let mut expr = self.parse_equality()?;
        while self.matches(&Token::AndAnd) {
            let rhs = self.parse_equality()?;
            expr = Expr::Binary(Box::new(expr), BinaryOp::LogicalAnd, Box::new(rhs));
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
        let mut expr = self.parse_term()?;
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
        } else if self.matches(&Token::Bang) {
            Ok(Expr::LogicalNot(Box::new(self.parse_unary()?)))
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

    fn parse_postfix(&mut self) -> CustResult<Expr> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.matches(&Token::PlusPlus) {
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
            Expr::Var(_) | Expr::ArrayGet { .. } | Expr::Deref(_) => Ok(Expr::Increment {
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
            args.push(self.parse_expr()?);

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

    fn expect_type(&mut self) -> CustResult<()> {
        let found = self.advance();
        match &found.kind {
            Token::Int | Token::Char => Ok(()),
            token => Err(Self::error_at(
                format!("expected type, found {token:?}"),
                &found,
            )),
        }
    }

    fn expect_type_after(&mut self, context: &str) -> CustResult<()> {
        let found = self.advance();
        match &found.kind {
            Token::Int | Token::Char => Ok(()),
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
            _ => None,
        }
    }

    fn is_assignment_operator(token: &Token) -> bool {
        matches!(
            token,
            Token::Assign | Token::PlusAssign | Token::MinusAssign
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

        if Self::is_primary_assignment_value_start(self.peek_next()) {
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
    live_scope_ids: HashSet<usize>,
    next_scope_id: usize,
    functions: HashMap<String, Function>,
    call_depth: usize,
    max_loop_iterations: Option<usize>,
    loop_iterations: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scope {
    id: usize,
    values: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExecFlow {
    None,
    Return(i64),
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Scalar(i64),
    Array(Rc<RefCell<ArrayValue>>),
    Pointer(PointerValue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PointerValue {
    Null,
    Scalar {
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
    read_only: bool,
}

impl ArrayValue {
    fn mutable_zeroed(len: usize) -> Self {
        Self {
            elements: vec![0; len],
            read_only: false,
        }
    }

    fn read_only(elements: Vec<i64>) -> Self {
        Self {
            elements,
            read_only: true,
        }
    }
}

impl Interpreter {
    fn new(options: InterpretOptions) -> Self {
        Self {
            scopes: Vec::new(),
            live_scope_ids: HashSet::new(),
            next_scope_id: 0,
            functions: HashMap::new(),
            call_depth: 0,
            max_loop_iterations: options.max_loop_iterations,
            loop_iterations: 0,
        }
    }

    fn run(&mut self, program: &Program) -> CustResult<i64> {
        self.functions = program.functions.clone();
        self.call_function("main", &[])
    }

    fn call_function(&mut self, name: &str, arg_exprs: &[Expr]) -> CustResult<i64> {
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
        for (param, arg_expr) in function.params.iter().zip(arg_exprs) {
            let arg = match param.kind {
                ParamKind::Scalar => Value::Scalar(self.eval(arg_expr)?),
                ParamKind::Array(expected_len) => {
                    self.eval_array_argument(name, &param.name, expected_len, arg_expr)?
                }
                ParamKind::Pointer => Value::Pointer(self.eval_pointer(arg_expr)?),
            };
            if param_scope.insert(param.name.clone(), arg).is_some() {
                return Err(CustError::new(format!(
                    "parameter '{}' already declared in this function",
                    param.name
                )));
            }
        }

        self.call_depth += 1;
        self.push_scope_with_values(param_scope);
        let result = match self.exec_block(&function.body) {
            Ok(ExecFlow::Return(value)) => Ok(value),
            Ok(ExecFlow::None) => Err(CustError::new(format!(
                "function '{name}' finished without return"
            ))),
            Ok(ExecFlow::Break) => Err(CustError::new("break outside loop")),
            Ok(ExecFlow::Continue) => Err(CustError::new("continue outside loop")),
            Err(error) => Err(error),
        };
        self.pop_scope();
        self.call_depth -= 1;
        result
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

    fn push_scope(&mut self) {
        self.push_scope_with_values(HashMap::new());
    }

    fn push_scope_with_values(&mut self, values: HashMap<String, Value>) {
        let id = self.next_scope_id;
        self.next_scope_id += 1;
        self.live_scope_ids.insert(id);
        self.scopes.push(Scope { id, values });
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

    fn find_variable(&self, name: &str) -> Option<&Value> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.values.get(name))
    }

    fn find_variable_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.scopes
            .iter_mut()
            .rev()
            .find_map(|scope| scope.values.get_mut(name))
    }

    fn find_scalar(&self, name: &str) -> CustResult<i64> {
        match self.find_variable(name) {
            Some(Value::Scalar(value)) => Ok(*value),
            Some(Value::Array(_)) => Err(CustError::new(format!("array '{name}' used as scalar"))),
            Some(Value::Pointer(_)) => {
                Err(CustError::new(format!("pointer '{name}' used as scalar")))
            }
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn find_array(&self, name: &str) -> CustResult<Rc<RefCell<ArrayValue>>> {
        match self.find_variable(name) {
            Some(Value::Array(values)) => Ok(Rc::clone(values)),
            Some(Value::Scalar(_)) => {
                Err(CustError::new(format!("variable '{name}' is not an array")))
            }
            Some(Value::Pointer(_)) => {
                Err(CustError::new(format!("pointer '{name}' is not an array")))
            }
            None => Err(CustError::new(format!("undefined variable '{name}'"))),
        }
    }

    fn address_of_scalar(&self, name: &str) -> CustResult<PointerValue> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.values.get(name) {
                return match value {
                    Value::Scalar(_) => Ok(PointerValue::Scalar {
                        scope_id: scope.id,
                        name: name.to_string(),
                    }),
                    Value::Array(_) => Err(CustError::new(format!(
                        "array '{name}' requires indexed address-of"
                    ))),
                    Value::Pointer(_) => Err(CustError::new(format!(
                        "pointer '{name}' cannot be addressed in this pointer milestone"
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
            Expr::AddressOf(name) => self.address_of_scalar(name),
            Expr::AddressOfArray { name, index } => {
                if let Some(Value::Pointer(pointer)) = self.find_variable(name).cloned() {
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
                Some(Value::Pointer(_)) => {
                    let pointer = self.eval_pointer(value)?;
                    if let Some(Value::Pointer(slot)) = self.find_variable_mut(name) {
                        *slot = pointer.clone();
                    }
                    Ok(pointer)
                }
                Some(Value::Scalar(_)) => Err(CustError::new(format!(
                    "variable '{name}' is not a pointer"
                ))),
                Some(Value::Array(_)) => Err(CustError::new(format!(
                    "array '{name}' requires indexed address-of"
                ))),
                None => Err(CustError::new(format!(
                    "assignment to undeclared variable '{name}'"
                ))),
            },
            Expr::Var(name) => match self.find_variable(name) {
                Some(Value::Pointer(pointer)) => Ok(pointer.clone()),
                Some(Value::Array(array)) => Ok(PointerValue::ArrayBase {
                    array: Rc::clone(array),
                    source_name: Some(name.clone()),
                }),
                Some(Value::Scalar(_)) => Err(CustError::new(format!(
                    "variable '{name}' is not a pointer"
                ))),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            _ => Err(CustError::new("expected pointer expression")),
        }
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
                    .and_then(|scope| scope.values.get(name));
                match value {
                    Some(Value::Scalar(value)) => Ok(*value),
                    _ => Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    ))),
                }
            }
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
                let slot = self
                    .scopes
                    .iter_mut()
                    .find(|scope| scope.id == *scope_id)
                    .and_then(|scope| scope.values.get_mut(name));
                match slot {
                    Some(Value::Scalar(slot)) => {
                        *slot = value;
                        Ok(())
                    }
                    _ => Err(CustError::new(format!(
                        "pointer to out-of-scope variable '{name}'"
                    ))),
                }
            }
            PointerValue::ArrayBase { .. } | PointerValue::ArrayElement { .. } => {
                self.assign_pointer_index(pointer, 0, value)
            }
        }
    }

    fn checked_pointer_index(
        &mut self,
        name: &str,
        index: &Expr,
    ) -> CustResult<(Rc<RefCell<ArrayValue>>, Option<String>, usize)> {
        let index_value = self.eval(index)?;
        let pointer = match self.find_variable(name) {
            Some(Value::Pointer(pointer)) => pointer.clone(),
            Some(Value::Scalar(_)) => {
                return Err(CustError::new(format!(
                    "variable '{name}' is not a pointer"
                )));
            }
            Some(Value::Array(_)) => {
                return Err(CustError::new(format!("array '{name}' is not a pointer")));
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
            | (PointerValue::ArrayBase { .. }, PointerValue::Scalar { .. })
            | (PointerValue::ArrayElement { .. }, PointerValue::Scalar { .. }) => false,
        }
    }

    fn eval_truthy(&mut self, expr: &Expr) -> CustResult<bool> {
        match expr {
            Expr::AddressOf(_) | Expr::AddressOfArray { .. } | Expr::StringLiteral(_) => {
                Ok(Self::pointer_truthy(&self.eval_pointer(expr)?))
            }
            Expr::Assign { name, .. } => match self.find_variable(name).cloned() {
                Some(Value::Pointer(_)) => Ok(Self::pointer_truthy(&self.eval_pointer(expr)?)),
                Some(Value::Scalar(_)) => Ok(self.eval(expr)? != 0),
                Some(Value::Array(_)) => Err(CustError::new(format!(
                    "array '{name}' requires indexed assignment"
                ))),
                None => Err(CustError::new(format!(
                    "assignment to undeclared variable '{name}'"
                ))),
            },
            Expr::Var(name) => match self.find_variable(name).cloned() {
                Some(Value::Pointer(pointer)) => Ok(Self::pointer_truthy(&pointer)),
                Some(Value::Array(array)) => Ok(!array.borrow().elements.is_empty()),
                Some(Value::Scalar(value)) => Ok(value != 0),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::Deref(_)
            | Expr::DerefSet { .. }
            | Expr::DerefCompoundSet { .. }
            | Expr::ArrayGet { .. }
            | Expr::ArraySet { .. }
            | Expr::ArrayCompoundSet { .. }
            | Expr::StringGet { .. }
            | Expr::Call { .. } => Ok(self.eval(expr)? != 0),
            Expr::Number(value) => Ok(*value != 0),
            Expr::UnaryPlus(_)
            | Expr::UnaryMinus(_)
            | Expr::LogicalNot(_)
            | Expr::CompoundAssign { .. }
            | Expr::Increment { .. }
            | Expr::Conditional { .. }
            | Expr::Binary(_, _, _) => Ok(self.eval(expr)? != 0),
        }
    }

    fn eval_assignment_expr(&mut self, name: &str, value: &Expr) -> CustResult<i64> {
        match self.find_variable(name).cloned() {
            Some(Value::Scalar(_)) => {
                let value = self.eval(value)?;
                if let Some(Value::Scalar(slot)) = self.find_variable_mut(name) {
                    *slot = value;
                }
                Ok(value)
            }
            Some(Value::Pointer(_)) => Err(CustError::new("pointer value used as scalar")),
            Some(Value::Array(_)) => Err(CustError::new(format!(
                "array '{name}' requires indexed assignment"
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
            Some(Value::Scalar(current)) => {
                let rhs = self.eval(value)?;
                let result = Self::apply_compound_op(current, op, rhs);
                if let Some(Value::Scalar(slot)) = self.find_variable_mut(name) {
                    *slot = result;
                }
                Ok(result)
            }
            Some(Value::Pointer(_)) => Err(CustError::new("pointer arithmetic is not supported")),
            Some(Value::Array(_)) => Err(CustError::new(format!(
                "array '{name}' requires indexed assignment"
            ))),
            None => Err(CustError::new(format!(
                "assignment to undeclared variable '{name}'"
            ))),
        }
    }

    fn apply_compound_op(lhs: i64, op: CompoundOp, rhs: i64) -> i64 {
        match op {
            CompoundOp::Add => lhs + rhs,
            CompoundOp::Sub => lhs - rhs,
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

    fn eval_increment_expr(
        &mut self,
        target: &Expr,
        op: IncrementOp,
        prefix: bool,
    ) -> CustResult<i64> {
        match target {
            Expr::Var(name) => match self.find_variable(name).cloned() {
                Some(Value::Scalar(current)) => {
                    let updated = Self::apply_increment_op(current, op);
                    if let Some(Value::Scalar(slot)) = self.find_variable_mut(name) {
                        *slot = updated;
                    }
                    Ok(Self::increment_result(current, updated, prefix))
                }
                Some(Value::Pointer(_)) => {
                    Err(CustError::new("pointer arithmetic is not supported"))
                }
                Some(Value::Array(_)) => Err(CustError::new(format!(
                    "array '{name}' requires indexed assignment"
                ))),
                None => Err(CustError::new(format!("undefined variable '{name}'"))),
            },
            Expr::ArrayGet { name, index } => {
                let (array, index) = match self.find_variable(name).cloned() {
                    Some(Value::Pointer(pointer)) => {
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
            Expr::Deref(pointer) => {
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
            Some(Value::Pointer(pointer)) => {
                let index_value = self.eval(index)?;
                let (array, _, index) = self.checked_pointer_value_index(&pointer, index_value)?;
                (array, index)
            }
            Some(_) | None => self.checked_array_index(name, index)?,
        };
        let current = array.borrow().elements[index];
        let rhs = self.eval(value)?;
        let result = Self::apply_compound_op(current, op, rhs);
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
        let pointer = self.eval_pointer(pointer)?;
        let current = self.deref_pointer(&pointer)?;
        let rhs = self.eval(value)?;
        let result = Self::apply_compound_op(current, op, rhs);
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

    fn exec_stmt(&mut self, stmt: &Stmt) -> CustResult<ExecFlow> {
        match stmt {
            Stmt::Empty => Ok(ExecFlow::None),
            Stmt::VarDecl(name, expr) => {
                let value = self.eval(expr)?;
                let scope = self.current_scope_mut();
                if scope.contains_key(name) {
                    return Err(CustError::new(format!(
                        "variable '{name}' already declared in this scope"
                    )));
                }
                scope.insert(name.clone(), Value::Scalar(value));
                Ok(ExecFlow::None)
            }
            Stmt::PointerDecl(name, expr) => {
                let pointer = self.eval_pointer(expr)?;
                let scope = self.current_scope_mut();
                if scope.contains_key(name) {
                    return Err(CustError::new(format!(
                        "variable '{name}' already declared in this scope"
                    )));
                }
                scope.insert(name.clone(), Value::Pointer(pointer));
                Ok(ExecFlow::None)
            }
            Stmt::ArrayDecl { name, len } => {
                let scope = self.current_scope_mut();
                if scope.contains_key(name) {
                    return Err(CustError::new(format!(
                        "variable '{name}' already declared in this scope"
                    )));
                }
                scope.insert(
                    name.clone(),
                    Value::Array(Rc::new(RefCell::new(ArrayValue::mutable_zeroed(*len)))),
                );
                Ok(ExecFlow::None)
            }
            Stmt::Assign(name, expr) => {
                let existing = self.find_variable(name).cloned();
                match existing {
                    Some(Value::Scalar(_)) => {
                        let value = self.eval(expr)?;
                        if let Some(Value::Scalar(slot)) = self.find_variable_mut(name) {
                            *slot = value;
                        }
                        Ok(ExecFlow::None)
                    }
                    Some(Value::Pointer(_)) => {
                        let pointer = self.eval_pointer(expr)?;
                        if let Some(Value::Pointer(slot)) = self.find_variable_mut(name) {
                            *slot = pointer;
                        }
                        Ok(ExecFlow::None)
                    }
                    Some(Value::Array(_)) => Err(CustError::new(format!(
                        "array '{name}' requires indexed assignment"
                    ))),
                    None => Err(CustError::new(format!(
                        "assignment to undeclared variable '{name}'"
                    ))),
                }
            }
            Stmt::DerefAssign { pointer, value } => {
                let pointer = self.eval_pointer(pointer)?;
                let value = self.eval(value)?;
                self.assign_deref_pointer(&pointer, value)?;
                Ok(ExecFlow::None)
            }
            Stmt::ArrayAssign { name, index, value } => {
                let value = self.eval(value)?;
                match self.find_variable(name).cloned() {
                    Some(Value::Pointer(pointer)) => {
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
            Stmt::Expr(expr) => {
                self.eval(expr)?;
                Ok(ExecFlow::None)
            }
            Stmt::Return(expr) => Ok(ExecFlow::Return(self.eval(expr)?)),
            Stmt::Break => Ok(ExecFlow::Break),
            Stmt::Continue => Ok(ExecFlow::Continue),
            Stmt::Block(statements) => self.exec_block(statements),
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                if self.eval_truthy(cond)? {
                    self.exec_block(then_branch)
                } else {
                    self.exec_block(else_branch)
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
                    match self.exec_block(body)? {
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
        }
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

            match self.exec_block(body)? {
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

            match self.exec_block(body)? {
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
                    Some(Value::Pointer(pointer)) => {
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
                let pointer = self.eval_pointer(pointer)?;
                let value = self.eval(value)?;
                self.assign_deref_pointer(&pointer, value)?;
                Ok(value)
            }
            Expr::DerefCompoundSet { pointer, op, value } => {
                self.eval_deref_compound_set(pointer, *op, value)
            }
            Expr::Deref(pointer) => {
                let pointer = self.eval_pointer(pointer)?;
                self.deref_pointer(&pointer)
            }
            Expr::ArrayGet { name, index } => match self.find_variable(name).cloned() {
                Some(Value::Pointer(_)) => {
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
            Expr::Call { name, args } => self.call_function(name, args),
            Expr::UnaryPlus(inner) => self.eval(inner),
            Expr::UnaryMinus(inner) => Ok(-self.eval(inner)?),
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
                        return Err(CustError::new("pointer arithmetic is not supported"));
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
