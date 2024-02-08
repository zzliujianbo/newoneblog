use std::sync::OnceLock;

use super::json;
use serde::Deserialize;

pub static CONF: OnceLock<Conf> = OnceLock::new();

#[derive(Deserialize, Debug)]
pub struct Conf {
    /// 服务器IP
    pub server_ip: String,
    /// 服务器端口
    pub server_port: u16,
    /// 网站标题
    pub title: String,
    /// 网站关键字
    pub keywords: String,
    /// 网站描述
    pub description: String,
    /// md文件存放路径
    pub markdown_path: String,
    /// 网站模板路径
    pub template_path: String,
    /// md转换为html文件存放路径
    pub public_path: String,
    /// 忽略的 markdown 文件或者路径
    pub ignore_markdown_path: Vec<String>,
    /// 底部html代码
    pub footer_html: String,
}

impl Conf {
    pub fn init_by_file(file_path: &str) {
        let conf: Conf = Self::new_by_file(file_path);
        CONF.set(conf);
    }

    pub fn new_by_file(file_path: &str) -> Self {
        let conf: Conf = json::read_from_file(file_path).unwrap();
        conf
    }
}
