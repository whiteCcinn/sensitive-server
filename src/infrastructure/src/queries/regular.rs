use diesel::prelude::*;
use diesel::result::Error;

use crate::Repo;
use crate::models::{SensitiveRegular, NewSensitiveRegular};
use crate::schema::sensitive_regular;

use domain::regular::SensitiveRegularQuery;

pub fn insert(repo: &Repo, word: NewSensitiveRegular) -> Result<SensitiveRegular, Error> {
    diesel::insert_into(sensitive_regular::table)
        .values(&word)
        .get_result(&repo.conn())
}

pub fn delete(repo: &Repo, pid: i32) -> Result<(), Error> {
    use crate::schema::sensitive_regular::dsl::{sensitive_regular, id};

    diesel::delete(sensitive_regular.filter(id.eq(pid)))
        .execute(&repo.conn())
        // Discard the number of deleted rows
        .map(|_| ())
}

pub fn find(repo: &Repo, pid: i32) -> Result<SensitiveRegular, Error> {
    use crate::schema::sensitive_regular::dsl::{sensitive_regular, id};

    sensitive_regular
        .filter(id.eq(pid))
        .select(sensitive_regular::all_columns())
        .first(&repo.conn())
}


pub fn fetch_all(repo: &Repo, query: SensitiveRegularQuery) -> Result<Vec<SensitiveRegular>, Error> {
    use crate::schema::sensitive_regular::dsl::*;
    let q = sensitive_regular
        .select(sensitive_regular::all_columns())
        .into_boxed();

    let q = if let Some(a) = query.regulars {
        q.filter(regulars.eq(a))
    } else {
        q
    };

    q.load(&repo.conn())
}