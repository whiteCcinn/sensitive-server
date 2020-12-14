#[macro_use]
extern crate log;

use std::path::PathBuf;
// pub mod web;

// use async_std::task::block_on;
// use domain::{connection::Repo, Repository};
// // use domain::{connection::Repo};

use sensitive_server_web::*;

// use sensitive_server_web::web::configuration::Config;
// use sensitive_server_web::domain;
// //use web::get_app;

fn main() -> Result<(), std::io::Error> {
    let settings = web::configuration::Config::new(PathBuf::default()).expect("Failed to load configuration");

    info!("启动服务");

    domain::connection::A();
    // let state = Repository(Repo::new(&settings.database.connection_string()));
//    let app = get_app(state);
//    let address = format!(
//        "{}:{}",
//        settings.application.host, settings.application.port
//    );
//
//    block_on(async {
//        app.listen(address).await?;
//        Ok(())
//    })
    Ok(())
}
