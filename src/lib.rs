use std::collections::HashMap;
use std::error::Error;
use std::fmt;

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
    For,
    Break,
    Continue,
    Ident(String),
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
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
    LBrace,
    RBrace,
    Comma,
    Semi,
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
    Var(String),
    Call { name: String, args: Vec<Expr> },
    UnaryPlus(Box<Expr>),
    UnaryMinus(Box<Expr>),
    LogicalNot(Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Program {
    functions: HashMap<String, Function>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Function {
    params: Vec<String>,
    body: Vec<Stmt>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Stmt {
    Empty,
    VarDecl(String, Expr),
    Assign(String, Expr),
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
/// - `for (initializer; condition; increment) { ... }`
/// - `break;` and `continue;` inside loops
/// - empty statements (`;`) and side-effect-free expression statements (`expr;`)
/// - integer arithmetic/comparisons/logical operators: `+ - * / % == != < <= > >= && || !`
pub fn interpret(source: &str) -> CustResult<i64> {
    let tokens = lex(source)?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let mut interpreter = Interpreter::default();
    interpreter.run(&program)
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
                    "for" => Token::For,
                    "break" => Token::Break,
                    "continue" => Token::Continue,
                    _ => Token::Ident(text),
                };
                tokens.push(LocatedToken::new(kind, start_line, start_column));
            }
            '+' => {
                push_token(&mut tokens, Token::Plus, line, column);
                advance_position(c, &mut line, &mut column, &mut i);
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
        let name = self.expect_ident()?;
        self.expect(Token::LParen)?;
        let params = self.parse_params()?;
        self.expect(Token::RParen)?;
        let body = self.parse_block()?;
        Ok((name, Function { params, body }))
    }

    fn parse_params(&mut self) -> CustResult<Vec<String>> {
        let mut params = Vec::new();
        if self.check(&Token::RParen) {
            return Ok(params);
        }

        loop {
            self.expect_type()?;
            params.push(self.expect_ident()?);

            if self.matches(&Token::Comma) {
                if self.check(&Token::RParen) {
                    return Err(Self::error_at(
                        format!(
                            "expected function parameter after ',', found {:?}",
                            self.peek()
                        ),
                        self.peek_located(),
                    ));
                }
            } else if self.check(&Token::RParen) {
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

    fn parse_block(&mut self) -> CustResult<Vec<Stmt>> {
        self.expect(Token::LBrace)?;
        let mut statements = Vec::new();
        while !self.check(&Token::RBrace) {
            if self.check(&Token::Eof) {
                let eof = self.peek_located().clone();
                return Err(Self::error_at("unterminated block".to_string(), &eof));
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
            Token::LBrace => Ok(Stmt::Block(self.parse_block()?)),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
            Token::Ident(_) if self.peek_next() == &Token::Assign => self.parse_assign(),
            Token::Ident(_)
            | Token::Number(_)
            | Token::Plus
            | Token::Minus
            | Token::Bang
            | Token::LParen => self.parse_expr_stmt_with_semi(true),
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
        let name = self.expect_ident()?;
        self.expect(Token::Assign)?;
        let expr = self.parse_expr()?;
        if require_semi {
            self.expect(Token::Semi)?;
        }
        Ok(Stmt::VarDecl(name, expr))
    }

    fn parse_assign(&mut self) -> CustResult<Stmt> {
        self.parse_assign_with_semi(true)
    }

    fn parse_assign_with_semi(&mut self, require_semi: bool) -> CustResult<Stmt> {
        let name = self.expect_ident()?;
        self.expect(Token::Assign)?;
        let expr = self.parse_expr()?;
        if require_semi {
            self.expect(Token::Semi)?;
        }
        Ok(Stmt::Assign(name, expr))
    }

    fn parse_expr_stmt_with_semi(&mut self, require_semi: bool) -> CustResult<Stmt> {
        let expr = self.parse_expr()?;
        if require_semi {
            self.expect(Token::Semi)?;
        }
        Ok(Stmt::Expr(expr))
    }

    fn parse_return(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Return)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semi)?;
        Ok(Stmt::Return(expr))
    }

    fn parse_break(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Break)?;
        self.expect(Token::Semi)?;
        Ok(Stmt::Break)
    }

    fn parse_continue(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Continue)?;
        self.expect(Token::Semi)?;
        Ok(Stmt::Continue)
    }

    fn parse_if(&mut self) -> CustResult<Stmt> {
        self.expect(Token::If)?;
        self.expect(Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(Token::RParen)?;
        let then_branch = self.parse_block()?;
        let else_branch = if self.matches(&Token::Else) {
            self.parse_block()?
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
        self.expect(Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(Token::RParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::While { cond, body })
    }

    fn parse_for(&mut self) -> CustResult<Stmt> {
        self.expect(Token::For)?;
        self.expect(Token::LParen)?;

        let init = if self.matches(&Token::Semi) {
            None
        } else if matches!(self.peek(), Token::Int | Token::Char) {
            Some(Box::new(self.parse_var_decl()?))
        } else if matches!(self.peek(), Token::Ident(_)) && self.peek_next() == &Token::Assign {
            Some(Box::new(self.parse_assign()?))
        } else if self.starts_expr() {
            Some(Box::new(self.parse_expr_stmt_with_semi(true)?))
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
            self.expect(Token::Semi)?;
            Some(expr)
        };

        let increment = if self.check(&Token::RParen) {
            None
        } else if matches!(self.peek(), Token::Ident(_)) && self.peek_next() == &Token::Assign {
            Some(Box::new(self.parse_assign_with_semi(false)?))
        } else if self.starts_expr() {
            Some(Box::new(self.parse_expr_stmt_with_semi(false)?))
        } else {
            return Err(Self::error_at(
                format!("unexpected token in for increment: {:?}", self.peek()),
                self.peek_located(),
            ));
        };
        self.expect(Token::RParen)?;

        let body = self.parse_block()?;
        Ok(Stmt::For {
            init,
            cond,
            increment,
            body,
        })
    }

    fn parse_expr(&mut self) -> CustResult<Expr> {
        self.parse_logical_or()
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
        if self.matches(&Token::Plus) {
            Ok(Expr::UnaryPlus(Box::new(self.parse_unary()?)))
        } else if self.matches(&Token::Minus) {
            Ok(Expr::UnaryMinus(Box::new(self.parse_unary()?)))
        } else if self.matches(&Token::Bang) {
            Ok(Expr::LogicalNot(Box::new(self.parse_unary()?)))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> CustResult<Expr> {
        let found = self.advance();
        match found.kind.clone() {
            Token::Number(value) => Ok(Expr::Number(value)),
            Token::Ident(name) => {
                if self.matches(&Token::LParen) {
                    let args = self.parse_call_args()?;
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Token::LParen => {
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
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
                if self.check(&Token::RParen) {
                    return Err(Self::error_at(
                        format!(
                            "expected function call argument after ',', found {:?}",
                            self.peek()
                        ),
                        self.peek_located(),
                    ));
                }
            } else if self.check(&Token::RParen) {
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
        matches!(
            self.peek(),
            Token::Ident(_)
                | Token::Number(_)
                | Token::Plus
                | Token::Minus
                | Token::Bang
                | Token::LParen
        )
    }

    fn peek_located(&self) -> &LocatedToken {
        self.tokens
            .get(self.pos)
            .expect("lexer always appends an EOF token")
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

const MAX_CALL_DEPTH: usize = 256;

#[derive(Default)]
struct Interpreter {
    scopes: Vec<HashMap<String, i64>>,
    functions: HashMap<String, Function>,
    call_depth: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecFlow {
    None,
    Return(i64),
    Break,
    Continue,
}

impl Interpreter {
    fn run(&mut self, program: &Program) -> CustResult<i64> {
        self.functions = program.functions.clone();
        self.call_function("main", &[])
    }

    fn call_function(&mut self, name: &str, args: &[i64]) -> CustResult<i64> {
        let function = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| CustError::new(format!("undefined function '{name}'")))?;

        if function.params.len() != args.len() {
            return Err(CustError::new(format!(
                "function '{name}' expected {} arguments, got {}",
                function.params.len(),
                args.len()
            )));
        }

        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(CustError::new(format!(
                "function call depth limit exceeded while calling '{name}'"
            )));
        }

        self.call_depth += 1;

        let mut param_scope = HashMap::new();
        for (param, arg) in function.params.iter().zip(args) {
            if param_scope.insert(param.clone(), *arg).is_some() {
                self.call_depth -= 1;
                return Err(CustError::new(format!(
                    "parameter '{param}' already declared in this function"
                )));
            }
        }

        self.scopes.push(param_scope);
        let result = match self.exec_block(&function.body) {
            Ok(ExecFlow::Return(value)) => Ok(value),
            Ok(ExecFlow::None) => Err(CustError::new(format!(
                "function '{name}' finished without return"
            ))),
            Ok(ExecFlow::Break) => Err(CustError::new("break outside loop")),
            Ok(ExecFlow::Continue) => Err(CustError::new("continue outside loop")),
            Err(error) => Err(error),
        };
        self.scopes.pop();
        self.call_depth -= 1;
        result
    }

    fn exec_block(&mut self, statements: &[Stmt]) -> CustResult<ExecFlow> {
        self.scopes.push(HashMap::new());
        for stmt in statements {
            match self.exec_stmt(stmt) {
                Ok(ExecFlow::None) => {}
                Ok(flow) => {
                    self.scopes.pop();
                    return Ok(flow);
                }
                Err(error) => {
                    self.scopes.pop();
                    return Err(error);
                }
            }
        }
        self.scopes.pop();
        Ok(ExecFlow::None)
    }

    fn current_scope_mut(&mut self) -> &mut HashMap<String, i64> {
        self.scopes
            .last_mut()
            .expect("exec_block always creates a current scope")
    }

    fn find_variable_mut(&mut self, name: &str) -> Option<&mut i64> {
        self.scopes
            .iter_mut()
            .rev()
            .find_map(|scope| scope.get_mut(name))
    }

    fn find_variable(&self, name: &str) -> Option<i64> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
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
                scope.insert(name.clone(), value);
                Ok(ExecFlow::None)
            }
            Stmt::Assign(name, expr) => {
                let value = self.eval(expr)?;
                if let Some(slot) = self.find_variable_mut(name) {
                    *slot = value;
                    Ok(ExecFlow::None)
                } else {
                    Err(CustError::new(format!(
                        "assignment to undeclared variable '{name}'"
                    )))
                }
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
                if self.eval(cond)? != 0 {
                    self.exec_block(then_branch)
                } else {
                    self.exec_block(else_branch)
                }
            }
            Stmt::While { cond, body } => {
                let mut iterations = 0usize;
                while self.eval(cond)? != 0 {
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
        self.scopes.push(HashMap::new());
        let result = self.exec_for_in_current_scope(init, cond, increment, body);
        self.scopes.pop();
        result
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
                Some(cond) if self.eval(cond)? == 0 => break,
                Some(_) | None => {}
            }

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
            Expr::Var(name) => self
                .find_variable(name)
                .ok_or_else(|| CustError::new(format!("undefined variable '{name}'"))),
            Expr::Call { name, args } => {
                let evaluated_args = args
                    .iter()
                    .map(|arg| self.eval(arg))
                    .collect::<CustResult<Vec<_>>>()?;
                self.call_function(name, &evaluated_args)
            }
            Expr::UnaryPlus(inner) => self.eval(inner),
            Expr::UnaryMinus(inner) => Ok(-self.eval(inner)?),
            Expr::LogicalNot(inner) => Ok((self.eval(inner)? == 0) as i64),
            Expr::Binary(left, op, right) => {
                let lhs = self.eval(left)?;
                match op {
                    BinaryOp::LogicalAnd => {
                        if lhs == 0 {
                            return Ok(0);
                        }
                        return Ok((self.eval(right)? != 0) as i64);
                    }
                    BinaryOp::LogicalOr => {
                        if lhs != 0 {
                            return Ok(1);
                        }
                        return Ok((self.eval(right)? != 0) as i64);
                    }
                    _ => {}
                }
                let rhs = self.eval(right)?;
                match op {
                    BinaryOp::Add => Ok(lhs + rhs),
                    BinaryOp::Sub => Ok(lhs - rhs),
                    BinaryOp::Mul => Ok(lhs * rhs),
                    BinaryOp::Div if rhs == 0 => Err(CustError::new("division by zero")),
                    BinaryOp::Div => Ok(lhs / rhs),
                    BinaryOp::Rem if rhs == 0 => Err(CustError::new("division by zero")),
                    BinaryOp::Rem => Ok(lhs % rhs),
                    BinaryOp::Eq => Ok((lhs == rhs) as i64),
                    BinaryOp::Ne => Ok((lhs != rhs) as i64),
                    BinaryOp::Lt => Ok((lhs < rhs) as i64),
                    BinaryOp::Le => Ok((lhs <= rhs) as i64),
                    BinaryOp::Gt => Ok((lhs > rhs) as i64),
                    BinaryOp::Ge => Ok((lhs >= rhs) as i64),
                    BinaryOp::LogicalAnd | BinaryOp::LogicalOr => unreachable!(
                        "logical operators are handled before evaluating the right operand"
                    ),
                }
            }
        }
    }
}
