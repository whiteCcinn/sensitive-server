mod helpers;

use helpers::test_db::get_test_repo;
use helpers::create_word;
use sensitive_server_infrastructure::queries::words;

#[test]
fn test_create_words() {
    let repo = get_test_repo();
    let word = create_word(&repo);
    let results = words::find(&repo, word.id);

    assert!(results.is_ok());
}