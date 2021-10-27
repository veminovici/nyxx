use std::io::{self, Write};

fn main() {
    let _ = env_logger::builder().try_init();
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len().cmp(&1) {
        std::cmp::Ordering::Equal => run_file(args[0].clone()),
        std::cmp::Ordering::Greater => {
            eprintln!("usage: nyxx-in [script]");
            std::process::exit(64);
        }
        _ => run_prompt(),
    }
}

fn run_file(file_path: String) {
    let buffer = std::fs::read_to_string(&file_path).expect("Failed to read file");
    run(&buffer);
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        stdin
            .read_line(&mut buffer)
            .expect("Failed to read from console");

        let script = buffer.trim();
        if script.starts_with(":q") {
            break;
        }

        run(script);
        buffer.clear();
    }
}

fn run(script: &str) {
    eprintln!("running script [{}]", script);
}
