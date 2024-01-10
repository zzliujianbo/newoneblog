use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf, MAIN_SEPARATOR_STR},
};

use crate::conf::Conf;
use crate::str;
use log::{debug, error, info};
use tera::{self, Context, Tera};

/// markdown to html
pub async fn run(conf: &Conf) {
    let md_paths = scan_mdfiles(&conf.markdown_path);
    let _ = handle_mdfiles(md_paths, conf).await;
}

/// 扫描markdown文件
pub fn scan_mdfiles(md_base_path: &str) -> Vec<PathBuf> {
    //递归获取markdown文件
    let mut md_paths = Vec::new();
    let dir = match fs::read_dir(md_base_path) {
        Ok(dir) => dir,
        Err(e) => {
            error!("read_dir error: {}", e);
            return md_paths;
        }
    };
    for entry in dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                error!("read_dir entry error: {}", e);
                continue;
            }
        };
        let path = entry.path();
        if path.is_dir() {
            let vec = scan_mdfiles(path.to_str().unwrap());
            md_paths.extend(vec);
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    md_paths.push(path);
                }
            }
        }
    }
    md_paths
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
        "{}{}",
        Path::new(&conf.markdown_path)
            .to_str()
            .unwrap()
            .trim_end_matches('/'),
        MAIN_SEPARATOR_STR
    );
    //生成html文件
    md_paths.iter().for_each(|md_path| {
        let md_path_str = md_path.to_str().unwrap();
        let (pinyin_filename, md_content) = match get_md_data(md_path) {
            Some((pinyin_filename, md_content)) => (pinyin_filename, md_content),
            None => {
                error!("get_md_data error: {}", md_path_str);
                return;
            }
        };
        info!("{} --> pinyin_filename: {}", md_path_str, pinyin_filename);
        let mut html_path =
            Path::new(&conf.public_path).join(md_path_str.trim_start_matches(&md_base_path));
        html_path.set_file_name(format!("{}.html", pinyin_filename));
        let html_file_path = str::topinyin(html_path.to_str().unwrap());
        info!("{} --> map html: {}", md_path_str, html_file_path);

        let mut context = Context::new();
        let html_content = markdown::to_html(&md_content);
        context.insert("title", &conf.title);
        context.insert("keyword", &conf.keyword);
        context.insert("content", &html_content);
        write_html(context, "content.html", &html_file_path, &tera);
    });
    //生成首页
    let mut context = Context::new();
    context.insert("title", &conf.title);
    context.insert("keyword", &conf.keyword);
    //context.insert("content", &html_content);
    write_html(
        context,
        "index.html",
        &format!("{}/index.html", &conf.public_path),
        &tera,
    );

    //复制assets文件夹
    let assets_path = Path::new(&conf.template_path).join("assets");
    let assets_path_str = assets_path.to_str().unwrap();
    // let public_assets_path = Path::new(&conf.public_path);
    // let public_assets_path_str = public_assets_path.to_str().unwrap();
    info!("copy assets: {} --> {}", assets_path_str, conf.public_path);
    //fs::copy(assets_path_str, &conf.public_path).unwrap();
    match fs_extra::dir::copy(
        assets_path_str,
        &conf.public_path,
        &fs_extra::dir::CopyOptions::new().overwrite(true),
    ) {
        Ok(_) => {
            info!("copy assets success");
        }
        Err(e) => {
            error!("copy assets error: {}", e);
        }
    }
    Ok(())
}

pub fn get_md_data(md_file: &Path) -> Option<(String, String)> {
    Some((
        str::topinyin(md_file.file_stem()?.to_str()?),
        read_to_string(md_file).ok()?,
    ))
}

pub fn write_html(context: Context, template_name: &str, html_file: &str, tera: &Tera) {
    info!("render html: {}", html_file);

    let content = match tera.render(template_name, &context) {
        Ok(content) => content,
        Err(e) => {
            error!("render error: {}", e);
            return;
        }
    };
    debug!("html_file_path: {}, content: {}", html_file, content);
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
