//use conf::Conf;
mod conf;
mod json;
mod md;
mod server;
mod str;
use log::info;

pub async fn run() -> Result<(), std::io::Error> {
    info!("newoneblog start");
    let conf = conf::Conf::new_by_file("conf.json");
    info!("conf: {:#?}", conf);
    md::run(&conf).await;
    server::run(&conf).await;
    Ok(())
}
