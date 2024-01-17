use regex::Regex;
use std::borrow::Cow;
use std::sync::OnceLock;

use pinyin::ToPinyin;

use regex::Replacer;

macro_rules! impl_regex_replace {
    ($re_name:ident,$re_str:expr) => {
        /// 正则表达式验证
        ///
        #[doc = concat!("正则表达式替换：`", stringify!($re_str), "`")]
        pub fn $re_name<'h, R: Replacer>(text: &'h str, rep: R) -> Cow<'h, str> {
            static RE: OnceLock<Regex> = OnceLock::new();
            RE.get_or_init(|| Regex::new($re_str).unwrap())
                .replace_all(text, rep)
        }
    };
}

pub fn topinyin(str: &str) -> String {
    let mut new_str = String::new();
    for f in str.chars() {
        let pinyin = f.to_pinyin();
        if let Some(pinyin) = pinyin {
            new_str.push_str(pinyin.plain());
        } else {
            new_str.push(f);
        }
    }
    new_str
}

impl_regex_replace!(remove_html_tag, r"<[^>]+>");
