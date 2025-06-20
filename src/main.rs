use axum::{Router, extract::Path, routing::get};
use rand::prelude::*;

#[tokio::main]
async fn main() {
    // TODO: set up db

    let app = Router::<()>::new().route("/book/{id}", get(get_book));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("was unable to bind to specified port");
    axum::serve(listener, app)
        .await
        .expect("something catastrophic happened while working");
}

/// Actual endpoint for getting the book to the user.
/// Gets the book text from `find_book`, then encodes as HTML,
/// then moves on.
async fn get_book(Path(id): Path<usize>) -> String {
    format!("book with ID: {}, \r\n{}", id, find_book(id).await)
}

/// Generates a random book.
/// Should ideally be placed immediately in a database, as this can be expensive.
fn random_book(len: usize) -> String {
    rand::rng()
        .random_iter()
        .filter(|c: &char| {
            c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace()
        })
        .take(len)
        .collect::<Vec<char>>()
        .into_iter()
        .collect()
}

/// Locates a book and returns it to caller.
/// TODO: interact with a database so we have the same book across different requests.
async fn find_book(id: usize) -> String {
    match get_book_from_db(id).await {
        Some(book) => book,
        None => {
            let len = rand::random_range(0..4096);
            let book = random_book(len);
            insert_book_into_db(id, &book).await;
            book
        }
    }
}

/// Fetches a book with `id` from the database.
async fn get_book_from_db(id: usize) -> Option<String> {
    todo!()
}

/// Places a `book` with `id` in database.
async fn insert_book_into_db(id: usize, book: &String) {
    todo!()
}

