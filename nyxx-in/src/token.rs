pub enum TokenValue {
    // Single-char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two characters tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Ident(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl std::fmt::Display for TokenValue {
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


/// Represents a position in the source file.
pub struct Pos {
    line: usize,
    col: usize,
}

impl Pos {
    pub fn new(line: usize, col: usize) -> Self {
        Pos { line, col }
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ln {}, Col {}", self.line, self.col)
    }
}

/// Represents a span in the text. It starts at a line:column and ends
/// at the line_end:column_end (inclusive). Line starts at 1, column at 0.
pub struct Span {
    start: Pos,
    end: Pos,
}
