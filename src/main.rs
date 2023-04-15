mod ast_printer;
mod error;
mod expr;
mod scanner;
mod token;
mod util;

use std::io::Write;

use scanner::Scanner;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let arg_len = std::env::args().len();
    if arg_len > 2 {
        panic!("Usage: jlox [script]");
    } else if arg_len == 2 {
        let file_name = std::env::args().nth(1).unwrap();
        println!("{file_name}");
        run_file(&file_name).expect("Failed to interpret");
    } else {
        println!("Prompt...");
        run_prompt().expect("Failed to interpret");
    }
}

pub fn run_file(file: &str) -> Result<()> {
    let contents = std::fs::read_to_string(file)?;
    run(contents.as_str())?;
    Ok(())
}

pub fn run_prompt() -> Result<()> {
    let mut buf = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().expect("Failed to flush");
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line.");
        if buf == "\n" {
            break;
        }
        run(buf.as_str())?;
        buf.clear();
    }
    Ok(())
}

pub fn run(line: &str) -> Result<()> {
    let mut scanner = Scanner::new(line.to_string());
    let tokens = scanner.scan_tokens()?;
    tokens.iter().for_each(|t| {
        println!("{}", t);
    });
    Ok(())
}
