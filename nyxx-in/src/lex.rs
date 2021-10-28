use std::iter::Peekable;
use std::str::{Chars, FromStr};

use super::{Span, Token, TokenValue};

const CHAR_LEFT_PAREN: char = '(';
const CHAR_RIGHT_PAREN: char = ')';
const CHAR_LEFT_BRACE: char = '{';
const CHAR_RIGHT_BRACE: char = '}';
const CHAR_COMMA: char = ',';
const CHAR_DOT: char = '.';
const CHAR_MINUS: char = '-';
const CHAR_PLUS: char = '+';
const CHAR_SEMICOLON: char = ';';
const CHAR_STAR: char = '*';
const CHAR_BANG: char = '!';
const CHAR_EQUAL: char = '=';
const CHAR_LESS: char = '<';
const CHAR_GREATER: char = '>';
const CHAR_SLASH: char = '/';
const CHAR_DOUBLE_QUOTE: char = '"';
const CHAR_NEWLINE: char = '\n';
const CHAR_WHITESPACE: char = ' ';
const CHAR_CARRIAGE_RETURN: char = '\r';
const CHAR_TAB: char = '\t';
const CHAR_0: char = '0';
const CHAR_9: char = '9';
const CHAR_LOWERCASE_A: char = 'a';
const CHAR_LOWERCASE_Z: char = 'z';
const CHAR_UPPERCASE_A: char = 'A';
const CHAR_UPPERCASE_Z: char = 'Z';
const CHAR_UNDERSCORE: char = '_';

static KEYWORDS: &[(&str, TokenValue)] = &[
    ("and", TokenValue::And),
    ("class", TokenValue::Class),
    ("else", TokenValue::Else),
    ("false", TokenValue::False),
    ("for", TokenValue::For),
    ("fun", TokenValue::Fun),
    ("if", TokenValue::If),
    ("nil", TokenValue::Nil),
    ("or", TokenValue::Or),
    ("print", TokenValue::Print),
    ("return", TokenValue::Return),
    ("super", TokenValue::Super),
    ("this", TokenValue::This),
    ("true", TokenValue::True),
    ("var", TokenValue::Var),
    ("while", TokenValue::While),
];

/// The lexer for a source string
pub struct Lexer {
    source: String,
}

impl Lexer {
    /// Creates a new lexer
    pub fn new(source: String) -> Self {
        Lexer { source }
    }

    /// returns an iterator
    pub fn iter(&self) -> LexerIter {
        LexerIter {
            ctx: LexContext::new(&self.source),
            eof_sent: false,
        }
    }
}

/// Creates an iterator for the tokens
pub struct LexerIter<'a> {
    ctx: LexContext<'a>,
    eof_sent: bool,
}

impl<'a> Iterator for LexerIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof_sent {
            return None;
        }

        if let Some(c) = self.ctx.read_char() {
            let match_token = match c {
                // Single charactoer token
                CHAR_LEFT_PAREN => Some(self.ctx.left_paren()),
                CHAR_RIGHT_PAREN => Some(self.ctx.right_paren()),
                CHAR_LEFT_BRACE => Some(self.ctx.left_brace()),
                CHAR_RIGHT_BRACE => Some(self.ctx.right_brace()),
                CHAR_COMMA => Some(self.ctx.comma()),
                CHAR_DOT => Some(self.ctx.dot()),
                CHAR_MINUS => Some(self.ctx.minus()),
                CHAR_PLUS => Some(self.ctx.plus()),
                CHAR_SEMICOLON => Some(self.ctx.semicolon()),
                CHAR_STAR => Some(self.ctx.star()),
                CHAR_BANG => {
                    if let Some(CHAR_EQUAL) = self.ctx.peek_char() {
                        let _ = self.ctx.read_char().unwrap();
                        Some(self.ctx.bang_equal())
                    } else {
                        Some(self.ctx.bang())
                    }
                }
                CHAR_EQUAL => {
                    if let Some(CHAR_EQUAL) = self.ctx.peek_char() {
                        let _ = self.ctx.read_char().unwrap();
                        Some(self.ctx.equal_equal())
                    } else {
                        Some(self.ctx.equal())
                    }
                }
                CHAR_LESS => {
                    if let Some(CHAR_EQUAL) = self.ctx.peek_char() {
                        let _ = self.ctx.read_char().unwrap();
                        Some(self.ctx.less_equal())
                    } else {
                        Some(self.ctx.less())
                    }
                }
                CHAR_GREATER => {
                    if let Some(CHAR_EQUAL) = self.ctx.peek_char() {
                        let _ = self.ctx.read_char().unwrap();
                        Some(self.ctx.greater_equal())
                    } else {
                        Some(self.ctx.greater())
                    }
                }
                CHAR_SLASH => {
                    if let Some(CHAR_SLASH) = self.ctx.peek_char() {
                        let cmnt = self.ctx.read_line();
                        Some(self.ctx.comment(cmnt))
                    } else {
                        Some(self.ctx.slash())
                    }
                }
                CHAR_DOUBLE_QUOTE => {
                    if let Some(s) = self.ctx.read_string() {
                        Some(self.ctx.string(s))
                    } else {
                        log::error!("We didnt finish the string");
                        panic!("We should finish the string")
                    }
                }
                CHAR_NEWLINE => Some(self.ctx.newline()),
                ws if is_whitespace(ws) => {
                    let ws = self.ctx.read_ws(ws);
                    Some(self.ctx.whitespace(ws))
                }
                // Digit
                digit if is_digit(digit) => {
                    if let Some(number) = self.ctx.read_number(digit) {
                        Some(self.ctx.number(number))
                    } else {
                        log::error!("We didnt finish the number");
                        panic!("We should finish the number")
                    }
                }
                // Alpha
                alpha if is_alpha(alpha) => {
                    let ident = self.ctx.read_identifier(alpha);

                    let srch = KEYWORDS.binary_search_by_key(&ident.as_str(), |&(k, _)| k);
                    let token_value = match srch {
                        Ok(index) => KEYWORDS[index].1.clone(),
                        Err(_) => TokenValue::Ident(ident),
                    };

                    Some(self.ctx.token(token_value))
                }
                unexpected => panic!("Unknown char {}", unexpected),
            };

            return match_token;
        }

        self.eof_sent = true;
        Some(Token::new(TokenValue::Eof, self.ctx.span))
    }
}

