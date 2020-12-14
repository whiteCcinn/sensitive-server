use crate::{DatabaseError};

#[derive(thiserror::Error, Debug)]
pub enum GetSensitiveWordsError {
    #[error("There is no words by id {id:?}.")]
    WordsNotFound {
        id: i32,
        #[source]
        source: DatabaseError,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}