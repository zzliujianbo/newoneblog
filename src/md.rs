use std::fmt::format;

use crate::conf::Conf;
use log::info;
use tera::{self, Context};

/// markdown to html
pub async fn to_html(conf: &Conf) {
    let md_files = scan_md(&conf.markdown_path).await;
    gen_html(md_files, conf).await;
}

/// 扫描markdown文件
pub async fn scan_md(md_path: &str) -> Vec<String> {
    vec!["newoneblog/readme.md".to_string()]
}

/// 生成html文件
pub async fn gen_html(md_files: Vec<String>, conf: &Conf) -> Result<(), tera::Error> {
    let tera = tera::Tera::new(&format!(
        "{}/**/*",
        conf.template_path.trim_end_matches('/')
    ));

    let templates = match tera {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    };

    //读取markdown文件
    md_files.iter().for_each(|md_file| {
        info!("md_file: {}", md_file);
        let mut context = Context::new();
        context.insert("content", "");
        let r = templates.render("content.html", &context);
        match r {
            Ok(s) => {
                info!("s:{}", s);
            }
            Err(e) => {
                info!("e:{}", e);
            }
        }
    });

    Ok(())
}
