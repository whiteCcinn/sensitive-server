#![allow(dead_code)]

use fake::fake;

pub mod generate;
pub mod test_db;

use sensitive_server_infrastructure::models::{NewSensitiveWords, SensitiveWords};
use sensitive_server_infrastructure::queries::{words};
use sensitive_server_infrastructure::Repo;

pub fn create_words(repo: &Repo, num_words: i32) -> Vec<SensitiveWords> {
    (0..num_words).map(|_| create_word(repo)).collect()
}

pub fn create_word(repo: &Repo) -> SensitiveWords {
    let new_word = NewSensitiveWords { words: fake!(Internet.user_name).to_string() };
    words::insert(&repo, new_word).expect("Failed to create word")
}
