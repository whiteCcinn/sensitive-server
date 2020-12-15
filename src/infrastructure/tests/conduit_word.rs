mod helpers;

use helpers::test_db::get_test_repo;
use helpers::{create_word};
use sensitive_server_infrastructure::queries::words;
use crate::helpers::create_words;

#[test]
fn test_create_word() {
    let repo = get_test_repo();
    let word = create_word(&repo);
    let results = words::find(&repo, word.id);

    assert!(results.is_ok());
}

#[test]
fn lists_words() {
    let repo = get_test_repo();
    let _word = create_words(&repo, 5);
    let results = words::fetch_all(&repo, Default::default()).expect("Failed to get words");

    assert_eq!(results.len(), 5);
}

#[test]
fn delete_words() {
    let n = 5;
    let repo = get_test_repo();
    let words = create_words(&repo, n);

    let id = words[0].id;
    words::delete(&repo, id).expect("Failed to delete word");

    let results = words::fetch_all(&repo, Default::default()).expect("Failed to get words");

    assert_eq!(results.len() as i32, n - 1);
}