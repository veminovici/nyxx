use std::iter::Peekable;
use std::str::Chars;

struct LexContext<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> LexContext<'a> {
    fn new(source: &'a str) -> LexContext<'a> {
        Self {
            source: source.chars().peekable(),
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.source.next()
    }
}

//
// Lex iterator
//

struct LexIter<'a> {
    ctx: LexContext<'a>,
}

impl<'a> Iterator for LexIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctx.read_char()
    }
}

//
// Lexer
//

struct Lexer {}

impl Lexer {
    fn iter(source: &str) -> LexIter {
        LexIter {
            ctx: LexContext::new(source),
        }
    }
}

fn main() {
    Lexer::iter("Hello world!").for_each(|c| println!("{:?}", c));
}
