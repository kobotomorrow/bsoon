use std::error::Error;

use clap::{Parser, ValueEnum};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(ValueEnum, Debug, Clone)]
enum Publisher {
    #[value(alias="o")]
    Oreilly,
}

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub struct Input {
    /// Publisher
    #[arg(short='p', long="publisher")]
    entry_types: Publisher,
}

pub fn get_args() -> MyResult<Input> {
    Ok(Input::parse())
}

pub fn run(input: Input) -> MyResult<()> {
    println!("{:?}", input);
    Ok(())
}