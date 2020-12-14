use crate::models::{SensitiveWords, NewSensitiveWords};
use crate::schema::sensitive_words;
use diesel::prelude::*;
use crate::Repo;
use diesel::result::Error;

// pub fn insert(repo: &Repo, word: NewSensitiveWords) -> Result<SensitiveWords, Error> {
pub fn insert(repo: &Repo, word: NewSensitiveWords) -> QueryResult<usize> {
    // diesel::insert_into(sensitive_words::table)
    //     .values(&word)
    //     .get_result(&repo.conn())

    diesel::insert_into(sensitive_words::table).default_values().execute(&repo.conn())
}

pub fn delete(repo: &Repo, pid: i32) -> Result<(), Error> {
    use crate::schema::sensitive_words::dsl::{sensitive_words, id};

    diesel::delete(sensitive_words.filter(id.eq(pid)))
        .execute(&repo.conn())
        // Discard the number of deleted rows
        .map(|_| ())
}

pub fn find(repo: &Repo, pid: i32) -> Result<SensitiveWords, Error> {
    use crate::schema::sensitive_words::dsl::{sensitive_words, id};

    sensitive_words
        .filter(id.eq(pid))
        .select(sensitive_words::all_columns())
        .first(&repo.conn())
}