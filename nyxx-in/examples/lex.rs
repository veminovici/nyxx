use nyxx_in::*;

fn main() {
    let _ = env_logger::builder().try_init();

    let source = "var language=\n\"lox\";".to_string();
    println!("source=[{}]", source);
    println!();

    let tokens = lex(&source);
    println!("tokens={:?}", tokens);
}
