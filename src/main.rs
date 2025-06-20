use axum::{Router, extract::Path, routing::get};
use rand::prelude::*;

type Book = String;

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
/// Should ideally be placed immediately in a database,
/// as this can be expensive in memory.
fn random_book(len: usize) -> Book {
    rand::rng()
        .random_iter()  // TODO: can we make this more efficient by having the randomizer generate
                        // chars within range instead of filtering?
        .filter(|c: &char| {
            c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace()
        })
        .take(len)
        .into_iter()
        .collect()
}

/// Locates a book and returns it to caller.
async fn find_book(id: usize) -> Book {
    match get_book_from_db(id).await {
        Some(book) => book,
        None => {
            claim_book(id).await;
            // TODO: do we want to leave the client waiting for gen, or do we want to return a
            // "hey, come back later"?
            let len = rand::random_range(0..4096);
            let book = random_book(len);
            insert_book_into_db(id, &book).await;
            book
        }
    }
}

/// Fetches a book with `id` from the database.
async fn get_book_from_db(id: usize) -> Option<Book> {
    todo!()
}

/// Places a `book` with `id` in database.
async fn insert_book_into_db(id: usize, book: &Book) {
    todo!()
}

/// With multiple clients, and a long time for generating books,
/// thread needs to claim book.
async fn claim_book(id: usize) {
    todo!()
}

