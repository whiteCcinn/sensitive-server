mod helpers;

use helpers::test_db::get_test_repo;
use sensitive_server_infrastructure::queries::regular;
use crate::helpers::{
    create_regulars, create_regular,
};

#[test]
fn test_create_regular() {
    let repo = get_test_repo();
    let regular = create_regular(&repo);
    let results = regular::find(&repo, regular.id);

    assert!(results.is_ok());
}

#[test]
fn lists_regulars() {
    let repo = get_test_repo();
    let _regular = create_regulars(&repo, 5);
    let results = regular::fetch_all(&repo, Default::default()).expect("Failed to get words");

    assert_eq!(results.len(), 5);
}

#[test]
fn delete_regular() {
    let n = 5;
    let repo = get_test_repo();
    let regulars = create_regulars(&repo, n);

    let id = regulars[0].id;
    regular::delete(&repo, id).expect("Failed to delete word");

    let results = regular::fetch_all(&repo, Default::default()).expect("Failed to get words");

    assert_eq!(results.len() as i32, n - 1);
}