use diesel::prelude::*;
use diesel::result::Error;

use crate::Repo;
use crate::models::{SensitiveWords, NewSensitiveWords};
use crate::schema::sensitive_words;

use domain::words::SensitiveWordQuery;

pub fn insert(repo: &Repo, word: NewSensitiveWords) -> Result<SensitiveWords, Error> {
    diesel::insert_into(sensitive_words::table)
        .values(&word)
        .get_result(&repo.conn())
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


pub fn fetch_all(repo: &Repo, query: SensitiveWordQuery) -> Result<Vec<SensitiveWords>, Error> {
    use crate::schema::sensitive_words::dsl::*;
    let q = sensitive_words
        .select(sensitive_words::all_columns())
        .into_boxed();

    let q = if let Some(a) = query.word {
        q.filter(words.eq(a))
    } else {
        q
    };

    q.load(&repo.conn())
}