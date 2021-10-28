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

/// Parses the source string and returns the collection of tokens
pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut ctx = LexContext::new(source);

    while let Some((c, span)) = ctx.read_char() {
        let match_token = match c {
            // Single charactoer token
            CHAR_LEFT_PAREN => Some(Token::left_paren(span)),
            CHAR_RIGHT_PAREN => Some(Token::right_paren(span)),
            CHAR_LEFT_BRACE => Some(Token::left_brace(span)),
            CHAR_RIGHT_BRACE => Some(Token::right_brace(span)),
            CHAR_COMMA => Some(Token::comma(span)),
            CHAR_DOT => Some(Token::dot(span)),
            CHAR_MINUS => Some(Token::minus(span)),
            CHAR_PLUS => Some(Token::plus(span)),
            CHAR_SEMICOLON => Some(Token::semicolon(span)),
            CHAR_STAR => Some(Token::star(span)),
            CHAR_BANG => {
                if let Some(CHAR_EQUAL) = ctx.peek_char() {
                    let (_, span_end) = ctx.read_char().unwrap();
                    Some(Token::bang_equal(span_end))
                } else {
                    Some(Token::bang(span))
                }
            }
            CHAR_EQUAL => {
                if let Some(CHAR_EQUAL) = ctx.peek_char() {
                    let (_, span_end) = ctx.read_char().unwrap();
                    Some(Token::equal_equal(span_end))
                } else {
                    Some(Token::equal(span))
                }
            }
            CHAR_LESS => {
                if let Some(CHAR_EQUAL) = ctx.peek_char() {
                    let (_, span_end) = ctx.read_char().unwrap();
                    Some(Token::less_equal(span_end))
                } else {
                    Some(Token::less(span))
                }
            }
            CHAR_GREATER => {
                if let Some(CHAR_EQUAL) = ctx.peek_char() {
                    let (_, span_end) = ctx.read_char().unwrap();
                    Some(Token::greater_equal(span_end))
                } else {
                    Some(Token::greater(span))
                }
            }
            CHAR_SLASH => {
                if let Some(CHAR_SLASH) = ctx.peek_char() {
                    let (cmnt, span_end) = ctx.read_line();
                    Some(Token::comment(cmnt, span_end))
                } else {
                    Some(Token::slash(span))
                }
            }
            CHAR_DOUBLE_QUOTE => {
                if let Some((str, span_end)) = ctx.read_string() {
                    Some(Token::string(str, span_end))
                } else {
                    log::error!("We didnt finish the string");
                    panic!("We should finish the string")
                }
            }
            CHAR_NEWLINE => None,
            CHAR_WHITESPACE | CHAR_TAB | CHAR_CARRIAGE_RETURN => None,
            // Digit
            digit if is_digit(digit) => {
                if let Some((number, span_end)) = ctx.read_number(digit) {
                    Some(Token::number(number, span_end))
                } else {
                    log::error!("We didnt finish the number");
                    panic!("We should finish the number")
                }
            }
            // Alpha
            alpha if is_alpha(alpha) => {
                println!("start_span={}", ctx.span);
                let (ident, span) = ctx.read_identifier(alpha);
                println!("end_span={}", span);

                let srch = KEYWORDS.binary_search_by_key(&ident.as_str(), |&(k, _)| k);
                let token_value = match srch {
                    Ok(index) => KEYWORDS[index].1.clone(),
                    Err(_) => TokenValue::Ident(ident),
                };

                Some(Token::new(token_value, span))
            }
            unexpected => {
                log::error!("Unexpected char {}", unexpected);
                break;
            }
        };

        if let Some(token) = match_token {
            log::info!("found token={}", token);
            tokens.push(token);
        }
    }

    tokens.push(Token::new(TokenValue::Eof, ctx.span));
    tokens
}

/// The Lex context
pub struct LexContext<'a> {
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
    pub(crate) fn read_line(&mut self) -> (String, Span) {
        let mut buffer = String::new();

        while let Some(c) = self.source.next() {
            self.span.new_column();
            buffer.push(c);

            if c == CHAR_NEWLINE {
                self.span.new_line();
                break;
            }
        }

        (buffer, self.span)
    }

    /// Read the next character in the stream
    pub(crate) fn read_char(&mut self) -> Option<(char, Span)> {
        if let Some(c) = self.source.next() {
            self.span.new_column();
            if c == CHAR_NEWLINE {
                self.span.new_line();
            }

            Some((c, self.span))
        } else {
            None
        }
    }

    /// Read an identifier
    pub(crate) fn read_identifier(&mut self, first_alpha: char) -> (String, Span) {
        let mut buffer = format!("{}", first_alpha);

        while let Some(maybe_alphanumeric) = self.source.peek() {
            if is_alphanum(*maybe_alphanumeric) {
                buffer.push(*maybe_alphanumeric);
                self.read_char();
            } else {
                break;
            }
        }

        (buffer, self.span.extract())
    }

    /// Read a string
    pub(crate) fn read_string(&mut self) -> Option<(String, Span)> {
        let mut buffer = String::new();
        let mut string_terminated = false;

        while let Some(c) = self.source.next() {
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
            Some((buffer, self.span))
        } else {
            None
        }
    }

    pub(crate) fn read_number(&mut self, first_digit: char) -> Option<(f64, Span)> {
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
            Some((number, self.span))
        } else {
            None
        }
    }
}

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
