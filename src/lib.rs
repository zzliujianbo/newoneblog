//use conf::Conf;
mod conf;
mod json;
mod md;
mod server;
mod str;
use log::info;

pub async fn run() -> Result<(), std::io::Error> {
    info!("newoneblog start");
    conf::Conf::init_by_file("conf.json");
    md::run().await;
    server::run().await
}
