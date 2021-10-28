use nyxx_in::*;

fn main() {
    let source = "var language=\"lox\";";
    let tokens = lex(source);
    println!("tokens={:?}", tokens);
}
