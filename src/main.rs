use rand::prelude::*;

fn main() {
    for _ in 0..5 {
        let len = rand::random_range(0..4096);
        let book = random_book(len);
        println!("book: {}", book);
        println!("size: {}", book.chars().count());
    }
}

fn random_book(len: usize) -> String {
    rand::rng()
        .random_iter()
        .filter(|c: &char| c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace())
        .take(len)
        .collect::<Vec<char>>()
        .into_iter()
        .collect()
}

