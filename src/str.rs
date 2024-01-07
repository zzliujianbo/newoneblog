use pinyin::ToPinyin;

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
