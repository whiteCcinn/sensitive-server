#[macro_use]
extern crate log;
extern crate log4rs;

pub mod controller;
pub mod configuration;
pub mod app;


use domain::words::Repository;

/// The shared state of our application.
/// It's generic with respect to the actual implementation of the repository:
/// this enables swapping different implementations, both for production usage
/// or ease of testing (mocks and stubs).
pub struct Context<R: 'static + Repository + Sync + Send> {
    pub repository: R,
}