use crate::{
    conf::{Conf, CONF},
    str::remove_html_tag,
};
use crate::{md, str};
use chrono::{DateTime, Local, NaiveDateTime};
use log::{debug, error, info};
use serde::Serialize;
use std::process::Command;
use std::{
    fs::{self, read_to_string},
    path::{Path, MAIN_SEPARATOR_STR},
};
use tera::{Context, Tera};

/// markdown to html
pub async fn run() {
    let conf = CONF.get().unwrap();
    let public_path = Path::new(&conf.public_path);
    //判断public文件夹是否存在
    if !public_path.exists() {
        panic!("public_path not exists: {}", &conf.public_path);
    }

    let md_path = Path::new(&conf.markdown_path);
    //判断markdown文件夹是否存在
    if !md_path.exists() {
        panic!("markdown_path not exists: {}", &conf.markdown_path);
    }

    let tera = tera(&conf.template_path).unwrap();

    let mut context = Context::new();
    context.insert("title", &conf.title);
    context.insert("keywords", &conf.keywords);
    context.insert("description", &conf.description);
    context.insert("about_url", "/about.html");
    context.insert("footer_html", &conf.footer_html);
    context.insert(
        "build_md_time",
        &Local::now().format("%Y%m%d%H%M%S").to_string(),
    );
    let mut global_include_script = String::from("<script>");
    for path in &conf.global_include_script {
        match fs::read_to_string(path) {
            Ok(s) => global_include_script.push_str(&s),
            Err(e) => error!("read global_include_script error: {}", e),
        }
    }
    global_include_script.push_str("</script>");
    context.insert("global_include_script", &global_include_script);
    context.insert(
        "global_include_script_path",
        &conf.global_include_script_path,
    );

    let mut md_metas = handle_md(
        &conf.markdown_path,
        &conf.markdown_path,
        &conf.public_path,
        &tera,
        context.clone(),
    );
    md_metas.sort_by_key(|k| k.update_date);
    md_metas.reverse();

    //生成首页
    let mut index_context = context.clone();
    index_context.insert("md_metas", &md_metas);
    write_html(
        "index.html",
        &format!("{}/index.html", &conf.public_path),
        index_context,
        &tera,
    );

    //复制assets文件夹
    let assets_path = Path::new(&conf.template_path).join("assets");
    info!(
        "copy assets: {} --> {}",
        assets_path.to_str().unwrap(),
        conf.public_path
    );
    match fs_extra::dir::copy(
        assets_path,
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
}

/// 扫描并处理markdown文件
fn handle_md(
    md_path: &str,
    md_base_path: &str,
    public_path: &str,
    tera: &Tera,
    context: Context,
) -> Vec<MarkdownMetadata> {
    let mut md_metas: Vec<MarkdownMetadata> = Vec::new();
    let dir = match fs::read_dir(md_path) {
        Ok(dir) => dir,
        Err(e) => {
            error!("read_dir error: {}", e);
            return md_metas;
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
        let path_str = &path.to_str().unwrap().replace('\\', "/");
        let is_ignore = CONF
            .get()
            .unwrap()
            .ignore_markdown_path
            .iter()
            .any(|ignore_path| path_str.starts_with(ignore_path));
        if is_ignore {
            info!("ignore path: {}", path_str);
            continue;
        }
        debug!("for path: {}", path_str);
        let md_pinyin_relative_path = str::topinyin(&md_relative_path(path_str, md_base_path));
        let context = context.clone();
        if path.is_dir() {
            //创建文件夹
            create_html_dir(&md_pinyin_relative_path, public_path);
            let vec = handle_md(path_str, md_base_path, public_path, tera, context);
            md_metas.extend(vec);
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                let file = md_path_to_html_path(&md_pinyin_relative_path, public_path);
                if ext == "md" {
                    //md转html
                    let html_file = format!("{}.html", &file.trim_end_matches(".md"));
                    md_metas.push(md_to_html(
                        "content.html",
                        path_str,
                        &html_file,
                        context,
                        public_path,
                        tera,
                    ));
                } else {
                    //复制文件
                    info!("copy file: {} --> {}", path_str, file);
                    match fs::copy(path, &file) {
                        Ok(_) => {
                            info!("copy file success: {}", file);
                        }
                        Err(e) => {
                            error!("copy file error: {}", e);
                        }
                    }
                }
            }
        }
    }
    md_metas
}

/// 获取markdown文件相对路径
///
/// md_path: markdown实际文件路径
///
/// md_base_path: markdown文件夹根路径（配置中路径）
fn md_relative_path(md_path: &str, md_base_path: &str) -> String {
    md_path
        .trim_start_matches(md_base_path.trim_end_matches('/').trim_end_matches('\\'))
        .trim_start_matches('/')
        .trim_end_matches('/')
        // .trim_start_matches(MAIN_SEPARATOR_STR)
        // .trim_end_matches(MAIN_SEPARATOR_STR)
        .to_string()
}

fn create_html_dir(md_relative_dir: &str, public_path: &str) -> String {
    let html_dir = md_path_to_html_path(md_relative_dir, public_path);
    if !Path::new(&html_dir).exists() {
        fs::create_dir_all(&html_dir).unwrap();
    }
    html_dir
}

fn md_path_to_html_path(md_relative_path: &str, public_path: &str) -> String {
    format!(
        "{}/{}",
        public_path.trim_end_matches('/').trim_end_matches('\\'),
        //MAIN_SEPARATOR_STR,
        md_relative_path
    )
}

fn md_metadata<P: AsRef<Path>>(md_path: &P) -> Option<MarkdownMetadata> {
    let md = md_path.as_ref();
    let md_str = md.to_str()?;
    let file_metadata = fs::metadata(md).ok()?;
    let content = read_to_string(md).ok()?;
    let html_content = markdown::to_html_with_options(&content, &markdown::Options::gfm()).unwrap();
    //截取html内容的前500个字符作为描述
    let description = html_content
        .chars()
        .take(500)
        .collect::<String>()
        .replace("\n", "");
    // let date = match file_metadata.created() {
    //     Ok(date) => {
    //         let date: DateTime<Local> = date.into();
    //         Some(date.naive_local())
    //     }

    //     Err(e) => {
    //         error!("get file created time error: {}", e);
    //         None
    //     }
    // };

    //获取markdown文件最后的修改时间
    //https://blog.wayneshao.com/posts/9412.html
    //https://github.com/Dream4ever/Knowledge-Base/issues/69
    let conf = CONF.get().unwrap();
    let args = [
        "-C",
        &conf.markdown_path,
        "log",
        "-1",
        "--pretty=format:\"%aI\"",
        "--",
        &md_str
            .trim_start_matches(&conf.markdown_path)
            .trim_start_matches('/'),
    ];
    let git_date = Command::new("git").args(&args).output();
    debug!("{} --> git {:?} out_put:{:?}", md_str, args, git_date);
    let date = match git_date {
        Ok(data) => {
            let date = String::from_utf8(data.stdout).unwrap();
            debug!("{} --> git update_time: {}", md_str, date);
            if date.is_empty() {
                return None;
            }
            Some(
                DateTime::parse_from_rfc3339(&date.trim_matches('"'))
                    .unwrap()
                    .naive_local(),
            )
        }
        Err(e) => {
            error!("get file updated time error: {}", e);
            None
        }
    };

    Some(MarkdownMetadata {
        title: md.file_stem()?.to_str()?.to_string(),
        update_date: date,
        categories: Vec::new(),
        description: remove_html_tag(&description, "").to_string(),
        content: content,
        path: md.to_str()?.to_string(),
        html_url: "".to_string(),
        html_path: "".to_string(),
        html_content: html_content,
    })
}

fn tera(template_path: &str) -> Result<Tera, tera::Error> {
    tera::Tera::new(&format!(
        "{}/**/*.html",
        template_path.trim_end_matches('/')
    ))
}

fn md_to_html(
    template_name: &str,
    md_file: &str,
    html_file: &str,
    mut context: Context,
    public_path: &str,
    tera: &Tera,
) -> MarkdownMetadata {
    let md_metadata = md_metadata(&md_file)
        .unwrap()
        .html_path(html_file.to_string())
        .html_url(html_file.trim_start_matches(public_path).replace("\\", "/"));
    context.insert("md_title", &md_metadata.title);
    context.insert("md_content", &md_metadata.html_content);
    info!("render {} --> {}", md_file, html_file);
    write_html(template_name, &html_file, context, tera);
    md_metadata
}

fn write_html(template_name: &str, html_file: &str, context: Context, tera: &Tera) {
    info!("render html: {}", html_file);

    let content = match tera.render(template_name, &context) {
        Ok(content) => content,
        Err(e) => {
            error!("render html error: {}", e);
            return;
        }
    };

    match fs::write(html_file, content) {
        Ok(_) => {
            debug!("write html success: {}", html_file);
        }
        Err(e) => {
            error!("write html error: {}", e);
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MarkdownMetadata {
    ///标题
    pub title: String,
    ///时间
    pub update_date: Option<NaiveDateTime>,
    //pub tags: Vec<String>,
    ///分类
    pub categories: Vec<String>,
    ///描述
    pub description: String,
    ///内容
    pub content: String,
    ///文件路径
    pub path: String,
    ///访问URL
    pub html_url: String,
    ///html文件路径
    pub html_path: String,
    ///html内容
    pub html_content: String,
}
impl MarkdownMetadata {
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn update_date(mut self, update_time: Option<NaiveDateTime>) -> Self {
        self.update_date = update_time;
        self
    }

    pub fn categories(mut self, categories: Vec<String>) -> Self {
        self.categories = categories;
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    pub fn path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    pub fn html_url(mut self, html_url: String) -> Self {
        self.html_url = html_url;
        self
    }

    pub fn html_path(mut self, html_path: String) -> Self {
        self.html_path = html_path;
        self
    }

    pub fn html_content(mut self, html_content: String) -> Self {
        self.html_content = html_content;
        self
    }
}
