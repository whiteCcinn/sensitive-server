use crate::{DatabaseError};

#[derive(thiserror::Error, Debug)]
pub enum GetSensitiveRegularError {
    #[error("There is no regular by id {id:?}.")]
    RegularNotFound {
        id: i32,
        #[source]
        source: DatabaseError,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}