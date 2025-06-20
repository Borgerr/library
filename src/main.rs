use axum::{
    Router,
    extract::{Path, State},
    routing::get,
};
use dashmap::DashSet;
use lazy_static::lazy_static;
use rand::prelude::*;
use sqlx::PgPool;
use tokio::time::{Duration, sleep};

use std::sync::Arc;

lazy_static! {
    static ref GENERATING_IDS: Arc<DashSet<i64>> = Arc::new(DashSet::new());
}

type Book = String;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;
    //sqlx::migrate!().run(&pool).await?;

    let app = Router::new()
        .route("/book/{id}", get(get_book))
        .with_state(pool);
    // TODO: possibly change URL depending on config
    let addr = format!(
        "{}:{}",
        &dotenvy::var("LIBMGR_ADDR")?,
        &dotenvy::var("LIBMGR_PORT")?
    );
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("was unable to bind to specified port");
    axum::serve(listener, app)
        .await
        .expect("something catastrophic happened while working");

    Ok(())
}

/// Actual endpoint for getting the book to the user.
/// Gets the book text from `find_book`, then encodes as HTML,
/// then moves on.
async fn get_book(Path(id): Path<i64>, State(pool): State<PgPool>) -> String {
    // TODO: use maud to encode in HTML, or feed into some frontend
    // I'd currently prefer to do the former
    // but that depends on how the style comes out
    format!("book with ID: {}, \r\n{}", id, find_book(id, pool).await)
}

/// Locates a book and returns it to caller.
/// Generates and inserts into database if necessary.
async fn find_book(id: i64, pool: PgPool) -> Book {
    // TODO: add env vars for max book size and how long to sleep per loop iteration?
    match get_book_from_db(id, &pool).await {
        Some(book) => book,
        None => {
            if can_claim_book(id) {
                // TODO: do we want to leave the client waiting for gen,
                // or do we want to return a "hey, come back later"?
                GENERATING_IDS.insert(id);
                let len = rand::random_range(0..(1 << 22));
                let book = random_book(len);
                insert_book_into_db(id, &book, &pool).await;
                GENERATING_IDS.remove(&id);
                book
            } else {
                while !can_claim_book(id) {
                    sleep(Duration::from_millis(10)).await
                }
                get_book_from_db(id, &pool)
                    .await
                    .expect("book should be in DB by now")
            }
        }
    }
}

/// Called when book not present in database and we need to claim the gen for it.
/// With multiple clients, and a long time for generating books,
/// thread needs to claim book.
/// Returns a bool saying whether or not we can claim a book with id.
fn can_claim_book(id: i64) -> bool {
    !GENERATING_IDS.contains(&id)
}

/// Generates a random book.
/// Should ideally be placed immediately in a database,
/// as this can be expensive in memory.
fn random_book(len: usize) -> Book {
    rand::rng()
        .random_iter() // TODO: can we make this more efficient by having the randomizer generate
        // chars within range instead of filtering?
        .filter(|c: &char| {
            c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace()
        })
        .take(len)
        .into_iter()
        .collect()
}

/*
 * --------------------------------------------------------------------------------
 * DATABASE FUNCTIONS
 * --------------------------------------------------------------------------------
 */
use sqlx::Row;

/// Fetches a book with `id` from the database.
async fn get_book_from_db(id: i64, pool: &PgPool) -> Option<Book> {
    let res = sqlx::query("SELECT words FROM books WHERE book_id == $1")
        .bind(id)
        .fetch_one(pool)
        .await;
    match res {
        Ok(book) => book.get("words"),
        Err(_) => None,
    }
}

/// Places a `book` with `id` in database.
async fn insert_book_into_db(id: i64, book: &Book, pool: &PgPool) {
    sqlx::query("INSERT INTO books VALUES ($1, $2);")
        .bind(id)
        .bind(book)
        .execute(pool)
        .await
        .expect("double check insertion query");
}
