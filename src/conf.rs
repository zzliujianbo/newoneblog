use super::json;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Conf {
    pub server_ip: String,
    pub server_port: u16,
    pub title: String,
    pub keyword: String,
    pub markdown_path: String,
    pub template_path: String,
    pub public_path: String,
    pub about_md_path: String,
}

impl Conf {
    pub fn new_by_file(file_path: &str) -> Self {
        let conf: Conf = json::read_from_file(file_path).unwrap();
        conf
    }
}
