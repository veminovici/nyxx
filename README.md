# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Nyxx... 
A collection of rust crates for an interpreter.

## Lexer
The **nyxx-in** exposes the **Lexer** structure which can parse an input source string and return the list of tokens.
In order to do that, you need to get the **iter** from the lexer which will give you access to an **Iterator**.

```rust
let source = "var language=\n\"lox\";".to_string();
let lexer = Lexer::new(source);
lexer.iter().for_each(|tkn| println!("{:?}", tkn));
```
should return the following list of tokens:
```
VAR@(1:0-1:3)
WS( )@(1:3-1:4)
IDENT(language)@(1:4-1:12)
EQUAL@(1:12-1:13)
NEWLINE@(1:13-2:0)
STRING(lox)@(2:0-2:5)
SEMICOLON@(2:5-2:6)
EOF@(2:6-2:6)
```

## Build Status

[![Github Actions](https://buildstats.info/github/chart/veminovici/nyxx)](https://github.com/veminovici/nyxx)

## Resources & Credits
[Crafting interpreters](http://craftinginterpreters.com/)

## A *thank you* note !!!

> You can contact me at veminovici@hotmail.com. Code designed and written in Päädu, on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.
