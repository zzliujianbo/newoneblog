use super::json;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Conf {
    /// 服务器IP
    pub server_ip: String,
    /// 服务器端口
    pub server_port: u16,
    /// 网站标题
    pub title: String,
    /// 网站关键字
    pub keyword: String,
    /// md文件存放路径
    pub markdown_path: String,
    /// 网站模板路径
    pub template_path: String,
    /// md转换为html文件存放路径
    pub public_path: String,
    /// 关于我们md文件存放路径
    pub about_md_path: String,
}

impl Conf {
    pub fn new_by_file(file_path: &str) -> Self {
        let conf: Conf = json::read_from_file(file_path).unwrap();
        conf
    }
}
