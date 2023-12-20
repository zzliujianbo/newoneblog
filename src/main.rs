use newoneblog;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    newoneblog::run().await
}
