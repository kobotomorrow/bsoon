use std::{env, error::Error, fs::File};

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
            let fetched_books = publisher::oreilly::fetch_books()?;
            let existed_books = csv_to_books()?;
            let mut new_books = vec![];
            for fetched_book in fetched_books {
                if !existed_books.contains(&fetched_book) {
                    new_books.push(fetched_book);
                }
            }

            if new_books.is_empty() {
                notify("今回は新しい書籍情報はありませんでした".to_string());
                return Ok(());
            }
            notify(format_json_text(&new_books));

            let all_books = [&existed_books[..], &new_books[..]].concat();
            write_books(&all_books)?;
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

fn notify(text: String) {
    let url = env::var("BSOON_WEBHOOK_URL").unwrap();
    let _ = ureq::post(&url).set(
        "Content-Type", "application/json"
    ).send_json(ureq::json!({
        "text": text
    }));
}

fn format_json_text(books: &Vec<Book>) -> String {
    let mut text = String::from("新しい書籍情報が追加されました\n");
    for book in books {
        text.push_str(&format!(
            "タイトル: {}\n著者: {}\nリンク: {}\n\n",
            book.title,
            book.author,
            book.link
        ));
    }
    text
}