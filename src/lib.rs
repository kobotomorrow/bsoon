use std::error::Error;

use clap::{Parser, ValueEnum};

mod publisher;

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
    match input.entry_types {
        Publisher::Oreilly => {
            let response = publisher::oreilly::fetch()?;
            let books = publisher::oreilly::parse(response)?;
            for book in books {
                println!("Title: {}", book.title);
                println!("Author: {}", book.author);
                println!("Description: {}", book.description);
                println!("Link: {}", book.link);
                println!("Release Date: {}", book.release_date);
                println!();
            }
        }
    }
    Ok(())
}
