#[macro_use]
extern crate diesel;

pub mod connection;
pub mod models;
pub mod schema;
pub mod queries;
pub mod repository;
pub mod shims;

use diesel::PgConnection;
pub use repository::Repository;

pub type Repo = connection::Repo<PgConnection>;