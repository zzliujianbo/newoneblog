//use conf::Conf;
mod conf;
mod json;
use log::{debug, info};

pub async fn run() -> Result<(), std::io::Error> {
    info!("newoneblog start");
    let conf = conf::Conf::new_by_file("conf.json");
    info!("conf: {:#?}", conf);
    Ok(())
}
