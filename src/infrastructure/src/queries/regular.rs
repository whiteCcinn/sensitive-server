use crate::models::{SensitiveRegular, NewSensitiveRegular};
use crate::schema::sensitive_regular;
use diesel::prelude::*;
use crate::Repo;
use diesel::result::Error;

pub fn insert(repo: &Repo, regular: NewSensitiveRegular) -> Result<SensitiveRegular, Error> {
    diesel::insert_into(sensitive_regular::table)
        .values(&regular)
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