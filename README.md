# Simplee..Nyxx...
A rust crate

## Run
You can run nyxx in two modes:

In the **prompt** mode you can introduce and interpret one line at a time.
```rust
RUST_LOG=trace target/debug/nyxx
```

In the **script** mode, you pass a source file.
```rust
RUST_LOG=trace target/debug/nyxx hello.lx
```

## Lexer
The **nyxx-in** exposes the **Lexer** structure which can parse an input source string and return the list of tokens.
In order to do that, you need to get the **iter** from the lexer which will give you access to an **Iterator**.

```rust
let source = "var language=\n\"lox\";".to_string();
let lexer = Lexer::new(source);
lexer.iter().for_each(|tkn| println!("{:?}", tkn));
```
shuld return the following list of tokens:
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

## Resources
[Crafting interpreters](http://craftinginterpreters.com/)
