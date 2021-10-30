# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Nyxx...Lexer...
A crate which exposes a lexer. The create is part of the **nyxx** project.

## Nyxx Lexer
The **nyxx-in** exposes the **Lexer** structure which can parse an input source string and return the list of tokens.
In order to do that, you need to get the **iter** from the lexer which will give you access to an **Iterator**.

```rust
let source = "var language=\n\"lox\";";
Lexer::with_source(source).for_each(|tkn| println!("{:?}", tkn));
```

The code above will return the below list of tokens. For each token you can see its type and the optional values attached to it (e.g. *STRING*) and its span (the line and column).

```
VAR @ [1:0..3]
WS( ) @ [1:3]
IDENT(language) @ [1:4..12]
EQUAL @ [1:12]
NEWLINE @ (1:13..2:0)
STRING(lox) @ [2:0..5]
SEMICOLON @ [2:5]
EOF @ [2:6..6]
```

## Nyxx Project
The **nyxx** project is an attempt to implement an interpreter. For more details on this project, please read its [readme](https://github.com/veminovici/nyxx/blob/main/README.md) file.
