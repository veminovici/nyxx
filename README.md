# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Nyxx... 
A collection of rust crates for an interpreter.

</br>

[![CI Pipeline](https://github.com/veminovici/nyxx/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/veminovici/nyxx/actions/workflows/ci.yml)
[![Last commit](https://img.shields.io/github/last-commit/veminovici/nyxx)](https://github.com/veminovici/nyxx)
[![Repo size](https://img.shields.io/github/repo-size/veminovici/nyxx)](https://github.com/veminovici/nyxx)

</br>

## Lexer
The **nyxx-in** exposes the **Lexer** structure which can parse an input source string and return the list of tokens.
In order to do that, you need to get the **iter** from the lexer which will give you access to an **Iterator**.

```rust
let source = "var language=\n\"lox\";".to_string();
let lexer = Lexer::new(source);
lexer.iter().for_each(|tkn| println!("{:?}", tkn));
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

## Build Status

[![Github Actions](https://buildstats.info/github/chart/veminovici/nyxx)](https://github.com/veminovici/nyxx)

## Resources & Credits
[Crafting interpreters](http://craftinginterpreters.com/)
[Test coverage](https://vladfilippov.com/blog/rust-code-coverage-tools/)

## Contact

> You can contact me at veminovici@hotmail.com. Code designed and written in Päädu, on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.
