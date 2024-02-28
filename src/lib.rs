use std::{error::Error, fs::File, io::Write};

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

#[derive(Debug, PartialEq, Clone)]
struct Book {
    pub title: String,
    pub author: String,
    pub description: String,
    pub link: String,
    pub release_date: String,
}

pub fn get_args() -> MyResult<Input> {
    Ok(Input::parse())
}

pub fn run(input: Input) -> MyResult<()> {
    create_initial_file()?;
    match input.entry_types {
        Publisher::Oreilly => {
            let new_books = publisher::oreilly::fetch_books()?;
            let existed_books = csv_to_books()?;
            let mut books: Vec<Book> = existed_books.clone();
            for new_book in new_books {
                if !existed_books.contains(&new_book) {
                    books.push(new_book);
                }
            }
            write_books(&books)?;
        }
    }
    Ok(())
}

fn create_initial_file() -> MyResult<()> {
    let file = File::open("books.csv");
    if file.is_ok() {
        return Ok(());
    }
    File::create("books.csv")?;
    Ok(())
}

fn write_books(books: &Vec<Book>) -> MyResult<()> {
    let mut writer = csv::Writer::from_path("books.csv")?;
    write_header(&mut writer)?;
    for book in books {
        write_book(&mut writer, book)?;
    }
    writer.flush()?;
    Ok(())
}

fn write_header(writer: &mut csv::Writer<File>) -> MyResult<()> {
    writer.write_record(&["Publisher", "Title", "Author", "Description", "Link", "Date"])?;
    Ok(())
}

fn write_book(writer: &mut csv::Writer<File>, book: &Book) -> MyResult<()> {
    writer.write_record(&["Oreilly", &book.title, &book.author, &book.description, &book.link, &book.release_date])?;
    Ok(())
}

fn csv_to_books() -> MyResult<Vec<Book>> {
    let mut reader = csv::Reader::from_path("books.csv")?;
    let mut books = vec![];
    for result in reader.records() {
        let record = result?;
        let book = Book {
            title: record[1].to_string(),
            author: record[2].to_string(),
            description: record[3].to_string(),
            link: record[4].to_string(),
            release_date: record[5].to_string(),
        };
        books.push(book);
    }
    Ok(books)
}