//
// Lexer context
//

struct LexContext<'a> {
    source: Peekable<Chars<'a>>,
    span: Span,
}

impl<'a> LexContext<'a> {
    /// Creates a new instace of the lex context
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            span: Span::new(),
        }
    }

    /// Peeks to the first chararcter in the stream.
    pub(crate) fn peek_char(&mut self) -> Option<char> {
        self.source.peek().copied()
    }

    /// Read till the end of the line
    pub(crate) fn read_line(&mut self) -> String {
        let mut buffer = String::new();

        for c in &mut self.source {
            self.span.new_column();
            buffer.push(c);

            if c == CHAR_NEWLINE {
                self.span.new_line();
                break;
            }
        }

        buffer
    }

    /// Read the next character in the stream
    pub(crate) fn read_char(&mut self) -> Option<char> {
        if let Some(c) = self.source.next() {
            self.span.new_column();
            if c == CHAR_NEWLINE {
                self.span.new_line();
            }

            Some(c)
        } else {
            None
        }
    }

    /// Read an identifier
    pub(crate) fn read_identifier(&mut self, first_alpha: char) -> String {
        let mut buffer = format!("{}", first_alpha);

        while let Some(maybe_alphanumeric) = self.source.peek() {
            if is_alphanum(*maybe_alphanumeric) {
                buffer.push(*maybe_alphanumeric);
                self.read_char();
            } else {
                break;
            }
        }

        buffer
    }

    /// Read a string
    pub(crate) fn read_string(&mut self) -> Option<String> {
        let mut buffer = String::new();
        let mut string_terminated = false;

        for c in &mut self.source {
            self.span.new_column();
            if c == CHAR_NEWLINE {
                self.span.new_line();
            }

            if c == CHAR_DOUBLE_QUOTE {
                string_terminated = true;
                break;
            }

            buffer.push(c);
        }

        if string_terminated {
            Some(buffer)
        } else {
            None
        }
    }

    pub(crate) fn read_number(&mut self, first_digit: char) -> Option<f64> {
        let mut buffer = format!("{}", first_digit);

        // Read leading digits
        while let Some(maybe_digit) = self.source.peek().copied() {
            if is_digit(maybe_digit) {
                buffer.push(maybe_digit);
                self.read_char();
            } else {
                break;
            }
        }

        // Try reading "." and the rest of the digits
        if let Some(maybe_dot) = self.source.peek().copied() {
            if maybe_dot == CHAR_DOT {
                buffer.push(maybe_dot);
                self.read_char();

                let mut read_additional_digits = false;

                while let Some(maybe_digit) = self.source.peek().copied() {
                    if is_digit(maybe_digit) {
                        buffer.push(maybe_digit);
                        self.read_char();
                        read_additional_digits = true;
                    } else {
                        break;
                    }
                }

                // Lox does not support leading or trailing dot in
                // number literals. This is not a valid number
                // literal, if we encountered no digits after ".".
                // Also note: we have to error here, because we
                // already consumed at least the "." from the input
                // and would have to "return" it if we didn't match
                // something. Fortunately there is nothing in Lox yet
                // that would require us to recover (e.g. methods on
                // numbers -> "4.sqrt()")
                if !read_additional_digits {
                    return None;
                }
            }
        }

        if let Ok(number) = f64::from_str(&buffer) {
            Some(number)
        } else {
            None
        }
    }

    pub(crate) fn read_ws(&mut self, first_ws: char) -> String {
        let mut buffer = format!("{}", first_ws);

        while let Some(maybe_ws) = self.source.peek().copied() {
            if is_whitespace(maybe_ws) {
                buffer.push(maybe_ws);
                self.read_char();
            } else {
                break;
            }
        }

        buffer
    }

    pub(crate) fn token(&mut self, token_value: TokenValue) -> Token {
        Token::new(token_value, self.span.extract())
    }

    #[inline]
    pub(crate) fn left_paren(&mut self) -> Token {
        Token::left_paren(self.span.extract())
    }

    #[inline]
    pub(crate) fn right_paren(&mut self) -> Token {
        Token::right_paren(self.span.extract())
    }

    #[inline]
    pub(crate) fn left_brace(&mut self) -> Token {
        Token::left_brace(self.span.extract())
    }

    #[inline]
    pub(crate) fn right_brace(&mut self) -> Token {
        Token::right_brace(self.span.extract())
    }

    #[inline]
    pub(crate) fn comma(&mut self) -> Token {
        Token::comma(self.span.extract())
    }

    #[inline]
    pub(crate) fn dot(&mut self) -> Token {
        Token::dot(self.span.extract())
    }

    #[inline]
    pub(crate) fn minus(&mut self) -> Token {
        Token::minus(self.span.extract())
    }

    #[inline]
    pub(crate) fn plus(&mut self) -> Token {
        Token::plus(self.span.extract())
    }

    #[inline]
    pub(crate) fn semicolon(&mut self) -> Token {
        Token::semicolon(self.span.extract())
    }

    #[inline]
    pub(crate) fn star(&mut self) -> Token {
        Token::star(self.span.extract())
    }

    #[inline]
    pub(crate) fn bang(&mut self) -> Token {
        Token::bang(self.span.extract())
    }

    #[inline]
    pub(crate) fn bang_equal(&mut self) -> Token {
        Token::bang_equal(self.span.extract())
    }

    #[inline]
    pub(crate) fn equal(&mut self) -> Token {
        Token::equal(self.span.extract())
    }

    #[inline]
    pub(crate) fn equal_equal(&mut self) -> Token {
        Token::equal_equal(self.span.extract())
    }

    #[inline]
    pub(crate) fn less(&mut self) -> Token {
        Token::less(self.span.extract())
    }

    #[inline]
    pub(crate) fn less_equal(&mut self) -> Token {
        Token::less_equal(self.span.extract())
    }

    #[inline]
    pub(crate) fn greater(&mut self) -> Token {
        Token::greater(self.span.extract())
    }

    #[inline]
    pub(crate) fn greater_equal(&mut self) -> Token {
        Token::greater_equal(self.span.extract())
    }

    #[inline]
    pub(crate) fn slash(&mut self) -> Token {
        Token::slash(self.span.extract())
    }

    #[inline]
    pub(crate) fn comment(&mut self, c: String) -> Token {
        Token::comment(c, self.span.extract())
    }

    #[inline]
    pub(crate) fn string(&mut self, s: String) -> Token {
        Token::string(s, self.span.extract())
    }

    #[inline]
    pub(crate) fn newline(&mut self) -> Token {
        Token::newline(self.span.extract())
    }

    #[inline]
    pub(crate) fn whitespace(&mut self, ws: String) -> Token {
        Token::whitespace(ws, self.span.extract())
    }

    #[inline]
    pub(crate) fn number(&mut self, number: f64) -> Token {
        Token::number(number, self.span.extract())
    }
}

//
// Utility functions
//

fn is_digit(c: char) -> bool {
    c >= CHAR_0 && c <= CHAR_9
}

fn is_alpha(c: char) -> bool {
    c >= CHAR_LOWERCASE_A && c <= CHAR_LOWERCASE_Z
        || c >= CHAR_UPPERCASE_A && c <= CHAR_UPPERCASE_Z
        || c == CHAR_UNDERSCORE
}

fn is_alphanum(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_whitespace(c: char) -> bool {
    c == CHAR_WHITESPACE || c == CHAR_TAB || c == CHAR_CARRIAGE_RETURN
}
