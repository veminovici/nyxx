//! The parser.
//!
//! # Grammar
//!
//! ```text
//! prog        → declaration* EOF ;
//! ```
//!
//! ## Statements
//!
//! We express "statement precedence" in production rules. Declaration
//! statements are not allowed everywhere other stmts are, so we have
//! to give them lower precedence, specifying them earlier in the
//! production rules. Places that disallow declarations will use the
//! later, higher-precedence rules only.
//!
//! ```text
//! declaration → varDecl
//!             → funDecl
//!             → classDecl
//!             | statement ;
//!
//! funDecl     → "fun" function ;
//! classDecl   → "class" IDENTIFIER "{" function* "}" ;
//! function    → IDENTIFIER "(" params? ")" block ;
//! params      → IDENTIFIER ( "," IDENTIFIER )* ;
//!
//! statement → exprStmt
//!           | ifStmt
//!           | forStmt
//!           | whileStmt
//!           | printStmt
//!           | returnStmt
//!           | block ;
//!
//! exprStmt  → expr ";" ;
//! ifStmt    → "if" "(" expression ")" statement ( "else" statement )? ;
//! forStmt   → "for" "(" ( varDecl | exprStmt | ";" )
//!                       expression? ";"
//!                       expression? ")" statement ;
//! whileStmt → "while" "(" expression ")" statement ;
//! printStmt → "print" expr ";" ;
//! returnStmt → "return" expression? ";" ;
//! block     → "{" declaration* "}" ;
//! ```
//!
//! ## Expressions
//!
//! We express op precedence in production rules.
//!
//! ```text
//! expression     → assign ;
//! assign         → ( call "." )? IDENTIFIER "=" assignment
//!                | logic_or;
//! logic_or       → logic_and ( "or" logic_and )* ;
//! logic_and      → equality ( "and" equality )* ;
//!
//! equality       → comparison ( ( "!=" | "==" ) comparison )* ;
//! comparison     → addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
//! addition       → multiplication ( ( "-" | "+" ) multiplication )* ;
//! multiplication → unary ( ( "/" | "*" ) unary )* ;
//! unary          → ( "!" | "-" ) unary
//!                | call ;
//! call           → primary ( "(" args? ")" | "." IDENTIFIER )* ;
//! primary        → "true" | "false" | "nil" | "this"
//!                | NUMBER | STRING | IDENTIFIER | "(" expr ")"
//!                | "super" "." IDENTIFIER ;
//!
//! args           → expression ( "," expression )* ;
//! ```
