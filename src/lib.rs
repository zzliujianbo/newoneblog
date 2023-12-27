//use conf::Conf;
mod conf;
mod json;
mod md;
mod server;
use log::info;

pub async fn run() -> Result<(), std::io::Error> {
    info!("newoneblog start");
    let conf = conf::Conf::new_by_file("conf.json");
    info!("conf: {:#?}", conf);
    md::to_html(&conf).await;
    server::run(&conf).await;
    Ok(())
}
