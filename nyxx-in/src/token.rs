use super::Span;
use std::fmt::{Debug, Display};

/// Represents the tokens supported by the language.
#[derive(Clone)]
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
    /// Comment
    Comment(String),
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
    // Others
    /// Whitespace
    Whitespace(String),
    /// New line
    NewLine,
    /// EOF
    Eof,
}

impl Debug for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::And => write!(f, "AND"),
            TokenValue::Bang => write!(f, "BANG"),
            TokenValue::BangEqual => write!(f, "BANG_EQUAL"),
            TokenValue::Class => write!(f, "CLASS"),
            TokenValue::Comma => write!(f, "COMMA"),
            TokenValue::Comment(c) => write!(f, "COMM({})", c),
            TokenValue::Dot => write!(f, "DOT"),
            TokenValue::Else => write!(f, "ELSE"),
            TokenValue::Eof => write!(f, "EOF"),
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
            TokenValue::NewLine => write!(f, "NEWLINE"),
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
            TokenValue::Whitespace(w) => write!(f, "WS({})", w),
            TokenValue::While => write!(f, "WHILE"),
        }
    }
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::And => write!(f, "and"),
            TokenValue::Bang => write!(f, "!"),
            TokenValue::BangEqual => write!(f, "!="),
            TokenValue::Class => write!(f, "class"),
            TokenValue::Comma => write!(f, ","),
            TokenValue::Comment(c) => write!(f, "// {}", c),
            TokenValue::Dot => write!(f, "."),
            TokenValue::Else => write!(f, "else"),
            TokenValue::Eof => write!(f, "eof"),
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
            TokenValue::NewLine => write!(f, "<nl>"),
            TokenValue::Nil => write!(f, "nil"),
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
            TokenValue::Whitespace(_) => write!(f, "<ws>"),
            TokenValue::While => write!(f, "while"),
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
    pub(crate) fn new(tkn_value: TokenValue, span: Span) -> Self {
        Self { tkn_value, span }
    }

    /// Create a new left-paren token
    #[inline]
    pub(crate) fn left_paren(span: Span) -> Self {
        Token::new(TokenValue::LeftParen, span)
    }

    /// Create a new right-paren token
    #[inline]
    pub(crate) fn right_paren(span: Span) -> Self {
        Token::new(TokenValue::RightParen, span)
    }

    /// Create a new left-brace token
    #[inline]
    pub(crate) fn left_brace(span: Span) -> Self {
        Token::new(TokenValue::LeftBrace, span)
    }

    /// Create a new right-brace token
    #[inline]
    pub(crate) fn right_brace(span: Span) -> Self {
        Token::new(TokenValue::RightBrace, span)
    }

    /// Create a new comma token
    #[inline]
    pub(crate) fn comma(span: Span) -> Self {
        Token::new(TokenValue::Comma, span)
    }

    /// Create a new dot token
    #[inline]
    pub(crate) fn dot(span: Span) -> Self {
        Token::new(TokenValue::Dot, span)
    }

    /// Create a new minus token
    #[inline]
    pub(crate) fn minus(span: Span) -> Self {
        Token::new(TokenValue::Minus, span)
    }

    /// Create a new plus token
    #[inline]
    pub(crate) fn plus(span: Span) -> Self {
        Token::new(TokenValue::Plus, span)
    }

    /// Create a new semicolon token
    #[inline]
    pub(crate) fn semicolon(span: Span) -> Self {
        Token::new(TokenValue::Semicolon, span)
    }

    /// Create a new star token
    #[inline]
    pub(crate) fn star(span: Span) -> Self {
        Token::new(TokenValue::Star, span)
    }

    /// Create a new bang token
    #[inline]
    pub(crate) fn bang(span: Span) -> Self {
        Token::new(TokenValue::Bang, span)
    }

    /// Create a new bang-equal token
    #[inline]
    pub(crate) fn bang_equal(span: Span) -> Self {
        Token::new(TokenValue::BangEqual, span)
    }

    /// Create a new equal token
    #[inline]
    pub(crate) fn equal(span: Span) -> Self {
        Token::new(TokenValue::Equal, span)
    }

    /// Create a new equal-equal token
    #[inline]
    pub(crate) fn equal_equal(span: Span) -> Self {
        Token::new(TokenValue::EqualEqual, span)
    }

    /// Create a new less token
    #[inline]
    pub(crate) fn less(span: Span) -> Self {
        Token::new(TokenValue::Less, span)
    }

    /// Create a new less-equal token
    #[inline]
    pub(crate) fn less_equal(span: Span) -> Self {
        Token::new(TokenValue::LessEqual, span)
    }

    /// Create a new greater token
    #[inline]
    pub(crate) fn greater(span: Span) -> Self {
        Token::new(TokenValue::Greater, span)
    }

    /// Create a new greater-equal token
    #[inline]
    pub(crate) fn greater_equal(span: Span) -> Self {
        Token::new(TokenValue::GreaterEqual, span)
    }

    /// Create a new slash token
    #[inline]
    pub(crate) fn slash(span: Span) -> Self {
        Token::new(TokenValue::Slash, span)
    }

    /// Create a new comment token
    #[inline]
    pub(crate) fn comment(c: String, span: Span) -> Self {
        Token::new(TokenValue::Comment(c), span)
    }

    /// Create a new string token
    #[inline]
    pub(crate) fn string(str: String, span: Span) -> Self {
        Token::new(TokenValue::String(str), span)
    }

    /// Create a new number token
    #[inline]
    pub(crate) fn number(n: f64, span: Span) -> Self {
        Token::new(TokenValue::Number(n), span)
    }

    /// Create a whitespace token
    #[inline]
    pub(crate) fn whitespace(ws: String, span: Span) -> Self {
        Token::new(TokenValue::Whitespace(ws), span)
    }

    /// Create a newline token
    #[inline]
    pub(crate) fn newline(span: Span) -> Self {
        Token::new(TokenValue::NewLine, span)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.tkn_value, self.span)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}@{:?}", self.tkn_value, self.span)
    }
}
