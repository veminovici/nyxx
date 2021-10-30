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

    /// Create a identifier token
    #[inline]
    pub(crate) fn identifier(i: &str, span: Span) -> Self {
        Token::new(TokenValue::Ident(i.to_string()), span)
    }
}

//
// Formatting
//

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.tkn_value, self.span)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} @ {:?}", self.tkn_value, self.span)
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_left_paren() {
        let t = Token::left_paren(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::LeftParen));
    }

    #[test]
    fn test_token_right_paren() {
        let t = Token::right_paren(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::RightParen));
    }

    #[test]
    fn test_token_left_brace() {
        let t = Token::left_brace(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::LeftBrace));
    }

    #[test]
    fn test_token_right_brace() {
        let t = Token::right_brace(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::RightBrace));
    }

    #[test]
    fn test_token_comma() {
        let t = Token::comma(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Comma));
    }

    #[test]
    fn test_token_dot() {
        let t = Token::dot(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Dot));
    }

    #[test]
    fn test_token_minus() {
        let t = Token::minus(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Minus));
    }

    #[test]
    fn test_token_plus() {
        let t = Token::plus(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Plus));
    }

    #[test]
    fn test_token_semicolon() {
        let t = Token::semicolon(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Semicolon));
    }

    #[test]
    fn test_token_star() {
        let t = Token::star(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Star));
    }

    #[test]
    fn test_token_bang() {
        let t = Token::bang(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Bang));
    }

    #[test]
    fn test_token_bang_equal() {
        let t = Token::bang_equal(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::BangEqual));
    }

    #[test]
    fn test_token_equal() {
        let t = Token::equal(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Equal));
    }

    #[test]
    fn test_token_equal_equal() {
        let t = Token::equal_equal(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::EqualEqual));
    }

    #[test]
    fn test_token_less() {
        let t = Token::less(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Less));
    }

    #[test]
    fn test_token_less_equal() {
        let t = Token::less_equal(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::LessEqual));
    }

    #[test]
    fn test_token_greater() {
        let t = Token::greater(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Greater));
    }

    #[test]
    fn test_token_greater_equal() {
        let t = Token::greater_equal(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::GreaterEqual));
    }

    #[test]
    fn test_token_slash() {
        let t = Token::slash(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Slash));
    }

    #[test]
    fn test_token_comment() {
        let t = Token::comment("test".to_string(), Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        match t.tkn_value {
            TokenValue::Comment(c) => assert_eq!(c, "test"),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_token_string() {
        let t = Token::string("test".to_string(), Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        match t.tkn_value {
            TokenValue::String(c) => assert_eq!(c, "test"),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_token_number() {
        let t = Token::number(10., Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        match t.tkn_value {
            TokenValue::Number(c) => assert!((c - 10.) * (c - 10.) < f64::EPSILON),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_token_ws() {
        let t = Token::whitespace("test".to_string(), Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        match t.tkn_value {
            TokenValue::Whitespace(c) => assert_eq!(c, "test"),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_identifier() {
        let t = Token::identifier("language", Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        match t.tkn_value {
            TokenValue::Ident(c) => assert_eq!(c, "language"),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_token_newline() {
        let t = Token::newline(Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::NewLine));
    }

    #[test]
    fn test_token_eof() {
        let t = Token::new(TokenValue::Eof, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Eof));
    }

    #[test]
    fn test_token_and() {
        let t = Token::new(TokenValue::And, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::And));
    }

    #[test]
    fn test_token_or() {
        let t = Token::new(TokenValue::Or, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Or));
    }

    #[test]
    fn test_token_class() {
        let t = Token::new(TokenValue::Class, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Class));
    }

    #[test]
    fn test_token_this() {
        let t = Token::new(TokenValue::This, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::This));
    }

    #[test]
    fn test_token_else() {
        let t = Token::new(TokenValue::Else, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Else));
    }

    #[test]
    fn test_token_false() {
        let t = Token::new(TokenValue::False, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::False));
    }

    #[test]
    fn test_token_true() {
        let t = Token::new(TokenValue::True, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::True));
    }

    #[test]
    fn test_token_fun() {
        let t = Token::new(TokenValue::Fun, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Fun));
    }

    #[test]
    fn test_token_for() {
        let t = Token::new(TokenValue::For, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::For));
    }

    #[test]
    fn test_token_if() {
        let t = Token::new(TokenValue::If, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::If));
    }

    #[test]
    fn test_token_nil() {
        let t = Token::new(TokenValue::Nil, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Nil));
    }

    #[test]
    fn test_token_print() {
        let t = Token::new(TokenValue::Print, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Print));
    }

    #[test]
    fn test_token_return() {
        let t = Token::new(TokenValue::Return, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Return));
    }

    #[test]
    fn test_token_super() {
        let t = Token::new(TokenValue::Super, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Super));
    }

    #[test]
    fn test_token_var() {
        let t = Token::new(TokenValue::Var, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::Var));
    }

    #[test]
    fn test_token_while() {
        let t = Token::new(TokenValue::While, Span::new());
        assert!(!format!("{}", t).is_empty());
        assert!(!format!("{:?}", t).is_empty());
        assert!(matches!(t.tkn_value, TokenValue::While));
    }
}
