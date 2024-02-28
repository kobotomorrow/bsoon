use std::fs::read_to_string;
use roxmltree::Document;
use crate::MyResult;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub description: String,
    pub link: String,
    pub release_date: String,
}

pub fn fetch_books() -> MyResult<Vec<Book>> {
    // TODO: API call
    let text = read_to_string("./support/test.xml")?;
    let books = parse(text)?;
    Ok(books)
}

fn parse(xml_text: String) -> MyResult<Vec<Book>> {
    let doc = Document::parse(&xml_text)?;
    let mut titles = vec![];
    let mut authors = vec![];
    let mut descriptions = vec![];
    let mut links = vec![];
    let mut release_dates = vec![];
    for node in doc.descendants() {
        if node.has_tag_name("title") {
            titles.push(node.text().unwrap());
        }
        if node.has_tag_name("creator") {
            authors.push(node.text().unwrap());
        }
        if node.has_tag_name("description") {
            descriptions.push(node.text().unwrap());
        }
        if node.has_tag_name("link") {
            links.push(node.text().unwrap());
        }
        if node.has_tag_name("date") {
            release_dates.push(node.text().unwrap());
        }
    }

    let mut books = vec![];
    for i in 0..titles.len() {
        let book = Book {
            title: String::from(titles[i]),
            author: String::from(authors[i]),
            description: String::from(descriptions[i]),
            link: String::from(links[i]),
            release_date: String::from(release_dates[i]),
        };
        books.push(book);
    }
    Ok(books)
}
