use std::{
    fmt::format,
    fs::{self, read_to_string, File},
    io::Read,
    path::{Path, PathBuf},
};

use crate::conf::Conf;
use crate::str;
use log::{debug, error, info};
use tera::{self, Context, Tera};

/// markdown to html
pub async fn run(conf: &Conf) {
    let md_paths = scan_mdfiles(&conf.markdown_path).await;
    handle_mdfiles(md_paths, conf).await;
}

/// 扫描markdown文件
pub async fn scan_mdfiles(md_base_path: &str) -> Vec<PathBuf> {
    vec![Path::new("./markdown/newoneblog/readme.md").to_path_buf()]
}

/// 生成html文件
pub async fn handle_mdfiles(md_paths: Vec<PathBuf>, conf: &Conf) -> Result<(), tera::Error> {
    let tera = match tera::Tera::new(&format!(
        "{}/**/*",
        conf.template_path.trim_end_matches('/')
    )) {
        Ok(tera) => tera,
        Err(e) => {
            error!("tera error, {}", e);
            return Err(e);
        }
    };

    let md_base_path = format!(
        "{}/",
        Path::new(&conf.markdown_path)
            .to_str()
            .unwrap()
            .trim_end_matches('/')
    );
    //生成html文件
    md_paths.iter().for_each(|md_path| {
        let (pinyin_filename, md_content) = match get_md_data(md_path) {
            Some((pinyin_filename, md_content)) => (pinyin_filename, md_content),
            None => {
                error!("get_md_data error: {}", md_path.to_str().unwrap());
                return;
            }
        };
        info!(
            "{} --> pinyin_filename: {}",
            md_path.to_str().unwrap(),
            pinyin_filename
        );
        let mut html_path = Path::new(&conf.public_path)
            .join(md_path.to_str().unwrap().trim_start_matches(&md_base_path));
        html_path.set_file_name(format!("{}.html", pinyin_filename));
        let html_file_path = html_path.to_str().unwrap();
        info!(
            "{} --> map html: {}",
            md_path.to_str().unwrap(),
            html_file_path
        );
        write_html(markdown::to_html(&md_content), html_file_path, &tera);
    });

    Ok(())
}

pub fn get_md_data(md_file: &Path) -> Option<(String, String)> {
    Some((
        str::topinyin(md_file.file_stem()?.to_str()?),
        read_to_string(md_file).ok()?,
    ))
}

pub fn write_html(html_content: String, html_file: &str, tera: &Tera) {
    info!("render html: {}", html_file);
    let mut context = Context::new();
    context.insert("content", &html_content);
    let content = match tera.render("content.html", &context) {
        Ok(content) => content,
        Err(e) => {
            error!("render error: {}", e);
            return;
        }
    };
    debug!("html_file_path:{}, content: {}", html_file, content);
    //判断文件夹是否存在
    let html_file_path = Path::new(html_file);
    if !html_file_path.parent().unwrap().exists() {
        fs::create_dir_all(Path::new(html_file).parent().unwrap()).unwrap();
    }

    match fs::write(html_file, content) {
        Ok(_) => {
            debug!("write html success: {}", html_file);
        }
        Err(e) => {
            error!("write html error: {}", e);
        }
    }
}
