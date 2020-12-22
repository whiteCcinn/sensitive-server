#[macro_use]
extern crate log;


use std::path::PathBuf;
use async_std::task::block_on;
use sensitive_server_web::configuration::Config;
use sensitive_server_web::app::get_app;

fn main() -> Result<(), std::io::Error> {
    let settings = Config::new(PathBuf::default()).expect("Failed to load configuration");

    info!("启动服务");

    // let state = Repository(Repo::new(&settings.database.connection_string()));
    let app = get_app();
    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );

    block_on(async {
        app.listen(address).await?;
        Ok(())
    })
    // Ok(())
}
