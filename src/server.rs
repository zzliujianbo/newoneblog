use log::info;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};

use crate::conf::{Conf, CONF};

pub async fn run() -> Result<(), std::io::Error> {
    let conf = CONF.get().unwrap();
    let addr = format!("{}:{}", conf.server_ip, conf.server_port);
    info!("Starting server {} ...", addr);
    let app = Route::new().nest(
        "/",
        StaticFilesEndpoint::new(conf.public_path.clone()).show_files_listing(), // Use the cloned public_path
    );
    Server::new(TcpListener::bind(addr)).run(app).await
}
