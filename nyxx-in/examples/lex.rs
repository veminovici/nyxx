use nyxx_in::*;

fn main() {
    let _ = env_logger::builder().try_init();

    let source = "var language=\n\"lox\";".to_string();
    println!("source=[{}]", source);
    println!();

    let lexer = Lexer::new(source);
    lexer.iter().for_each(|tkn| println!("token: {:?}", tkn));
}
