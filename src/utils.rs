/// detect if the string is toml, xresources or something else
/// 0 - toml document
/// 1 - Xresources document
/// 9 - unrecognized
pub fn detect_string_type(s: &str) -> u8 {
    match toml::from_str::<toml::Value>(s) {
        Ok(_) => 0,
        Err(_) => {
            let cursor_regex =
                regex::Regex::new(r#".*cursorColor:\s*?(#[a-fA-F0-9]{6,8})\s?"#).unwrap();
            match cursor_regex.is_match(s) {
                true => 1,
                false => 9,
            }
        }
    }
}
