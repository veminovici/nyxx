use std::fmt::{Debug, Display};

/// Represents the tokens supported by the language.
pub enum TokenValue {
    // Single-char tokens
    /// Left paren
    LeftParen,
    /// Right paren
    RightParen,
    /// Left brace
    LeftBrace,
    /// Right brace
    RightBrace,
    /// Comma
    Comma,
    /// Dot
    Dot,
    /// Minus
    Minus,
    /// Plus
    Plus,
    /// Semicolon
    Semicolon,
    /// Slash
    Slash,
    /// Start
    Star,
    // One or two characters tokens
    /// Bang
    Bang,
    /// BangEqual
    BangEqual,
    /// Equal
    Equal,
    /// EqualEqual
    EqualEqual,
    /// Greater
    Greater,
    /// GreaterEqual
    GreaterEqual,
    /// Less
    Less,
    /// LessEqual
    LessEqual,
    // Literals
    /// Identity
    Ident(String),
    /// String
    String(String),
    /// Number
    Number(f64),
    // Keywords
    /// And
    And,
    /// Class
    Class,
    /// Else
    Else,
    /// False
    False,
    /// Fun
    Fun,
    /// For
    For,
    /// If
    If,
    /// Nil
    Nil,
    /// Or
    Or,
    /// Print
    Print,
    /// Return
    Return,
    /// Super
    Super,
    /// This
    This,
    /// True
    True,
    /// Var
    Var,
    /// While
    While,
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::And => write!(f, "AND"),
            TokenValue::Bang => write!(f, "BANG"),
            TokenValue::BangEqual => write!(f, "BANG_EQUAL"),
            TokenValue::Class => write!(f, "CLASS"),
            TokenValue::Comma => write!(f, "COMMA"),
            TokenValue::Dot => write!(f, "DOT"),
            TokenValue::Else => write!(f, "ELSE"),
            TokenValue::Equal => write!(f, "EQUAL"),
            TokenValue::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenValue::False => write!(f, "FALSE"),
            TokenValue::For => write!(f, "FOR"),
            TokenValue::Fun => write!(f, "FUN"),
            TokenValue::Greater => write!(f, "GREATER"),
            TokenValue::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenValue::Ident(s) => write!(f, "IDENT({})", s),
            TokenValue::If => write!(f, "IF"),
            TokenValue::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenValue::LeftParen => write!(f, "LEFT_PAREN"),
            TokenValue::Less => write!(f, "LESS"),
            TokenValue::LessEqual => write!(f, "LESS_EQUAL"),
            TokenValue::Minus => write!(f, "MINUS"),
            TokenValue::Nil => write!(f, "NIL"),
            TokenValue::Number(n) => write!(f, "NUMBER({})", n),
            TokenValue::Or => write!(f, "OR"),
            TokenValue::Plus => write!(f, "PLUS"),
            TokenValue::Print => write!(f, "PRINT"),
            TokenValue::Return => write!(f, "RETURN"),
            TokenValue::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenValue::RightParen => write!(f, "RIGHT_PAREN"),
            TokenValue::Semicolon => write!(f, "SEMICOLON"),
            TokenValue::Slash => write!(f, "SLASH"),
            TokenValue::Star => write!(f, "STAR"),
            TokenValue::String(s) => write!(f, "STRING({})", s),
            TokenValue::Super => write!(f, "SUPER"),
            TokenValue::This => write!(f, "THIS"),
            TokenValue::True => write!(f, "TRUE"),
            TokenValue::Var => write!(f, "VAR"),
            TokenValue::While => write!(f, "WHILE"),
        }
    }
}

impl Debug for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::And => write!(f, "and"),
            TokenValue::Bang => write!(f, "!"),
            TokenValue::BangEqual => write!(f, "!="),
            TokenValue::Class => write!(f, "class"),
            TokenValue::Comma => write!(f, ","),
            TokenValue::Dot => write!(f, "."),
            TokenValue::Else => write!(f, "else"),
            TokenValue::Equal => write!(f, "="),
            TokenValue::EqualEqual => write!(f, "=="),
            TokenValue::False => write!(f, "false"),
            TokenValue::For => write!(f, "for"),
            TokenValue::Fun => write!(f, "fun"),
            TokenValue::Greater => write!(f, ">"),
            TokenValue::GreaterEqual => write!(f, ">="),
            TokenValue::Ident(s) => write!(f, "{}", s),
            TokenValue::If => write!(f, "if"),
            TokenValue::LeftBrace => write!(f, "["),
            TokenValue::LeftParen => write!(f, ")"),
            TokenValue::Less => write!(f, "<"),
            TokenValue::LessEqual => write!(f, "<=>"),
            TokenValue::Minus => write!(f, "-"),
            TokenValue::Nil => write!(f, "nilL"),
            TokenValue::Number(n) => write!(f, "{}", n),
            TokenValue::Or => write!(f, "or"),
            TokenValue::Plus => write!(f, "+"),
            TokenValue::Print => write!(f, "print"),
            TokenValue::Return => write!(f, "return"),
            TokenValue::RightBrace => write!(f, "]"),
            TokenValue::RightParen => write!(f, ")"),
            TokenValue::Semicolon => write!(f, ";"),
            TokenValue::Slash => write!(f, "/"),
            TokenValue::Star => write!(f, "*"),
            TokenValue::String(s) => write!(f, "{}", s),
            TokenValue::Super => write!(f, "super"),
            TokenValue::This => write!(f, "this"),
            TokenValue::True => write!(f, "true"),
            TokenValue::Var => write!(f, "var"),
            TokenValue::While => write!(f, "while"),
        }
    }
}

/// Represents a position in the source file.
#[derive(Clone, Copy)]
pub struct Pos {
    line: usize,
    col: usize,
}

impl Pos {
    /// Creates a new positions
    pub fn new(line: usize, col: usize) -> Self {
        Pos { line, col }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ln {}, Col {}", self.line, self.col)
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

/// Represents a span in the text. It starts at a line:column and ends
/// at the line_end:column_end (inclusive). Line starts at 1, column at 0.
#[derive(Clone, Copy)]
pub struct Span {
    start: Pos,
    end: Pos,
}

impl Span {
    /// Creates a new span instance
    pub fn with_pos(start: Pos, end: Pos) -> Self {
        Span { start, end }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) - ({})", self.start, self.end)
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start.line == self.end.line {
            write!(
                f,
                "{:?}:{:?}-{:?}",
                self.start.line, self.start.col, self.end.col
            )
        } else {
            write!(f, "{:?}-{:?}", self.start, self.end)
        }
    }
}

/// Represents a token found in the source content
pub struct Token {
    tkn_value: TokenValue,
    span: Span,
}

impl Token {
    /// Create a new instance of the token
    pub fn new(tkn_value: TokenValue, span: Span) -> Self {
        Self { tkn_value, span }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.tkn_value, self.span)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({:?}", self.tkn_value, self.span)
    }
}
