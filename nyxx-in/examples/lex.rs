use nyxx_in::*;

fn main() {
    let _ = env_logger::builder().try_init();

    // let source = "var language=\n\"lox\";".to_string();
    // println!("source=[{}]", source);
    // println!();

    let lexer: Lexer = "var language=\n\"lox\";".to_string().into();
    lexer.iter().for_each(|tkn| println!("{:?}", tkn));
}
