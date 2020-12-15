#![allow(dead_code)]

use fake::fake;

pub mod generate;
pub mod test_db;

use sensitive_server_infrastructure::models::{
    NewSensitiveWords, SensitiveWords,
    NewSensitiveRegular, SensitiveRegular,
};
use sensitive_server_infrastructure::queries::{words, regular};
use sensitive_server_infrastructure::Repo;

pub fn create_words(repo: &Repo, num_words: i32) -> Vec<SensitiveWords> {
    (0..num_words).map(|_| create_word(repo)).collect()
}

pub fn create_word(repo: &Repo) -> SensitiveWords {
    let new_word = NewSensitiveWords { words: fake!(Internet.user_name).to_string() };
    words::insert(&repo, new_word).expect("Failed to create word")
}

pub fn create_regulars(repo: &Repo, num_words: i32) -> Vec<SensitiveRegular> {
    (0..num_words).map(|_| create_regular(repo)).collect()
}

pub fn create_regular(repo: &Repo) -> SensitiveRegular {
    let new_regular = NewSensitiveRegular { regulars: fake!(Internet.user_name).to_string() };
    regular::insert(&repo, new_regular).expect("Failed to create word")
}