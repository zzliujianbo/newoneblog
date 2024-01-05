use std::{fmt::format, fs::File, path::Path};

use crate::conf::Conf;
use log::{error, info, debug};
use pinyin::{to_pinyin_vec, Pinyin};
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
        context.insert("content", "123");
        let r = templates.render("content.html", &context);
        match r {
            Ok(s) => {
                let md_path = Path::new(md_file);
                //获取文件路径
                let file_name = md_path.file_name();
                let file_name = match file_name {
                    Some(s) => s.to_str().unwrap(),
                    None => {
                        error!("file_name is None");
                        return;
                    }
                };
                debug!("file_name:{}", file_name);
                //中文转拼音
                let pinyin = to_pinyin_vec(file_name, Pinyin::plain);
                let pinyin = pinyin.join("");
                debug!("pinyin:{}", pinyin);
                let mut html = Path::new(&conf.public_path).join(md_path.parent().unwrap());
                html.set_file_name(format!("{}.html", pinyin));
                let html_path_str = html.to_str().unwrap();
                let Ok(f) = File::create(html_path_str) else {
                    error!("create file error");
                    return;
                };

                info!("f:{}", html_path_str);
            }
            Err(e) => {
                info!("e:{}", e);
            }
        }
    });

    Ok(())
}
