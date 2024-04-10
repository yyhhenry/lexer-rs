use anyhow::Result;
use clap::Parser;
use logos::Logos;

#[derive(thiserror::Error, Debug, PartialEq, Clone, Default)]
enum LexingError {
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Unknown character")]
    #[default]
    UnknownCharacter,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexingError)]
#[logos(skip r"\s+")]
enum Token {
    #[regex(r"[a-zA-Z][a-zA-Z0-9]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(i64),
}

#[derive(Parser)]
struct Args {
    /// The input file
    input_file: String,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let input = std::fs::read_to_string(&args.input_file)?;
    let mut lexer = Token::lexer(&input);
    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => {
                println!("{:?}", token);
            }
            Err(e) => {
                eprintln!("Error: {:?} {:?}", e, lexer.slice());
            }
        };
    }
    Ok(())
}
