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
    Return,
    If,
    Else,
    While,
    Ident(String),
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
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
    Semi,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number(i64),
    Var(String),
    UnaryMinus(Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Stmt {
    VarDecl(String, Expr),
    Assign(String, Expr),
    Return(Expr),
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Vec<Stmt>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
}

/// Interpret a small, safe C subset and return `main()`'s integer exit value.
///
/// Supported v0.1 syntax:
/// - `int main() { ... }`
/// - `int name = expression;`
/// - `name = expression;`
/// - `return expression;`
/// - `if (expression) { ... } else { ... }`
/// - `while (expression) { ... }`
/// - integer arithmetic/comparisons: `+ - * / % == != < <= > >=`
pub fn interpret(source: &str) -> CustResult<i64> {
    let tokens = lex(source)?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let mut interpreter = Interpreter::default();
    interpreter.run(&program)
}

fn lex(source: &str) -> CustResult<Vec<Token>> {
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
                    CustError::new(format!(
                        "integer literal out of range at line {start_line}, column {start_column}"
                    ))
                })?;
                tokens.push(Token::Number(value));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    advance_position(chars[i], &mut line, &mut column, &mut i);
                }
                let text: String = chars[start..i].iter().collect();
                tokens.push(match text.as_str() {
                    "int" => Token::Int,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    _ => Token::Ident(text),
                });
            }
            '+' => {
                tokens.push(Token::Plus);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '-' => {
                tokens.push(Token::Minus);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '*' => {
                tokens.push(Token::Star);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '/' => {
                tokens.push(Token::Slash);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '%' => {
                tokens.push(Token::Percent);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '(' => {
                tokens.push(Token::LParen);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            ')' => {
                tokens.push(Token::RParen);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '{' => {
                tokens.push(Token::LBrace);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '}' => {
                tokens.push(Token::RBrace);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            ';' => {
                tokens.push(Token::Semi);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '=' if chars.get(i + 1) == Some(&'=') => {
                tokens.push(Token::Eq);
                advance_position('=', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '!' if chars.get(i + 1) == Some(&'=') => {
                tokens.push(Token::Ne);
                advance_position('!', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '<' if chars.get(i + 1) == Some(&'=') => {
                tokens.push(Token::Le);
                advance_position('<', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '>' if chars.get(i + 1) == Some(&'=') => {
                tokens.push(Token::Ge);
                advance_position('>', &mut line, &mut column, &mut i);
                advance_position('=', &mut line, &mut column, &mut i);
            }
            '=' => {
                tokens.push(Token::Assign);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '<' => {
                tokens.push(Token::Lt);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            '>' => {
                tokens.push(Token::Gt);
                advance_position(c, &mut line, &mut column, &mut i);
            }
            other => {
                return Err(CustError::new(format!(
                    "unexpected character '{other}' at line {line}, column {column}"
                )));
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
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
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse_program(&mut self) -> CustResult<Vec<Stmt>> {
        self.expect(Token::Int)?;
        self.expect_ident_named("main")?;
        self.expect(Token::LParen)?;
        self.expect(Token::RParen)?;
        let body = self.parse_block()?;
        self.expect(Token::Eof)?;
        Ok(body)
    }

    fn parse_block(&mut self) -> CustResult<Vec<Stmt>> {
        self.expect(Token::LBrace)?;
        let mut statements = Vec::new();
        while !self.check(&Token::RBrace) {
            if self.check(&Token::Eof) {
                return Err(CustError::new("unterminated block"));
            }
            statements.push(self.parse_stmt()?);
        }
        self.expect(Token::RBrace)?;
        Ok(statements)
    }

    fn parse_stmt(&mut self) -> CustResult<Stmt> {
        match self.peek() {
            Token::Int => self.parse_var_decl(),
            Token::Return => self.parse_return(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Ident(_) => self.parse_assign(),
            token => Err(CustError::new(format!(
                "unexpected token in statement: {token:?}"
            ))),
        }
    }

    fn parse_var_decl(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Int)?;
        let name = self.expect_ident()?;
        self.expect(Token::Assign)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semi)?;
        Ok(Stmt::VarDecl(name, expr))
    }

    fn parse_assign(&mut self) -> CustResult<Stmt> {
        let name = self.expect_ident()?;
        self.expect(Token::Assign)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semi)?;
        Ok(Stmt::Assign(name, expr))
    }

    fn parse_return(&mut self) -> CustResult<Stmt> {
        self.expect(Token::Return)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semi)?;
        Ok(Stmt::Return(expr))
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

    fn parse_expr(&mut self) -> CustResult<Expr> {
        self.parse_equality()
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
        if self.matches(&Token::Minus) {
            Ok(Expr::UnaryMinus(Box::new(self.parse_unary()?)))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> CustResult<Expr> {
        match self.advance() {
            Token::Number(value) => Ok(Expr::Number(value)),
            Token::Ident(name) => Ok(Expr::Var(name)),
            Token::LParen => {
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            token => Err(CustError::new(format!(
                "expected expression, found {token:?}"
            ))),
        }
    }

    fn expect(&mut self, expected: Token) -> CustResult<()> {
        let found = self.advance();
        if found == expected {
            Ok(())
        } else {
            Err(CustError::new(format!(
                "expected {expected:?}, found {found:?}"
            )))
        }
    }

    fn expect_ident(&mut self) -> CustResult<String> {
        match self.advance() {
            Token::Ident(name) => Ok(name),
            token => Err(CustError::new(format!(
                "expected identifier, found {token:?}"
            ))),
        }
    }

    fn expect_ident_named(&mut self, expected: &str) -> CustResult<()> {
        let name = self.expect_ident()?;
        if name == expected {
            Ok(())
        } else {
            Err(CustError::new(format!(
                "expected function '{expected}', found '{name}'"
            )))
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
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        if !matches!(token, Token::Eof) {
            self.pos += 1;
        }
        token
    }
}

#[derive(Default)]
struct Interpreter {
    env: HashMap<String, i64>,
}

impl Interpreter {
    fn run(&mut self, statements: &[Stmt]) -> CustResult<i64> {
        match self.exec_block(statements)? {
            Some(value) => Ok(value),
            None => Err(CustError::new("main() finished without return")),
        }
    }

    fn exec_block(&mut self, statements: &[Stmt]) -> CustResult<Option<i64>> {
        for stmt in statements {
            if let Some(value) = self.exec_stmt(stmt)? {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> CustResult<Option<i64>> {
        match stmt {
            Stmt::VarDecl(name, expr) => {
                if self.env.contains_key(name) {
                    return Err(CustError::new(format!(
                        "variable '{name}' already declared"
                    )));
                }
                let value = self.eval(expr)?;
                self.env.insert(name.clone(), value);
                Ok(None)
            }
            Stmt::Assign(name, expr) => {
                if !self.env.contains_key(name) {
                    return Err(CustError::new(format!(
                        "assignment to undeclared variable '{name}'"
                    )));
                }
                let value = self.eval(expr)?;
                self.env.insert(name.clone(), value);
                Ok(None)
            }
            Stmt::Return(expr) => Ok(Some(self.eval(expr)?)),
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
                    if let Some(value) = self.exec_block(body)? {
                        return Ok(Some(value));
                    }
                }
                Ok(None)
            }
        }
    }

    fn eval(&self, expr: &Expr) -> CustResult<i64> {
        match expr {
            Expr::Number(value) => Ok(*value),
            Expr::Var(name) => self
                .env
                .get(name)
                .copied()
                .ok_or_else(|| CustError::new(format!("undefined variable '{name}'"))),
            Expr::UnaryMinus(inner) => Ok(-self.eval(inner)?),
            Expr::Binary(left, op, right) => {
                let lhs = self.eval(left)?;
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
                }
            }
        }
    }
}
